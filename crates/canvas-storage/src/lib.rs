use rusqlite::{params, Connection, Result};

pub struct Storage {
    conn: Connection,
}

#[derive(Debug, Clone)]
pub struct WidgetRecord {
    pub id: String,
    pub workspace_id: String,
    pub widget_type: String,
    pub title: String,
    pub pos_x: i32,
    pub pos_y: i32,
}

impl Storage {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let mut storage = Self { conn };
        storage.init_schema()?;
        Ok(storage)
    }

    fn init_schema(&mut self) -> Result<()> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "CREATE TABLE IF NOT EXISTS workspaces (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                is_local_only INTEGER NOT NULL DEFAULT 1,
                created_at INTEGER NOT NULL
            );",
            [],
        )?;
        tx.execute(
            "CREATE TABLE IF NOT EXISTS widgets (
                id TEXT PRIMARY KEY,
                workspace_id TEXT NOT NULL REFERENCES workspaces(id),
                widget_type TEXT NOT NULL,
                title TEXT,
                pos_x INTEGER NOT NULL DEFAULT 100,
                pos_y INTEGER NOT NULL DEFAULT 100,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );",            [],
        )?;
        tx.execute(
            "CREATE TABLE IF NOT EXISTS documents (
                widget_id TEXT PRIMARY KEY REFERENCES widgets(id),
                yrs_snapshot BLOB NOT NULL,
                yrs_state_vector BLOB NOT NULL,
                schema_version INTEGER NOT NULL DEFAULT 1
            );",
            [],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn ensure_local_workspace(&self) -> Result<String> {
        let mut stmt = self.conn.prepare("SELECT id FROM workspaces WHERE is_local_only = 1 LIMIT 1")?;
        let mut rows = stmt.query([])?;
        
        if let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            Ok(id)
        } else {
            let id = uuid::Uuid::new_v4().to_string();
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
            self.conn.execute(
                "INSERT INTO workspaces (id, name, is_local_only, created_at) VALUES (?1, ?2, 1, ?3)",
                params![id, "Local Workspace", now as i64],
            )?;
            Ok(id)
        }
    }

    pub fn get_all_widgets(&self, workspace_id: &str) -> Result<Vec<WidgetRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, workspace_id, widget_type, title, pos_x, pos_y FROM widgets WHERE workspace_id = ?1 ORDER BY created_at ASC"
        )?;
        let rows = stmt.query_map(params![workspace_id], |row| {
            Ok(WidgetRecord {
                id: row.get(0)?,
                workspace_id: row.get(1)?,
                widget_type: row.get(2)?,
                title: row.get(3)?,
                pos_x: row.get(4)?,
                pos_y: row.get(5)?,
            })
        })?;
        
        let mut widgets = Vec::new();
        for w in rows {
            widgets.push(w?);
        }
        Ok(widgets)
    }

    pub fn create_widget(&self, workspace_id: &str, widget_type: &str, pos_x: i32, pos_y: i32) -> Result<WidgetRecord> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        self.conn.execute(
            "INSERT INTO widgets (id, workspace_id, widget_type, title, pos_x, pos_y, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)",
            params![id, workspace_id, widget_type, "Sticky Note", pos_x, pos_y, now as i64],
        )?;
        Ok(WidgetRecord {
            id,
            workspace_id: workspace_id.to_string(),
            widget_type: widget_type.to_string(),
            title: "Sticky Note".to_string(),
            pos_x,
            pos_y,
        })
    }

    pub fn update_widget_position(&self, widget_id: &str, pos_x: i32, pos_y: i32) -> Result<()> {
        self.conn.execute(
            "UPDATE widgets SET pos_x = ?1, pos_y = ?2, updated_at = ?3 WHERE id = ?4",
            params![pos_x, pos_y, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64, widget_id],
        )?;
        Ok(())
    }

    pub fn delete_widget(&self, widget_id: &str) -> Result<()> {
        // First delete associated document (foreign key constraint)
        self.conn.execute("DELETE FROM documents WHERE widget_id = ?1", params![widget_id])?;
        self.conn.execute("DELETE FROM widgets WHERE id = ?1", params![widget_id])?;
        Ok(())
    }

    pub fn load_document(&self, widget_id: &str) -> Result<Option<(Vec<u8>, Vec<u8>)>> {
        let mut stmt = self.conn.prepare("SELECT yrs_snapshot, yrs_state_vector FROM documents WHERE widget_id = ?1")?;
        let mut rows = stmt.query(params![widget_id])?;
        if let Some(row) = rows.next()? {
            let snapshot: Vec<u8> = row.get(0)?;
            let state_vector: Vec<u8> = row.get(1)?;
            Ok(Some((snapshot, state_vector)))
        } else {
            Ok(None)
        }
    }

    pub fn save_document(&self, widget_id: &str, snapshot: &[u8], state_vector: &[u8]) -> Result<()> {
        self.conn.execute(
            "INSERT INTO documents (widget_id, yrs_snapshot, yrs_state_vector) VALUES (?1, ?2, ?3)
             ON CONFLICT(widget_id) DO UPDATE SET yrs_snapshot = excluded.yrs_snapshot, yrs_state_vector = excluded.yrs_state_vector",
            params![widget_id, snapshot, state_vector],
        )?;
        Ok(())
    }
}
