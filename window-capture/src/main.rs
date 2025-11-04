mod window_finder;
mod capture;
mod app;

use eframe::egui;
use app::WindowCaptureApp;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("RuneLauncher - Window Capture"),
        ..Default::default()
    };

    eframe::run_native(
        "Window Capture",
        options,
        Box::new(|cc| Ok(Box::new(WindowCaptureApp::new(cc)))),
    )
}
