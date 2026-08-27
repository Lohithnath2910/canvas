use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use slint::ComponentHandle;

use canvas_doc::sheet::{cell_key, column_label};
use canvas_doc::{CalendarDoc, KanbanDoc, SheetDoc, StickyNoteDoc, TodoListDoc, UndoHistory};

use crate::widget::{LiveWidget, WidgetDoc, WidgetWindow};
use crate::widget_types;
use canvas_storage::{Store, WindowPlacement, KIND_STICKY_NOTE};
use canvas_ui::{
    platform_system_sans, CalendarChip, CalendarTick, CalendarWindow, CanvasTray, DayCell,
    HomeWindow, KanbanCardData, KanbanColumnData, KanbanWindow, Motion, NoteRow, PaletteRow,
    PaletteWindow, SheetCellData, SheetColData, SheetRowData, SpreadsheetWindow, StickyNoteWindow,
    TodoItemData, TodoWindow, Typography, WidgetTypeData,
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
            WidgetWindow::Spreadsheet(sheet) => wire_sheet(app, id, sheet),
        };
        if let Some(widget) = app.borrow_mut().widgets.get_mut(id) {
            widget.poll_timer = Some(poll_timer);
        }
        if matches!(window, WidgetWindow::Todo(_)) {
            backfill_todo_positions(app, id);
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

fn backfill_todo_positions(app: &SharedApp, id: &str) {
    let Ok(mut a) = app.try_borrow_mut() else {
        return;
    };
    let Some(widget) = a.widgets.get_mut(id) else {
        return;
    };
    let WidgetDoc::Todo(doc) = &widget.doc else {
        return;
    };
    let Some(update) = doc.backfill_positions() else {
        return;
    };
    widget.pending_updates.push(update);
    widget.text_dirty = true;
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
            priority: item.priority,
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
        window.on_item_moved(move |item_id, index| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            end_todo_row_edit(&handle);
            let index = index.max(0) as usize;
            todo_edit(&app, &id, |doc| doc.move_item(item_id.as_ref(), index));
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        window.on_item_priority_cycled(move |item_id| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            end_todo_row_edit(&handle);
            todo_edit(&app, &id, |doc| doc.cycle_priority(item_id.as_ref()));
        });
    }

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
        window.on_time_typed(move |text| {
            let Some(window) = handle.upgrade() else {
                return;
            };
            match ct::parse_clock(text.as_str()) {
                Some(minute) => {
                    window.set_pick_minute(minute as i32);
                    window.set_pick_time_error(false);
                }
                None => window.set_pick_time_error(true),
            }
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

    let week = doc.events_between(week_start, week_end);

    let spans: Vec<(i64, i64)> = week
        .iter()
        .map(|event| {
            let from = ct::minute_of_day(event.start.max(week_start), offset);
            let to = ct::minute_of_day(event.end, offset);
            let to = if to <= from { ct::MINUTES_PER_DAY } else { to };
            (from, to)
        })
        .collect();
    let now_minute = ct::minute_of_day(now, offset);
    let in_this_week = (week_start..week_end).contains(&now);
    let lane = ct::timeline_window(&spans, in_this_week.then_some(now_minute));

    let chips: Vec<CalendarChip> = week
        .iter()
        .zip(spans.iter())
        .map(|(event, span)| {
            let (start, width) = ct::span_fraction(span.0, span.1, lane);
            let shown_from = event.start.max(week_start);
            CalendarChip {
                id: event.id.clone().into(),
                time: ct::hhmm(event.start, offset).into(),
                name: event.title.clone().into(),
                day: ct::day_index(shown_from, week_start, offset),
                start,
                span: width,
                live: event.start <= now && now < event.end,
            }
        })
        .collect();

    let lane_px = (widget.placement.width as f32 - 2.0 * 16.0 - 40.0).max(80.0);
    let step = ct::tick_step(lane.1 - lane.0, lane_px);
    let mut ticks: Vec<CalendarTick> = Vec::new();
    let mut minute = lane.0;
    while minute < lane.1 {
        ticks.push(CalendarTick {
            label: format!("{:02}", minute / 60).into(),
            at: ct::minute_fraction(minute, lane),
        });
        minute += step * 60;
    }
    let now_at = if in_this_week {
        ct::minute_fraction(now_minute, lane)
    } else {
        -1.0
    };

    let bar = info_bar(&week, now, offset);

    let window = window.clone_strong();
    drop(a);

    let model: slint::VecModel<CalendarChip> = slint::VecModel::from(chips);
    window.set_chips(slint::ModelRc::new(model));
    window.set_ticks(slint::ModelRc::new(slint::VecModel::from(ticks)));
    window.set_now_at(now_at);
    window.set_today(today);
    window.set_info_label(bar.label.into());
    window.set_info_name(bar.name.into());
    window.set_info_span(bar.span.into());
    window.set_info_left(bar.left.into());
    window.set_info_live(bar.live);
}

struct InfoBar {
    label: String,
    name: String,
    span: String,
    left: String,
    live: bool,
}

fn info_bar(week: &[canvas_doc::CalendarEvent], now: i64, offset: i64) -> InfoBar {
    use crate::calendar_time as ct;

    let span = |e: &canvas_doc::CalendarEvent| {
        format!(
            "{}\u{2013}{}",
            ct::hhmm(e.start, offset),
            ct::hhmm(e.end, offset)
        )
    };
    let minutes = |ms: i64| (ms + ct::MINUTE_MS - 1) / ct::MINUTE_MS;

    if let Some(running) = week.iter().find(|e| e.start <= now && now < e.end) {
        return InfoBar {
            label: "NOW".into(),
            name: running.title.clone(),
            span: span(running),
            left: format!("{} MIN LEFT", minutes(running.end - now)),
            live: true,
        };
    }

    if let Some(next) = week
        .iter()
        .filter(|e| e.start > now)
        .min_by_key(|e| e.start)
    {
        return InfoBar {
            label: "NEXT UP".into(),
            name: next.title.clone(),
            span: span(next),
            left: format!("IN {} MIN", minutes(next.start - now)),
            live: false,
        };
    }

    InfoBar {
        label: "NOTHING SCHEDULED".into(),
        name: String::new(),
        span: String::new(),
        left: String::new(),
        live: false,
    }
}

#[derive(Clone, Default)]
struct EventDraft {
    id: Option<String>,
    start: i64,
    end: i64,
    month: (i64, u32),
}

type Draft = Rc<RefCell<EventDraft>>;

fn calendar_edit(app: &SharedApp, id: &str, change: impl FnOnce(&CalendarDoc) -> Option<Vec<u8>>) {
    {
        let Ok(mut a) = app.try_borrow_mut() else {
            return;
        };
        let Some(widget) = a.widgets.get_mut(id) else {
            return;
        };
        let WidgetDoc::Calendar(doc) = &widget.doc else {
            return;
        };
        let Some(update) = change(doc) else {
            return;
        };
        widget.pending_updates.push(update);
        widget.text_dirty = true;
    }
    refresh_calendar(app, id);
    App::refresh_home(app);
}

fn draft_labels(window: &CalendarWindow, draft: &EventDraft, offset: i64) {
    use crate::calendar_time as ct;
    let stamp = |ts: i64| format!("{} {}", ct::date_stamp(ts, offset), ct::hhmm(ts, offset));
    window.set_ev_start_label(stamp(draft.start).into());
    window.set_ev_end_label(stamp(draft.end).into());
}

fn paint_draft_month(window: &CalendarWindow, month: (i64, u32), today: i64) {
    use crate::calendar_time as ct;

    let (year, m) = month;
    window.set_pick_month(format!("{} {year}", ct::month_name(m)).into());
    window.set_pick_stamp(ct::date_stamp(window.get_pick_day() as i64 * ct::DAY_MS, 0).into());

    let cells: Vec<DayCell> = ct::month_grid(year, m, FIRST_WEEKDAY)
        .into_iter()
        .map(|day| {
            let (y, mm, d) = ct::civil_from_days(day);
            DayCell {
                label: d.to_string().into(),
                epoch_day: day as i32,
                in_month: (y, mm) == (year, m),
                today: day == today,
            }
        })
        .collect();
    window.set_pick_days(slint::ModelRc::new(slint::VecModel::from(cells)));
}

fn open_picker(window: &CalendarWindow, draft: &Draft, which: i32, offset: i64) {
    use crate::calendar_time as ct;

    let mut d = draft.borrow_mut();
    let ts = if which == 1 { d.start } else { d.end };
    let local_day = (ts + offset).div_euclid(ct::DAY_MS);
    let (y, m, _) = ct::civil_from_days(local_day);
    d.month = (y, m);
    drop(d);

    window.set_pick_day(local_day as i32);
    let minute = ct::minute_of_day(ts, offset);
    window.set_pick_minute(minute as i32);
    window.set_pick_time(ct::format_clock_12(minute).into());
    window.set_pick_time_error(false);
    window.set_pick_heads(slint::ModelRc::new(slint::VecModel::from(weekday_heads(
        FIRST_WEEKDAY,
    ))));
    let today = (canvas_storage::now_millis() + offset).div_euclid(ct::DAY_MS);
    paint_draft_month(window, (y, m), today);
}

fn wire_calendar(app: &SharedApp, id: &str, window: &CalendarWindow) -> slint::Timer {
    let owned_id = id.to_string();
    let poll_timer = slint::Timer::default();
    let draft: Draft = Rc::new(RefCell::new(EventDraft::default()));

    let anchor: DragAnchor = Rc::new(std::cell::Cell::new(None));

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        let draft = draft.clone();
        window.on_day_clicked(move |col, frac| {
            use crate::calendar_time as ct;
            let Some(app) = weak.upgrade() else {
                return;
            };
            let Some(window) = handle.upgrade() else {
                return;
            };

            let offset = ct::local_offset_ms();
            let now = canvas_storage::now_millis();
            let week_start = ct::week_start_utc(now, offset);
            let day_start = week_start + col.max(0) as i64 * ct::DAY_MS;
            let clicked_minute = (frac.clamp(0.0, 1.0) as f64 * 1440.0) as i64;

            let existing = {
                let Ok(a) = app.try_borrow() else {
                    return;
                };
                let Some(widget) = a.widgets.get(&id) else {
                    return;
                };
                let WidgetDoc::Calendar(doc) = &widget.doc else {
                    return;
                };
                doc.events_between(day_start, day_start + ct::DAY_MS)
                    .into_iter()
                    .find(|e| {
                        let from = ct::minute_of_day(e.start, offset);
                        let until = from + (e.duration() / ct::MINUTE_MS).max(1);
                        clicked_minute >= from && clicked_minute < until
                    })
            };

            match existing {
                Some(event) => {
                    window.set_ev_name(event.title.clone().into());
                    window.set_ev_location(event.location.clone().into());
                    window.set_ev_notes(event.notes.clone().into());
                    *draft.borrow_mut() = EventDraft {
                        id: Some(event.id.clone()),
                        start: event.start,
                        end: event.end,
                        month: (0, 1),
                    };
                    window.set_sheet_existing(true);
                }
                None => {
                    let snapped = (clicked_minute / 30) * 30;
                    let start = day_start + snapped * ct::MINUTE_MS;
                    window.set_ev_name("".into());
                    window.set_ev_location("".into());
                    window.set_ev_notes("".into());
                    *draft.borrow_mut() = EventDraft {
                        id: None,
                        start,
                        end: start + 60 * ct::MINUTE_MS,
                        month: (0, 1),
                    };
                    window.set_sheet_existing(false);
                }
            }

            draft_labels(&window, &draft.borrow(), offset);
            window.set_pick_mode(0);
            window.set_sheet_open(true);
        });
    }

    {
        let handle = window.as_weak();
        let draft = draft.clone();
        window.on_pick_opened(move |which| {
            let Some(window) = handle.upgrade() else {
                return;
            };
            open_picker(
                &window,
                &draft,
                which,
                crate::calendar_time::local_offset_ms(),
            );
        });
    }

    {
        let handle = window.as_weak();
        let draft = draft.clone();
        window.on_pick_month_stepped(move |delta| {
            use crate::calendar_time as ct;
            let Some(window) = handle.upgrade() else {
                return;
            };
            let stepped = {
                let mut d = draft.borrow_mut();
                let (y, m) = d.month;
                d.month = ct::month_step(y, m, delta);
                d.month
            };
            let offset = ct::local_offset_ms();
            let today = (canvas_storage::now_millis() + offset).div_euclid(ct::DAY_MS);
            paint_draft_month(&window, stepped, today);
        });
    }

    {
        let handle = window.as_weak();
        window.on_time_typed(move |text| {
            let Some(window) = handle.upgrade() else {
                return;
            };
            match crate::calendar_time::parse_clock(text.as_str()) {
                Some(minute) => {
                    window.set_pick_minute(minute as i32);
                    window.set_pick_time_error(false);
                }
                None => window.set_pick_time_error(true),
            }
        });
    }

    {
        let handle = window.as_weak();
        let draft = draft.clone();
        window.on_pick_committed(move |which| {
            use crate::calendar_time as ct;
            let Some(window) = handle.upgrade() else {
                return;
            };
            if window.get_pick_time_error() {
                return;
            }
            let offset = ct::local_offset_ms();
            let (y, m, d) = ct::civil_from_days(window.get_pick_day() as i64);
            let picked = ct::compose_local_ms(y, m, d, window.get_pick_minute() as i64, offset);

            {
                let mut draft = draft.borrow_mut();
                if which == 1 {
                    let span = (draft.end - draft.start).max(0);
                    draft.start = picked;
                    draft.end = picked + span;
                } else {
                    draft.end = picked.max(draft.start);
                }
            }
            draft_labels(&window, &draft.borrow(), offset);
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        let draft = draft.clone();
        window.on_event_saved(move || {
            let Some(app) = weak.upgrade() else {
                return;
            };
            let Some(window) = handle.upgrade() else {
                return;
            };
            let d = draft.borrow().clone();
            let name = window.get_ev_name().to_string();
            let name = if name.trim().is_empty() {
                canvas_storage::DEFAULT_TITLE.to_string()
            } else {
                name
            };
            let location = window.get_ev_location().to_string();
            let notes = window.get_ev_notes().to_string();

            calendar_edit(&app, &id, |doc| {
                let event_id = match &d.id {
                    Some(existing) => {
                        doc.set_title(existing, &name);
                        doc.set_span(existing, d.start, d.end);
                        existing.clone()
                    }
                    None => doc.add_event(&name, d.start, d.end).0,
                };
                doc.set_location(&event_id, &location);
                doc.set_notes(&event_id, &notes);
                Some(doc.snapshot())
            });

            window.set_sheet_open(false);
            window.set_pick_mode(0);
        });
    }

    {
        let handle = window.as_weak();
        window.on_event_cancelled(move || {
            if let Some(window) = handle.upgrade() {
                window.set_sheet_open(false);
                window.set_pick_mode(0);
            }
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        let draft = draft.clone();
        window.on_event_deleted(move || {
            let Some(app) = weak.upgrade() else {
                return;
            };
            let existing = draft.borrow().id.clone();
            if let Some(event_id) = existing {
                calendar_edit(&app, &id, |doc| doc.delete_event(&event_id));
            }
            if let Some(window) = handle.upgrade() {
                window.set_sheet_open(false);
                window.set_pick_mode(0);
            }
        });
    }

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
        let painted_minute = Rc::new(std::cell::Cell::new(i64::MIN));
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
            let minute = now.div_euclid(crate::calendar_time::MINUTE_MS);
            if window.get_today() != crate::calendar_time::day_index(now, week_start, offset)
                || painted_minute.get() != minute
            {
                painted_minute.set(minute);
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
        let mut y = KANBAN_COL_HEAD_H + KANBAN_CARD_GAP;
        for card in doc.cards_in(&column.id) {
            let meta = card_meta(&card.assignee, card.due);
            let h = card_height(&meta, card.priority);
            cards.push(KanbanCardData {
                id: card.id.into(),
                col: col as i32,
                y,
                h,
                title: card.title.into(),
                meta: meta.into(),
                priority: card.priority,
            });
            y += h + KANBAN_CARD_GAP;
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

const KANBAN_COL_HEAD_H: i32 = 32;
const KANBAN_CARD_GAP: i32 = 8;
const KANBAN_CARD_TALL: i32 = 70;
const KANBAN_CARD_SHORT: i32 = 46;

fn card_height(meta: &str, priority: i32) -> i32 {
    if meta.is_empty() && priority <= 0 {
        KANBAN_CARD_SHORT
    } else {
        KANBAN_CARD_TALL
    }
}

fn drop_index(heights: &[i32], drop_y: i32) -> usize {
    let mut y = KANBAN_COL_HEAD_H + KANBAN_CARD_GAP;
    for (i, h) in heights.iter().enumerate() {
        if drop_y < y + h / 2 {
            return i;
        }
        y += h + KANBAN_CARD_GAP;
    }
    heights.len()
}

fn card_meta(assignee: &str, due: i64) -> String {
    let who = assignee.trim().to_uppercase();
    if due == 0 {
        return who;
    }
    let offset = crate::calendar_time::local_offset_ms();
    let when = crate::calendar_time::day_stamp(due, offset);
    if who.is_empty() {
        return when;
    }
    format!("{who} · {when}")
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

#[derive(Clone, Default)]
struct CardDraft {
    id: String,
    due: i64,
    month: (i64, u32),
}

fn paint_card_month(window: &KanbanWindow, month: (i64, u32), today: i64) {
    use crate::calendar_time as ct;

    let (year, m) = month;
    window.set_pick_month(format!("{} {year}", ct::month_name(m)).into());
    window.set_pick_stamp(ct::date_stamp(window.get_pick_day() as i64 * ct::DAY_MS, 0).into());

    let cells: Vec<DayCell> = ct::month_grid(year, m, FIRST_WEEKDAY)
        .into_iter()
        .map(|day| {
            let (y, mm, d) = ct::civil_from_days(day);
            DayCell {
                label: d.to_string().into(),
                epoch_day: day as i32,
                in_month: (y, mm) == (year, m),
                today: day == today,
            }
        })
        .collect();
    window.set_pick_days(slint::ModelRc::new(slint::VecModel::from(cells)));
}

fn due_label(due: i64, offset: i64) -> String {
    if due == 0 {
        String::new()
    } else {
        crate::calendar_time::date_stamp(due, offset)
    }
}

fn wire_kanban(app: &SharedApp, id: &str, window: &KanbanWindow) -> slint::Timer {
    let owned_id = id.to_string();
    let poll_timer = slint::Timer::default();
    let card_draft: Rc<RefCell<CardDraft>> = Rc::new(RefCell::new(CardDraft::default()));

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        let draft = card_draft.clone();
        window.on_card_opened(move |card_id| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            let Some(window) = handle.upgrade() else {
                return;
            };
            let found = {
                let Ok(a) = app.try_borrow() else {
                    return;
                };
                let Some(widget) = a.widgets.get(&id) else {
                    return;
                };
                let WidgetDoc::Kanban(doc) = &widget.doc else {
                    return;
                };
                doc.cards().into_iter().find(|c| c.id == card_id.as_str())
            };
            let Some(card) = found else {
                return;
            };

            let offset = crate::calendar_time::local_offset_ms();
            window.set_card_title(card.title.clone().into());
            window.set_card_assignee(card.assignee.clone().into());
            window.set_card_description(card.description.clone().into());
            window.set_card_due_label(due_label(card.due, offset).into());
            window.set_card_priority(card.priority);
            *draft.borrow_mut() = CardDraft {
                id: card.id.clone(),
                due: card.due,
                month: (0, 1),
            };
            window.set_pick_mode(0);
            window.set_editing_id(card.id.into());
            window.set_sheet_open(true);
        });
    }

    {
        let handle = window.as_weak();
        let draft = card_draft.clone();
        window.on_pick_opened(move || {
            use crate::calendar_time as ct;
            let Some(window) = handle.upgrade() else {
                return;
            };
            let offset = ct::local_offset_ms();
            let now = canvas_storage::now_millis();
            let anchor = {
                let d = draft.borrow();
                if d.due == 0 {
                    now
                } else {
                    d.due
                }
            };
            let local_day = (anchor + offset).div_euclid(ct::DAY_MS);
            let (y, m, _) = ct::civil_from_days(local_day);
            draft.borrow_mut().month = (y, m);

            window.set_pick_day(local_day as i32);
            window.set_pick_heads(slint::ModelRc::new(slint::VecModel::from(weekday_heads(
                FIRST_WEEKDAY,
            ))));
            paint_card_month(&window, (y, m), (now + offset).div_euclid(ct::DAY_MS));
        });
    }

    {
        let handle = window.as_weak();
        let draft = card_draft.clone();
        window.on_pick_month_stepped(move |delta| {
            use crate::calendar_time as ct;
            let Some(window) = handle.upgrade() else {
                return;
            };
            let stepped = {
                let mut d = draft.borrow_mut();
                let (y, m) = d.month;
                d.month = ct::month_step(y, m, delta);
                d.month
            };
            let offset = ct::local_offset_ms();
            let today = (canvas_storage::now_millis() + offset).div_euclid(ct::DAY_MS);
            paint_card_month(&window, stepped, today);
        });
    }

    {
        let handle = window.as_weak();
        let draft = card_draft.clone();
        window.on_pick_committed(move || {
            use crate::calendar_time as ct;
            let Some(window) = handle.upgrade() else {
                return;
            };
            let offset = ct::local_offset_ms();
            let (y, m, d) = ct::civil_from_days(window.get_pick_day() as i64);
            let due = ct::compose_local_ms(y, m, d, 0, offset);
            draft.borrow_mut().due = due;
            window.set_card_due_label(due_label(due, offset).into());
        });
    }

    {
        let handle = window.as_weak();
        let draft = card_draft.clone();
        window.on_pick_cleared(move || {
            let Some(window) = handle.upgrade() else {
                return;
            };
            draft.borrow_mut().due = 0;
            window.set_card_due_label("".into());
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        let draft = card_draft.clone();
        window.on_card_committed(move || {
            let Some(app) = weak.upgrade() else {
                return;
            };
            let Some(window) = handle.upgrade() else {
                return;
            };
            let d = draft.borrow().clone();
            let title = window.get_card_title().to_string();
            let title = if title.trim().is_empty() {
                NEW_CARD_TITLE.to_string()
            } else {
                title
            };
            let assignee = window.get_card_assignee().to_string();
            let description = window.get_card_description().to_string();

            window.set_sheet_open(false);
            window.set_pick_mode(0);
            window.set_editing_id("".into());

            kanban_edit(&app, &id, |doc| {
                doc.set_title(&d.id, &title);
                doc.set_assignee(&d.id, &assignee);
                doc.set_description(&d.id, &description);
                doc.set_due(&d.id, d.due);
                Some(doc.snapshot())
            });
        });
    }

    {
        let handle = window.as_weak();
        window.on_card_cancelled(move || {
            if let Some(window) = handle.upgrade() {
                window.set_sheet_open(false);
                window.set_pick_mode(0);
                window.set_editing_id("".into());
            }
        });
    }

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
        window.on_card_moved(move |card_id, col, drop_y| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            end_kanban_card_edit(&handle);
            kanban_edit(&app, &id, |doc| {
                let column = doc.columns().into_iter().nth(col.max(0) as usize)?;
                let heights: Vec<i32> = doc
                    .cards_in(&column.id)
                    .into_iter()
                    .filter(|c| c.id.as_str() != card_id.as_str())
                    .map(|c| card_height(&card_meta(&c.assignee, c.due), c.priority))
                    .collect();
                doc.move_card(card_id.as_ref(), &column.id, drop_index(&heights, drop_y))
            });
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let handle = window.as_weak();
        window.on_card_priority_cycled(move |card_id| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            kanban_edit(&app, &id, |doc| doc.cycle_priority(card_id.as_ref()));
            let Some(window) = handle.upgrade() else {
                return;
            };
            if window.get_sheet_open() {
                let Ok(a) = app.try_borrow() else {
                    return;
                };
                if let Some(widget) = a.widgets.get(&id) {
                    if let WidgetDoc::Kanban(doc) = &widget.doc {
                        if let Some(card) =
                            doc.cards().into_iter().find(|c| c.id == card_id.as_str())
                        {
                            window.set_card_priority(card.priority);
                        }
                    }
                }
            }
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
const SHEET_COL_W: i32 = 80;
const SHEET_ROW_H: i32 = 28;
const SHEET_GRIP: i32 = 8;
const SHEET_MIN_COL_W: i32 = 48;
const SHEET_MAX_COL_W: i32 = 400;
const SHEET_MIN_ROW_H: i32 = 20;
const SHEET_MAX_ROW_H: i32 = 200;

struct SheetResize {
    is_row: bool,
    index: usize,
    start: i32,
    press: i32,
    size: i32,
}

#[derive(Default)]
struct SheetUi {
    rows: Vec<String>,
    cols: Vec<String>,
    row_y: Vec<i32>,
    row_h: Vec<i32>,
    col_x: Vec<i32>,
    col_w: Vec<i32>,
    spans: HashMap<(usize, usize), (usize, usize)>,
    anchor_of: HashMap<(usize, usize), (usize, usize)>,
    anchor: (usize, usize),
    sel: (usize, usize, usize, usize),
    resizing: Option<SheetResize>,
}

impl SheetUi {
    fn empty(&self) -> bool {
        self.rows.is_empty() || self.cols.is_empty()
    }

    fn clamp(&mut self) {
        if self.empty() {
            self.sel = (0, 0, 0, 0);
            return;
        }
        let (r, c, nr, nc) = self.sel;
        let r = r.min(self.rows.len() - 1);
        let c = c.min(self.cols.len() - 1);
        let nr = nr.max(1).min(self.rows.len() - r);
        let nc = nc.max(1).min(self.cols.len() - c);
        self.sel = (r, c, nr, nc);
        self.anchor = (
            self.anchor.0.min(self.rows.len() - 1),
            self.anchor.1.min(self.cols.len() - 1),
        );
    }

    fn uncover(&mut self) {
        let (r, c, nr, nc) = self.sel;
        if nr == 1 && nc == 1 {
            if let Some(&(ar, ac)) = self.anchor_of.get(&(r, c)) {
                self.sel = (ar, ac, 1, 1);
                self.anchor = (ar, ac);
            }
        }
    }

    fn span_of(&self, r: usize, c: usize) -> (usize, usize) {
        *self.spans.get(&(r, c)).unwrap_or(&(1, 1))
    }

    fn select_to(&mut self, row: usize, col: usize) {
        let (ar, ac) = self.anchor;
        let r = ar.min(row);
        let c = ac.min(col);
        self.sel = (r, c, ar.max(row) - r + 1, ac.max(col) - c + 1);
    }
}

fn sheet_axis_at(starts: &[i32], sizes: &[i32], value: i32) -> Option<usize> {
    if starts.is_empty() {
        return None;
    }
    if value < 0 {
        return Some(0);
    }
    for i in 0..starts.len() {
        if value < starts[i] + sizes[i] {
            return Some(i);
        }
    }
    Some(starts.len() - 1)
}

fn sheet_axis_boundary(starts: &[i32], sizes: &[i32], value: i32) -> Option<usize> {
    (0..starts.len()).find(|&i| (value - (starts[i] + sizes[i])).abs() <= SHEET_GRIP)
}

fn sheet_span_size(sizes: &[i32], start: usize, span: usize) -> i32 {
    sizes[start..(start + span).min(sizes.len())].iter().sum()
}

fn sheet_stamp(r: usize, c: usize, nr: usize, nc: usize) -> String {
    let head = format!("{}{}", column_label(c), r + 1);
    if nr <= 1 && nc <= 1 {
        return head;
    }
    format!("{head}:{}{}", column_label(c + nc - 1), r + nr)
}

fn refresh_sheet(app: &SharedApp, id: &str, ui: &Rc<RefCell<SheetUi>>) {
    let Ok(a) = app.try_borrow() else {
        return;
    };
    let Some(widget) = a.widgets.get(id) else {
        return;
    };
    let (WidgetDoc::Sheet(doc), WidgetWindow::Spreadsheet(window)) = (&widget.doc, &widget.window)
    else {
        return;
    };
    if rows_locked_by_editor(window.get_editing_id().as_ref()) {
        return;
    }

    let rows = doc.rows();
    let cols = doc.cols();

    let mut row_y = Vec::with_capacity(rows.len());
    let mut row_h = Vec::with_capacity(rows.len());
    let mut total_h = 0;
    for row in &rows {
        let h = doc.row_height(row).map(|v| v as i32).unwrap_or(SHEET_ROW_H);
        row_y.push(total_h);
        row_h.push(h);
        total_h += h;
    }
    let mut col_x = Vec::with_capacity(cols.len());
    let mut col_w = Vec::with_capacity(cols.len());
    let mut total_w = 0;
    for col in &cols {
        let w = doc.col_width(col).map(|v| v as i32).unwrap_or(SHEET_COL_W);
        col_x.push(total_w);
        col_w.push(w);
        total_w += w;
    }

    let row_index: HashMap<&str, usize> = rows
        .iter()
        .enumerate()
        .map(|(i, r)| (r.as_str(), i))
        .collect();
    let col_index: HashMap<&str, usize> = cols
        .iter()
        .enumerate()
        .map(|(i, c)| (c.as_str(), i))
        .collect();

    let mut spans: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut anchor_of: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    for merge in doc.merges() {
        let (Some(&r), Some(&c)) = (
            row_index.get(merge.row.as_str()),
            col_index.get(merge.col.as_str()),
        ) else {
            continue;
        };
        let nr = merge.rows.min(rows.len() - r);
        let nc = merge.cols.min(cols.len() - c);
        spans.insert((r, c), (nr, nc));
        for rr in r..r + nr {
            for cc in c..c + nc {
                if (rr, cc) != (r, c) {
                    anchor_of.insert((rr, cc), (r, c));
                }
            }
        }
    }

    let col_data: Vec<SheetColData> = (0..cols.len())
        .map(|i| SheetColData {
            label: column_label(i).into(),
            x: col_x[i],
            w: col_w[i],
        })
        .collect();
    let row_data: Vec<SheetRowData> = (0..rows.len())
        .map(|i| SheetRowData {
            label: (i + 1).to_string().into(),
            y: row_y[i],
            h: row_h[i],
        })
        .collect();

    let mut cells = Vec::with_capacity(rows.len() * cols.len());
    for (r, row) in rows.iter().enumerate() {
        for (c, col) in cols.iter().enumerate() {
            if anchor_of.contains_key(&(r, c)) {
                continue;
            }
            let (nr, nc) = *spans.get(&(r, c)).unwrap_or(&(1, 1));
            cells.push(SheetCellData {
                key: cell_key(row, col).into(),
                x: col_x[c],
                y: row_y[r],
                w: sheet_span_size(&col_w, c, nc),
                h: sheet_span_size(&row_h, r, nr),
                text: doc.value(row, col).into(),
            });
        }
    }

    let window = window.clone_strong();
    drop(a);

    {
        let mut u = ui.borrow_mut();
        u.rows = rows;
        u.cols = cols;
        u.row_y = row_y;
        u.row_h = row_h;
        u.col_x = col_x;
        u.col_w = col_w;
        u.spans = spans;
        u.anchor_of = anchor_of;
        u.clamp();
        u.uncover();
    }

    window.set_grid_w(total_w);
    window.set_grid_h(total_h);
    window.set_cols(slint::ModelRc::new(slint::VecModel::from(col_data)));
    window.set_rows(slint::ModelRc::new(slint::VecModel::from(row_data)));
    window.set_cells(slint::ModelRc::new(slint::VecModel::from(cells)));

    paint_sheet_selection(app, id, ui);
}

fn paint_sheet_selection(app: &SharedApp, id: &str, ui: &Rc<RefCell<SheetUi>>) {
    let Ok(a) = app.try_borrow() else {
        return;
    };
    let Some(widget) = a.widgets.get(id) else {
        return;
    };
    let (WidgetDoc::Sheet(doc), WidgetWindow::Spreadsheet(window)) = (&widget.doc, &widget.window)
    else {
        return;
    };
    let window = window.clone_strong();

    let u = ui.borrow();
    if u.empty() {
        drop(u);
        drop(a);
        window.set_sel_w(0);
        window.set_sel_h(0);
        window.set_sel_key(slint::SharedString::new());
        window.set_sel_stamp(slint::SharedString::new());
        window.set_merge_enabled(false);
        return;
    }

    let (r, c, nr, nc) = u.sel;
    let merged = u.spans.contains_key(&(r, c));
    let ranged = nr > 1 || nc > 1;
    let (draw_r, draw_c) = if ranged { (nr, nc) } else { u.span_of(r, c) };

    let sel_x = u.col_x[c];
    let sel_y = u.row_y[r];
    let sel_w = sheet_span_size(&u.col_w, c, draw_c);
    let sel_h = sheet_span_size(&u.row_h, r, draw_r);
    let key = cell_key(&u.rows[r], &u.cols[c]);
    let text = doc.value(&u.rows[r], &u.cols[c]);
    let stamp = sheet_stamp(r, c, draw_r, draw_c);
    drop(u);
    drop(a);

    window.set_sel_x(sel_x);
    window.set_sel_y(sel_y);
    window.set_sel_w(sel_w);
    window.set_sel_h(sel_h);
    window.set_sel_key(key.into());
    window.set_sel_stamp(stamp.into());
    window.set_edit_text(text.into());
    window.set_merge_label(
        if merged && !ranged {
            "UNMERGE"
        } else {
            "MERGE"
        }
        .into(),
    );
    window.set_merge_enabled(ranged || merged);
}

fn sheet_edit(
    app: &SharedApp,
    id: &str,
    ui: &Rc<RefCell<SheetUi>>,
    change: impl FnOnce(&SheetDoc) -> Option<Vec<u8>>,
) {
    {
        let Ok(mut a) = app.try_borrow_mut() else {
            return;
        };
        let Some(widget) = a.widgets.get_mut(id) else {
            return;
        };
        let WidgetDoc::Sheet(doc) = &widget.doc else {
            return;
        };
        let Some(update) = change(doc) else {
            return;
        };
        widget.pending_updates.push(update);
        widget.text_dirty = true;
    }
    refresh_sheet(app, id, ui);
    App::refresh_home(app);
}

fn end_sheet_cell_edit(handle: &slint::Weak<SpreadsheetWindow>) {
    if let Some(window) = handle.upgrade() {
        window.set_editing_id(slint::SharedString::new());
    }
}

fn sheet_selected_ids(ui: &Rc<RefCell<SheetUi>>) -> Option<(String, String)> {
    let u = ui.borrow();
    if u.empty() {
        return None;
    }
    let (r, c, _, _) = u.sel;
    Some((u.rows[r].clone(), u.cols[c].clone()))
}

fn wire_sheet(app: &SharedApp, id: &str, window: &SpreadsheetWindow) -> slint::Timer {
    let owned_id = id.to_string();
    let poll_timer = slint::Timer::default();
    let ui: Rc<RefCell<SheetUi>> = Rc::new(RefCell::new(SheetUi::default()));

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let ui = ui.clone();
        window.on_cell_pressed(move |x, y| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            {
                let mut u = ui.borrow_mut();
                let (Some(row), Some(col)) = (
                    sheet_axis_at(&u.row_y, &u.row_h, y),
                    sheet_axis_at(&u.col_x, &u.col_w, x),
                ) else {
                    return;
                };
                u.anchor = (row, col);
                u.sel = (row, col, 1, 1);
                u.uncover();
            }
            paint_sheet_selection(&app, &id, &ui);
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let ui = ui.clone();
        window.on_cell_dragged(move |x, y| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            {
                let mut u = ui.borrow_mut();
                let (Some(row), Some(col)) = (
                    sheet_axis_at(&u.row_y, &u.row_h, y),
                    sheet_axis_at(&u.col_x, &u.col_w, x),
                ) else {
                    return;
                };
                if u.sel == (row.min(u.anchor.0), col.min(u.anchor.1), 1, 1)
                    && u.anchor == (row, col)
                {
                    return;
                }
                u.select_to(row, col);
            }
            paint_sheet_selection(&app, &id, &ui);
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let ui = ui.clone();
        window.on_nav(move |dr, dc| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            {
                let mut u = ui.borrow_mut();
                if u.empty() {
                    return;
                }
                let (r, c, _, _) = u.sel;
                let (nr, nc) = u.span_of(r, c);
                let step_r = if dr > 0 { nr as i32 } else { 1 };
                let step_c = if dc > 0 { nc as i32 } else { 1 };
                let row = (r as i32 + dr * step_r).clamp(0, u.rows.len() as i32 - 1) as usize;
                let col = (c as i32 + dc * step_c).clamp(0, u.cols.len() as i32 - 1) as usize;
                u.sel = (row, col, 1, 1);
                u.anchor = (row, col);
                u.uncover();
            }
            paint_sheet_selection(&app, &id, &ui);
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let ui = ui.clone();
        window.on_cell_edited(move |value| {
            let Some(app) = weak.upgrade() else {
                return;
            };
            let Some((row, col)) = sheet_selected_ids(&ui) else {
                return;
            };
            sheet_edit(&app, &id, &ui, |doc| {
                doc.set_value(&row, &col, value.as_ref())
            });
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let ui = ui.clone();
        let handle = window.as_weak();
        window.on_cell_cleared(move || {
            let Some(app) = weak.upgrade() else {
                return;
            };
            end_sheet_cell_edit(&handle);
            let Some((row, col)) = sheet_selected_ids(&ui) else {
                return;
            };
            sheet_edit(&app, &id, &ui, |doc| doc.set_value(&row, &col, ""));
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let ui = ui.clone();
        let handle = window.as_weak();
        window.on_edit_committed(move || {
            end_sheet_cell_edit(&handle);
            if let Some(app) = weak.upgrade() {
                refresh_sheet(&app, &id, &ui);
            }
        });
    }

    {
        let ui = ui.clone();
        let handle = window.as_weak();
        window.on_head_pressed(move |x| {
            let Some(window) = handle.upgrade() else {
                return;
            };
            let mut u = ui.borrow_mut();
            let found = sheet_axis_boundary(&u.col_x, &u.col_w, x);
            match found {
                Some(index) => {
                    let size = u.col_w[index];
                    let guide = u.col_x[index] + size;
                    u.resizing = Some(SheetResize {
                        is_row: false,
                        index,
                        start: size,
                        press: x,
                        size,
                    });
                    window.set_guide_x(guide);
                }
                None => {
                    u.resizing = None;
                    window.set_guide_x(-1);
                }
            }
        });
    }

    {
        let ui = ui.clone();
        let handle = window.as_weak();
        window.on_head_dragged(move |x| {
            let Some(window) = handle.upgrade() else {
                return;
            };
            let mut u = ui.borrow_mut();
            let Some(resize) = u.resizing.as_mut() else {
                return;
            };
            if resize.is_row {
                return;
            }
            resize.size = (resize.start + x - resize.press).clamp(SHEET_MIN_COL_W, SHEET_MAX_COL_W);
            let (index, size) = (resize.index, resize.size);
            window.set_guide_x(u.col_x[index] + size);
        });
    }

    {
        let ui = ui.clone();
        let handle = window.as_weak();
        window.on_gutter_pressed(move |y| {
            let Some(window) = handle.upgrade() else {
                return;
            };
            let mut u = ui.borrow_mut();
            let found = sheet_axis_boundary(&u.row_y, &u.row_h, y);
            match found {
                Some(index) => {
                    let size = u.row_h[index];
                    let guide = u.row_y[index] + size;
                    u.resizing = Some(SheetResize {
                        is_row: true,
                        index,
                        start: size,
                        press: y,
                        size,
                    });
                    window.set_guide_y(guide);
                }
                None => {
                    u.resizing = None;
                    window.set_guide_y(-1);
                }
            }
        });
    }

    {
        let ui = ui.clone();
        let handle = window.as_weak();
        window.on_gutter_dragged(move |y| {
            let Some(window) = handle.upgrade() else {
                return;
            };
            let mut u = ui.borrow_mut();
            let Some(resize) = u.resizing.as_mut() else {
                return;
            };
            if !resize.is_row {
                return;
            }
            resize.size = (resize.start + y - resize.press).clamp(SHEET_MIN_ROW_H, SHEET_MAX_ROW_H);
            let (index, size) = (resize.index, resize.size);
            window.set_guide_y(u.row_y[index] + size);
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let ui = ui.clone();
        let handle = window.as_weak();
        window.on_resize_ended(move || {
            if let Some(window) = handle.upgrade() {
                window.set_guide_x(-1);
                window.set_guide_y(-1);
            }
            let Some(app) = weak.upgrade() else {
                return;
            };
            let done = ui.borrow_mut().resizing.take();
            let Some(resize) = done else {
                return;
            };
            if resize.size == resize.start {
                return;
            }
            let target = {
                let u = ui.borrow();
                if resize.is_row {
                    u.rows.get(resize.index).cloned()
                } else {
                    u.cols.get(resize.index).cloned()
                }
            };
            let Some(target) = target else {
                return;
            };
            sheet_edit(&app, &id, &ui, |doc| {
                if resize.is_row {
                    doc.set_row_height(&target, resize.size as u32)
                } else {
                    doc.set_col_width(&target, resize.size as u32)
                }
            });
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let ui = ui.clone();
        let handle = window.as_weak();
        window.on_merge_requested(move || {
            let Some(app) = weak.upgrade() else {
                return;
            };
            end_sheet_cell_edit(&handle);
            let plan = {
                let u = ui.borrow();
                if u.empty() {
                    return;
                }
                let (r, c, nr, nc) = u.sel;
                let ranged = nr > 1 || nc > 1;
                let merged = u.spans.contains_key(&(r, c));
                Some((u.rows[r].clone(), u.cols[c].clone(), nr, nc, ranged, merged))
            };
            let Some((row, col, nr, nc, ranged, merged)) = plan else {
                return;
            };
            {
                let mut u = ui.borrow_mut();
                u.sel = (u.sel.0, u.sel.1, 1, 1);
            }
            sheet_edit(&app, &id, &ui, |doc| {
                if ranged {
                    doc.merge(&row, &col, nr, nc)
                } else if merged {
                    doc.unmerge(&row, &col)
                } else {
                    None
                }
            });
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let ui = ui.clone();
        let handle = window.as_weak();
        window.on_row_added(move || {
            let Some(app) = weak.upgrade() else {
                return;
            };
            end_sheet_cell_edit(&handle);
            sheet_edit(&app, &id, &ui, |doc| Some(doc.insert_row(usize::MAX).1));
        });
    }

    {
        let weak = Rc::downgrade(app);
        let id = owned_id.clone();
        let ui = ui.clone();
        let handle = window.as_weak();
        window.on_col_added(move || {
            let Some(app) = weak.upgrade() else {
                return;
            };
            end_sheet_cell_edit(&handle);
            sheet_edit(&app, &id, &ui, |doc| Some(doc.insert_col(usize::MAX).1));
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

    refresh_sheet(app, id, &ui);
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

    fn seven_by_seven() -> super::SheetUi {
        let mut u = super::SheetUi::default();
        for i in 0..7 {
            u.rows.push(format!("r{i}"));
            u.cols.push(format!("c{i}"));
            u.row_y.push(i * 28);
            u.row_h.push(28);
            u.col_x.push(i * 80);
            u.col_w.push(80);
        }
        u.sel = (0, 0, 1, 1);
        u
    }

    #[test]
    fn a_pixel_lands_in_the_column_it_is_inside() {
        let u = seven_by_seven();
        assert_eq!(super::sheet_axis_at(&u.col_x, &u.col_w, 0), Some(0));
        assert_eq!(super::sheet_axis_at(&u.col_x, &u.col_w, 79), Some(0));
        assert_eq!(super::sheet_axis_at(&u.col_x, &u.col_w, 80), Some(1));
        assert_eq!(super::sheet_axis_at(&u.col_x, &u.col_w, 559), Some(6));
        assert_eq!(super::sheet_axis_at(&u.row_y, &u.row_h, 55), Some(1));
    }

    #[test]
    fn a_pixel_outside_the_grid_clamps_rather_than_missing() {
        let u = seven_by_seven();
        assert_eq!(super::sheet_axis_at(&u.col_x, &u.col_w, -20), Some(0));
        assert_eq!(super::sheet_axis_at(&u.col_x, &u.col_w, 9_000), Some(6));
        assert_eq!(super::sheet_axis_at(&[], &[], 10), None);
    }

    #[test]
    fn a_column_boundary_is_grabbed_only_within_the_grip() {
        let u = seven_by_seven();
        assert_eq!(super::sheet_axis_boundary(&u.col_x, &u.col_w, 80), Some(0));
        assert_eq!(super::sheet_axis_boundary(&u.col_x, &u.col_w, 84), Some(0));
        assert_eq!(super::sheet_axis_boundary(&u.col_x, &u.col_w, 76), Some(0));
        assert_eq!(super::sheet_axis_boundary(&u.col_x, &u.col_w, 70), None);
        assert_eq!(super::sheet_axis_boundary(&u.col_x, &u.col_w, 160), Some(1));
    }

    #[test]
    fn a_span_measures_the_columns_it_covers_and_never_runs_off_the_end() {
        let u = seven_by_seven();
        assert_eq!(super::sheet_span_size(&u.col_w, 0, 1), 80);
        assert_eq!(super::sheet_span_size(&u.col_w, 1, 3), 240);
        assert_eq!(super::sheet_span_size(&u.col_w, 5, 9), 160);
        assert_eq!(super::sheet_span_size(&u.row_h, 6, 4), 28);
    }

    #[test]
    fn a_stamp_names_one_cell_or_a_range() {
        assert_eq!(super::sheet_stamp(0, 0, 1, 1), "A1");
        assert_eq!(super::sheet_stamp(1, 1, 1, 1), "B2");
        assert_eq!(super::sheet_stamp(1, 1, 2, 3), "B2:D3");
        assert_eq!(super::sheet_stamp(0, 25, 1, 2), "Z1:AA1");
    }

    #[test]
    fn dragging_backwards_still_selects_a_forward_rectangle() {
        let mut u = seven_by_seven();
        u.anchor = (4, 4);
        u.select_to(1, 2);
        assert_eq!(u.sel, (1, 2, 4, 3));
        u.select_to(6, 6);
        assert_eq!(u.sel, (4, 4, 3, 3));
    }

    #[test]
    fn a_selection_survives_the_sheet_shrinking_under_it() {
        let mut u = seven_by_seven();
        u.sel = (6, 6, 1, 1);
        u.anchor = (6, 6);
        u.rows.truncate(3);
        u.row_y.truncate(3);
        u.row_h.truncate(3);
        u.clamp();
        assert_eq!(u.sel, (2, 6, 1, 1));
        assert_eq!(u.anchor, (2, 6));
    }

    #[test]
    fn a_range_is_trimmed_to_what_is_left_of_the_sheet() {
        let mut u = seven_by_seven();
        u.sel = (5, 5, 4, 4);
        u.clamp();
        assert_eq!(u.sel, (5, 5, 2, 2));
    }

    #[test]
    fn an_empty_sheet_has_no_selection_rather_than_a_panic() {
        let mut u = super::SheetUi {
            sel: (3, 3, 2, 2),
            ..Default::default()
        };
        u.clamp();
        assert_eq!(u.sel, (0, 0, 0, 0));
        assert!(u.empty());
    }

    #[test]
    fn landing_on_a_covered_cell_moves_to_the_merge_anchor() {
        let mut u = seven_by_seven();
        u.spans.insert((1, 1), (2, 2));
        for cell in [(1, 2), (2, 1), (2, 2)] {
            u.anchor_of.insert(cell, (1, 1));
        }
        u.sel = (2, 2, 1, 1);
        u.uncover();
        assert_eq!(u.sel, (1, 1, 1, 1));
        assert_eq!(u.anchor, (1, 1));
        assert_eq!(u.span_of(1, 1), (2, 2));
        assert_eq!(u.span_of(3, 3), (1, 1));
    }

    #[test]
    fn a_range_selection_is_left_alone_by_uncover() {
        let mut u = seven_by_seven();
        u.anchor_of.insert((2, 2), (1, 1));
        u.sel = (2, 2, 2, 2);
        u.uncover();
        assert_eq!(u.sel, (2, 2, 2, 2));
    }
}

#[cfg(test)]
mod kanban_card_tests {
    use super::*;

    #[test]
    fn nothing_set_draws_no_meta_line_at_all() {
        assert_eq!(card_meta("", 0), "");
        assert_eq!(card_height(&card_meta("", 0), 0), KANBAN_CARD_SHORT);
    }

    #[test]
    fn an_assignee_alone_is_not_paired_with_a_stray_separator() {
        assert_eq!(card_meta("mira", 0), "MIRA");
        assert_eq!(card_meta("  mira  ", 0), "MIRA");
        assert_eq!(card_height(&card_meta("mira", 0), 0), KANBAN_CARD_TALL);
    }

    #[test]
    fn a_due_date_alone_stands_without_the_word_unassigned() {
        let meta = card_meta("", 1_756_000_000_000);
        assert!(!meta.is_empty());
        assert!(!meta.contains("UNASSIGNED"), "{meta}");
        assert!(!meta.contains('·'), "{meta}");
    }

    #[test]
    fn both_set_reads_as_assignee_then_date() {
        let meta = card_meta("mira", 1_756_000_000_000);
        assert!(meta.starts_with("MIRA · "), "{meta}");
    }

    #[test]
    fn a_drop_above_every_card_lands_first() {
        let heights = [KANBAN_CARD_TALL, KANBAN_CARD_TALL];
        assert_eq!(drop_index(&heights, 0), 0);
        assert_eq!(drop_index(&heights, KANBAN_COL_HEAD_H), 0);
    }

    #[test]
    fn a_drop_below_every_card_lands_last() {
        let heights = [KANBAN_CARD_SHORT, KANBAN_CARD_TALL, KANBAN_CARD_SHORT];
        assert_eq!(drop_index(&heights, 10_000), heights.len());
    }

    #[test]
    fn mixed_heights_still_land_on_the_card_the_pointer_is_over() {
        let heights = [KANBAN_CARD_SHORT, KANBAN_CARD_TALL, KANBAN_CARD_SHORT];
        let mut top = KANBAN_COL_HEAD_H + KANBAN_CARD_GAP;
        for (i, h) in heights.iter().enumerate() {
            assert_eq!(drop_index(&heights, top), i, "top of card {i}");
            assert_eq!(drop_index(&heights, top + h - 1), i + 1, "foot of card {i}");
            top += h + KANBAN_CARD_GAP;
        }
    }

    #[test]
    fn an_empty_column_takes_a_drop_at_index_zero() {
        assert_eq!(drop_index(&[], 0), 0);
        assert_eq!(drop_index(&[], 500), 0);
    }
}

#[cfg(test)]
mod kanban_priority_height_tests {
    use super::*;

    #[test]
    fn a_priority_alone_is_enough_to_earn_the_second_row() {
        assert_eq!(card_height("", 0), KANBAN_CARD_SHORT);
        assert_eq!(card_height("", 1), KANBAN_CARD_TALL);
        assert_eq!(card_height("", 3), KANBAN_CARD_TALL);
    }
}
