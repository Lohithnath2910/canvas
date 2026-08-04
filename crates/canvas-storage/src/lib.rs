use rusqlite::{params, Connection, Result};

pub struct Storage {
    conn: Connection,
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
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );",
            [],
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

    pub fn ensure_widget(&self, workspace_id: &str, widget_type: &str) -> Result<String> {
        let mut stmt = self.conn.prepare("SELECT id FROM widgets WHERE workspace_id = ?1 AND widget_type = ?2 LIMIT 1")?;
        let mut rows = stmt.query(params![workspace_id, widget_type])?;
        
        if let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            Ok(id)
        } else {
            let id = uuid::Uuid::new_v4().to_string();
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
            self.conn.execute(
                "INSERT INTO widgets (id, workspace_id, widget_type, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
                params![id, workspace_id, widget_type, "Sticky Note", now as i64],
            )?;
            Ok(id)
        }
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
