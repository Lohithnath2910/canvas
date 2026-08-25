slint::include_modules!();

pub use slint;

pub fn platform_system_sans() -> &'static str {
    if cfg!(target_os = "macos") {
        "SF Pro Text"
    } else if cfg!(target_os = "windows") {
        "Segoe UI Variable"
    } else {
        "Inter"
    }
}
