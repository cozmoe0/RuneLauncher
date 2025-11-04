#[cfg(windows)]
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetClassNameW, GetWindowTextW, IsWindowVisible,
};
#[cfg(windows)]
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub hwnd: isize,
    pub title: String,
    pub class_name: String,
}

impl WindowInfo {
    #[cfg(windows)]
    pub fn get_hwnd(&self) -> HWND {
        HWND(self.hwnd)
    }
}

#[cfg(windows)]
pub fn find_windows_by_class(class_name: &str) -> Vec<WindowInfo> {
    let windows = Arc::new(Mutex::new(Vec::new()));
    let target_class = class_name.to_lowercase();
    let windows_clone = windows.clone();

    unsafe {
        let _ = EnumWindows(
            Some(enum_window_callback),
            LPARAM(Box::into_raw(Box::new((windows_clone, target_class))) as isize),
        );
    }

    Arc::try_unwrap(windows).unwrap().into_inner().unwrap()
}

#[cfg(windows)]
unsafe extern "system" fn enum_window_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let data_ptr = lparam.0 as *mut (Arc<Mutex<Vec<WindowInfo>>>, String);
    let (windows, target_class) = &*data_ptr;

    if IsWindowVisible(hwnd).as_bool() {
        let mut class_name_buf = [0u16; 256];
        let len = GetClassNameW(hwnd, &mut class_name_buf);

        if len > 0 {
            let class_name = String::from_utf16_lossy(&class_name_buf[..len as usize]);

            if class_name.to_lowercase().contains(target_class) {
                let mut title_buf = [0u16; 256];
                let title_len = GetWindowTextW(hwnd, &mut title_buf);
                let title = if title_len > 0 {
                    String::from_utf16_lossy(&title_buf[..title_len as usize])
                } else {
                    String::from("(No Title)")
                };

                windows.lock().unwrap().push(WindowInfo {
                    hwnd: hwnd.0,
                    title,
                    class_name,
                });
            }
        }
    }

    true.into()
}

#[cfg(windows)]
pub fn find_jag_render_view() -> Option<WindowInfo> {
    let windows = find_windows_by_class("JagRenderView");
    windows.into_iter().next()
}

#[cfg(not(windows))]
#[allow(dead_code)]
pub fn find_windows_by_class(_class_name: &str) -> Vec<WindowInfo> {
    vec![]
}

#[cfg(not(windows))]
pub fn find_jag_render_view() -> Option<WindowInfo> {
    None
}
