use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use base64::Engine;
use openidconnect::core::{CoreAuthPrompt, CoreAuthenticationFlow, CoreClient, CoreErrorResponseType, CoreGenderClaim, CoreIdToken, CoreIdTokenClaims, CoreIdTokenVerifier, CoreProviderMetadata, CoreResponseType};
use openidconnect::{ClientId, CsrfToken, EmptyAdditionalClaims, EndpointMaybeSet, EndpointNotSet, EndpointSet, IdToken, Nonce, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, StandardErrorResponse};
use reqwest::redirect::Policy;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_json::map::Values;
use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder, WindowEvent};
use tokio::io::AsyncBufReadExt;
use url::Url;
use uuid::Uuid;
use crate::error;
use crate::error::Error;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

const OIDC_DISCOVERY_URL: &str = "https://account.jagex.com/.well-known/openid-configuration";
const AUTH_CODE_CLIENT_ID: &str = "com_jagex_auth_desktop_launcher";
const AUTH_CODE_REDIRECT_URI: &str = "https://secure.runescape.com/m=weblogin/launcher-redirect";
const AUTH_CODE_SCOPE: &str = "openid offline gamesso.token.create user.profile.read user.entitlement.read user.game.read user.sku.read user.voucher.redeem";

#[derive(Debug, Clone)]
pub struct AuthFlow {
    pub client: CoreClient<
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointMaybeSet,
        EndpointMaybeSet
    >,
    pub authorization_request_url: String,
    pub challenge: PkceCodeChallenge,
    pub verifier: String,
    pub csrf_token: CsrfToken,
    pub nonce: Nonce,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub id_token: String,
    pub scope: String,
    pub token_type: String
}

#[derive(Debug, Clone)]
pub struct GameSession {
    pub code: String,
    pub id_token: String,
    pub state: String,
    pub session_id: String
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct GameCharacter {
    pub account_id: String,
    pub display_name: String,
    pub user_hash: String,
    pub is_members: bool
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Account {
    pub email: String,
    pub account_name: String,
    pub characters: Vec<GameCharacter>
}

pub async fn begin_login() -> error::Result<AuthFlow> {
    let provider_metadata = provider_metadata().await?;

    let client_id = ClientId::new(AUTH_CODE_CLIENT_ID.to_string());
    let redirect_url = RedirectUrl::new(AUTH_CODE_REDIRECT_URI.to_string())?;
    let client = CoreClient::from_provider_metadata(provider_metadata, client_id, None)
        .set_redirect_uri(redirect_url);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let scopes = AUTH_CODE_SCOPE.split(" ").collect::<Vec<&str>>()
        .into_iter()
        .map(|v| Scope::new(v.to_string()))
        .collect::<Vec<Scope>>();

    let (auth_url, csrf_token, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::Hybrid(vec![
                CoreResponseType::Code
            ]),
            CsrfToken::new_random,
            Nonce::new_random
        )
        .set_pkce_challenge(pkce_challenge.clone())
        .add_scopes(scopes)
        .url();

    let flow = AuthFlow {
        client,
        authorization_request_url: auth_url.to_string(),
        challenge: pkce_challenge,
        verifier: pkce_verifier.into_secret(),
        csrf_token,
        nonce
    };
    Ok(flow)
}

pub async fn oauth_token(
    flow: AuthFlow,
    code: String,
    state: String
) -> error::Result<OAuthToken> {
    let http_client = tauri_plugin_http::reqwest::ClientBuilder::new().redirect(Policy::none()).build()?;

    if flow.csrf_token.into_secret() != state {
        return Err(Error::Reason("Provided state from OAuth flow does not match the CSRF token.".to_string()));
    }

    let params = &[
        ("client_id", flow.client.client_id().to_string()),
        ("redirect_uri", flow.client.redirect_uri().unwrap().to_string()),
        ("grant_type", "authorization_code".to_string()),
        ("code", code),
        ("code_verifier", flow.verifier.to_string())
    ];

    let response = http_client
        .post(flow.client.token_uri().unwrap().as_str())
        .form(&params)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_data = response.json::<StandardErrorResponse<CoreErrorResponseType>>().await?;
        log::error!("Failed to get oauth token response from OAuth. Error: {}", error_data);
        return Err(Error::Reason("Failed to get oauth token response from OAuth.".to_string()));
    }

    let auth_token = response.json::<OAuthToken>().await?;
    Ok(auth_token)
}

async fn provider_metadata() -> error::Result<CoreProviderMetadata> {
    let http_client = tauri_plugin_http::reqwest::ClientBuilder::new().build()?;
    let metadata: CoreProviderMetadata = http_client.get(OIDC_DISCOVERY_URL).send().await?.json().await?;

    Ok(metadata)
}

pub async fn authorize<R: Runtime>(app_handle: AppHandle<R>, flow: AuthFlow) -> error::Result<(String, String)> {
    log::info!("Starting OAuth authorization. Opening authorization window popup.");

    let (tx, mut rx) = tokio::sync::oneshot::channel::<error::Result<(String, String)>>();
    let tx = Arc::new(Mutex::new(Some(tx)));
    
    if let Some(auth_window) = &app_handle.get_webview_window("auth") {
        let _ = auth_window.close()?;
        let _ = auth_window.destroy()?;
        tokio::time::sleep(Duration::from_millis(250)).await;
    }

    let main_window = &app_handle.get_webview_window("main").unwrap();
    let main_window_pos: (i32, i32) = (main_window.inner_position()?.x, main_window.inner_position()?.y);
    let main_window_size: (u32, u32) = (main_window.inner_size()?.width, main_window.inner_size()?.height);

    let window = WebviewWindowBuilder::new(
        &app_handle,
        "auth",
        WebviewUrl::External(flow.authorization_request_url.parse()?)
    )
        .title("Login with Jagex Account")
        .inner_size(480., 700.)
        .position((main_window_pos.0 as f64 + main_window_size.0 as f64)  + 32f64, main_window_pos.1 as f64)
        .center()
        .focused(true)
        .on_navigation(move |url| {
            let url_str = url.as_str().to_string();
            if url_str.to_lowercase().starts_with(&flow.client.redirect_uri().unwrap().as_str().to_lowercase()) {
                let code = url.query_pairs()
                    .find(|(k, _)| k == "code")
                    .map(|(_, v)| v.to_string());

                let state = url.query_pairs()
                    .find(|(k, _)| k == "state")
                    .map(|(_, v)| v.to_string());

                if let Ok(mut tx_guard) = tx.lock() {
                    if let Some(sender) = tx_guard.take() {
                        return match (code, state) {
                            (Some(code), Some(state)) => {
                                let _ = sender.send(Ok((code, state)));
                                false
                            }
                            _ => {
                                log::error!("Invalid redirect URL: {}", url_str);
                                let _ = sender.send(Err(Error::InvalidRedirectUrl(url_str)));
                                false
                            }
                        }
                    }
                }
            }

            true
        })
        .build()?;

    let result = rx.await
        .map_err(|_| Error::Reason("OAuth flow was cancelled!".to_string()))?;
    if result.is_ok() {
        // Close the window.
        {
            window.close()?;
        }
        log::info!("Finished OAuth authorization. Closing authorization window popup.");

        Ok(result?)
    } else {
        log::warn!("OAuth authorization cancelled.");
        Err(result.unwrap_err())
    }
}

pub async fn game_session<R: Runtime>(
    app_handle: AppHandle<R>,
    flow: AuthFlow,
    oauth_token: OAuthToken
) -> error::Result<GameSession> {
    let session_id_url = get_session_id_request_url(flow, oauth_token.clone()).await?;

    let (tx, mut rx) = tokio::sync::oneshot::channel::<error::Result<String>>();
    let tx = Arc::new(Mutex::new(Some(tx)));

    let window = WebviewWindowBuilder::new(
        &app_handle,
        "auth_session_id",
        WebviewUrl::External(session_id_url)
    )
        .title("Fetching Session Id")
        .skip_taskbar(true)
        .center()
        .inner_size(500., 500.)
        .visible(false)
        .on_navigation(move |url| {
            let url_str = url.as_str().to_string();
            if url_str.starts_with("http://localhost") {
                if let Ok(mut tx_guard) = tx.lock() {
                    if let Some(sender) = tx_guard.take() {
                        let _ = sender.send(Ok(url_str));
                        return false;
                    }
                }
            }

            true
        })
        .build()?;

    let result_url = rx.await
        .map_err(|_| Error::Reason("Failed to get session id.".to_string()))??;

    let mut data = parse_query_params(result_url.as_str());

    // Build the session id request

    let http_client = tauri_plugin_http::reqwest::ClientBuilder::new().redirect(Policy::none()).build()?;

    let params = json!({
        "idToken": &data["id_token"]
    });

    let response = http_client
        .post("https://auth.jagex.com/game-session/v1/sessions")
        .header("content-type", "application/json")
        .json(&params)
        .send().await?;

    if !response.status().is_success() {
        let error_data = response.json::<StandardErrorResponse<CoreErrorResponseType>>().await?;
        log::error!("Failed to get session id response from Jagex. Error: {}", error_data);
        return Err(Error::Reason("Failed to get session id response from Jagex.".to_string()));
    }

    let response_json = response.text().await?.to_string();
    let json = serde_json::from_str::<Value>(response_json.as_str().trim())?;

    data.insert("session_id".to_string(), json.get("sessionId".to_string()).unwrap().to_string());

    let game_session = GameSession {
        code: data["code"].to_string(),
        state: data["state"].to_string(),
        id_token: data["id_token"].to_string(),
        session_id: data["session_id"].to_string()
    };

    {
        let _ = window.close()?;
    }

    Ok(game_session)
}

fn parse_query_params(query: &str) -> HashMap<String, String> {
    query
        .replace("http://localhost/#", "")
        .split("&")
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, "=");
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => {
                    let decoded_key = urlencoding::decode(key).ok()?.into_owned();
                    let decoded_value = urlencoding::decode(value).ok()?.into_owned();
                    Some((decoded_key, decoded_value))
                }
                _ => None
            }
        }).collect()
}

async fn get_session_id_request_url(flow: AuthFlow, oauth_token: OAuthToken) -> error::Result<Url> {
    let client_id = ClientId::new("1fddee4e-b100-4f4e-b2b0-097f9088f9d2".to_string());
    let provider_metadata = provider_metadata().await?;
    let client = CoreClient::from_provider_metadata(provider_metadata, client_id, None)
        .set_redirect_uri(RedirectUrl::new("http://localhost".to_string())?);

    let scopes = &[
        "openid",
        "offline"
    ];
    let scopes = scopes.iter().map(|s| Scope::new(s.to_string())).collect::<Vec<Scope>>();

    let (result_url, csrf_token, nonce) = client.authorize_url(
        CoreAuthenticationFlow::Hybrid(vec![
            CoreResponseType::Code,
            CoreResponseType::IdToken
        ]),
        CsrfToken::new_random,
        Nonce::new_random
    )
        .add_scopes(scopes)
        .add_extra_param("id_token_hint", oauth_token.id_token.to_string())
        .url();

    Ok(result_url)
}

fn decode_jwt_claims_unverified(token: &str) -> error::Result<serde_json::Value> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(Error::Reason("Invalid JWT format".to_string()));
    }

    // Decode the payload (second part)
    let payload = parts[1];

    // Add padding if necessary for base64 decoding
    let padded_payload = match payload.len() % 4 {
        2 => format!("{}==", payload),
        3 => format!("{}=", payload),
        _ => payload.to_string(),
    };

    // Decode base64url
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(padded_payload.as_bytes())
        .map_err(|e| Error::Reason(format!("Failed to decode base64: {:?}", e)))?;

    // Parse JSON
    let claims: serde_json::Value = serde_json::from_slice(&decoded)?;
    Ok(claims)
}


pub async fn characters(
    session: GameSession
) -> error::Result<Vec<GameCharacter>> {
    let http_client = tauri_plugin_http::reqwest::ClientBuilder::new().redirect(Policy::none()).build()?;

    let response = http_client
        .get("https://auth.jagex.com/game-session/v1/accounts")
        .bearer_auth(session.session_id.replace("\"", ""))
        .send()
        .await?;

    if !response.status().is_success() {
        let error_data = response.json::<StandardErrorResponse<CoreErrorResponseType>>().await?;
        log::error!("Failed to get characters response from Jagex. Error: {}", error_data);
        return Err(Error::Reason("Failed to get characters response from Jagex.".to_string()));
    }

    let json = response.text().await?.to_string();
    let json = serde_json::from_str::<Value>(json.as_str().trim())?;

    let mut results = Vec::<GameCharacter>::new();
    for entry in json.as_array().unwrap().iter() {
        let account_id = entry["accountId"].to_string().replace("\"", "");
        let display_name = entry["displayName"].to_string().replace("\"", "");
        let user_hash = entry["userHash"].to_string().replace("\"", "");
        let is_members = false;
        results.push(GameCharacter {
            account_id,
            display_name,
            user_hash,
            is_members
        });
    }

    Ok(results)
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountInfo {
    pub nickname: String,
    pub display_name: String,
    pub id: String,
    pub user_id: String,
    pub email: Option<String>
}

pub async fn account_info(
    flow: AuthFlow,
    oauth_token: OAuthToken
) -> error::Result<AccountInfo> {
    let client = flow.client;
    let id_token = CoreIdToken::from_str(&oauth_token.id_token.as_str())?;
    let id_token_verifier = client.id_token_verifier().insecure_disable_signature_check();
    let claims: &CoreIdTokenClaims = match id_token.claims(&id_token_verifier, &flow.nonce) {
        Ok(claims) => claims,
        Err(e) => {
            log::error!("Failed to get claims from id token: {:?}", e);
            return Err(Error::Reason("Failed to get claims from id token.".to_string()));
        }
    };
    let claims_json = serde_json::to_value(&claims)?;

    let sub = claims_json["sub"].to_string().replace("\"", "");
    let nickname = claims_json["nickname"].to_string().replace("\"", "");

    log::debug!("AccountID: {} - Sub: {}", nickname, sub);

    let http_client = tauri_plugin_http::reqwest::ClientBuilder::new().redirect(Policy::none()).build()?;
    let response = http_client.get(format!("https://api.jagex.com/v1/users/{}/displayName", sub))
        .bearer_auth(&oauth_token.access_token)
        .send().await?;

    if !response.status().is_success() {
        let error_data = response.json::<StandardErrorResponse<CoreErrorResponseType>>().await?;
        log::error!("Failed to get account info response from Jagex. Error: {}", error_data);
        return Err(Error::Reason("Failed to get account info response from Jagex.".to_string()));
    }

    let response_json = response.text().await?.to_string();
    let response_json = serde_json::from_str::<Value>(response_json.as_str().trim())?;

    let display_name = response_json["displayName"].to_string().replace("\"", "");
    let id = response_json["id"].to_string().replace("\"", "");
    let user_id = response_json["userId"].to_string().replace("\"", "");

    Ok(AccountInfo {
        nickname,
        display_name,
        id,
        user_id,
        email: None
    })
}