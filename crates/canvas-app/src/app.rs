use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use slint::ComponentHandle;

use canvas_doc::StickyNoteDoc;
use canvas_storage::{Store, WindowPlacement, KIND_STICKY_NOTE};
use canvas_ui::{
    platform_system_sans, CanvasTray, HomeWindow, NoteRow, PaletteRow, PaletteWindow,
    StickyNoteWindow, Typography,
};

pub const ACTION_NEW_NOTE: &str = "new-note";
pub const ACTION_NEW_TODO: &str = "new-todo";
pub const ACTION_SETTINGS: &str = "settings";

const PLATE_MARGIN: f32 = 24.0;
const FLUSH_INTERVAL: Duration = Duration::from_millis(500);
const PIN_POLL_INTERVAL: Duration = Duration::from_millis(700);
const OPEN_DELAY: Duration = Duration::from_millis(16);
const CLOSE_FADE: Duration = Duration::from_millis(100);
const KEEP_VISIBLE_W: f32 = 120.0;
const KEEP_VISIBLE_H: f32 = 48.0;

fn clamp_to_screen(
    pos: slint::PhysicalPosition,
    size: slint::PhysicalSize,
    scale: f32,
) -> slint::PhysicalPosition {
    let Some(bounds) = crate::screen::virtual_bounds() else {
        return pos;
    };
    let margin = (PLATE_MARGIN * scale).round() as i32;
    let keep_w = (KEEP_VISIBLE_W * scale).round() as i32;
    let keep_h = (KEEP_VISIBLE_H * scale).round() as i32;
    let width = size.width as i32;

    let x_min = bounds.x - width + margin + keep_w;
    let x_max = bounds.x + bounds.width - margin - keep_w;
    let y_min = bounds.y - margin;
    let y_max = bounds.y + bounds.height - margin - keep_h;

    slint::PhysicalPosition::new(pos.x.min(x_max).max(x_min), pos.y.min(y_max).max(y_min))
}

fn clamp_window_onto_screen(window: &slint::Window) -> Option<slint::PhysicalPosition> {
    let scale = window.scale_factor();
    let pos = window.position();
    let clamped = clamp_to_screen(pos, window.size(), scale);
    if clamped.x == pos.x && clamped.y == pos.y {
        return None;
    }
    window.set_position(clamped);
    Some(clamped)
}

trait Fadeable: ComponentHandle + 'static {
    fn set_opened_flag(&self, value: bool);
    fn set_closing_flag(&self, value: bool);
}

impl Fadeable for StickyNoteWindow {
    fn set_opened_flag(&self, value: bool) {
        self.set_opened(value);
    }
    fn set_closing_flag(&self, value: bool) {
        self.set_closing(value);
    }
}

impl Fadeable for HomeWindow {
    fn set_opened_flag(&self, value: bool) {
        self.set_opened(value);
    }
    fn set_closing_flag(&self, value: bool) {
        self.set_closing(value);
    }
}

fn play_open<T: Fadeable>(window: &T) {
    window.set_closing_flag(false);
    window.set_opened_flag(false);
    let weak = window.as_weak();
    slint::Timer::single_shot(OPEN_DELAY, move || {
        if let Some(window) = weak.upgrade() {
            window.set_opened_flag(true);
        }
    });
}

fn play_close<T: Fadeable>(window: T, done: impl FnOnce() + 'static) {
    window.set_closing_flag(true);
    slint::Timer::single_shot(CLOSE_FADE, move || {
        let _ = window.hide();
        window.set_closing_flag(false);
        window.set_opened_flag(false);
        done();
    });
}

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
    palette: Option<PaletteWindow>,
    tray: Option<CanvasTray>,
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
            palette: None,
            tray: None,
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
        App::install_tray(&app);
        App::install_hotkeys(&app);
        App::start_flush_timer(&app);
        Ok(app)
    }

    pub fn open_home(app: &SharedApp) -> Result<(), Box<dyn std::error::Error>> {
        let existing = app.borrow().home.as_ref().map(|h| h.clone_strong());
        if let Some(home) = existing {
            play_open(&home);
            home.show()?;
            clamp_window_onto_screen(home.window());
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
                    App::delete_note(&app, id.as_ref());
                    App::refresh_home(&app);
                }
            });
        }

        {
            let weak = Rc::downgrade(app);
            home.on_close_requested(move || {
                let Some(app) = weak.upgrade() else {
                    return;
                };
                App::flush_all(&app);
                App::hide_home(&app);
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
                win.set_position(clamp_to_screen(
                    slint::PhysicalPosition::new(
                        pos.x + (dx * scale).round() as i32,
                        pos.y + (dy * scale).round() as i32,
                    ),
                    win.size(),
                    scale,
                ));
            });
        }

        {
            let weak = Rc::downgrade(app);
            home.window().on_close_requested(move || {
                if let Some(app) = weak.upgrade() {
                    App::flush_all(&app);
                    App::hide_home(&app);
                }
                slint::CloseRequestResponse::KeepWindowShown
            });
        }

        app.borrow_mut().home = Some(home.clone_strong());
        App::refresh_home(app);
        play_open(&home);
        home.show()?;
        let scale = home.window().scale_factor();
        home.set_resize_border(home.get_resize_border() * scale);
        clamp_window_onto_screen(home.window());
        App::sync_tray(app);
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
        let existing = app.borrow().notes.get(id).map(|n| n.window.clone_strong());
        if let Some(window) = existing {
            play_open(&window);
            window.show()?;
            clamp_window_onto_screen(window.window());
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
        play_open(&window);
        window.show()?;
        let scale = window.window().scale_factor();
        window.set_resize_border(window.get_resize_border() * scale);

        if let Some(moved) = clamp_window_onto_screen(window.window()) {
            let logical = moved.to_logical(scale);
            let mut a = app.borrow_mut();
            if let Some(note) = a.notes.get_mut(id) {
                note.placement.x = (logical.x + PLATE_MARGIN).round() as i32;
                note.placement.y = (logical.y + PLATE_MARGIN).round() as i32;
                note.placement_dirty = true;
            }
        }
        Ok(())
    }

    pub fn close_note(app: &SharedApp, id: &str) {
        let mut a = app.borrow_mut();
        if let Some(mut note) = a.notes.remove(id) {
            play_close(note.window, || {});
            for update in note.pending_updates.drain(..) {
                let _ = a.store.append_revision(id, &update);
            }
            let _ = a
                .store
                .save_document(id, &note.doc.snapshot(), &note.doc.state_vector());
            let _ = a.store.save_placement(id, &note.placement);
        }
        let _ = a.store.set_open(id, false);
    }

    pub fn delete_note(app: &SharedApp, id: &str) {
        let mut a = app.borrow_mut();
        if let Some(note) = a.notes.remove(id) {
            play_close(note.window, || {});
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
                    let _ = store.save_document(id, &note.doc.snapshot(), &note.doc.state_vector());
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
            let next = clamp_to_screen(
                slint::PhysicalPosition::new(
                    pos.x + (dx * scale).round() as i32,
                    pos.y + (dy * scale).round() as i32,
                ),
                win.size(),
                scale,
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
        window.on_palette_requested(move || {
            if let Some(app) = weak.upgrade() {
                App::show_palette(&app);
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
            slint::CloseRequestResponse::KeepWindowShown
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
                let size = window
                    .window()
                    .size()
                    .to_logical(window.window().scale_factor());
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

fn default_actions() -> Vec<PaletteRow> {
    vec![
        PaletteRow {
            id: ACTION_NEW_NOTE.into(),
            label: "New note".into(),
            meta: "CTRL+N".into(),
            icon: "plus".into(),
        },
        PaletteRow {
            id: ACTION_NEW_TODO.into(),
            label: "New todo".into(),
            meta: "CTRL+T".into(),
            icon: "check".into(),
        },
        PaletteRow {
            id: ACTION_SETTINGS.into(),
            label: "Settings".into(),
            meta: "CTRL+,".into(),
            icon: "settings".into(),
        },
    ]
}

fn count_label(count: usize) -> String {
    match count {
        0 => "NO RESULTS".to_string(),
        1 => "1 RESULT".to_string(),
        n => format!("{n} RESULTS"),
    }
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

impl App {
    pub fn install_tray(app: &SharedApp) {
        let tray = match CanvasTray::new() {
            Ok(tray) => tray,
            Err(e) => {
                eprintln!(
                    "canvas: no system tray available ({e}); quitting when the last window closes"
                );
                return;
            }
        };

        {
            let weak = Rc::downgrade(app);
            tray.on_open_home(move || {
                if let Some(app) = weak.upgrade() {
                    App::show_home(&app);
                }
            });
        }

        {
            let weak = Rc::downgrade(app);
            tray.on_new_note(move || {
                let Some(app) = weak.upgrade() else {
                    return;
                };
                if let Err(e) = App::create_note(&app) {
                    eprintln!("canvas: create failed: {e}");
                }
                App::refresh_home(&app);
            });
        }

        {
            let weak = Rc::downgrade(app);
            tray.on_quit_app(move || {
                if let Some(app) = weak.upgrade() {
                    App::quit(&app);
                }
            });
        }

        if let Err(e) = tray.show() {
            eprintln!("canvas: tray icon failed to show ({e})");
            return;
        }

        app.borrow_mut().tray = Some(tray);
        App::sync_tray(app);
    }

    pub fn tray_present(app: &SharedApp) -> bool {
        app.borrow().tray.is_some()
    }

    pub fn sync_tray(app: &SharedApp) {
        let Ok(a) = app.try_borrow() else {
            return;
        };
        let visible = a
            .home
            .as_ref()
            .map(|h| h.window().is_visible())
            .unwrap_or(false);
        if let Some(tray) = &a.tray {
            tray.set_home_open(visible);
        }
    }

    pub fn install_hotkeys(app: &SharedApp) {
        let weak = Rc::downgrade(app);
        let registered = crate::hotkey::install(move |id| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            match id {
                crate::hotkey::HOTKEY_PALETTE => App::show_palette(&app),
                crate::hotkey::HOTKEY_NEW_NOTE => {
                    if let Err(e) = App::create_note(&app) {
                        eprintln!("canvas: create failed: {e}");
                    }
                    App::refresh_home(&app);
                }
                _ => {}
            }
        });
        if !registered {
            eprintln!(
                "canvas: Ctrl+K / Ctrl+N are not registered globally — another app may hold them.\n\
                 canvas: Ctrl+K still works while a note's body has focus."
            );
        }
    }

    pub fn show_palette(app: &SharedApp) {
        let existing = app.borrow().palette.as_ref().map(|p| p.clone_strong());
        let palette = match existing {
            Some(palette) => palette,
            None => {
                let palette = match PaletteWindow::new() {
                    Ok(palette) => palette,
                    Err(e) => {
                        eprintln!("canvas: palette failed to open: {e}");
                        return;
                    }
                };
                palette
                    .global::<Typography>()
                    .set_system_sans(platform_system_sans().into());
                {
                    let weak = Rc::downgrade(app);
                    palette.on_dismissed(move || {
                        if let Some(app) = weak.upgrade() {
                            App::hide_palette(&app);
                        }
                    });
                }
                {
                    let weak = Rc::downgrade(app);
                    palette.on_activated(move |id| {
                        if let Some(app) = weak.upgrade() {
                            App::activate_palette_row(&app, id.as_ref());
                        }
                    });
                }
                {
                    let weak = Rc::downgrade(app);
                    palette.on_query_changed(move |query| {
                        if let Some(app) = weak.upgrade() {
                            App::refresh_palette(&app, query.as_ref());
                        }
                    });
                }
                app.borrow_mut().palette = Some(palette.clone_strong());
                palette
            }
        };

        palette.set_query(slint::SharedString::new());
        App::refresh_palette(app, "");

        if let Some(bounds) = crate::screen::primary_bounds() {
            let win = palette.window();
            win.set_size(slint::PhysicalSize::new(
                bounds.width as u32,
                bounds.height as u32,
            ));
            win.set_position(slint::PhysicalPosition::new(bounds.x, bounds.y));
        }

        if let Err(e) = palette.show() {
            eprintln!("canvas: palette failed to show: {e}");
            return;
        }
        crate::desktop_layer::take_foreground(palette.window());
    }

    pub fn refresh_palette(app: &SharedApp, query: &str) {
        let Ok(a) = app.try_borrow() else {
            return;
        };
        let Some(palette) = a.palette.as_ref().map(|p| p.clone_strong()) else {
            return;
        };

        let needle = query.trim().to_lowercase();
        let rows = if needle.is_empty() {
            drop(a);
            default_actions()
        } else {
            let widgets = a.store.list_widgets().unwrap_or_default();
            let matched: Vec<PaletteRow> = widgets
                .iter()
                .filter(|w| w.title.to_lowercase().contains(&needle))
                .map(|w| PaletteRow {
                    id: w.id.clone().into(),
                    label: w.title.clone().into(),
                    meta: if a.notes.contains_key(&w.id) {
                        "OPEN".into()
                    } else {
                        slint::SharedString::new()
                    },
                    icon: "note".into(),
                })
                .collect();
            drop(a);
            matched
        };

        palette.set_count_label(count_label(rows.len()).into());
        palette.set_selected(0);
        let model: slint::VecModel<PaletteRow> = slint::VecModel::from(rows);
        palette.set_rows(slint::ModelRc::new(model));
    }

    pub fn activate_palette_row(app: &SharedApp, id: &str) {
        match id {
            ACTION_NEW_NOTE => {
                App::hide_palette(app);
                if let Err(e) = App::create_note(app) {
                    eprintln!("canvas: create failed: {e}");
                }
                App::refresh_home(app);
            }
            ACTION_NEW_TODO => {
                eprintln!("canvas: todo widgets do not exist yet (Stage W, SW-01)");
            }
            ACTION_SETTINGS => {
                eprintln!("canvas: there is no settings screen yet");
            }
            _ => {
                App::hide_palette(app);
                let _ = app.borrow().store.set_open(id, true);
                if let Err(e) = App::open_note(app, id) {
                    eprintln!("canvas: open failed: {e}");
                }
                App::refresh_home(app);
            }
        }
    }

    pub fn hide_palette(app: &SharedApp) {
        if let Some(palette) = &app.borrow().palette {
            let _ = palette.hide();
        }
    }

    pub fn hide_home(app: &SharedApp) {
        let Some(home) = app.borrow().home.as_ref().map(|h| h.clone_strong()) else {
            return;
        };
        let weak = Rc::downgrade(app);
        play_close(home, move || {
            if let Some(app) = weak.upgrade() {
                App::sync_tray(&app);
                crate::maybe_quit(&app);
            }
        });
    }

    pub fn show_home(app: &SharedApp) {
        if let Err(e) = App::open_home(app) {
            eprintln!("canvas: home failed to open: {e}");
        }
        App::refresh_home(app);
        App::sync_tray(app);
    }

    pub fn quit(app: &SharedApp) {
        App::flush_all(app);
        {
            let mut a = app.borrow_mut();
            for note in a.notes.values() {
                let _ = note.window.hide();
            }
            if let Some(home) = &a.home {
                let _ = home.hide();
            }
            if let Some(palette) = &a.palette {
                let _ = palette.hide();
            }
            if let Some(tray) = &a.tray {
                let _ = tray.hide();
            }
            a.tray = None;
        }
        let _ = slint::quit_event_loop();
    }
}

#[cfg(test)]
mod tests {
    use super::{count_label, default_actions, ACTION_NEW_NOTE, ACTION_NEW_TODO, ACTION_SETTINGS};

    #[test]
    fn count_label_is_singular_plural_and_empty() {
        assert_eq!(count_label(0), "NO RESULTS");
        assert_eq!(count_label(1), "1 RESULT");
        assert_eq!(count_label(7), "7 RESULTS");
    }

    #[test]
    fn empty_query_offers_the_three_spec_actions() {
        let rows = default_actions();
        let ids: Vec<String> = rows.iter().map(|r| r.id.to_string()).collect();
        assert_eq!(ids, vec![ACTION_NEW_NOTE, ACTION_NEW_TODO, ACTION_SETTINGS]);
        assert!(rows.iter().all(|r| r.meta.starts_with("CTRL+")));
    }
}
