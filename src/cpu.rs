use rusqlite::{params, Connection, Result};
use sysinfo::System;

#[derive(Debug)]
pub struct CpuMetrics {
    pub used_percent: f32,
}

impl CpuMetrics {
    pub fn collect() -> Self {
        let mut system = System::new_all();
        system.refresh_cpu_usage();
        let used_percent = system.global_cpu_usage();
        CpuMetrics { used_percent }
    }

    pub fn write_to_db(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cpu_metrics (
                id INTEGER PRIMARY KEY,
                used_percent REAL NOT NULL,
                timestamp TEXT DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "INSERT INTO cpu_metrics (used_percent) VALUES (?1)",
            params![self.used_percent],
        )?;

        Ok(())
    }
}
