use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use slint::ComponentHandle;

use canvas_doc::StickyNoteDoc;
use canvas_storage::{Store, WindowPlacement, KIND_STICKY_NOTE};
use canvas_ui::{platform_system_sans, HomeWindow, NoteRow, StickyNoteWindow, Typography};

const PLATE_MARGIN: f32 = 24.0;
const FLUSH_INTERVAL: Duration = Duration::from_millis(500);
const PIN_POLL_INTERVAL: Duration = Duration::from_millis(700);

struct Note {
    window: StickyNoteWindow,
    doc: StickyNoteDoc,
    placement: WindowPlacement,
    pending_updates: Vec<Vec<u8>>,
    text_dirty: bool,
    placement_dirty: bool,
}

pub struct App {
    store: Store,
    notes: HashMap<String, Note>,
    home: Option<HomeWindow>,
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
            home: None,
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

        for id in to_open {
            App::open_note(&app, &id)?;
        }

        App::open_home(&app)?;
        App::start_flush_timer(&app);
        Ok(app)
    }

    pub fn open_home(app: &SharedApp) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(home) = &app.borrow().home {
            home.show()?;
            return Ok(());
        }

        let home = HomeWindow::new()?;
        home.global::<Typography>()
            .set_system_sans(platform_system_sans().into());

        {
            let weak = Rc::downgrade(app);
            home.on_create_note(move || {
                if let Some(app) = weak.upgrade() {
                    if let Err(e) = App::create_note(&app) {
                        eprintln!("canvas: create failed: {e}");
                    }
                    App::refresh_home(&app);
                }
            });
        }

        {
            let weak = Rc::downgrade(app);
            home.on_open_row(move |id| {
                let Some(app) = weak.upgrade() else {
                    return;
                };
                let id = id.to_string();
                let is_open = app.borrow().notes.contains_key(&id);
                if is_open {
                    App::close_note(&app, &id);
                } else {
                    let _ = app.borrow().store.set_open(&id, true);
                    if let Err(e) = App::open_note(&app, &id) {
                        eprintln!("canvas: open failed: {e}");
                    }
                }
                App::refresh_home(&app);
            });
        }

        {
            let weak = Rc::downgrade(app);
            home.on_delete_row(move |id| {
                if let Some(app) = weak.upgrade() {
                    App::delete_note(&app, &id.to_string());
                    App::refresh_home(&app);
                }
            });
        }

        {
            let handle = home.as_weak();
            home.on_window_moved(move |dx, dy| {
                let Some(home) = handle.upgrade() else {
                    return;
                };
                let win = home.window();
                let scale = win.scale_factor();
                let pos = win.position();
                win.set_position(slint::PhysicalPosition::new(
                    pos.x + (dx * scale).round() as i32,
                    pos.y + (dy * scale).round() as i32,
                ));
            });
        }

        {
            let weak = Rc::downgrade(app);
            home.window().on_close_requested(move || {
                if let Some(app) = weak.upgrade() {
                    App::flush_all(&app);
                    crate::maybe_quit(&app);
                }
                slint::CloseRequestResponse::HideWindow
            });
        }

        app.borrow_mut().home = Some(home.clone_strong());
        App::refresh_home(app);
        home.show()?;
        Ok(())
    }

    pub fn refresh_home(app: &SharedApp) {
        let Ok(a) = app.try_borrow() else {
            return;
        };
        let Some(home) = a.home.as_ref().map(|h| h.clone_strong()) else {
            return;
        };
        let Ok(widgets) = a.store.list_widgets() else {
            return;
        };

        let rows: Vec<NoteRow> = widgets
            .iter()
            .map(|w| {
                let open = a.notes.contains_key(&w.id);
                let preview = if let Some(note) = a.notes.get(&w.id) {
                    note.doc.get_text()
                } else {
                    a.store
                        .get_document(&w.id)
                        .ok()
                        .flatten()
                        .filter(|b| !b.is_empty())
                        .map(|b| StickyNoteDoc::load(&b).get_text())
                        .unwrap_or_default()
                };
                NoteRow {
                    id: w.id.clone().into(),
                    title: w.title.clone().into(),
                    preview: first_line(&preview).into(),
                    kind: w.widget_type.clone().into(),
                    open,
                }
            })
            .collect();

        let open_count = a.notes.len() as i32;
        drop(a);

        let model: slint::VecModel<NoteRow> = slint::VecModel::from(rows);
        home.set_rows(slint::ModelRc::new(model));
        home.set_open_count(open_count);
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
                pending_updates: Vec::new(),
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
        if let Some(mut note) = a.notes.remove(id) {
            let _ = note.window.hide();
            for update in note.pending_updates.drain(..) {
                let _ = a.store.append_revision(id, &update);
            }
            let _ = a.store.save_document(id, &note.doc.snapshot(), &note.doc.state_vector());
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
            let App { store, notes, .. } = &mut *a;
            for (id, note) in notes.iter_mut() {
                if note.text_dirty {
                    for update in note.pending_updates.drain(..) {
                        let _ = store.append_revision(id, &update);
                    }
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
        let App { store, notes, .. } = &mut *a;
        for (id, note) in notes.iter_mut() {
            for update in note.pending_updates.drain(..) {
                let _ = store.append_revision(id, &update);
            }
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
            let App { store, notes, .. } = &mut *a;
            let Some(note) = notes.get_mut(&id) else {
                return;
            };
            if let Some(update) = note.doc.set_text(&text) {
                note.pending_updates.push(update);
                note.text_dirty = true;
            }
            let _ = store;
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
        window.on_title_edited(move |title| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            let trimmed = title.trim();
            let title = if trimmed.is_empty() {
                canvas_storage::DEFAULT_TITLE
            } else {
                trimmed
            };
            let Ok(a) = app.try_borrow() else {
                return;
            };
            let _ = a.store.set_title(&id, title);
            drop(a);
            App::refresh_home(&app);
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.on_close_requested(move || {
            let Some(app) = weak.upgrade() else {
                return;
            };
            App::close_note(&app, &id);
            App::refresh_home(&app);
            crate::maybe_quit(&app);
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
            App::refresh_home(&app);
            crate::maybe_quit(&app);
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.window().on_close_requested(move || {
            if let Some(app) = weak.upgrade() {
                App::close_note(&app, &id);
                App::refresh_home(&app);
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

fn first_line(text: &str) -> String {
    let line = text.lines().next().unwrap_or("").trim();
    if line.chars().count() > 60 {
        let truncated: String = line.chars().take(59).collect();
        format!("{truncated}…")
    } else {
        line.to_string()
    }
}

impl App {
    pub fn home_visible(app: &SharedApp) -> bool {
        app.borrow()
            .home
            .as_ref()
            .map(|h| h.window().is_visible())
            .unwrap_or(false)
    }
}
