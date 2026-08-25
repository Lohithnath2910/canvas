pub struct Bounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[cfg(windows)]
mod imp {
    use super::Bounds;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetSystemMetrics, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN,
        SM_YVIRTUALSCREEN,
    };

    pub fn virtual_bounds() -> Option<Bounds> {
        let (x, y, width, height) = unsafe {
            (
                GetSystemMetrics(SM_XVIRTUALSCREEN),
                GetSystemMetrics(SM_YVIRTUALSCREEN),
                GetSystemMetrics(SM_CXVIRTUALSCREEN),
                GetSystemMetrics(SM_CYVIRTUALSCREEN),
            )
        };
        if width <= 0 || height <= 0 {
            return None;
        }
        Some(Bounds {
            x,
            y,
            width,
            height,
        })
    }
}

#[cfg(not(windows))]
mod imp {
    use super::Bounds;

    pub fn virtual_bounds() -> Option<Bounds> {
        None
    }
}

pub use imp::virtual_bounds;
