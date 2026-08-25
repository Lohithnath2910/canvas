use std::time::Duration;

use slint::ComponentHandle;

use canvas_doc::{StickyNoteDoc, TodoListDoc, UndoHistory};
use canvas_storage::{WindowPlacement, KIND_DOCUMENT, KIND_STICKY_NOTE, KIND_TODO};
use canvas_ui::{platform_system_sans, Motion, StickyNoteWindow, TodoWindow, Typography};

const OPEN_DELAY: Duration = Duration::from_millis(16);
const CLOSE_FADE: Duration = Duration::from_millis(100);
const DOC_PAD: f32 = 24.0;

pub enum WidgetDoc {
    StickyNote(StickyNoteDoc),
    Todo(TodoListDoc),
}

impl WidgetDoc {
    pub fn load(kind: &str, snapshot: Option<&[u8]>) -> Option<Self> {
        match kind {
            KIND_STICKY_NOTE | KIND_DOCUMENT => Some(WidgetDoc::StickyNote(match snapshot {
                Some(bytes) if !bytes.is_empty() => StickyNoteDoc::load(bytes),
                _ => StickyNoteDoc::new(),
            })),
            KIND_TODO => Some(WidgetDoc::Todo(match snapshot {
                Some(bytes) if !bytes.is_empty() => TodoListDoc::load(bytes),
                _ => TodoListDoc::new(),
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
        }
    }

    pub fn set_text(&self, text: &str) -> Option<Vec<u8>> {
        match self {
            WidgetDoc::StickyNote(doc) => doc.set_text(text),
            WidgetDoc::Todo(_) => None,
        }
    }

    pub fn snapshot(&self) -> Vec<u8> {
        match self {
            WidgetDoc::StickyNote(doc) => doc.snapshot(),
            WidgetDoc::Todo(doc) => doc.snapshot(),
        }
    }

    pub fn state_vector(&self) -> Vec<u8> {
        match self {
            WidgetDoc::StickyNote(doc) => doc.state_vector(),
            WidgetDoc::Todo(doc) => doc.state_vector(),
        }
    }
}

pub enum WidgetWindow {
    StickyNote(StickyNoteWindow),
    Todo(TodoWindow),
}

impl Clone for WidgetWindow {
    fn clone(&self) -> Self {
        match self {
            WidgetWindow::StickyNote(window) => WidgetWindow::StickyNote(window.clone_strong()),
            WidgetWindow::Todo(window) => WidgetWindow::Todo(window.clone_strong()),
        }
    }
}

impl WidgetWindow {
    pub fn create(kind: &str, reduced_motion: bool) -> Option<Result<Self, slint::PlatformError>> {
        match kind {
            KIND_STICKY_NOTE | KIND_DOCUMENT => Some(StickyNoteWindow::new().map(|window| {
                window
                    .global::<Typography>()
                    .set_system_sans(platform_system_sans().into());
                window.global::<Motion>().set_reduced(reduced_motion);
                if kind == KIND_DOCUMENT {
                    window.set_pad(DOC_PAD);
                    window.set_delete_label("DELETE DOCUMENT".into());
                }
                WidgetWindow::StickyNote(window)
            })),
            KIND_TODO => Some(TodoWindow::new().map(|window| {
                window
                    .global::<Typography>()
                    .set_system_sans(platform_system_sans().into());
                window.global::<Motion>().set_reduced(reduced_motion);
                WidgetWindow::Todo(window)
            })),
            _ => None,
        }
    }

    pub fn slint(&self) -> &slint::Window {
        match self {
            WidgetWindow::StickyNote(window) => window.window(),
            WidgetWindow::Todo(window) => window.window(),
        }
    }

    pub fn show(&self) -> Result<(), slint::PlatformError> {
        match self {
            WidgetWindow::StickyNote(window) => window.show(),
            WidgetWindow::Todo(window) => window.show(),
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
        }
    }

    pub fn set_reduced_motion(&self, reduced: bool) {
        match self {
            WidgetWindow::StickyNote(window) => window.global::<Motion>().set_reduced(reduced),
            WidgetWindow::Todo(window) => window.global::<Motion>().set_reduced(reduced),
        }
    }

    pub fn set_title(&self, title: &str) {
        match self {
            WidgetWindow::StickyNote(window) => window.set_note_title(title.into()),
            WidgetWindow::Todo(window) => window.set_note_title(title.into()),
        }
    }

    pub fn set_text(&self, text: &str) {
        match self {
            WidgetWindow::StickyNote(window) => window.set_note_text(text.into()),
            WidgetWindow::Todo(_) => {}
        }
    }

    pub fn apply_placement(&self, placement: &WindowPlacement) {
        match self {
            WidgetWindow::StickyNote(window) => {
                window.set_pinned(placement.pinned);
                window.set_tone(placement.tone as i32);
            }
            WidgetWindow::Todo(window) => window.set_pinned(placement.pinned),
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
        }
    }

    fn set_opened(&self, value: bool) {
        match self {
            WidgetWindow::StickyNote(window) => window.set_opened(value),
            WidgetWindow::Todo(window) => window.set_opened(value),
        }
    }

    fn set_closing(&self, value: bool) {
        match self {
            WidgetWindow::StickyNote(window) => window.set_closing(value),
            WidgetWindow::Todo(window) => window.set_closing(value),
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
