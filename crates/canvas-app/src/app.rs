use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use slint::ComponentHandle;

use canvas_doc::{KanbanDoc, StickyNoteDoc, TodoListDoc, UndoHistory};

use crate::widget::{LiveWidget, WidgetDoc, WidgetWindow};
use crate::widget_types;
use canvas_storage::{Store, WindowPlacement, KIND_STICKY_NOTE};
use canvas_ui::{
    platform_system_sans, CalendarChip, CalendarWindow, CanvasTray, DayCell, HomeWindow,
    KanbanCardData, KanbanColumnData, KanbanWindow, Motion, NoteRow, PaletteRow, PaletteWindow,
    StickyNoteWindow, TodoItemData, TodoWindow, Typography, WidgetTypeData,
};

pub const ACTION_SETTINGS: &str = "settings";

const PLATE_MARGIN: f32 = 24.0;
const FLUSH_INTERVAL: Duration = Duration::from_millis(500);
const PIN_POLL_INTERVAL: Duration = Duration::from_millis(700);
const OPEN_DELAY: Duration = Duration::from_millis(16);
const CLOSE_FADE: Duration = Duration::from_millis(100);
const PALETTE_RELEASE: Duration = Duration::from_millis(16);
const KEEP_VISIBLE_W: f32 = 120.0;
const KEEP_VISIBLE_H: f32 = 48.0;
const NEW_CARD_TITLE: &str = "New card";

pub const OPEN_FLAG: &str = "--open";
pub const OPEN_KIND_FLAG: &str = "--open-kind";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenRequest {
    Nothing,
    Id(String),
    Kind(&'static str),
}

impl OpenRequest {
    pub fn from_args<I, S>(args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let args: Vec<String> = args.into_iter().map(|a| a.as_ref().to_string()).collect();
        let mut found = OpenRequest::Nothing;
        let mut i = 0;
        while i < args.len() {
            let flag = args[i].as_str();
            if flag != OPEN_FLAG && flag != OPEN_KIND_FLAG {
                i += 1;
                continue;
            }
            let value = args
                .get(i + 1)
                .filter(|v| !v.starts_with("--"))
                .ok_or_else(|| match flag {
                    OPEN_KIND_FLAG => format!(
                        "{OPEN_KIND_FLAG} needs a widget type. One of: {}",
                        widget_types::kind_words()
                    ),
                    _ => format!("{OPEN_FLAG} needs a widget id. Run --dump to list them"),
                })?;
            let next = if flag == OPEN_KIND_FLAG {
                let kind = widget_types::resolve_kind(value).ok_or_else(|| {
                    format!(
                        "{value:?} is not a built widget type. One of: {}",
                        widget_types::kind_words()
                    )
                })?;
                OpenRequest::Kind(kind)
            } else {
                OpenRequest::Id(value.clone())
            };
            if found != OpenRequest::Nothing {
                return Err(format!(
                    "{OPEN_FLAG} and {OPEN_KIND_FLAG} name one widget between them, not two"
                ));
            }
            found = next;
            i += 2;
        }
        Ok(found)
    }
}

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

type DragAnchor = Rc<std::cell::Cell<Option<(i32, i32)>>>;

fn begin_drag(window: &slint::Window, anchor: &DragAnchor) {
    let pos = window.position();
    anchor.set(crate::desktop_layer::cursor_pos().map(|(cx, cy)| (cx - pos.x, cy - pos.y)));
}

fn drag_move(
    window: &slint::Window,
    anchor: &DragAnchor,
    dx: f32,
    dy: f32,
) -> slint::PhysicalPosition {
    let scale = window.scale_factor();
    let pos = window.position();
    let target = match (anchor.get(), crate::desktop_layer::cursor_pos()) {
        (Some((ax, ay)), Some((cx, cy))) => slint::PhysicalPosition::new(cx - ax, cy - ay),
        _ => slint::PhysicalPosition::new(
            pos.x + (dx * scale).round() as i32,
            pos.y + (dy * scale).round() as i32,
        ),
    };
    let next = clamp_to_screen(target, window.size(), scale);
    if next.x != pos.x || next.y != pos.y {
        window.set_position(next);
    }
    next
}

fn fill_screen(window: &slint::Window, bounds: &crate::screen::Bounds, scale: f32) {
    let scale = if scale > 0.0 { scale } else { 1.0 };
    window.set_size(slint::LogicalSize::new(
        bounds.width as f32 / scale,
        bounds.height as f32 / scale,
    ));
    window.set_position(slint::PhysicalPosition::new(bounds.x, bounds.y));
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

pub struct App {
    store: Store,
    widgets: HashMap<String, LiveWidget>,
    home: Option<HomeWindow>,
    palette: Option<PaletteWindow>,
    reduced_motion: bool,
    tray: Option<CanvasTray>,
}

pub type SharedApp = Rc<RefCell<App>>;

impl App {
    pub fn boot(request: OpenRequest) -> Result<SharedApp, Box<dyn std::error::Error>> {
        let dir = Store::default_path();
        std::fs::create_dir_all(&dir)?;
        let store = Store::open(&dir.join("canvas_local.db"))?;

        let reduced_motion = crate::reduced_motion::wanted();
        eprintln!("canvas: reduced motion = {reduced_motion}");

        let app: SharedApp = Rc::new(RefCell::new(App {
            store,
            widgets: HashMap::new(),
            home: None,
            palette: None,
            reduced_motion,
            tray: None,
        }));

        if let Err(e) = App::honour_open_request(&app, &request) {
            eprintln!("canvas: --open ignored: {e}");
        }

        let to_open: Vec<String> = {
            let a = app.borrow();
            a.store
                .list_widgets()?
                .into_iter()
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
            App::open_widget(&app, &id)?;
        }

        App::open_home(&app)?;
        App::install_tray(&app);
        App::install_hotkeys(&app);
        App::start_flush_timer(&app);
        Ok(app)
    }

    fn honour_open_request(
        app: &SharedApp,
        request: &OpenRequest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let id = match request {
            OpenRequest::Nothing => return Ok(()),
            OpenRequest::Id(id) => {
                let a = app.borrow();
                a.store
                    .get_widget(id)?
                    .ok_or_else(|| format!("no widget {id} in this database"))?;
                id.clone()
            }
            OpenRequest::Kind(kind) => {
                let existing = {
                    let a = app.borrow();
                    a.store
                        .list_widgets()?
                        .into_iter()
                        .rev()
                        .find(|w| w.widget_type == *kind)
                        .map(|w| w.id)
                };
                match existing {
                    Some(id) => id,
                    None => App::insert_widget(app, kind)?,
                }
            }
        };
        app.borrow().store.set_open(&id, true)?;
        eprintln!("canvas: --open will show widget {}", &id[..8.min(id.len())]);
        Ok(())
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
        home.global::<Motion>()
            .set_reduced(app.borrow().reduced_motion);

        home.set_types(built_types_model());

        {
            let weak = Rc::downgrade(app);
            home.on_create_widget(move |kind| {
                if let Some(app) = weak.upgrade() {
                    if let Err(e) = App::create_widget(&app, kind.as_ref()) {
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
                let is_open = app.borrow().widgets.contains_key(&id);
                if is_open {
                    App::close_widget(&app, &id);
                } else {
                    let _ = app.borrow().store.set_open(&id, true);
                    if let Err(e) = App::open_widget(&app, &id) {
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
                    App::delete_widget(&app, id.as_ref());
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
            let anchor: DragAnchor = Rc::new(std::cell::Cell::new(None));
            {
                let anchor = anchor.clone();
                let handle = home.as_weak();
                home.on_drag_started(move || {
                    if let Some(home) = handle.upgrade() {
                        begin_drag(home.window(), &anchor);
                    }
                });
            }
            {
                let anchor = anchor.clone();
                home.on_drag_ended(move || anchor.set(None));
            }
            let handle = home.as_weak();
            home.on_window_moved(move |dx, dy| {
                let Some(home) = handle.upgrade() else {
                    return;
                };
                drag_move(home.window(), &anchor, dx, dy);
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
                let open = a.widgets.contains_key(&w.id);
                let preview = if let Some(widget) = a.widgets.get(&w.id) {
                    widget.doc.text()
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

        let open_count = a.widgets.len() as i32;
        drop(a);

        let model: slint::VecModel<NoteRow> = slint::VecModel::from(rows);
        home.set_rows(slint::ModelRc::new(model));
        home.set_open_count(open_count);
    }

    pub fn create_note(app: &SharedApp) -> Result<String, Box<dyn std::error::Error>> {
        App::create_widget(app, KIND_STICKY_NOTE)
    }

    fn insert_widget(app: &SharedApp, kind: &str) -> Result<String, Box<dyn std::error::Error>> {
        let a = app.borrow();
        let (row, _) = a.store.create_widget(kind)?;
        if let Some(doc) = WidgetDoc::seed(kind) {
            a.store
                .save_document(&row.id, &doc.snapshot(), &doc.state_vector())?;
        }
        Ok(row.id)
    }

    pub fn create_widget(
        app: &SharedApp,
        kind: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let id = App::insert_widget(app, kind)?;
        App::open_widget(app, &id)?;
        Ok(id)
    }

    pub fn open_widget(app: &SharedApp, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let existing = app.borrow().widgets.get(id).map(|w| w.window.clone());
        if let Some(window) = existing {
            window.play_open();
            window.show()?;
            clamp_window_onto_screen(window.slint());
            return Ok(());
        }

        let (kind, title, placement, doc, history) = {
            let a = app.borrow();
            let row = a
                .store
                .get_widget(id)?
                .ok_or_else(|| format!("no widget {id}"))?;
            let placement = a
                .store
                .get_placement(id)?
                .unwrap_or_else(|| WindowPlacement::default_for(&row.widget_type));
            let snapshot = a.store.get_document(id)?;
            let Some(doc) = WidgetDoc::load(&row.widget_type, snapshot.as_deref()) else {
                drop(a);
                eprintln!(
                    "canvas: {id} is a {:?} widget and that type has no document yet (Stage W)",
                    row.widget_type
                );
                return Ok(());
            };
            let revisions = a.store.revisions_with_time(id).unwrap_or_default();
            let history = UndoHistory::from_revisions(&revisions, &doc.text());
            (row.widget_type, row.title, placement, doc, history)
        };

        let reduced = app.borrow().reduced_motion;
        let Some(built) = WidgetWindow::create(&kind, reduced) else {
            eprintln!(
                "canvas: {id} is a {kind:?} widget and that type has no window yet (Stage W)"
            );
            return Ok(());
        };
        let window = built?;

        window.set_title(&title);
        window.set_text(&doc.text());
        window.apply_placement(&placement);

        window.slint().set_position(slint::LogicalPosition::new(
            placement.x as f32 - PLATE_MARGIN,
            placement.y as f32 - PLATE_MARGIN,
        ));
        window.slint().set_size(slint::LogicalSize::new(
            placement.width as f32 + 2.0 * PLATE_MARGIN,
            placement.height as f32 + 2.0 * PLATE_MARGIN,
        ));

        app.borrow_mut().widgets.insert(
            id.to_string(),
            LiveWidget {
                window: window.clone(),
                doc,
                history,
                placement,
                pending_updates: Vec::new(),
                text_dirty: false,
                placement_dirty: false,
                poll_timer: None,
            },
        );

        let poll_timer = match &window {
            WidgetWindow::StickyNote(note) => wire_note(app, id, note),
            WidgetWindow::Todo(todo) => wire_todo(app, id, todo),
            WidgetWindow::Calendar(calendar) => wire_calendar(app, id, calendar),
            WidgetWindow::Kanban(kanban) => wire_kanban(app, id, kanban),
        };
        if let Some(widget) = app.borrow_mut().widgets.get_mut(id) {
            widget.poll_timer = Some(poll_timer);
        }
        if matches!(window, WidgetWindow::Todo(_)) {
            refresh_todo(app, id);
        }
        if matches!(window, WidgetWindow::Calendar(_)) {
            refresh_calendar(app, id);
        }
        if matches!(window, WidgetWindow::Kanban(_)) {
            refresh_kanban(app, id);
        }
        window.play_open();
        window.show()?;
        let scale = window.slint().scale_factor();
        window.scale_resize_border();

        if let Some(moved) = clamp_window_onto_screen(window.slint()) {
            let logical = moved.to_logical(scale);
            let mut a = app.borrow_mut();
            if let Some(widget) = a.widgets.get_mut(id) {
                widget.placement.x = (logical.x + PLATE_MARGIN).round() as i32;
                widget.placement.y = (logical.y + PLATE_MARGIN).round() as i32;
                widget.placement_dirty = true;
            }
        }
        Ok(())
    }
    pub fn close_widget(app: &SharedApp, id: &str) {
        let mut a = app.borrow_mut();
        if let Some(mut widget) = a.widgets.remove(id) {
            widget.window.play_close();
            for update in widget.pending_updates.drain(..) {
                let _ = a.store.append_revision(id, &update);
            }
            let _ = a
                .store
                .save_document(id, &widget.doc.snapshot(), &widget.doc.state_vector());
            let _ = a.store.save_placement(id, &widget.placement);
        }
        let _ = a.store.set_open(id, false);
    }

    pub fn delete_widget(app: &SharedApp, id: &str) {
        let mut a = app.borrow_mut();
        if let Some(widget) = a.widgets.remove(id) {
            widget.window.play_close();
        }
        let _ = a.store.delete_widget(id);
    }

    pub fn undo_widget(app: &SharedApp, id: &str) {
        App::step_history(app, id, true);
    }

    pub fn redo_widget(app: &SharedApp, id: &str) {
        App::step_history(app, id, false);
    }

    fn step_history(app: &SharedApp, id: &str, backwards: bool) {
        let restored = {
            let Ok(mut a) = app.try_borrow_mut() else {
                return;
            };
            let Some(widget) = a.widgets.get_mut(id) else {
                return;
            };
            let step = if backwards {
                widget.history.undo()
            } else {
                widget.history.redo()
            };
            let Some(target) = step.map(str::to_string) else {
                return;
            };
            if let Some(update) = widget.doc.set_text(&target) {
                widget.pending_updates.push(update);
                widget.text_dirty = true;
            }
            (widget.window.clone(), target)
        };
        restored.0.set_text(&restored.1);
        App::refresh_home(app);
    }

    pub fn open_count(app: &SharedApp) -> usize {
        app.borrow().widgets.len()
    }

    fn refresh_reduced_motion(a: &mut App) {
        let wanted = crate::reduced_motion::wanted();
        if wanted == a.reduced_motion {
            return;
        }
        a.reduced_motion = wanted;
        if let Some(home) = a.home.as_ref() {
            home.global::<Motion>().set_reduced(wanted);
        }
        if let Some(palette) = a.palette.as_ref() {
            palette.global::<Motion>().set_reduced(wanted);
        }
        for widget in a.widgets.values() {
            widget.window.set_reduced_motion(wanted);
        }
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
            App::refresh_reduced_motion(&mut a);
            let App { store, widgets, .. } = &mut *a;
            for (id, widget) in widgets.iter_mut() {
                if widget.text_dirty {
                    for update in widget.pending_updates.drain(..) {
                        let _ = store.append_revision(id, &update);
                    }
                    let _ =
                        store.save_document(id, &widget.doc.snapshot(), &widget.doc.state_vector());
                    let _ = store.touch(id);
                    widget.history.checkpoint(&widget.doc.text());
                    widget.text_dirty = false;
                }
                if widget.placement_dirty {
                    let _ = store.save_placement(id, &widget.placement);
                    widget.placement_dirty = false;
                }
            }
        });
    }

    pub fn flush_all(app: &SharedApp) {
        let mut a = app.borrow_mut();
        let App { store, widgets, .. } = &mut *a;
        for (id, widget) in widgets.iter_mut() {
            for update in widget.pending_updates.drain(..) {
                let _ = store.append_revision(id, &update);
            }
            let _ = store.save_document(id, &widget.doc.snapshot(), &widget.doc.state_vector());
            let _ = store.save_placement(id, &widget.placement);
            widget.history.checkpoint(&widget.doc.text());
            widget.text_dirty = false;
            widget.placement_dirty = false;
        }
    }
}

fn rows_locked_by_editor(editing_id: &str) -> bool {
    !editing_id.is_empty()
}

fn end_todo_row_edit(handle: &slint::Weak<TodoWindow>) {
    if let Some(window) = handle.upgrade() {
        window.set_editing_id(slint::SharedString::new());
    }
}

fn end_kanban_card_edit(handle: &slint::Weak<KanbanWindow>) {
    if let Some(window) = handle.upgrade() {
        window.set_editing_id(slint::SharedString::new());
    }
}

fn refresh_todo(app: &SharedApp, id: &str) {
    let Ok(a) = app.try_borrow() else {
        return;
    };
    let Some(widget) = a.widgets.get(id) else {
        return;
    };
    let (WidgetDoc::Todo(doc), WidgetWindow::Todo(window)) = (&widget.doc, &widget.window) else {
        return;
    };
    let (open, done) = doc.counts();
    let rows: Vec<TodoItemData> = doc
        .items()
        .into_iter()
        .map(|item| TodoItemData {
            id: item.id.into(),
            text: item.text.into(),
            done: item.done,
            due: slint::SharedString::new(),
        })
        .collect();
    let window = window.clone_strong();
    drop(a);

    window.set_open_count(open as i32);
    window.set_done_count(done as i32);
    if rows_locked_by_editor(window.get_editing_id().as_ref()) {
        return;
    }
    let model: slint::VecModel<TodoItemData> = slint::VecModel::from(rows);
    window.set_items(slint::ModelRc::new(model));
}

fn todo_edit(app: &SharedApp, id: &str, change: impl FnOnce(&TodoListDoc) -> Option<Vec<u8>>) {
    {
        let Ok(mut a) = app.try_borrow_mut() else {
            return;
        };
        let Some(widget) = a.widgets.get_mut(id) else {
            return;
        };
        let WidgetDoc::Todo(doc) = &widget.doc else {
            return;
        };
        let Some(update) = change(doc) else {
            return;
        };
        widget.pending_updates.push(update);
        widget.text_dirty = true;
    }
    refresh_todo(app, id);
    App::refresh_home(app);
}

fn wire_todo(app: &SharedApp, id: &str, window: &TodoWindow) -> slint::Timer {
    let owned_id = id.to_string();
    let poll_timer = slint::Timer::default();

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        window.on_item_added(move |text| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            let text = text.trim().to_string();
            if text.is_empty() {
                return;
            }
            end_todo_row_edit(&handle);
            todo_edit(&app, &id, |doc| Some(doc.add_item(&text).1));
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        window.on_item_toggled(move |item_id| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            end_todo_row_edit(&handle);
            todo_edit(&app, &id, |doc| doc.toggle_item(item_id.as_ref()));
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.on_item_text_edited(move |item_id, text| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            todo_edit(&app, &id, |doc| {
                doc.set_text(item_id.as_ref(), text.as_ref())
            });
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        window.on_item_removed(move |item_id| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            end_todo_row_edit(&handle);
            todo_edit(&app, &id, |doc| doc.delete_item(item_id.as_ref()));
        });
    }

    {
        let handle = window.as_weak();
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.on_edit_committed(move || {
            if let Some(window) = handle.upgrade() {
                window.set_editing_id(slint::SharedString::new());
            }
            if let Some(app) = weak.upgrade() {
                refresh_todo(&app, &id);
            }
        });
    }

    let anchor: DragAnchor = Rc::new(std::cell::Cell::new(None));

    {
        let anchor = anchor.clone();
        let handle = window.as_weak();
        window.on_drag_started(move || {
            if let Some(window) = handle.upgrade() {
                begin_drag(window.window(), &anchor);
            }
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let anchor = anchor.clone();
        window.on_drag_ended(move || {
            anchor.set(None);
            let Some(app) = weak.upgrade() else {
                return;
            };
            let Ok(mut a) = app.try_borrow_mut() else {
                return;
            };
            if let Some(widget) = a.widgets.get_mut(&id) {
                widget.placement_dirty = true;
            }
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let anchor = anchor.clone();
        let handle = window.as_weak();
        window.on_window_moved(move |dx, dy| {
            let Some(window) = handle.upgrade() else {
                return;
            };
            let win = window.window();
            let scale = win.scale_factor();
            let next = drag_move(win, &anchor, dx, dy);

            let Some(app) = weak.upgrade() else {
                return;
            };
            let Ok(mut a) = app.try_borrow_mut() else {
                return;
            };
            if let Some(widget) = a.widgets.get_mut(&id) {
                let logical = next.to_logical(scale);
                widget.placement.x = (logical.x + PLATE_MARGIN).round() as i32;
                widget.placement.y = (logical.y + PLATE_MARGIN).round() as i32;
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
            if let Some(widget) = a.widgets.get_mut(&id) {
                widget.placement.pinned = pinned;
                widget.placement_dirty = true;
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
            App::close_widget(&app, &id);
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
            App::delete_widget(&app, &id);
            App::refresh_home(&app);
            crate::maybe_quit(&app);
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.window().on_close_requested(move || {
            if let Some(app) = weak.upgrade() {
                App::close_widget(&app, &id);
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
        poll_timer.start(slint::TimerMode::Repeated, PIN_POLL_INTERVAL, move || {
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
            if let Some(widget) = a.widgets.get_mut(&id) {
                let size = window
                    .window()
                    .size()
                    .to_logical(window.window().scale_factor());
                let w = (size.width - 2.0 * PLATE_MARGIN).round() as i32;
                let h = (size.height - 2.0 * PLATE_MARGIN).round() as i32;
                if w != widget.placement.width || h != widget.placement.height {
                    widget.placement.width = w;
                    widget.placement.height = h;
                    widget.placement_dirty = true;
                }
            }
        });
    }

    poll_timer
}

fn wire_note(app: &SharedApp, id: &str, window: &StickyNoteWindow) -> slint::Timer {
    let owned_id = id.to_string();
    let pin_timer = slint::Timer::default();

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.on_text_edited(move |text| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            let mut a = app.borrow_mut();
            let App { store, widgets, .. } = &mut *a;
            let Some(widget) = widgets.get_mut(&id) else {
                return;
            };
            if let Some(update) = widget.doc.set_text(&text) {
                widget.pending_updates.push(update);
                widget.text_dirty = true;
            }
            let _ = store;
        });
    }

    let anchor: DragAnchor = Rc::new(std::cell::Cell::new(None));

    {
        let anchor = anchor.clone();
        let handle = window.as_weak();
        window.on_drag_started(move || {
            if let Some(window) = handle.upgrade() {
                begin_drag(window.window(), &anchor);
            }
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let anchor = anchor.clone();
        window.on_drag_ended(move || {
            anchor.set(None);
            let Some(app) = weak.upgrade() else {
                return;
            };
            let Ok(mut a) = app.try_borrow_mut() else {
                return;
            };
            if let Some(widget) = a.widgets.get_mut(&id) {
                widget.placement_dirty = true;
            }
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let anchor = anchor.clone();
        let handle = window.as_weak();
        window.on_window_moved(move |dx, dy| {
            let Some(window) = handle.upgrade() else {
                return;
            };
            let win = window.window();
            let scale = win.scale_factor();
            let next = drag_move(win, &anchor, dx, dy);

            let Some(app) = weak.upgrade() else {
                return;
            };
            let Ok(mut a) = app.try_borrow_mut() else {
                return;
            };
            if let Some(widget) = a.widgets.get_mut(&id) {
                let logical = next.to_logical(scale);
                widget.placement.x = (logical.x + PLATE_MARGIN).round() as i32;
                widget.placement.y = (logical.y + PLATE_MARGIN).round() as i32;
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
            if let Some(widget) = a.widgets.get_mut(&id) {
                widget.placement.pinned = pinned;
                widget.placement_dirty = true;
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
            App::close_widget(&app, &id);
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
        window.on_undo_requested(move || {
            if let Some(app) = weak.upgrade() {
                App::undo_widget(&app, &id);
            }
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.on_redo_requested(move || {
            if let Some(app) = weak.upgrade() {
                App::redo_widget(&app, &id);
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
            App::delete_widget(&app, &id);
            App::refresh_home(&app);
            crate::maybe_quit(&app);
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.window().on_close_requested(move || {
            if let Some(app) = weak.upgrade() {
                App::close_widget(&app, &id);
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
        pin_timer.start(slint::TimerMode::Repeated, PIN_POLL_INTERVAL, move || {
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
            if let Some(widget) = a.widgets.get_mut(&id) {
                let size = window
                    .window()
                    .size()
                    .to_logical(window.window().scale_factor());
                let w = (size.width - 2.0 * PLATE_MARGIN).round() as i32;
                let h = (size.height - 2.0 * PLATE_MARGIN).round() as i32;
                if w != widget.placement.width || h != widget.placement.height {
                    widget.placement.width = w;
                    widget.placement.height = h;
                    widget.placement_dirty = true;
                }
            }
        });
    }

    pin_timer
}

pub fn dump() -> Result<(), Box<dyn std::error::Error>> {
    let dir = Store::default_path();
    let store = Store::open(&dir.join("canvas_local.db"))?;
    let widgets = store.list_widgets()?;
    println!("db: {}", dir.join("canvas_local.db").display());
    println!("widgets: {}", widgets.len());
    for w in widgets {
        let placement = store.get_placement(&w.id)?;
        let snapshot = store.get_document(&w.id)?;
        let text = WidgetDoc::load(&w.widget_type, snapshot.as_deref())
            .map(|doc| doc.text())
            .unwrap_or_else(|| "<no doc type for this widget>".to_string());
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

fn built_types_model() -> slint::ModelRc<WidgetTypeData> {
    let rows: Vec<WidgetTypeData> = widget_types::BUILT
        .iter()
        .map(|t| WidgetTypeData {
            kind: t.kind.into(),
            label: t.label.into(),
        })
        .collect();
    slint::ModelRc::new(slint::VecModel::from(rows))
}

fn default_actions() -> Vec<PaletteRow> {
    let mut rows: Vec<PaletteRow> = widget_types::BUILT
        .iter()
        .map(|t| PaletteRow {
            id: widget_types::action_id(t.kind).into(),
            label: widget_types::palette_label(t).into(),
            meta: t.shortcut.into(),
            icon: t.kind.into(),
        })
        .collect();
    rows.push(PaletteRow {
        id: ACTION_SETTINGS.into(),
        label: "Settings".into(),
        meta: "CTRL+,".into(),
        icon: "settings".into(),
    });
    rows
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

    fn known_scale(app: &SharedApp) -> f32 {
        let Ok(a) = app.try_borrow() else {
            return 1.0;
        };
        if let Some(home) = &a.home {
            return home.window().scale_factor();
        }
        a.widgets
            .values()
            .next()
            .map(|w| w.window.slint().scale_factor())
            .unwrap_or(1.0)
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
                palette
                    .global::<Motion>()
                    .set_reduced(app.borrow().reduced_motion);
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

        let bounds = crate::screen::primary_bounds();
        if let Some(bounds) = bounds {
            fill_screen(palette.window(), &bounds, App::known_scale(app));
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
                    meta: if a.widgets.contains_key(&w.id) {
                        "OPEN".into()
                    } else {
                        slint::SharedString::new()
                    },
                    icon: w.widget_type.clone().into(),
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
            ACTION_SETTINGS => {
                eprintln!("canvas: there is no settings screen yet");
            }
            _ => {
                App::hide_palette(app);
                if let Some(kind) = widget_types::kind_for_action(id) {
                    if let Err(e) = App::create_widget(app, kind) {
                        eprintln!("canvas: create failed: {e}");
                    }
                    App::refresh_home(app);
                    return;
                }
                let _ = app.borrow().store.set_open(id, true);
                if let Err(e) = App::open_widget(app, id) {
                    eprintln!("canvas: open failed: {e}");
                }
                App::refresh_home(app);
            }
        }
    }

    pub fn hide_palette(app: &SharedApp) {
        let taken = app.borrow_mut().palette.take();
        let Some(palette) = taken else {
            return;
        };
        let _ = palette.hide();
        slint::Timer::single_shot(PALETTE_RELEASE, move || drop(palette));
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
            for widget in a.widgets.values() {
                widget.window.hide();
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

pub fn gallery() -> Result<(), Box<dyn std::error::Error>> {
    use crate::calendar_time as ct;

    let window = canvas_ui::GalleryWindow::new()?;
    window
        .global::<canvas_ui::Typography>()
        .set_system_sans(canvas_ui::platform_system_sans().into());
    window
        .global::<canvas_ui::Motion>()
        .set_reduced(crate::reduced_motion::wanted());

    let offset = ct::local_offset_ms();
    let today = (canvas_storage::now_millis() + offset).div_euclid(ct::DAY_MS);
    let shown = Rc::new(std::cell::Cell::new({
        let (y, m, _) = ct::civil_from_days(today);
        (y, m)
    }));

    window.set_pick_day(today as i32);
    window.set_pick_heads(slint::ModelRc::new(slint::VecModel::from(weekday_heads(
        FIRST_WEEKDAY,
    ))));
    paint_month(&window, shown.get(), today);

    {
        let handle = window.as_weak();
        let shown = shown.clone();
        window.on_pick_month_stepped(move |delta| {
            let Some(window) = handle.upgrade() else {
                return;
            };
            let (y, m) = shown.get();
            shown.set(ct::month_step(y, m, delta));
            paint_month(&window, shown.get(), today);
        });
    }

    {
        let handle = window.as_weak();
        window.on_pick_committed(move || {
            let Some(window) = handle.upgrade() else {
                return;
            };
            let (y, m, d) = ct::civil_from_days(window.get_pick_day() as i64);
            let minute = window.get_pick_minute() as i64;
            let ts = ct::compose_local_ms(y, m, d, minute, offset);
            let (ry, rm, rd) = ct::local_civil(ts, offset);
            window.set_pick_result(
                format!(
                    "PICKED {} {} · EPOCH {ts} · BACK {ry}-{rm:02}-{rd:02} {}",
                    ct::date_stamp(ts, offset),
                    ct::hhmm(ts, offset),
                    ct::hhmm(ts, offset)
                )
                .into(),
            );
        });
    }

    window.show()?;
    slint::run_event_loop()?;
    Ok(())
}

const FIRST_WEEKDAY: u32 = 1;

fn weekday_heads(first_weekday: u32) -> Vec<slint::SharedString> {
    use crate::calendar_time as ct;
    (0..7)
        .map(|i| {
            ct::weekday_initial((first_weekday + i) % ct::DAYS_PER_WEEK as u32)
                .to_string()
                .into()
        })
        .collect()
}

fn paint_month(window: &canvas_ui::GalleryWindow, shown: (i64, u32), today: i64) {
    use crate::calendar_time as ct;

    let (year, month) = shown;
    window.set_pick_month(format!("{} {year}", ct::month_name(month)).into());
    window.set_pick_stamp(ct::date_stamp(window.get_pick_day() as i64 * ct::DAY_MS, 0).into());

    let cells: Vec<DayCell> = ct::month_grid(year, month, FIRST_WEEKDAY)
        .into_iter()
        .map(|day| {
            let (y, m, d) = ct::civil_from_days(day);
            DayCell {
                label: d.to_string().into(),
                epoch_day: day as i32,
                in_month: (y, m) == (year, month),
                today: day == today,
            }
        })
        .collect();
    window.set_pick_days(slint::ModelRc::new(slint::VecModel::from(cells)));
}

fn refresh_calendar(app: &SharedApp, id: &str) {
    use crate::calendar_time as ct;

    let Ok(a) = app.try_borrow() else {
        return;
    };
    let Some(widget) = a.widgets.get(id) else {
        return;
    };
    let (WidgetDoc::Calendar(doc), WidgetWindow::Calendar(window)) = (&widget.doc, &widget.window)
    else {
        return;
    };

    let offset = ct::local_offset_ms();
    let now = canvas_storage::now_millis();
    let week_start = ct::week_start_utc(now, offset);
    let week_end = week_start + ct::DAYS_PER_WEEK * ct::DAY_MS;
    let today = ct::day_index(now, week_start, offset);

    let chips: Vec<CalendarChip> = doc
        .events_between(week_start, week_end)
        .into_iter()
        .map(|event| {
            let (height, tone) = ct::chip_bucket(event.duration());
            let shown_from = event.start.max(week_start);
            CalendarChip {
                id: event.id.into(),
                label: ct::hhmm(event.start, offset).into(),
                day: ct::day_index(shown_from, week_start, offset),
                top: ct::day_fraction(shown_from, offset),
                height,
                tone,
            }
        })
        .collect();

    let window = window.clone_strong();
    drop(a);

    let model: slint::VecModel<CalendarChip> = slint::VecModel::from(chips);
    window.set_chips(slint::ModelRc::new(model));
    window.set_today(today);
}

fn wire_calendar(app: &SharedApp, id: &str, window: &CalendarWindow) -> slint::Timer {
    let owned_id = id.to_string();
    let poll_timer = slint::Timer::default();

    let anchor: DragAnchor = Rc::new(std::cell::Cell::new(None));

    {
        let anchor = anchor.clone();
        let handle = window.as_weak();
        window.on_drag_started(move || {
            if let Some(window) = handle.upgrade() {
                begin_drag(window.window(), &anchor);
            }
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let anchor = anchor.clone();
        window.on_drag_ended(move || {
            anchor.set(None);
            let Some(app) = weak.upgrade() else {
                return;
            };
            let Ok(mut a) = app.try_borrow_mut() else {
                return;
            };
            if let Some(widget) = a.widgets.get_mut(&id) {
                widget.placement_dirty = true;
            }
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let anchor = anchor.clone();
        let handle = window.as_weak();
        window.on_window_moved(move |dx, dy| {
            let Some(window) = handle.upgrade() else {
                return;
            };
            let win = window.window();
            let scale = win.scale_factor();
            let next = drag_move(win, &anchor, dx, dy);

            let Some(app) = weak.upgrade() else {
                return;
            };
            let Ok(mut a) = app.try_borrow_mut() else {
                return;
            };
            if let Some(widget) = a.widgets.get_mut(&id) {
                let logical = next.to_logical(scale);
                widget.placement.x = (logical.x + PLATE_MARGIN).round() as i32;
                widget.placement.y = (logical.y + PLATE_MARGIN).round() as i32;
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
            if let Some(widget) = a.widgets.get_mut(&id) {
                widget.placement.pinned = pinned;
                widget.placement_dirty = true;
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
            App::close_widget(&app, &id);
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
            App::delete_widget(&app, &id);
            App::refresh_home(&app);
            crate::maybe_quit(&app);
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.window().on_close_requested(move || {
            if let Some(app) = weak.upgrade() {
                App::close_widget(&app, &id);
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
        poll_timer.start(slint::TimerMode::Repeated, PIN_POLL_INTERVAL, move || {
            let Some(window) = handle.upgrade() else {
                return;
            };
            if window.get_pinned() {
                crate::desktop_layer::hold_at_desktop_layer(window.window());
            }

            let Some(app) = weak.upgrade() else {
                return;
            };
            {
                let Ok(mut a) = app.try_borrow_mut() else {
                    return;
                };
                if let Some(widget) = a.widgets.get_mut(&id) {
                    let size = window
                        .window()
                        .size()
                        .to_logical(window.window().scale_factor());
                    let w = (size.width - 2.0 * PLATE_MARGIN).round() as i32;
                    let h = (size.height - 2.0 * PLATE_MARGIN).round() as i32;
                    if w != widget.placement.width || h != widget.placement.height {
                        widget.placement.width = w;
                        widget.placement.height = h;
                        widget.placement_dirty = true;
                    }
                }
            }

            let offset = crate::calendar_time::local_offset_ms();
            let now = canvas_storage::now_millis();
            let week_start = crate::calendar_time::week_start_utc(now, offset);
            if window.get_today() != crate::calendar_time::day_index(now, week_start, offset) {
                refresh_calendar(&app, &id);
            }
        });
    }

    poll_timer
}

fn refresh_kanban(app: &SharedApp, id: &str) {
    let Ok(a) = app.try_borrow() else {
        return;
    };
    let Some(widget) = a.widgets.get(id) else {
        return;
    };
    let (WidgetDoc::Kanban(doc), WidgetWindow::Kanban(window)) = (&widget.doc, &widget.window)
    else {
        return;
    };

    let columns = doc.columns();
    let column_rows: Vec<KanbanColumnData> = columns
        .iter()
        .map(|c| KanbanColumnData {
            id: c.id.clone().into(),
            title: c.title.clone().into(),
            count: doc.card_count(&c.id) as i32,
        })
        .collect();

    let mut cards: Vec<KanbanCardData> = Vec::new();
    for (col, column) in columns.iter().enumerate() {
        for (row, card) in doc.cards_in(&column.id).into_iter().enumerate() {
            cards.push(KanbanCardData {
                id: card.id.into(),
                col: col as i32,
                row: row as i32,
                title: card.title.into(),
                meta: card_meta(&card.assignee, card.due).into(),
            });
        }
    }

    let window = window.clone_strong();
    drop(a);

    if rows_locked_by_editor(window.get_editing_id().as_ref()) {
        return;
    }
    let column_model: slint::VecModel<KanbanColumnData> = slint::VecModel::from(column_rows);
    window.set_columns(slint::ModelRc::new(column_model));
    let card_model: slint::VecModel<KanbanCardData> = slint::VecModel::from(cards);
    window.set_cards(slint::ModelRc::new(card_model));
}

fn card_meta(assignee: &str, due: i64) -> String {
    let assignee = assignee.trim();
    if assignee.is_empty() && due == 0 {
        return "UNASSIGNED".to_string();
    }
    let who = if assignee.is_empty() {
        "UNASSIGNED".to_string()
    } else {
        assignee.to_uppercase()
    };
    if due == 0 {
        return who;
    }
    let offset = crate::calendar_time::local_offset_ms();
    format!("{} · {}", who, crate::calendar_time::day_stamp(due, offset))
}

fn kanban_edit(app: &SharedApp, id: &str, change: impl FnOnce(&KanbanDoc) -> Option<Vec<u8>>) {
    {
        let Ok(mut a) = app.try_borrow_mut() else {
            return;
        };
        let Some(widget) = a.widgets.get_mut(id) else {
            return;
        };
        let WidgetDoc::Kanban(doc) = &widget.doc else {
            return;
        };
        let Some(update) = change(doc) else {
            return;
        };
        widget.pending_updates.push(update);
        widget.text_dirty = true;
    }
    refresh_kanban(app, id);
    App::refresh_home(app);
}

fn wire_kanban(app: &SharedApp, id: &str, window: &KanbanWindow) -> slint::Timer {
    let owned_id = id.to_string();
    let poll_timer = slint::Timer::default();

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        window.on_card_added(move |col| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            end_kanban_card_edit(&handle);
            kanban_edit(&app, &id, |doc| {
                let column = doc.columns().into_iter().nth(col.max(0) as usize)?;
                Some(doc.add_card(&column.id, NEW_CARD_TITLE).1)
            });
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        window.on_card_moved(move |card_id, col, row| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            end_kanban_card_edit(&handle);
            kanban_edit(&app, &id, |doc| {
                let column = doc.columns().into_iter().nth(col.max(0) as usize)?;
                doc.move_card(card_id.as_ref(), &column.id, row.max(0) as usize)
            });
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        window.on_card_removed(move |card_id| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            end_kanban_card_edit(&handle);
            kanban_edit(&app, &id, |doc| doc.delete_card(card_id.as_ref()));
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.on_card_title_edited(move |card_id, title| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            kanban_edit(&app, &id, |doc| {
                doc.set_title(card_id.as_ref(), title.as_ref())
            });
        });
    }

    {
        let handle = window.as_weak();
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.on_edit_committed(move || {
            if let Some(window) = handle.upgrade() {
                window.set_editing_id(slint::SharedString::new());
            }
            if let Some(app) = weak.upgrade() {
                refresh_kanban(&app, &id);
            }
        });
    }

    let anchor: DragAnchor = Rc::new(std::cell::Cell::new(None));

    {
        let anchor = anchor.clone();
        let handle = window.as_weak();
        window.on_drag_started(move || {
            if let Some(window) = handle.upgrade() {
                begin_drag(window.window(), &anchor);
            }
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let anchor = anchor.clone();
        window.on_drag_ended(move || {
            anchor.set(None);
            let Some(app) = weak.upgrade() else {
                return;
            };
            let Ok(mut a) = app.try_borrow_mut() else {
                return;
            };
            if let Some(widget) = a.widgets.get_mut(&id) {
                widget.placement_dirty = true;
            }
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let anchor = anchor.clone();
        let handle = window.as_weak();
        window.on_window_moved(move |dx, dy| {
            let Some(window) = handle.upgrade() else {
                return;
            };
            let win = window.window();
            let scale = win.scale_factor();
            let next = drag_move(win, &anchor, dx, dy);

            let Some(app) = weak.upgrade() else {
                return;
            };
            let Ok(mut a) = app.try_borrow_mut() else {
                return;
            };
            if let Some(widget) = a.widgets.get_mut(&id) {
                let logical = next.to_logical(scale);
                widget.placement.x = (logical.x + PLATE_MARGIN).round() as i32;
                widget.placement.y = (logical.y + PLATE_MARGIN).round() as i32;
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
            if let Some(widget) = a.widgets.get_mut(&id) {
                widget.placement.pinned = pinned;
                widget.placement_dirty = true;
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
            App::close_widget(&app, &id);
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
            App::delete_widget(&app, &id);
            App::refresh_home(&app);
            crate::maybe_quit(&app);
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        window.window().on_close_requested(move || {
            if let Some(app) = weak.upgrade() {
                App::close_widget(&app, &id);
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
        poll_timer.start(slint::TimerMode::Repeated, PIN_POLL_INTERVAL, move || {
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
            if let Some(widget) = a.widgets.get_mut(&id) {
                let size = window
                    .window()
                    .size()
                    .to_logical(window.window().scale_factor());
                let w = (size.width - 2.0 * PLATE_MARGIN).round() as i32;
                let h = (size.height - 2.0 * PLATE_MARGIN).round() as i32;
                if w != widget.placement.width || h != widget.placement.height {
                    widget.placement.width = w;
                    widget.placement.height = h;
                    widget.placement_dirty = true;
                }
            }
        });
    }

    poll_timer
}
#[cfg(test)]
mod tests {
    use super::{
        built_types_model, count_label, default_actions, rows_locked_by_editor, OpenRequest,
        ACTION_SETTINGS,
    };
    use crate::widget_types;
    use canvas_storage::{KIND_KANBAN, KIND_TODO};
    use slint::Model;

    #[test]
    fn a_live_row_editor_suppresses_the_model_rebuild() {
        assert!(!rows_locked_by_editor(""));
        assert!(rows_locked_by_editor("card-7"));
    }

    #[test]
    fn a_plain_launch_asks_for_nothing() {
        assert_eq!(
            OpenRequest::from_args(Vec::<String>::new()),
            Ok(OpenRequest::Nothing)
        );
        assert_eq!(
            OpenRequest::from_args(["--gpu", "--dump"]),
            Ok(OpenRequest::Nothing)
        );
    }

    #[test]
    fn open_kind_takes_either_word_for_a_type() {
        assert_eq!(
            OpenRequest::from_args(["--open-kind", "kanban"]),
            Ok(OpenRequest::Kind(KIND_KANBAN))
        );
        assert_eq!(
            OpenRequest::from_args(["--gpu", "--open-kind", "todo"]),
            Ok(OpenRequest::Kind(KIND_TODO))
        );
    }

    #[test]
    fn open_takes_a_widget_id() {
        assert_eq!(
            OpenRequest::from_args(["--open", "b6ff3785-0d18-434e-97c5-c9e5f4e21b3f"]),
            Ok(OpenRequest::Id(
                "b6ff3785-0d18-434e-97c5-c9e5f4e21b3f".to_string()
            ))
        );
    }

    #[test]
    fn a_flag_with_no_value_names_what_it_wanted() {
        let e = OpenRequest::from_args(["--open-kind"]).unwrap_err();
        assert!(e.contains("kanban"), "{e}");
        let e = OpenRequest::from_args(["--open", "--gpu"]).unwrap_err();
        assert!(e.contains("--dump"), "{e}");
    }

    #[test]
    fn an_unbuilt_type_is_refused_rather_than_created() {
        let e = OpenRequest::from_args(["--open-kind", "whiteboard"]).unwrap_err();
        assert!(e.contains("whiteboard"), "{e}");
        assert!(e.contains("calendar"), "{e}");
    }

    #[test]
    fn two_open_flags_are_refused() {
        assert!(OpenRequest::from_args(["--open", "a", "--open-kind", "todo"]).is_err());
        assert!(OpenRequest::from_args(["--open-kind", "todo", "--open-kind", "note"]).is_err());
    }

    #[test]
    fn count_label_is_singular_plural_and_empty() {
        assert_eq!(count_label(0), "NO RESULTS");
        assert_eq!(count_label(1), "1 RESULT");
        assert_eq!(count_label(7), "7 RESULTS");
    }

    #[test]
    fn an_empty_query_offers_every_built_type_then_settings() {
        let rows = default_actions();
        let ids: Vec<String> = rows.iter().map(|r| r.id.to_string()).collect();
        let mut expected: Vec<String> = widget_types::BUILT
            .iter()
            .map(|t| widget_types::action_id(t.kind))
            .collect();
        expected.push(ACTION_SETTINGS.to_string());
        assert_eq!(ids, expected);
    }

    #[test]
    fn a_palette_row_icon_is_the_kind_the_glyph_map_keys_on() {
        let rows = default_actions();
        for (row, t) in rows.iter().zip(widget_types::BUILT) {
            assert_eq!(row.icon, t.kind);
        }
    }

    #[test]
    fn home_and_the_palette_offer_the_same_list() {
        let home: Vec<String> = built_types_model()
            .iter()
            .map(|t| t.kind.to_string())
            .collect();
        let palette: Vec<String> = default_actions()
            .iter()
            .filter_map(|r| widget_types::kind_for_action(&r.id).map(str::to_string))
            .collect();
        assert_eq!(home, palette);
        assert_eq!(home.len(), widget_types::BUILT.len());
    }
}
