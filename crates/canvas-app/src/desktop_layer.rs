#[cfg(windows)]
mod imp {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows_sys::Win32::Foundation::{HWND, POINT};
    use windows_sys::Win32::System::Threading::AttachThreadInput;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::SetFocus;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        BringWindowToTop, GetCursorPos, GetForegroundWindow, GetWindowThreadProcessId,
        SetForegroundWindow, SetWindowPos, HWND_BOTTOM, HWND_NOTOPMOST, SWP_NOACTIVATE,
        SWP_NOMOVE, SWP_NOSIZE,
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

    pub fn take_foreground(window: &slint::Window) {
        let Some(hwnd) = hwnd_of(window) else {
            return;
        };
        unsafe {
            let foreground = GetForegroundWindow();
            if foreground == hwnd {
                return;
            }
            let ours = GetWindowThreadProcessId(hwnd, std::ptr::null_mut());
            let theirs = if foreground.is_null() {
                0
            } else {
                GetWindowThreadProcessId(foreground, std::ptr::null_mut())
            };
            let borrowed = theirs != 0 && theirs != ours && AttachThreadInput(ours, theirs, 1) != 0;
            SetForegroundWindow(hwnd);
            BringWindowToTop(hwnd);
            SetFocus(hwnd);
            if borrowed {
                AttachThreadInput(ours, theirs, 0);
            }
        }
    }

    pub fn cursor_pos() -> Option<(i32, i32)> {
        let mut point = POINT { x: 0, y: 0 };
        if unsafe { GetCursorPos(&mut point) } == 0 {
            return None;
        }
        Some((point.x, point.y))
    }
}

#[cfg(not(windows))]
mod imp {
    pub fn apply_pin(_window: &slint::Window, _pinned: bool) {}
    pub fn hold_at_desktop_layer(_window: &slint::Window) {}
    pub fn take_foreground(_window: &slint::Window) {}

    pub fn cursor_pos() -> Option<(i32, i32)> {
        None
    }
}

pub use imp::{apply_pin, cursor_pos, hold_at_desktop_layer, take_foreground};
