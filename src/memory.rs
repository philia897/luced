use rusqlite::{params, Connection, Result};
use sysinfo::System;

#[derive(Debug)]
pub struct MemoryMetrics {
    pub used_gb: f32,
    pub used_percent: f32,
}

impl MemoryMetrics {
    pub fn collect() -> Self {
        let mut system = System::new_all();
        system.refresh_memory();
        let used: f32 = system.used_memory() as f32;
        let used_percent: f32 = used as f32 / system.total_memory() as f32 * 100.0;
        let used_gb: f32 = used / (1024.0 * 1024.0 * 1024.0);
        MemoryMetrics { used_gb, used_percent }
    }

    pub fn write_to_db(&self, conn: &Connection) -> Result<()>{
        conn.execute(
            "CREATE TABLE IF NOT EXISTS memory_metrics (
                id INTEGER PRIMARY KEY,
                used_gb REAL NOT NULL,
                used_percent REAL NOT NULL,
                timestamp TEXT DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "INSERT INTO memory_metrics (used_gb, used_percent) VALUES (?1, ?2)",
            params![self.used_percent, self.used_gb],
        )?;
        Ok(())
    }
}
