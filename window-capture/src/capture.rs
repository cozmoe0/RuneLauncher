#[cfg(windows)]
use windows::Win32::Foundation::HWND;
#[cfg(windows)]
use windows::Win32::Graphics::Gdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetWindowDC, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP, HDC, SRCCOPY};
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;
use anyhow::Result;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct WindowCapture {
    #[cfg(windows)]
    hwnd: HWND,
    #[cfg(not(windows))]
    hwnd: isize,
    #[cfg(windows)]
    #[allow(dead_code)]
    last_frame: Arc<Mutex<Option<CapturedFrame>>>,
    #[cfg(not(windows))]
    last_frame: Arc<Mutex<Option<CapturedFrame>>>,
    // Cached DC and bitmap for reuse
    #[cfg(windows)]
    cached_dc: Arc<Mutex<Option<CachedDCContext>>>,
}

#[cfg(windows)]
struct CachedDCContext {
    hdc_mem: HDC,
    hbitmap: HBITMAP,
    width: u32,
    height: u32,
}

#[derive(Clone)]
pub struct CapturedFrame {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl WindowCapture {
    #[cfg(windows)]
    pub fn new(hwnd: HWND) -> Self {
        Self {
            hwnd,
            last_frame: Arc::new(Mutex::new(None)),
            cached_dc: Arc::new(Mutex::new(None)),
        }
    }

    #[cfg(not(windows))]
    pub fn new(hwnd: isize) -> Self {
        Self {
            hwnd,
            last_frame: Arc::new(Mutex::new(None)),
        }
    }

    #[cfg(windows)]
    fn get_or_create_dc_context(&self, width: u32, height: u32, hdc_window: HDC) -> Result<CachedDCContext> {
        let mut cached = self.cached_dc.lock();

        // Check if we can reuse the cached context
        if let Some(ctx) = cached.as_ref() {
            if ctx.width == width && ctx.height == height {
                return Ok(CachedDCContext {
                    hdc_mem: ctx.hdc_mem,
                    hbitmap: ctx.hbitmap,
                    width: ctx.width,
                    height: ctx.height,
                });
            } else {
                // Dimensions changed, cleanup old context
                unsafe {
                    DeleteObject(ctx.hbitmap);
                    DeleteDC(ctx.hdc_mem);
                }
            }
        }

        // Create new context
        unsafe {
            let hdc_mem = CreateCompatibleDC(hdc_window);
            if hdc_mem.is_invalid() {
                anyhow::bail!("Failed to create compatible DC");
            }

            let hbitmap = CreateCompatibleBitmap(hdc_window, width as i32, height as i32);
            if hbitmap.is_invalid() {
                DeleteDC(hdc_mem);
                anyhow::bail!("Failed to create compatible bitmap");
            }

            let ctx = CachedDCContext {
                hdc_mem,
                hbitmap,
                width,
                height,
            };

            *cached = Some(CachedDCContext {
                hdc_mem: ctx.hdc_mem,
                hbitmap: ctx.hbitmap,
                width: ctx.width,
                height: ctx.height,
            });

            Ok(ctx)
        }
    }

    #[cfg(windows)]
    pub fn capture_frame(&self) -> Result<CapturedFrame> {
        unsafe {
            let mut rect = std::mem::zeroed();
            GetClientRect(self.hwnd, &mut rect)?;

            let width = (rect.right - rect.left) as u32;
            let height = (rect.bottom - rect.top) as u32;

            if width == 0 || height == 0 {
                anyhow::bail!("Window has zero dimensions");
            }

            let hdc_window = GetWindowDC(self.hwnd);
            if hdc_window.is_invalid() {
                anyhow::bail!("Failed to get window DC");
            }

            // Use cached DC context for better performance
            let dc_ctx = self.get_or_create_dc_context(width, height, hdc_window)?;

            let old_bitmap = SelectObject(dc_ctx.hdc_mem, dc_ctx.hbitmap);

            // Capture the window
            BitBlt(
                dc_ctx.hdc_mem,
                0,
                0,
                width as i32,
                height as i32,
                hdc_window,
                0,
                0,
                SRCCOPY,
            )?;

            // Get bitmap data
            let mut bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width as i32,
                    biHeight: -(height as i32), // Negative for top-down bitmap
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [std::mem::zeroed(); 1],
            };

            let buffer_size = (width * height * 4) as usize;
            let mut buffer = vec![0u8; buffer_size];

            let result = GetDIBits(
                dc_ctx.hdc_mem,
                dc_ctx.hbitmap,
                0,
                height,
                Some(buffer.as_mut_ptr() as *mut _),
                &mut bmi,
                DIB_RGB_COLORS,
            );

            // Cleanup (but keep DC cached)
            SelectObject(dc_ctx.hdc_mem, old_bitmap);
            ReleaseDC(self.hwnd, hdc_window);

            if result == 0 {
                anyhow::bail!("GetDIBits failed");
            }

            // Convert BGRA to RGBA
            for chunk in buffer.chunks_exact_mut(4) {
                chunk.swap(0, 2); // Swap B and R
            }

            let frame = CapturedFrame {
                width,
                height,
                data: buffer,
            };

            *self.last_frame.lock() = Some(frame.clone());

            Ok(frame)
        }
    }

    #[cfg(not(windows))]
    pub fn capture_frame(&self) -> Result<CapturedFrame> {
        anyhow::bail!("Window capture is only supported on Windows")
    }

    #[allow(dead_code)]
    pub fn get_last_frame(&self) -> Option<CapturedFrame> {
        self.last_frame.lock().clone()
    }

    #[cfg(windows)]
    #[allow(dead_code)]
    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }

    #[cfg(not(windows))]
    #[allow(dead_code)]
    pub fn hwnd(&self) -> isize {
        self.hwnd
    }
}

impl Drop for WindowCapture {
    fn drop(&mut self) {
        #[cfg(windows)]
        {
            // Cleanup cached resources
            if let Some(ctx) = self.cached_dc.lock().take() {
                unsafe {
                    DeleteObject(ctx.hbitmap);
                    DeleteDC(ctx.hdc_mem);
                }
            }
        }
        log::info!("WindowCapture dropped for HWND: {:?}", self.hwnd);
    }
}
