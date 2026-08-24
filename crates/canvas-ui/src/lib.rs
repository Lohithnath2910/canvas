//! Compiled Slint UI for Canvas.
//!
//! Nothing here imports `std-widgets`. The Slint compiler defaults to the
//! `fluent` style whenever none is configured, and Fluent's `TextEdit` bakes in
//! a blue accent bar that is not overridable from outside (CLAUDE.md §13.2), so
//! every control in this crate is built from bare primitives instead.

slint::include_modules!();

pub use slint;

/// The body face is the OS's own system sans (CLAUDE.md §5.2). Silkscreen and
/// Inter are embedded at compile time by `theme/typography.slint`.
pub fn platform_system_sans() -> &'static str {
    if cfg!(target_os = "macos") {
        "SF Pro Text"
    } else if cfg!(target_os = "windows") {
        "Segoe UI Variable"
    } else {
        "Inter"
    }
}
