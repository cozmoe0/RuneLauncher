use std::io;
use std::ops::Deref;
use openidconnect::core::CoreErrorResponseType;
use serde::{Serialize, Serializer};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error: {0}")]
    Reason(String),

    #[error("Redirect URL does not contain the expected parameters: {0}")]
    InvalidRedirectUrl(String),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    TauriError(#[from] tauri::Error),
    
    #[error(transparent)]
    TauriPlugin(#[from] tauri::plugin::BuilderError),

    #[error(transparent)]
    TauriHttp(#[from] tauri_plugin_http::reqwest::Error),

    #[error("{}: {}", _0.error(),_0.error_description().cloned().unwrap_or_default())]
    Auth(openidconnect::StandardErrorResponse<CoreErrorResponseType>),

    #[error(transparent)]
    AuthUrlParse(#[from] openidconnect::url::ParseError),
    
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    
    #[error(transparent)]
    JWTError(#[from] jsonwebtoken::errors::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}