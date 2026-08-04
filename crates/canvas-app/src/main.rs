#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use canvas_ui::MainWindow;
use slint::ComponentHandle;
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), slint::PlatformError> {
    // Choose renderer based on CLI args
    if std::env::args().any(|arg| arg == "--gpu") {
        std::env::set_var("SLINT_BACKEND", "winit-femtovg");
    } else {
        std::env::set_var("SLINT_BACKEND", "winit-software");
    }

    // 1. Initialize SQLite storage
    let storage = canvas_storage::Storage::new("canvas_local.db").expect("Failed to initialize database");
    let workspace_id = storage.ensure_local_workspace().unwrap();
    let widget_id = storage.ensure_widget(&workspace_id, "sticky_note").unwrap();

    // 2. Load or create document
    let doc = if let Some((snapshot, _)) = storage.load_document(&widget_id).unwrap() {
        canvas_doc::StickyNoteDoc::load(&snapshot)
    } else {
        canvas_doc::StickyNoteDoc::new()
    };

    let doc = Rc::new(RefCell::new(doc));
    
    // 3. Setup UI
    let window = MainWindow::new()?;
    window.set_note_text(doc.borrow().get_text().into());

    let storage = Rc::new(storage);
    
    let doc_clone = doc.clone();
    let storage_clone = storage.clone();
    let widget_id_clone = widget_id.clone();
    window.on_text_changed(move |new_text| {
        let d = doc_clone.borrow();
        d.update_text(&new_text);
        
        let snapshot = d.snapshot();
        let sv = d.state_vector();
        
        // Save to SQLite
        storage_clone.save_document(&widget_id_clone, &snapshot, &sv).unwrap();
    });

    let window_weak = window.as_weak();
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
        }
    });

    window.run()
}
