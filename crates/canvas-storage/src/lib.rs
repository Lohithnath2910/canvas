use rusqlite::{params, Connection, OptionalExtension};
use std::path::{Path, PathBuf};

pub const SCHEMA_VERSION: i64 = 1;
pub const DEFAULT_TITLE: &str = "UNTITLED";

pub const KIND_STICKY_NOTE: &str = "sticky_note";
pub const KIND_TODO: &str = "todo";
pub const KIND_KANBAN: &str = "kanban";
pub const KIND_CALENDAR: &str = "calendar";
pub const KIND_SPREADSHEET: &str = "spreadsheet";

pub type Result<T> = std::result::Result<T, rusqlite::Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct Widget {
    pub id: String,
    pub workspace_id: String,
    pub widget_type: String,
    pub title: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowPlacement {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub z_order: i64,
    pub pinned: bool,
    pub tone: i64,
    pub open: bool,
}

impl WindowPlacement {
    pub fn default_for(widget_type: &str) -> Self {
        let (width, height) = default_size(widget_type);
        Self {
            x: 120,
            y: 120,
            width,
            height,
            z_order: 0,
            pinned: false,
            tone: 0,
            open: true,
        }
    }
}

pub fn default_size(widget_type: &str) -> (i32, i32) {
    match widget_type {
        KIND_TODO => (340, 360),
        KIND_KANBAN => (640, 460),
        KIND_CALENDAR => (560, 460),
        KIND_SPREADSHEET => (640, 360),
        _ => (320, 280),
    }
}

pub struct Store {
    conn: Connection,
    workspace_id: String,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        Self::from_connection(conn)
    }

    pub fn open_in_memory() -> Result<Self> {
        Self::from_connection(Connection::open_in_memory()?)
    }

    fn from_connection(conn: Connection) -> Result<Self> {
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        let mut store = Self {
            conn,
            workspace_id: String::new(),
        };
        store.migrate()?;
        store.workspace_id = store.ensure_default_workspace()?;
        Ok(store)
    }

    pub fn default_path() -> PathBuf {
        let base = std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("."));
        base.join("Canvas")
    }

    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    fn migrate(&self) -> Result<()> {
        let current: i64 = self
            .conn
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .unwrap_or(0);

        if current >= SCHEMA_VERSION {
            return Ok(());
        }

        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS workspaces (
                id TEXT PRIMARY KEY,
                name TEXT
            );

            CREATE TABLE IF NOT EXISTS widgets (
                id TEXT PRIMARY KEY,
                workspace_id TEXT,
                widget_type TEXT,
                title TEXT,
                created_at INTEGER,
                updated_at INTEGER
            );

            CREATE TABLE IF NOT EXISTS window_positions (
                widget_id TEXT PRIMARY KEY,
                x INTEGER, y INTEGER,
                width INTEGER, height INTEGER,
                monitor_id TEXT,
                z_order INTEGER,
                pinned INTEGER DEFAULT 0,
                tone INTEGER DEFAULT 0,
                open INTEGER DEFAULT 1
            );

            CREATE TABLE IF NOT EXISTS documents (
                widget_id TEXT PRIMARY KEY,
                yrs_snapshot BLOB,
                yrs_state_vector BLOB,
                schema_version INTEGER
            );

            CREATE TABLE IF NOT EXISTS revision_history (
                widget_id TEXT,
                update_blob BLOB,
                created_at INTEGER
            );

            CREATE INDEX IF NOT EXISTS idx_revision_widget
                ON revision_history (widget_id, created_at);

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT
            );

            CREATE TABLE IF NOT EXISTS assets_cache (
                id TEXT PRIMARY KEY,
                data BLOB
            );
            "#,
        )?;

        self.conn
            .pragma_update(None, "user_version", SCHEMA_VERSION)?;
        Ok(())
    }

    fn ensure_default_workspace(&self) -> Result<String> {
        let existing: Option<String> = self
            .conn
            .query_row("SELECT id FROM workspaces LIMIT 1", [], |r| r.get(0))
            .optional()?;
        if let Some(id) = existing {
            return Ok(id);
        }
        let id = uuid::Uuid::new_v4().to_string();
        self.conn.execute(
            "INSERT INTO workspaces (id, name) VALUES (?1, ?2)",
            params![id, "Default"],
        )?;
        Ok(id)
    }

    pub fn create_widget(&self, widget_type: &str) -> Result<(Widget, WindowPlacement)> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = now_millis();
        let widget = Widget {
            id: id.clone(),
            workspace_id: self.workspace_id.clone(),
            widget_type: widget_type.to_string(),
            title: DEFAULT_TITLE.to_string(),
            created_at: now,
            updated_at: now,
        };
        self.conn.execute(
            "INSERT INTO widgets (id, workspace_id, widget_type, title, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                widget.id,
                widget.workspace_id,
                widget.widget_type,
                widget.title,
                widget.created_at,
                widget.updated_at
            ],
        )?;

        let placement = WindowPlacement::default_for(widget_type);
        self.save_placement(&id, &placement)?;
        self.save_document(&id, &[], &[])?;
        Ok((widget, placement))
    }

    pub fn list_widgets(&self) -> Result<Vec<Widget>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, workspace_id, widget_type, title, created_at, updated_at
             FROM widgets ORDER BY created_at ASC",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(Widget {
                id: r.get(0)?,
                workspace_id: r.get(1)?,
                widget_type: r.get(2)?,
                title: r.get(3)?,
                created_at: r.get(4)?,
                updated_at: r.get(5)?,
            })
        })?;
        rows.collect()
    }

    pub fn get_widget(&self, id: &str) -> Result<Option<Widget>> {
        self.conn
            .query_row(
                "SELECT id, workspace_id, widget_type, title, created_at, updated_at
                 FROM widgets WHERE id = ?1",
                params![id],
                |r| {
                    Ok(Widget {
                        id: r.get(0)?,
                        workspace_id: r.get(1)?,
                        widget_type: r.get(2)?,
                        title: r.get(3)?,
                        created_at: r.get(4)?,
                        updated_at: r.get(5)?,
                    })
                },
            )
            .optional()
    }

    pub fn set_title(&self, id: &str, title: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE widgets SET title = ?2, updated_at = ?3 WHERE id = ?1",
            params![id, title, now_millis()],
        )?;
        Ok(())
    }

    pub fn touch(&self, id: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE widgets SET updated_at = ?2 WHERE id = ?1",
            params![id, now_millis()],
        )?;
        Ok(())
    }

    pub fn save_placement(&self, id: &str, p: &WindowPlacement) -> Result<()> {
        self.conn.execute(
            "INSERT INTO window_positions
                (widget_id, x, y, width, height, monitor_id, z_order, pinned, tone, open)
             VALUES (?1, ?2, ?3, ?4, ?5, NULL, ?6, ?7, ?8, ?9)
             ON CONFLICT(widget_id) DO UPDATE SET
                x = ?2, y = ?3, width = ?4, height = ?5,
                z_order = ?6, pinned = ?7, tone = ?8, open = ?9",
            params![
                id,
                p.x,
                p.y,
                p.width,
                p.height,
                p.z_order,
                p.pinned as i64,
                p.tone,
                p.open as i64
            ],
        )?;
        Ok(())
    }

    pub fn get_placement(&self, id: &str) -> Result<Option<WindowPlacement>> {
        self.conn
            .query_row(
                "SELECT x, y, width, height, z_order, pinned, tone, open
                 FROM window_positions WHERE widget_id = ?1",
                params![id],
                |r| {
                    Ok(WindowPlacement {
                        x: r.get(0)?,
                        y: r.get(1)?,
                        width: r.get(2)?,
                        height: r.get(3)?,
                        z_order: r.get(4)?,
                        pinned: r.get::<_, i64>(5)? != 0,
                        tone: r.get(6)?,
                        open: r.get::<_, i64>(7)? != 0,
                    })
                },
            )
            .optional()
    }

    pub fn set_open(&self, id: &str, open: bool) -> Result<()> {
        self.conn.execute(
            "UPDATE window_positions SET open = ?2 WHERE widget_id = ?1",
            params![id, open as i64],
        )?;
        Ok(())
    }

    pub fn next_z_order(&self) -> Result<i64> {
        let max: Option<i64> = self
            .conn
            .query_row("SELECT MAX(z_order) FROM window_positions", [], |r| {
                r.get(0)
            })
            .optional()?
            .flatten();
        Ok(max.unwrap_or(0) + 1)
    }

    pub fn save_document(&self, id: &str, snapshot: &[u8], state_vector: &[u8]) -> Result<()> {
        self.conn.execute(
            "INSERT INTO documents (widget_id, yrs_snapshot, yrs_state_vector, schema_version)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(widget_id) DO UPDATE SET
                yrs_snapshot = ?2, yrs_state_vector = ?3, schema_version = ?4",
            params![id, snapshot, state_vector, SCHEMA_VERSION],
        )?;
        Ok(())
    }

    pub fn get_document(&self, id: &str) -> Result<Option<Vec<u8>>> {
        self.conn
            .query_row(
                "SELECT yrs_snapshot FROM documents WHERE widget_id = ?1",
                params![id],
                |r| r.get(0),
            )
            .optional()
    }

    pub fn append_revision(&self, id: &str, update: &[u8]) -> Result<()> {
        self.conn.execute(
            "INSERT INTO revision_history (widget_id, update_blob, created_at)
             VALUES (?1, ?2, ?3)",
            params![id, update, now_millis()],
        )?;
        self.trim_revisions(id)?;
        Ok(())
    }

    pub fn revision_count(&self, id: &str) -> Result<i64> {
        self.conn.query_row(
            "SELECT COUNT(*) FROM revision_history WHERE widget_id = ?1",
            params![id],
            |r| r.get(0),
        )
    }

    pub fn revisions(&self, id: &str) -> Result<Vec<Vec<u8>>> {
        let mut stmt = self.conn.prepare(
            "SELECT update_blob FROM revision_history
             WHERE widget_id = ?1 ORDER BY created_at ASC, rowid ASC",
        )?;
        let rows = stmt.query_map(params![id], |r| r.get(0))?;
        rows.collect()
    }

    pub fn revisions_with_time(&self, id: &str) -> Result<Vec<(Vec<u8>, i64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT update_blob, created_at FROM revision_history
             WHERE widget_id = ?1 ORDER BY created_at ASC, rowid ASC",
        )?;
        let rows = stmt.query_map(params![id], |r| Ok((r.get(0)?, r.get(1)?)))?;
        rows.collect()
    }
    fn trim_revisions(&self, id: &str) -> Result<()> {
        const HIGH_WATER: i64 = 500;
        const KEEP: i64 = 200;

        if self.revision_count(id)? <= HIGH_WATER {
            return Ok(());
        }
        self.conn.execute(
            "DELETE FROM revision_history
             WHERE widget_id = ?1 AND rowid NOT IN (
                 SELECT rowid FROM revision_history
                 WHERE widget_id = ?1
                 ORDER BY created_at DESC, rowid DESC
                 LIMIT ?2
             )",
            params![id, KEEP],
        )?;
        Ok(())
    }

    pub fn delete_widget(&self, id: &str) -> Result<()> {
        let tx_stmts = [
            "DELETE FROM revision_history WHERE widget_id = ?1",
            "DELETE FROM documents WHERE widget_id = ?1",
            "DELETE FROM window_positions WHERE widget_id = ?1",
            "DELETE FROM widgets WHERE id = ?1",
        ];
        for sql in tx_stmts {
            self.conn.execute(sql, params![id])?;
        }
        Ok(())
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        self.conn
            .query_row(
                "SELECT value FROM settings WHERE key = ?1",
                params![key],
                |r| r.get(0),
            )
            .optional()
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = ?2",
            params![key, value],
        )?;
        Ok(())
    }
}

pub fn now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> Store {
        Store::open_in_memory().expect("in-memory store")
    }

    #[test]
    fn revisions_come_back_with_their_timestamps_in_order() {
        let s = store();
        let (w, _) = s.create_widget(KIND_STICKY_NOTE).unwrap();
        s.append_revision(&w.id, b"first").unwrap();
        s.append_revision(&w.id, b"second").unwrap();

        let rows = s.revisions_with_time(&w.id).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, b"first");
        assert_eq!(rows[1].0, b"second");
        assert!(rows[0].1 <= rows[1].1);
        assert!(rows[0].1 > 0);
    }
    #[test]
    fn creates_every_table() {
        let s = store();
        let mut stmt = s
            .conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap();
        let names: Vec<String> = stmt
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<Result<_>>()
            .unwrap();
        for expected in [
            "assets_cache",
            "documents",
            "revision_history",
            "settings",
            "widgets",
            "window_positions",
            "workspaces",
        ] {
            assert!(names.contains(&expected.to_string()), "missing {expected}");
        }
    }

    #[test]
    fn migration_is_idempotent() {
        let s = store();
        s.migrate().unwrap();
        s.migrate().unwrap();
        let v: i64 = s
            .conn
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .unwrap();
        assert_eq!(v, SCHEMA_VERSION);
    }

    #[test]
    fn new_widget_gets_untitled_and_its_type_size() {
        let s = store();
        let (note, place) = s.create_widget(KIND_STICKY_NOTE).unwrap();
        assert_eq!(note.title, DEFAULT_TITLE);
        assert_eq!((place.width, place.height), (320, 280));

        let (_, todo) = s.create_widget(KIND_TODO).unwrap();
        assert_eq!((todo.width, todo.height), (340, 360));

        let (_, kanban) = s.create_widget(KIND_KANBAN).unwrap();
        assert_eq!((kanban.width, kanban.height), (640, 460));
    }

    #[test]
    fn documents_table_is_widget_type_agnostic() {
        let s = store();
        let (note, _) = s.create_widget(KIND_STICKY_NOTE).unwrap();
        let (todo, _) = s.create_widget(KIND_TODO).unwrap();

        s.save_document(&note.id, b"note-bytes", b"sv1").unwrap();
        s.save_document(&todo.id, b"todo-bytes", b"sv2").unwrap();

        assert_eq!(s.get_document(&note.id).unwrap().unwrap(), b"note-bytes");
        assert_eq!(s.get_document(&todo.id).unwrap().unwrap(), b"todo-bytes");
    }

    #[test]
    fn placement_round_trips() {
        let s = store();
        let (w, _) = s.create_widget(KIND_STICKY_NOTE).unwrap();
        let p = WindowPlacement {
            x: 42,
            y: 99,
            width: 400,
            height: 500,
            z_order: 7,
            pinned: true,
            tone: 3,
            open: false,
        };
        s.save_placement(&w.id, &p).unwrap();
        assert_eq!(s.get_placement(&w.id).unwrap().unwrap(), p);
    }

    #[test]
    fn saving_placement_twice_updates_rather_than_duplicating() {
        let s = store();
        let (w, mut p) = s.create_widget(KIND_STICKY_NOTE).unwrap();
        p.x = 1;
        s.save_placement(&w.id, &p).unwrap();
        p.x = 2;
        s.save_placement(&w.id, &p).unwrap();
        assert_eq!(s.get_placement(&w.id).unwrap().unwrap().x, 2);
        let count: i64 = s
            .conn
            .query_row("SELECT COUNT(*) FROM window_positions", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn revision_history_is_bounded() {
        let s = store();
        let (w, _) = s.create_widget(KIND_STICKY_NOTE).unwrap();
        for i in 0..600 {
            s.append_revision(&w.id, format!("update-{i}").as_bytes())
                .unwrap();
        }
        let count = s.revision_count(&w.id).unwrap();
        assert!(
            count <= 500,
            "revision history grew unbounded: {count} rows after 600 updates"
        );
        assert!(count < 600, "nothing was trimmed at all: {count} rows");
    }

    #[test]
    fn revision_history_stays_bounded_over_many_edits() {
        let s = store();
        let (w, _) = s.create_widget(KIND_STICKY_NOTE).unwrap();
        for i in 0..5000 {
            s.append_revision(&w.id, format!("{i}").as_bytes()).unwrap();
        }
        let count = s.revision_count(&w.id).unwrap();
        assert!(count <= 500, "{count} rows after 5000 updates");
    }

    #[test]
    fn trimming_keeps_the_most_recent() {
        let s = store();
        let (w, _) = s.create_widget(KIND_STICKY_NOTE).unwrap();
        for i in 0..600 {
            s.append_revision(&w.id, format!("{i}").as_bytes()).unwrap();
        }
        let revs = s.revisions(&w.id).unwrap();
        let last = String::from_utf8(revs.last().unwrap().clone()).unwrap();
        assert_eq!(last, "599");
    }

    #[test]
    fn trimming_one_widget_leaves_others_alone() {
        let s = store();
        let (a, _) = s.create_widget(KIND_STICKY_NOTE).unwrap();
        let (b, _) = s.create_widget(KIND_STICKY_NOTE).unwrap();
        s.append_revision(&b.id, b"keep-me").unwrap();
        for i in 0..600 {
            s.append_revision(&a.id, format!("{i}").as_bytes()).unwrap();
        }
        assert_eq!(s.revision_count(&b.id).unwrap(), 1);
    }

    #[test]
    fn delete_removes_every_trace() {
        let s = store();
        let (w, _) = s.create_widget(KIND_STICKY_NOTE).unwrap();
        s.save_document(&w.id, b"bytes", b"sv").unwrap();
        s.append_revision(&w.id, b"update").unwrap();

        s.delete_widget(&w.id).unwrap();

        assert!(s.get_widget(&w.id).unwrap().is_none());
        assert!(s.get_placement(&w.id).unwrap().is_none());
        assert!(s.get_document(&w.id).unwrap().is_none());
        assert_eq!(s.revision_count(&w.id).unwrap(), 0);
    }

    #[test]
    fn open_flag_separates_closing_from_deleting() {
        let s = store();
        let (w, _) = s.create_widget(KIND_STICKY_NOTE).unwrap();
        s.save_document(&w.id, b"my text", b"sv").unwrap();

        s.set_open(&w.id, false).unwrap();

        assert!(!s.get_placement(&w.id).unwrap().unwrap().open);
        assert!(s.get_widget(&w.id).unwrap().is_some());
        assert_eq!(s.get_document(&w.id).unwrap().unwrap(), b"my text");
    }

    #[test]
    fn z_order_increases() {
        let s = store();
        let first = s.next_z_order().unwrap();
        let (w, mut p) = s.create_widget(KIND_STICKY_NOTE).unwrap();
        p.z_order = first;
        s.save_placement(&w.id, &p).unwrap();
        assert!(s.next_z_order().unwrap() > first);
    }

    #[test]
    fn listing_is_stable_by_creation_order() {
        let s = store();
        let (a, _) = s.create_widget(KIND_STICKY_NOTE).unwrap();
        let (b, _) = s.create_widget(KIND_TODO).unwrap();
        let ids: Vec<String> = s
            .list_widgets()
            .unwrap()
            .into_iter()
            .map(|w| w.id)
            .collect();
        assert_eq!(ids, vec![a.id, b.id]);
    }

    #[test]
    fn settings_round_trip() {
        let s = store();
        assert!(s.get_setting("theme").unwrap().is_none());
        s.set_setting("theme", "dark").unwrap();
        s.set_setting("theme", "light").unwrap();
        assert_eq!(s.get_setting("theme").unwrap().unwrap(), "light");
    }

    #[test]
    fn reopening_a_database_keeps_its_data() {
        let dir = std::env::temp_dir().join(format!("canvas-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("canvas_local.db");

        let id = {
            let s = Store::open(&path).unwrap();
            let (w, _) = s.create_widget(KIND_STICKY_NOTE).unwrap();
            s.save_document(&w.id, b"persisted", b"sv").unwrap();
            w.id
        };

        {
            let s = Store::open(&path).unwrap();
            assert!(s.get_widget(&id).unwrap().is_some());
            assert_eq!(s.get_document(&id).unwrap().unwrap(), b"persisted");
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
}
