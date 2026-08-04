#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use canvas_ui::MainWindow;
use slint::ComponentHandle;
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), slint::PlatformError> {
    if std::env::args().any(|arg| arg == "--gpu") {
        std::env::set_var("SLINT_BACKEND", "winit-femtovg");
    } else {
        std::env::set_var("SLINT_BACKEND", "winit-software");
    }

    let storage = canvas_storage::Storage::new("canvas_local.db").expect("Failed to initialize database");
    let workspace_id = storage.ensure_local_workspace().unwrap();
    
    let mut widgets = storage.get_all_widgets(&workspace_id).unwrap();
    if widgets.is_empty() {
        let default_widget = storage.create_widget(&workspace_id, "sticky_note", 100, 100).unwrap();
        widgets.push(default_widget);
    }

    let storage_rc = Rc::new(storage);
    for w in widgets {
        spawn_widget(storage_rc.clone(), workspace_id.clone(), w);
    }

    slint::run_event_loop()
}

fn spawn_widget(storage: Rc<canvas_storage::Storage>, workspace_id: String, widget: canvas_storage::WidgetRecord) {
    let doc = if let Some((snapshot, _)) = storage.load_document(&widget.id).unwrap() {
        canvas_doc::StickyNoteDoc::load(&snapshot)
    } else {
        canvas_doc::StickyNoteDoc::new()
    };
    let doc = Rc::new(RefCell::new(doc));
    
    let window = MainWindow::new().unwrap();
    window.set_note_text(doc.borrow().get_text().into());
    
    let win = window.window();
    win.set_position(slint::PhysicalPosition::new(widget.pos_x, widget.pos_y));
    
    let doc_clone = doc.clone();
    let storage_clone = storage.clone();
    let widget_id_clone = widget.id.clone();
    
    window.on_text_changed(move |new_text| {
        let d = doc_clone.borrow();
        d.update_text(&new_text);
        let snapshot = d.snapshot();
        let sv = d.state_vector();
        storage_clone.save_document(&widget_id_clone, &snapshot, &sv).unwrap();
    });

    let window_weak = window.as_weak();
    let storage_clone2 = storage.clone();
    let widget_id_clone2 = widget.id.clone();
    window.on_window_moved(move |dx, dy| {
        if let Some(app) = window_weak.upgrade() {
            let win = app.window();
            let pos = win.position();
            let scale_factor = win.scale_factor();
            let physical_dx = dx * scale_factor;
            let physical_dy = dy * scale_factor;
            let new_pos = slint::PhysicalPosition::new(
                pos.x + physical_dx as i32,
                pos.y + physical_dy as i32,
            );
            win.set_position(new_pos);
            let _ = storage_clone2.update_widget_position(&widget_id_clone2, new_pos.x, new_pos.y);
        }
    });

    let storage_clone3 = storage.clone();
    let workspace_id_clone = workspace_id.clone();
    let window_weak2 = window.as_weak();
    window.on_new_widget_requested(move || {
        if let Some(app) = window_weak2.upgrade() {
            let win = app.window();
            let pos = win.position();
            let new_widget = storage_clone3.create_widget(&workspace_id_clone, "sticky_note", pos.x + 50, pos.y + 50).unwrap();
            spawn_widget(storage_clone3.clone(), workspace_id_clone.clone(), new_widget);
        }
    });

    let window_weak3 = window.as_weak();
    window.on_close_widget_requested(move || {
        if let Some(app) = window_weak3.upgrade() {
            app.hide().unwrap();
        }
    });

    window.show().unwrap();
}
