use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use anyhow::anyhow;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use jsonwebtoken::jwk::Jwk;
use log::info;
use serde_json::Value;
use tauri::{Emitter, Manager, Runtime, WebviewUrl, WebviewWindowBuilder};
use crate::error;
use crate::error::Error;
use crate::state::auth;
use crate::state::auth::game_session;
use base64::{Engine as _, engine::general_purpose};
use rsa::{RsaPublicKey, BigUint};
use rsa::pkcs1::{EncodeRsaPublicKey, LineEnding};

#[tauri::command]
pub async fn login<R: Runtime>(
    app_handle: tauri::AppHandle<R>
) -> error::Result<()> {
    let flow = auth::begin_login().await?;
    
    // Emit event to update button text to "Authorizing..."
    app_handle.emit("login-progress", "Authorizing...")?;
    let (code, state) = auth::authorize(app_handle.clone(), flow.clone()).await?;
    
    // Emit event to update button text to "Getting Token..."
    app_handle.emit("login-progress", "Getting Token...")?;
    let oauth_token = auth::oauth_token(flow.clone(), code.clone(), state.clone()).await?;
    
    // Emit event to update button text to "Creating Session..."
    app_handle.emit("login-progress", "Getting Session...")?;
    let game_session = auth::game_session(app_handle.clone(), flow.clone(), oauth_token.clone()).await?;

    app_handle.emit("login-progress", "Getting Characters...")?;
    let characters = auth::characters(game_session.clone()).await?;

    // Create account with character data

    let account_info = auth::account_info(flow.clone(), oauth_token.clone()).await?;
    let account = auth::Account {
        email: account_info.nickname, // TODO: Extract from OAuth token
        account_name: account_info.display_name, // TODO: Extract from session data
        characters: characters.clone()
    };

    // Emit event with account data
    app_handle.emit("account-added", account)?;
    
    // Emit event to restore button text to original state
    app_handle.emit("login-complete", "")?;
    
    log::debug!("Game Session: {:?}", game_session);

    Ok(())
}