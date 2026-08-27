use std::time::Duration;

use slint::ComponentHandle;

use canvas_doc::{CalendarDoc, KanbanDoc, SheetDoc, StickyNoteDoc, TodoListDoc, UndoHistory};
use canvas_storage::{
    WindowPlacement, KIND_CALENDAR, KIND_KANBAN, KIND_SPREADSHEET, KIND_STICKY_NOTE, KIND_TODO,
};
use canvas_ui::{
    platform_system_sans, CalendarWindow, KanbanWindow, Motion, SpreadsheetWindow,
    StickyNoteWindow, TodoWindow, Typography,
};

const OPEN_DELAY: Duration = Duration::from_millis(16);
const CLOSE_FADE: Duration = Duration::from_millis(100);

pub enum WidgetDoc {
    StickyNote(StickyNoteDoc),
    Todo(TodoListDoc),
    Calendar(CalendarDoc),
    Kanban(KanbanDoc),
    Sheet(SheetDoc),
}

impl WidgetDoc {
    pub fn load(kind: &str, snapshot: Option<&[u8]>) -> Option<Self> {
        match kind {
            KIND_STICKY_NOTE => Some(WidgetDoc::StickyNote(match snapshot {
                Some(bytes) if !bytes.is_empty() => StickyNoteDoc::load(bytes),
                _ => StickyNoteDoc::new(),
            })),
            KIND_TODO => Some(WidgetDoc::Todo(match snapshot {
                Some(bytes) if !bytes.is_empty() => TodoListDoc::load(bytes),
                _ => TodoListDoc::new(),
            })),
            KIND_CALENDAR => Some(WidgetDoc::Calendar(match snapshot {
                Some(bytes) if !bytes.is_empty() => CalendarDoc::load(bytes),
                _ => CalendarDoc::new(),
            })),
            KIND_SPREADSHEET => Some(WidgetDoc::Sheet(match snapshot {
                Some(bytes) if !bytes.is_empty() => SheetDoc::load(bytes),
                _ => SheetDoc::seeded(),
            })),
            KIND_KANBAN => Some(WidgetDoc::Kanban(match snapshot {
                Some(bytes) if !bytes.is_empty() => KanbanDoc::load(bytes),
                _ => KanbanDoc::seeded(),
            })),
            _ => None,
        }
    }

    pub fn text(&self) -> String {
        match self {
            WidgetDoc::StickyNote(doc) => doc.get_text(),
            WidgetDoc::Todo(doc) => doc
                .items()
                .into_iter()
                .map(|i| i.text)
                .collect::<Vec<String>>()
                .join(", "),
            WidgetDoc::Calendar(doc) => doc
                .events()
                .into_iter()
                .map(|e| e.title)
                .collect::<Vec<String>>()
                .join(", "),
            WidgetDoc::Kanban(doc) => doc
                .cards()
                .into_iter()
                .map(|c| c.title)
                .collect::<Vec<String>>()
                .join(", "),
            WidgetDoc::Sheet(doc) => doc.filled_values().join(", "),
        }
    }

    pub fn set_text(&self, text: &str) -> Option<Vec<u8>> {
        match self {
            WidgetDoc::StickyNote(doc) => doc.set_text(text),
            WidgetDoc::Todo(_)
            | WidgetDoc::Calendar(_)
            | WidgetDoc::Kanban(_)
            | WidgetDoc::Sheet(_) => None,
        }
    }

    pub fn seed(kind: &str) -> Option<Self> {
        match kind {
            KIND_KANBAN => Some(WidgetDoc::Kanban(KanbanDoc::seeded())),
            KIND_SPREADSHEET => Some(WidgetDoc::Sheet(SheetDoc::seeded())),
            _ => None,
        }
    }

    pub fn snapshot(&self) -> Vec<u8> {
        match self {
            WidgetDoc::StickyNote(doc) => doc.snapshot(),
            WidgetDoc::Todo(doc) => doc.snapshot(),
            WidgetDoc::Calendar(doc) => doc.snapshot(),
            WidgetDoc::Kanban(doc) => doc.snapshot(),
            WidgetDoc::Sheet(doc) => doc.snapshot(),
        }
    }

    pub fn state_vector(&self) -> Vec<u8> {
        match self {
            WidgetDoc::StickyNote(doc) => doc.state_vector(),
            WidgetDoc::Todo(doc) => doc.state_vector(),
            WidgetDoc::Calendar(doc) => doc.state_vector(),
            WidgetDoc::Kanban(doc) => doc.state_vector(),
            WidgetDoc::Sheet(doc) => doc.state_vector(),
        }
    }
}

pub enum WidgetWindow {
    StickyNote(StickyNoteWindow),
    Todo(TodoWindow),
    Calendar(CalendarWindow),
    Kanban(KanbanWindow),
    Spreadsheet(SpreadsheetWindow),
}

impl Clone for WidgetWindow {
    fn clone(&self) -> Self {
        match self {
            WidgetWindow::StickyNote(window) => WidgetWindow::StickyNote(window.clone_strong()),
            WidgetWindow::Todo(window) => WidgetWindow::Todo(window.clone_strong()),
            WidgetWindow::Calendar(window) => WidgetWindow::Calendar(window.clone_strong()),
            WidgetWindow::Kanban(window) => WidgetWindow::Kanban(window.clone_strong()),
            WidgetWindow::Spreadsheet(window) => WidgetWindow::Spreadsheet(window.clone_strong()),
        }
    }
}

impl WidgetWindow {
    pub fn create(kind: &str, reduced_motion: bool) -> Option<Result<Self, slint::PlatformError>> {
        match kind {
            KIND_STICKY_NOTE => Some(StickyNoteWindow::new().map(|window| {
                window
                    .global::<Typography>()
                    .set_system_sans(platform_system_sans().into());
                window.global::<Motion>().set_reduced(reduced_motion);
                WidgetWindow::StickyNote(window)
            })),
            KIND_TODO => Some(TodoWindow::new().map(|window| {
                window
                    .global::<Typography>()
                    .set_system_sans(platform_system_sans().into());
                window.global::<Motion>().set_reduced(reduced_motion);
                WidgetWindow::Todo(window)
            })),
            KIND_CALENDAR => Some(CalendarWindow::new().map(|window| {
                window
                    .global::<Typography>()
                    .set_system_sans(platform_system_sans().into());
                window.global::<Motion>().set_reduced(reduced_motion);
                WidgetWindow::Calendar(window)
            })),
            KIND_KANBAN => Some(KanbanWindow::new().map(|window| {
                window
                    .global::<Typography>()
                    .set_system_sans(platform_system_sans().into());
                window.global::<Motion>().set_reduced(reduced_motion);
                WidgetWindow::Kanban(window)
            })),
            KIND_SPREADSHEET => Some(SpreadsheetWindow::new().map(|window| {
                window
                    .global::<Typography>()
                    .set_system_sans(platform_system_sans().into());
                window.global::<Motion>().set_reduced(reduced_motion);
                WidgetWindow::Spreadsheet(window)
            })),
            _ => None,
        }
    }

    pub fn slint(&self) -> &slint::Window {
        match self {
            WidgetWindow::StickyNote(window) => window.window(),
            WidgetWindow::Todo(window) => window.window(),
            WidgetWindow::Calendar(window) => window.window(),
            WidgetWindow::Kanban(window) => window.window(),
            WidgetWindow::Spreadsheet(window) => window.window(),
        }
    }

    pub fn show(&self) -> Result<(), slint::PlatformError> {
        match self {
            WidgetWindow::StickyNote(window) => window.show(),
            WidgetWindow::Todo(window) => window.show(),
            WidgetWindow::Calendar(window) => window.show(),
            WidgetWindow::Kanban(window) => window.show(),
            WidgetWindow::Spreadsheet(window) => window.show(),
        }
    }

    pub fn hide(&self) {
        match self {
            WidgetWindow::StickyNote(window) => {
                let _ = window.hide();
            }
            WidgetWindow::Todo(window) => {
                let _ = window.hide();
            }
            WidgetWindow::Calendar(window) => {
                let _ = window.hide();
            }
            WidgetWindow::Kanban(window) => {
                let _ = window.hide();
            }
            WidgetWindow::Spreadsheet(window) => {
                let _ = window.hide();
            }
        }
    }

    pub fn set_reduced_motion(&self, reduced: bool) {
        match self {
            WidgetWindow::StickyNote(window) => window.global::<Motion>().set_reduced(reduced),
            WidgetWindow::Todo(window) => window.global::<Motion>().set_reduced(reduced),
            WidgetWindow::Calendar(window) => window.global::<Motion>().set_reduced(reduced),
            WidgetWindow::Kanban(window) => window.global::<Motion>().set_reduced(reduced),
            WidgetWindow::Spreadsheet(window) => window.global::<Motion>().set_reduced(reduced),
        }
    }

    pub fn set_title(&self, title: &str) {
        match self {
            WidgetWindow::StickyNote(window) => window.set_note_title(title.into()),
            WidgetWindow::Todo(window) => window.set_note_title(title.into()),
            WidgetWindow::Calendar(window) => window.set_note_title(title.into()),
            WidgetWindow::Kanban(window) => window.set_note_title(title.into()),
            WidgetWindow::Spreadsheet(window) => window.set_note_title(title.into()),
        }
    }

    pub fn set_text(&self, text: &str) {
        match self {
            WidgetWindow::StickyNote(window) => window.set_note_text(text.into()),
            WidgetWindow::Todo(_)
            | WidgetWindow::Calendar(_)
            | WidgetWindow::Kanban(_)
            | WidgetWindow::Spreadsheet(_) => {}
        }
    }

    pub fn apply_placement(&self, placement: &WindowPlacement) {
        match self {
            WidgetWindow::StickyNote(window) => {
                window.set_pinned(placement.pinned);
                window.set_tone(placement.tone as i32);
            }
            WidgetWindow::Todo(window) => window.set_pinned(placement.pinned),
            WidgetWindow::Calendar(window) => window.set_pinned(placement.pinned),
            WidgetWindow::Kanban(window) => window.set_pinned(placement.pinned),
            WidgetWindow::Spreadsheet(window) => window.set_pinned(placement.pinned),
        }
    }

    pub fn scale_resize_border(&self) {
        match self {
            WidgetWindow::StickyNote(window) => {
                let scale = window.window().scale_factor();
                window.set_resize_border(window.get_resize_border() * scale);
            }
            WidgetWindow::Todo(window) => {
                let scale = window.window().scale_factor();
                window.set_resize_border(window.get_resize_border() * scale);
            }
            WidgetWindow::Calendar(window) => {
                let scale = window.window().scale_factor();
                window.set_resize_border(window.get_resize_border() * scale);
            }
            WidgetWindow::Kanban(window) => {
                let scale = window.window().scale_factor();
                window.set_resize_border(window.get_resize_border() * scale);
            }
            WidgetWindow::Spreadsheet(window) => {
                let scale = window.window().scale_factor();
                window.set_resize_border(window.get_resize_border() * scale);
            }
        }
    }

    fn set_opened(&self, value: bool) {
        match self {
            WidgetWindow::StickyNote(window) => window.set_opened(value),
            WidgetWindow::Todo(window) => window.set_opened(value),
            WidgetWindow::Calendar(window) => window.set_opened(value),
            WidgetWindow::Kanban(window) => window.set_opened(value),
            WidgetWindow::Spreadsheet(window) => window.set_opened(value),
        }
    }

    fn set_closing(&self, value: bool) {
        match self {
            WidgetWindow::StickyNote(window) => window.set_closing(value),
            WidgetWindow::Todo(window) => window.set_closing(value),
            WidgetWindow::Calendar(window) => window.set_closing(value),
            WidgetWindow::Kanban(window) => window.set_closing(value),
            WidgetWindow::Spreadsheet(window) => window.set_closing(value),
        }
    }

    fn weak_open(&self) -> Box<dyn Fn()> {
        match self {
            WidgetWindow::StickyNote(window) => {
                let weak = window.as_weak();
                Box::new(move || {
                    if let Some(window) = weak.upgrade() {
                        window.set_opened(true);
                    }
                })
            }
            WidgetWindow::Todo(window) => {
                let weak = window.as_weak();
                Box::new(move || {
                    if let Some(window) = weak.upgrade() {
                        window.set_opened(true);
                    }
                })
            }
            WidgetWindow::Calendar(window) => {
                let weak = window.as_weak();
                Box::new(move || {
                    if let Some(window) = weak.upgrade() {
                        window.set_opened(true);
                    }
                })
            }
            WidgetWindow::Kanban(window) => {
                let weak = window.as_weak();
                Box::new(move || {
                    if let Some(window) = weak.upgrade() {
                        window.set_opened(true);
                    }
                })
            }
            WidgetWindow::Spreadsheet(window) => {
                let weak = window.as_weak();
                Box::new(move || {
                    if let Some(window) = weak.upgrade() {
                        window.set_opened(true);
                    }
                })
            }
        }
    }

    pub fn play_open(&self) {
        self.set_closing(false);
        self.set_opened(false);
        let raise = self.weak_open();
        slint::Timer::single_shot(OPEN_DELAY, raise);
    }

    pub fn play_close(self) {
        self.set_closing(true);
        slint::Timer::single_shot(CLOSE_FADE, move || {
            self.hide();
            self.set_closing(false);
            self.set_opened(false);
        });
    }
}

pub struct LiveWidget {
    pub window: WidgetWindow,
    pub doc: WidgetDoc,
    pub history: UndoHistory,
    pub placement: WindowPlacement,
    pub pending_updates: Vec<Vec<u8>>,
    pub text_dirty: bool,
    pub placement_dirty: bool,
    pub poll_timer: Option<slint::Timer>,
}
