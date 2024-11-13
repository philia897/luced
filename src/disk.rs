use std::collections::HashMap;

use rusqlite::{params, Connection, Result};
use sysinfo::Disks;

#[derive(Debug)]
pub struct DiskMetrics {
    pub used_space_gb: f32,
    pub available_space_gb: f32,
}

#[derive(Debug)]
pub struct DisksMetrics {
    pub metrics: HashMap<String, DiskMetrics>,
}



impl DisksMetrics {
    pub fn collect(device_names: &[&str]) -> Self {
        let disks: Disks = Disks::new_with_refreshed_list();
        let mut metrics: HashMap<String, DiskMetrics> = HashMap::new();

        for disk in &disks {
            let mount_point = disk.mount_point().to_string_lossy().to_string();

            // Only gather information if the device name is in the list provided
            if device_names.contains(&mount_point.as_str()) {
                let name = disk.name().to_string_lossy();
                let key = format!("{} ({})", name, mount_point);

                let total_space_gb = disk.total_space() as f32 / (1024.0 * 1024.0 * 1024.0);
                let available_space_gb = disk.available_space() as f32 / (1024.0 * 1024.0 * 1024.0);
                let used_space_gb = total_space_gb - available_space_gb;

                metrics.insert(key, DiskMetrics{used_space_gb, available_space_gb});
            }
        }

        DisksMetrics { metrics }
    }

    pub fn write_to_db(&self, conn: &mut Connection) -> Result<()> {
        // Create the disk_metrics table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS disk_metrics (
                id INTEGER PRIMARY KEY,
                key TEXT NOT NULL,
                used_space_gb REAL NOT NULL,
                available_space_gb REAL NOT NULL,
                timestamp TEXT DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Prepare a transaction to insert the data
        let tx = conn.transaction()?;

        for (key, metric) in &self.metrics {
            tx.execute(
                "INSERT INTO disk_metrics (key, used_space_gb, available_space_gb)
                 VALUES (?1, ?2, ?3)",
                params![key, metric.used_space_gb, metric.available_space_gb],
            )?;
        }

        tx.commit()?;

        Ok(())
    }
}
