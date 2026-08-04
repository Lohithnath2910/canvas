#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use canvas_ui::MainWindow;
use slint::ComponentHandle;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

struct AppState {
    docs: Arc<Mutex<HashMap<String, Arc<Mutex<canvas_doc::StickyNoteDoc>>>>>,
    windows: Arc<Mutex<HashMap<String, slint::Weak<MainWindow>>>>,
    storage: Rc<canvas_storage::Storage>,
    workspace_id: String,
    tx_net: Option<tokio::sync::mpsc::Sender<canvas_sync::SyncMessage>>,
}

fn main() -> Result<(), slint::PlatformError> {
    if std::env::args().any(|arg| arg == "--gpu") {
        std::env::set_var("SLINT_BACKEND", "winit-femtovg");
    } else {
        std::env::set_var("SLINT_BACKEND", "winit-software");
    }

    let mut host_addr = None;
    let mut join_addr = None;
    let args: Vec<String> = std::env::args().collect();
    for i in 0..args.len() {
        if args[i] == "--host" && i + 1 < args.len() {
            host_addr = Some(args[i+1].clone());
        } else if args[i] == "--join" && i + 1 < args.len() {
            join_addr = Some(args[i+1].clone());
        }
    }

    let storage = canvas_storage::Storage::new("canvas_local.db").expect("Failed to initialize database");
    let workspace_id = storage.ensure_local_workspace().unwrap();
    
    let mut widgets = storage.get_all_widgets(&workspace_id).unwrap();
    if widgets.is_empty() {
        let default_widget = storage.create_widget(&workspace_id, "sticky_note", 100, 100).unwrap();
        widgets.push(default_widget);
    }

    let (tx_net, mut rx_net) = if host_addr.is_some() || join_addr.is_some() {
        let (tx_thread, rx_thread) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let channels = if let Some(addr) = host_addr {
                    canvas_sync::start_host(&addr).await
                } else if let Some(addr) = join_addr {
                    canvas_sync::SyncClient::connect(&addr).await.unwrap()
                } else {
                    unreachable!()
                };
                tx_thread.send(channels).unwrap();
                std::future::pending::<()>().await;
            });
        });
        let (tx, rx) = rx_thread.recv().unwrap();
        (Some(tx), Some(rx))
    } else {
        (None, None)
    };

    let docs = Arc::new(Mutex::new(HashMap::new()));
    let windows = Arc::new(Mutex::new(HashMap::new()));

    let app_state = Rc::new(AppState {
        docs: docs.clone(),
        windows: windows.clone(),
        storage: Rc::new(storage),
        workspace_id,
        tx_net,
    });

    for w in widgets {
        spawn_widget(app_state.clone(), w);
    }

    if let Some(mut rx) = rx_net {
        let net_docs = docs.clone();
        let net_windows = windows.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                while let Some(msg) = rx.recv().await {
                    let d = net_docs.clone();
                    let w = net_windows.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(doc) = d.lock().unwrap().get(&msg.widget_id) {
                            doc.lock().unwrap().apply_update(&msg.payload);
                            let text = doc.lock().unwrap().get_text();
                            if let Some(window_weak) = w.lock().unwrap().get(&msg.widget_id) {
                                if let Some(window) = window_weak.upgrade() {
                                    window.set_note_text(text.into());
                                }
                            }
                        }
                    });
                }
            });
        });
    }

    slint::run_event_loop()
}

fn spawn_widget(state: Rc<AppState>, widget: canvas_storage::WidgetRecord) {
    let doc = if let Some((snapshot, _)) = state.storage.load_document(&widget.id).unwrap() {
        canvas_doc::StickyNoteDoc::load(&snapshot)
    } else {
        canvas_doc::StickyNoteDoc::new()
    };
    let doc = Arc::new(Mutex::new(doc));
    
    state.docs.lock().unwrap().insert(widget.id.clone(), doc.clone());

    let window = MainWindow::new().unwrap();
    window.set_note_text(doc.lock().unwrap().get_text().into());
    
    let win = window.window();
    win.set_position(slint::PhysicalPosition::new(widget.pos_x, widget.pos_y));
    
    state.windows.lock().unwrap().insert(widget.id.clone(), window.as_weak());

    let doc_clone = doc.clone();
    let storage_clone = state.storage.clone();
    let widget_id_clone = widget.id.clone();
    let tx_net = state.tx_net.clone();
    
    window.on_text_changed(move |new_text| {
        let d = doc_clone.lock().unwrap();
        let payload = d.update_text(&new_text);
        if let Some(tx) = &tx_net {
            let _ = tx.blocking_send(canvas_sync::SyncMessage {
                widget_id: widget_id_clone.clone(),
                payload,
            });
        }
        let snapshot = d.snapshot();
        let sv = d.state_vector();
        storage_clone.save_document(&widget_id_clone, &snapshot, &sv).unwrap();
    });

    let window_weak = window.as_weak();
    let storage_clone2 = state.storage.clone();
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

    let state_clone3 = state.clone();
    let window_weak2 = window.as_weak();
    window.on_new_widget_requested(move || {
        if let Some(app) = window_weak2.upgrade() {
            let win = app.window();
            let pos = win.position();
            let new_widget = state_clone3.storage.create_widget(&state_clone3.workspace_id, "sticky_note", pos.x + 50, pos.y + 50).unwrap();
            spawn_widget(state_clone3.clone(), new_widget);
        }
    });

    let window_weak3 = window.as_weak();
    let state_clone4 = state.clone();
    let widget_id_clone3 = widget.id.clone();
    window.on_close_widget_requested(move || {
        if let Some(app) = window_weak3.upgrade() {
            let _ = state_clone4.storage.delete_widget(&widget_id_clone3);
            state_clone4.docs.lock().unwrap().remove(&widget_id_clone3);
            state_clone4.windows.lock().unwrap().remove(&widget_id_clone3);
            app.hide().unwrap();
        }
    });

    window.show().unwrap();
}
