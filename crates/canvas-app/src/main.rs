#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mimalloc::MiMalloc;

mod app;
mod calendar_time;
mod desktop_layer;
mod hotkey;
mod reduced_motion;
mod screen;
mod widget;
mod widget_types;

use app::{App, OpenRequest, SharedApp};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|a| a == "--dump") {
        return app::dump();
    }

    if std::env::args().any(|a| a == "--gallery") {
        select_backend();
        return app::gallery();
    }

    let request = match OpenRequest::from_args(std::env::args().skip(1)) {
        Ok(request) => request,
        Err(message) => {
            eprintln!("canvas: {message}");
            return Ok(());
        }
    };

    select_backend();

    let app = App::boot(request)?;
    slint::run_event_loop_until_quit()?;
    App::flush_all(&app);
    Ok(())
}

pub fn maybe_quit(app: &SharedApp) {
    if App::tray_present(app) {
        App::flush_all(app);
        return;
    }
    if App::open_count(app) == 0 && !App::home_visible(app) {
        App::flush_all(app);
        let _ = slint::quit_event_loop();
    }
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
