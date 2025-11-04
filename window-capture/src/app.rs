use eframe::egui;
use egui::{ColorImage, TextureHandle, TextureOptions};
use crate::window_finder::{find_jag_render_view, WindowInfo};
use crate::capture::{WindowCapture, CapturedFrame};
use std::time::{Duration, Instant};

pub struct WindowCaptureApp {
    capture: Option<WindowCapture>,
    texture: Option<TextureHandle>,
    target_window: Option<WindowInfo>,
    status_message: String,
    last_capture_time: Instant,
    capture_interval: Duration,
    fps_counter: FpsCounter,
    auto_refresh: bool,
    show_stats: bool,
}

struct FpsCounter {
    frame_times: Vec<Instant>,
    last_fps: f32,
}

impl FpsCounter {
    fn new() -> Self {
        Self {
            frame_times: Vec::new(),
            last_fps: 0.0,
        }
    }

    fn record_frame(&mut self) {
        let now = Instant::now();
        self.frame_times.push(now);

        // Keep only frames from the last second
        self.frame_times.retain(|time| now.duration_since(*time) < Duration::from_secs(1));

        if self.frame_times.len() > 1 {
            self.last_fps = self.frame_times.len() as f32;
        }
    }

    fn get_fps(&self) -> f32 {
        self.last_fps
    }
}

impl WindowCaptureApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            capture: None,
            texture: None,
            target_window: None,
            status_message: String::from("Ready to capture"),
            last_capture_time: Instant::now(),
            capture_interval: Duration::from_millis(16), // ~60 FPS
            fps_counter: FpsCounter::new(),
            auto_refresh: true,
            show_stats: true,
        }
    }

    fn find_and_attach_window(&mut self) {
        match find_jag_render_view() {
            Some(window_info) => {
                #[cfg(windows)]
                {
                    let hwnd = window_info.get_hwnd();
                    self.capture = Some(WindowCapture::new(hwnd));
                }
                #[cfg(not(windows))]
                {
                    self.capture = Some(WindowCapture::new(window_info.hwnd));
                }

                self.target_window = Some(window_info.clone());
                self.status_message = format!(
                    "Attached to window: {} ({})",
                    window_info.title, window_info.class_name
                );
                log::info!("Attached to window: {:?}", window_info);
            }
            None => {
                self.capture = None;
                self.target_window = None;
                self.status_message = String::from("No JagRenderView window found");
                log::warn!("No JagRenderView window found");
            }
        }
    }

    fn capture_frame(&mut self, ctx: &egui::Context) {
        if let Some(capture) = &self.capture {
            match capture.capture_frame() {
                Ok(frame) => {
                    self.update_texture(ctx, frame);
                    self.fps_counter.record_frame();
                    self.status_message = format!(
                        "Capturing at {:.1} FPS",
                        self.fps_counter.get_fps()
                    );
                }
                Err(e) => {
                    self.status_message = format!("Capture error: {}", e);
                    log::error!("Capture error: {}", e);
                }
            }
        }
    }

    fn update_texture(&mut self, ctx: &egui::Context, frame: CapturedFrame) {
        let color_image = ColorImage::from_rgba_unmultiplied(
            [frame.width as usize, frame.height as usize],
            &frame.data,
        );

        if let Some(texture) = &mut self.texture {
            texture.set(color_image, TextureOptions::LINEAR);
        } else {
            self.texture = Some(ctx.load_texture(
                "captured_window",
                color_image,
                TextureOptions::LINEAR,
            ));
        }
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("🔍 Find Window").clicked() {
                self.find_and_attach_window();
            }

            ui.separator();

            ui.checkbox(&mut self.auto_refresh, "Auto Refresh");

            if !self.auto_refresh && ui.button("📸 Capture Frame").clicked() {
                self.capture_frame(ui.ctx());
            }

            ui.separator();

            ui.label("FPS Target:");
            let mut fps_target = 1000.0 / self.capture_interval.as_millis() as f32;
            if ui.add(egui::Slider::new(&mut fps_target, 1.0..=144.0).suffix(" fps")).changed() {
                self.capture_interval = Duration::from_millis((1000.0 / fps_target) as u64);
            }

            ui.separator();

            ui.checkbox(&mut self.show_stats, "Show Stats");
        });
    }

    fn render_status(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Status:");
            ui.label(&self.status_message);
        });

        if let Some(window_info) = &self.target_window {
            ui.horizontal(|ui| {
                ui.label("Target:");
                ui.monospace(&window_info.title);
                ui.label(format!("(HWND: 0x{:X})", window_info.hwnd));
            });
        }

        if self.show_stats {
            ui.horizontal(|ui| {
                ui.label(format!("Capture FPS: {:.1}", self.fps_counter.get_fps()));
                ui.separator();
                if let Some(texture) = &self.texture {
                    ui.label(format!(
                        "Resolution: {}x{}",
                        texture.size()[0],
                        texture.size()[1]
                    ));
                }
            });
        }
    }

    fn render_captured_window(&self, ui: &mut egui::Ui) {
        if let Some(texture) = &self.texture {
            let available_size = ui.available_size();
            let texture_size = texture.size_vec2();

            // Calculate aspect ratio preserving scale
            let scale_x = available_size.x / texture_size.x;
            let scale_y = available_size.y / texture_size.y;
            let scale = scale_x.min(scale_y);

            let display_size = texture_size * scale;

            ui.centered_and_justified(|ui| {
                ui.image((texture.id(), display_size));
            });
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("No window captured yet. Click 'Find Window' to start.");
            });
        }
    }
}

impl eframe::App for WindowCaptureApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Continuous repaint for smooth updates
        ctx.request_repaint();

        // Auto-capture if enabled and interval elapsed
        if self.auto_refresh
            && self.capture.is_some()
            && self.last_capture_time.elapsed() >= self.capture_interval
        {
            self.capture_frame(ctx);
            self.last_capture_time = Instant::now();
        }

        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.add_space(5.0);
            self.render_controls(ui);
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);
            self.render_status(ui);
            ui.add_space(5.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_captured_window(ui);
        });
    }
}
