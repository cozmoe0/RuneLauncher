use tauri::{App, Runtime};
use crate::error;

pub fn build_tauri_app<R: Runtime>(builder: tauri::Builder<R>) -> error::Result<App<R>> {
    Ok(builder.build(tauri::generate_context!())?)
}
