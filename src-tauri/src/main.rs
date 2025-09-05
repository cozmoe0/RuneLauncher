// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::Level;
use tauri::Manager;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri_plugin_log::fern;
use crate::runner::build_tauri_app;
use crate::commands::auth_commands;

mod runner;
pub mod error;
pub mod commands;
pub mod state;

#[tokio::main]
async fn main() { 
    let builder = tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            handle.plugin(tauri_plugin_opener::init())?;
            handle.plugin(tauri_plugin_fs::init())?;
            handle.plugin(tauri_plugin_http::init())?;

            #[cfg(feature = "devtools")]
            {
                handle.plugin(tauri_plugin_devtools::init())?;
                handle.plugin(tauri_plugin_devtools_app::init())?;
            }

            #[cfg(feature = "logging")]
            {
                use tauri_plugin_log::{Target, TargetKind};
                use tauri_plugin_log::fern::colors::{Color, ColoredLevelConfig};

                let logger = tauri_plugin_log::Builder::new()
                    .level_for("runelauncher", Level::Trace.to_level_filter())
                    .level_for("reqwest", Level::Info.to_level_filter())
                    .with_colors(ColoredLevelConfig::default()
                        .trace(Color::BrightBlue)
                        .debug(Color::BrightMagenta)
                        .info(Color::BrightGreen)
                        .warn(Color::Yellow)
                        .error(Color::BrightRed)
                    ).build();
                handle.plugin(logger)?;
            }

            handle.plugin(tauri_plugin_single_instance::init(|app, argv, _cmd| {
                log::info!("SingleInstance plugin: Second instance triggered with args: {:?}", argv);
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }))?;
            handle.plugin(tauri_plugin_dialog::init())?;

            // --- Initialize System Tray ---
            let show_item = MenuItem::with_id(app, "show", "Open Rune Launcher", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(false)
                .tooltip("Rune Launcher")
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {

                    }
                    "quit" => {

                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {

                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            auth_commands::login,
        ]);

    log::trace!("Trace level");
    log::debug!("Debug level");
    log::info!("Info level");
    log::warn!("Warn level");
    log::error!("Error level");

    build_tauri_app(builder)
        .expect("Failed to run application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                log::info!("Exit requested default to allow async tasks to finish if any.");
            }
        });
}
