#[cfg(windows)]
mod imp {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, SetWindowPos, HWND_BOTTOM, HWND_NOTOPMOST, SWP_NOACTIVATE, SWP_NOMOVE,
        SWP_NOSIZE,
    };

    fn hwnd_of(window: &slint::Window) -> Option<HWND> {
        let handle = window.window_handle();
        let handle = handle.window_handle().ok()?;
        match handle.as_raw() {
            RawWindowHandle::Win32(h) => Some(h.hwnd.get() as HWND),
            _ => None,
        }
    }

    fn place(hwnd: HWND, insert_after: HWND) {
        unsafe {
            SetWindowPos(
                hwnd,
                insert_after,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            );
        }
    }

    pub fn apply_pin(window: &slint::Window, pinned: bool) {
        let Some(hwnd) = hwnd_of(window) else {
            return;
        };
        place(hwnd, if pinned { HWND_BOTTOM } else { HWND_NOTOPMOST });
    }

    pub fn hold_at_desktop_layer(window: &slint::Window) {
        let Some(hwnd) = hwnd_of(window) else {
            return;
        };
        if unsafe { GetForegroundWindow() } != hwnd {
            place(hwnd, HWND_BOTTOM);
        }
    }
}

#[cfg(not(windows))]
mod imp {
    pub fn apply_pin(_window: &slint::Window, _pinned: bool) {}
    pub fn hold_at_desktop_layer(_window: &slint::Window) {}
}

pub use imp::{apply_pin, hold_at_desktop_layer};
