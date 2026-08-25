#[cfg(windows)]
mod imp {
    use windows_sys::Win32::Foundation::BOOL;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SPI_GETCLIENTAREAANIMATION,
    };

    pub fn wanted() -> bool {
        let mut animations: BOOL = 1;
        let ok = unsafe {
            SystemParametersInfoW(
                SPI_GETCLIENTAREAANIMATION,
                0,
                std::ptr::addr_of_mut!(animations).cast(),
                0,
            )
        };
        ok != 0 && animations == 0
    }
}

#[cfg(not(windows))]
mod imp {
    pub fn wanted() -> bool {
        false
    }
}

pub fn wanted() -> bool {
    if let Some(forced) = forced() {
        return forced;
    }
    imp::wanted()
}

fn forced() -> Option<bool> {
    match std::env::var("CANVAS_REDUCED_MOTION").ok()?.as_str() {
        "1" | "true" | "on" => Some(true),
        "0" | "false" | "off" => Some(false),
        _ => None,
    }
}
