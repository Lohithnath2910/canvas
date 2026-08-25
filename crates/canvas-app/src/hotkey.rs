pub const HOTKEY_PALETTE: i32 = 1;
pub const HOTKEY_NEW_NOTE: i32 = 2;

#[cfg(windows)]
mod imp {
    use std::cell::RefCell;
    use std::ptr;

    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        RegisterHotKey, MOD_CONTROL, MOD_NOREPEAT, VK_K, VK_N,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, RegisterClassW, HWND_MESSAGE, WM_HOTKEY, WNDCLASSW,
    };

    type Sink = Box<dyn Fn(i32)>;

    thread_local! {
        static SINK: RefCell<Option<Sink>> = const { RefCell::new(None) };
    }

    unsafe extern "system" fn wndproc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if msg == WM_HOTKEY {
            let id = wparam as i32;
            let handler = SINK.with(|sink| sink.borrow().is_some());
            if handler {
                let _ = slint::invoke_from_event_loop(move || {
                    SINK.with(|sink| {
                        if let Some(f) = sink.borrow().as_ref() {
                            f(id);
                        }
                    });
                });
            }
            return 0;
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }

    fn wide(value: &str) -> Vec<u16> {
        value.encode_utf16().chain(std::iter::once(0)).collect()
    }

    pub fn install(on_hotkey: impl Fn(i32) + 'static) -> bool {
        SINK.with(|sink| *sink.borrow_mut() = Some(Box::new(on_hotkey)));

        let class_name = wide("CanvasHotkeySink");
        let class = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: ptr::null_mut(),
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null(),
            lpszClassName: class_name.as_ptr(),
        };
        if unsafe { RegisterClassW(&class) } == 0 {
            return false;
        }

        let hwnd = unsafe {
            CreateWindowExW(
                0,
                class_name.as_ptr(),
                ptr::null(),
                0,
                0,
                0,
                0,
                0,
                HWND_MESSAGE,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null(),
            )
        };
        if hwnd.is_null() {
            return false;
        }

        let modifiers = MOD_CONTROL | MOD_NOREPEAT;
        let palette =
            unsafe { RegisterHotKey(hwnd, super::HOTKEY_PALETTE, modifiers, VK_K as u32) != 0 };
        let new_note =
            unsafe { RegisterHotKey(hwnd, super::HOTKEY_NEW_NOTE, modifiers, VK_N as u32) != 0 };

        palette && new_note
    }
}

#[cfg(not(windows))]
mod imp {
    pub fn install(_on_hotkey: impl Fn(i32) + 'static) -> bool {
        false
    }
}

pub use imp::install;
