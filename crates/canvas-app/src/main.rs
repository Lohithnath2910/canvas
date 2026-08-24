
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mimalloc::MiMalloc;
use slint::ComponentHandle;

use canvas_ui::{platform_system_sans, StickyNoteWindow, Typography};

#[cfg(windows)]
mod desktop_layer;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> Result<(), slint::PlatformError> {
    select_backend();

    let window = StickyNoteWindow::new()?;

    window
        .global::<Typography>()
        .set_system_sans(platform_system_sans().into());

    wire_drag(&window);
    wire_controls(&window);

    window.run()
}

fn wire_controls(window: &StickyNoteWindow) {
    window.on_tone_changed(|tone| {
        eprintln!("canvas: plate tone -> {tone}");
    });

    wire_pin(window);

    let handle = window.as_weak();
    window.on_delete_confirmed(move || {
        if let Some(window) = handle.upgrade() {
            let _ = window.hide();
        }
        let _ = slint::quit_event_loop();
    });
}

#[cfg(windows)]
fn wire_pin(window: &StickyNoteWindow) {
    use std::time::Duration;

    let handle = window.as_weak();
    window.on_pin_toggled(move |pinned| {
        let Some(window) = handle.upgrade() else {
            return;
        };
        let Some(hwnd) = desktop_layer::hwnd_of(window.window()) else {
            eprintln!("canvas: no HWND, pin unavailable");
            return;
        };
        if pinned {
            desktop_layer::send_to_desktop_layer(hwnd);
        } else {
            desktop_layer::restore_normal_layer(hwnd);
        }
    });

    let handle = window.as_weak();
    let timer = Box::leak(Box::new(slint::Timer::default()));
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(700),
        move || {
            let Some(window) = handle.upgrade() else {
                return;
            };
            if !window.get_pinned() {
                return;
            }
            let Some(hwnd) = desktop_layer::hwnd_of(window.window()) else {
                return;
            };
            if !desktop_layer::is_foreground(hwnd) {
                desktop_layer::send_to_desktop_layer(hwnd);
            }
        },
    );
}

#[cfg(not(windows))]
fn wire_pin(window: &StickyNoteWindow) {
    window.on_pin_toggled(|pinned| {
        eprintln!("canvas: pin -> {pinned} (desktop layer is Windows-only for now)");
    });
}

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

fn select_backend() {
    let wants_gpu = std::env::args().any(|a| a == "--gpu");

    if wants_gpu && !cfg!(feature = "gpu") {
        eprintln!(
            "canvas: --gpu ignored — the femtovg renderer is not compiled in.\n\
             canvas: rebuild with `--features gpu` to enable it."
        );
    }

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
