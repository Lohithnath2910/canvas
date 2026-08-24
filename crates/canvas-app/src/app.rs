use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use slint::ComponentHandle;

use canvas_doc::StickyNoteDoc;
use canvas_storage::{Store, WindowPlacement, KIND_STICKY_NOTE};
use canvas_ui::{platform_system_sans, StickyNoteWindow, Typography};

const PLATE_MARGIN: f32 = 24.0;
const FLUSH_INTERVAL: Duration = Duration::from_millis(500);
const PIN_POLL_INTERVAL: Duration = Duration::from_millis(700);

struct Note {
    window: StickyNoteWindow,
    doc: StickyNoteDoc,
    placement: WindowPlacement,
    text_dirty: bool,
    placement_dirty: bool,
}

pub struct App {
    store: Store,
    notes: HashMap<String, Note>,
}

pub type SharedApp = Rc<RefCell<App>>;

impl App {
    pub fn boot() -> Result<SharedApp, Box<dyn std::error::Error>> {
        let dir = Store::default_path();
        std::fs::create_dir_all(&dir)?;
        let store = Store::open(&dir.join("canvas_local.db"))?;

        let app: SharedApp = Rc::new(RefCell::new(App {
            store,
            notes: HashMap::new(),
        }));

        let to_open: Vec<String> = {
            let a = app.borrow();
            a.store
                .list_widgets()?
                .into_iter()
                .filter(|w| w.widget_type == KIND_STICKY_NOTE)
                .filter(|w| {
                    a.store
                        .get_placement(&w.id)
                        .ok()
                        .flatten()
                        .map(|p| p.open)
                        .unwrap_or(true)
                })
                .map(|w| w.id)
                .collect()
        };

        if to_open.is_empty() && app.borrow().store.list_widgets()?.is_empty() {
            let id = App::create_note(&app)?;
            eprintln!("canvas: created first note {id}");
        } else {
            for id in to_open {
                App::open_note(&app, &id)?;
            }
        }

        App::start_flush_timer(&app);
        Ok(app)
    }

    pub fn create_note(app: &SharedApp) -> Result<String, Box<dyn std::error::Error>> {
        let id = {
            let a = app.borrow();
            let (widget, _) = a.store.create_widget(KIND_STICKY_NOTE)?;
            widget.id
        };
        App::open_note(app, &id)?;
        Ok(id)
    }

    pub fn open_note(app: &SharedApp, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(note) = app.borrow().notes.get(id) {
            note.window.show()?;
            return Ok(());
        }

        let (title, placement, doc) = {
            let a = app.borrow();
            let widget = a
                .store
                .get_widget(id)?
                .ok_or_else(|| format!("no widget {id}"))?;
            let placement = a
                .store
                .get_placement(id)?
                .unwrap_or_else(|| WindowPlacement::default_for(&widget.widget_type));
            let doc = match a.store.get_document(id)? {
                Some(bytes) if !bytes.is_empty() => StickyNoteDoc::load(&bytes),
                _ => StickyNoteDoc::new(),
            };
            (widget.title, placement, doc)
        };

        let window = StickyNoteWindow::new()?;
        window
            .global::<Typography>()
            .set_system_sans(platform_system_sans().into());
        window.set_note_title(title.into());
        window.set_note_text(doc.get_text().into());
        window.set_pinned(placement.pinned);
        window.set_tone(placement.tone as i32);

        window.window().set_position(slint::LogicalPosition::new(
            placement.x as f32 - PLATE_MARGIN,
            placement.y as f32 - PLATE_MARGIN,
        ));
        window.window().set_size(slint::LogicalSize::new(
            placement.width as f32 + 2.0 * PLATE_MARGIN,
            placement.height as f32 + 2.0 * PLATE_MARGIN,
        ));

        app.borrow_mut().notes.insert(
            id.to_string(),
            Note {
                window: window.clone_strong(),
                doc,
                placement,
                text_dirty: false,
                placement_dirty: false,
            },
        );

        wire_note(app, id, &window);
        window.show()?;
        Ok(())
    }

    pub fn close_note(app: &SharedApp, id: &str) {
        let mut a = app.borrow_mut();
        if let Some(note) = a.notes.remove(id) {
            let _ = note.window.hide();
            let _ = a.store.save_document(
                id,
                &note.doc.snapshot(),
                &note.doc.state_vector(),
            );
            let _ = a.store.save_placement(id, &note.placement);
        }
        let _ = a.store.set_open(id, false);
    }

    pub fn delete_note(app: &SharedApp, id: &str) {
        let mut a = app.borrow_mut();
        if let Some(note) = a.notes.remove(id) {
            let _ = note.window.hide();
        }
        let _ = a.store.delete_widget(id);
    }

    pub fn open_count(app: &SharedApp) -> usize {
        app.borrow().notes.len()
    }

    fn start_flush_timer(app: &SharedApp) {
        let weak = Rc::downgrade(app);
        let timer = Box::leak(Box::new(slint::Timer::default()));
        timer.start(slint::TimerMode::Repeated, FLUSH_INTERVAL, move || {
            let Some(app) = weak.upgrade() else {
                return;
            };
            let Ok(mut a) = app.try_borrow_mut() else {
                return;
            };
            let App { store, notes } = &mut *a;
            for (id, note) in notes.iter_mut() {
                if note.text_dirty {
                    let _ =
                        store.save_document(id, &note.doc.snapshot(), &note.doc.state_vector());
                    let _ = store.touch(id);
                    note.text_dirty = false;
                }
                if note.placement_dirty {
                    let _ = store.save_placement(id, &note.placement);
                    note.placement_dirty = false;
                }
            }
        });
    }

    pub fn flush_all(app: &SharedApp) {
        let mut a = app.borrow_mut();
        let App { store, notes } = &mut *a;
        for (id, note) in notes.iter_mut() {
            let _ = store.save_document(id, &note.doc.snapshot(), &note.doc.state_vector());
            let _ = store.save_placement(id, &note.placement);
            note.text_dirty = false;
            note.placement_dirty = false;
        }
    }
}

fn wire_note(app: &SharedApp, id: &str, window: &StickyNoteWindow) {
    let owned_id = id.to_string();

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.on_text_edited(move |text| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            let mut a = app.borrow_mut();
            let App { store, notes } = &mut *a;
            let Some(note) = notes.get_mut(&id) else {
                return;
            };
            if let Some(update) = note.doc.set_text(&text) {
                let _ = store.append_revision(&id, &update);
                note.text_dirty = true;
            }
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        window.on_window_moved(move |dx, dy| {
            let Some(window) = handle.upgrade() else {
                return;
            };
            let win = window.window();
            let scale = win.scale_factor();
            let pos = win.position();
            let next = slint::PhysicalPosition::new(
                pos.x + (dx * scale).round() as i32,
                pos.y + (dy * scale).round() as i32,
            );
            win.set_position(next);

            let Some(app) = weak.upgrade() else {
                return;
            };
            let Ok(mut a) = app.try_borrow_mut() else {
                return;
            };
            if let Some(note) = a.notes.get_mut(&id) {
                let logical = next.to_logical(scale);
                note.placement.x = (logical.x + PLATE_MARGIN).round() as i32;
                note.placement.y = (logical.y + PLATE_MARGIN).round() as i32;
                note.placement_dirty = true;
            }
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.on_tone_changed(move |tone| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            let mut a = app.borrow_mut();
            if let Some(note) = a.notes.get_mut(&id) {
                note.placement.tone = tone as i64;
                note.placement_dirty = true;
            }
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        window.on_pin_toggled(move |pinned| {
            if let Some(window) = handle.upgrade() {
                crate::desktop_layer::apply_pin(window.window(), pinned);
            }
            let Some(app) = weak.upgrade() else {
                return;
            };
            let mut a = app.borrow_mut();
            if let Some(note) = a.notes.get_mut(&id) {
                note.placement.pinned = pinned;
                note.placement_dirty = true;
            }
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.on_delete_confirmed(move || {
            let Some(app) = weak.upgrade() else {
                return;
            };
            App::delete_note(&app, &id);
            crate::maybe_quit(&app);
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.window().on_close_requested(move || {
            if let Some(app) = weak.upgrade() {
                App::close_note(&app, &id);
                crate::maybe_quit(&app);
            }
            slint::CloseRequestResponse::HideWindow
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        let timer = Box::leak(Box::new(slint::Timer::default()));
        timer.start(slint::TimerMode::Repeated, PIN_POLL_INTERVAL, move || {
            let Some(window) = handle.upgrade() else {
                return;
            };
            if window.get_pinned() {
                crate::desktop_layer::hold_at_desktop_layer(window.window());
            }

            let Some(app) = weak.upgrade() else {
                return;
            };
            let Ok(mut a) = app.try_borrow_mut() else {
                return;
            };
            if let Some(note) = a.notes.get_mut(&id) {
                let size = window.window().size().to_logical(window.window().scale_factor());
                let w = (size.width - 2.0 * PLATE_MARGIN).round() as i32;
                let h = (size.height - 2.0 * PLATE_MARGIN).round() as i32;
                if w != note.placement.width || h != note.placement.height {
                    note.placement.width = w;
                    note.placement.height = h;
                    note.placement_dirty = true;
                }
            }
        });
    }
}

pub fn dump() -> Result<(), Box<dyn std::error::Error>> {
    let dir = Store::default_path();
    let store = Store::open(&dir.join("canvas_local.db"))?;
    let widgets = store.list_widgets()?;
    println!("db: {}", dir.join("canvas_local.db").display());
    println!("widgets: {}", widgets.len());
    for w in widgets {
        let placement = store.get_placement(&w.id)?;
        let text = match store.get_document(&w.id)? {
            Some(bytes) if !bytes.is_empty() => StickyNoteDoc::load(&bytes).get_text(),
            _ => String::new(),
        };
        let revs = store.revision_count(&w.id)?;
        println!(
            "  {} type={} title={:?} revs={} open={:?} pos={:?} tone={:?} pinned={:?}",
            &w.id[..8],
            w.widget_type,
            w.title,
            revs,
            placement.as_ref().map(|p| p.open),
            placement.as_ref().map(|p| (p.x, p.y)),
            placement.as_ref().map(|p| p.tone),
            placement.as_ref().map(|p| p.pinned),
        );
        println!("    text: {text:?}");
    }
    Ok(())
}
