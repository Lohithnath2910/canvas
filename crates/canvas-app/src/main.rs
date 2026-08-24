//! Canvas — native offline-first desktop widget platform.
//!
//! See CLAUDE.md for the full spec. Hard constraints that this binary is
//! responsible for: no browser engine anywhere (§2.1), editing never blocks on
//! the network (§2.2), and the software renderer is the default because the
//! memory budget depends on it (§2.5).

// No console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mimalloc::MiMalloc;
use slint::ComponentHandle;

use canvas_ui::{platform_system_sans, StickyNoteWindow, Typography};

/// Measurably reduces idle RSS on Windows (§3).
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> Result<(), slint::PlatformError> {
    select_backend();

    let window = StickyNoteWindow::new()?;

    // The body face is the OS's own system sans, chosen per platform (§13.1).
    window
        .global::<Typography>()
        .set_system_sans(platform_system_sans().into());

    wire_drag(&window);

    window.run()
}

/// 1:1 pointer follow, no inertia (§5.5). The header hands us a delta in
/// logical pixels; the window position is physical, so the delta is scaled.
fn wire_drag(window: &StickyNoteWindow) {
    let handle = window.as_weak();
    window.on_window_moved(move |dx, dy| {
        let Some(window) = handle.upgrade() else {
            return;
        };
        let win = window.window();
        let scale = win.scale_factor();
        let pos = win.position();
        win.set_position(slint::PhysicalPosition::new(
            pos.x + (dx * scale).round() as i32,
            pos.y + (dy * scale).round() as i32,
        ));
    });
}

/// The CPU software renderer is the default and the memory budget (§2.5)
/// depends on it. GPU rendering is opt-in twice over: the `gpu` cargo feature
/// must be compiled in *and* `--gpu` passed at runtime.
fn select_backend() {
    let wants_gpu = std::env::args().any(|a| a == "--gpu");

    if wants_gpu && !cfg!(feature = "gpu") {
        eprintln!(
            "canvas: --gpu ignored — the femtovg renderer is not compiled in.\n\
             canvas: rebuild with `--features gpu` to enable it."
        );
    }

    // An explicit SLINT_BACKEND from the environment wins.
    if std::env::var_os("SLINT_BACKEND").is_some() {
        return;
    }

    #[cfg(feature = "gpu")]
    if wants_gpu {
        std::env::set_var("SLINT_BACKEND", "winit-femtovg");
        eprintln!("canvas: renderer = winit-femtovg (GPU, opted in)");
        return;
    }

    std::env::set_var("SLINT_BACKEND", "winit-software");
    eprintln!("canvas: renderer = winit-software");
}
