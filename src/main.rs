use clap::Parser;
use tokio::time::{self, Duration};
mod db;
mod cpu;
mod memory;
mod disk;
mod config;
mod macros;
// mod network;

use db::establish_connection;
use cpu::CpuMetrics;
use memory::MemoryMetrics;
use disk::DisksMetrics;
// use network::NetworkMetrics;
use config::{CliArgs, LuceConfig};

#[tokio::main]
async fn main() {
    let args: CliArgs = CliArgs::parse();
    let cfg: LuceConfig = match LuceConfig::read_config(&args.get_config_path()) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to read config: {}", e);
            return;
        }
    };

    let mut conn = establish_connection(&cfg.database_path).expect("Failed to connect to database");

    // Set up an interval timer to run the task every hour
    let mut interval= time::interval(Duration::from_secs(cfg.interval_sec));

    loop {
        interval.tick().await;

        // Collect and insert CPU metrics
        let cpu_metrics: CpuMetrics = CpuMetrics::collect();
        debug_println!("{:#?}", cpu_metrics);
        if let Err(e) = cpu_metrics.write_to_db(&conn) {
            eprintln!("Failed to write CPU metrics to database: {}", e);
        }
    
        // Collect and insert Memory metrics
        let memory_metrics: MemoryMetrics = MemoryMetrics::collect();
        debug_println!("{:#?}", memory_metrics);
        if let Err(e) = memory_metrics.write_to_db(&conn) {
            eprintln!("Failed to write Memory metrics to database: {}", e);
        }
    
        // Collect and insert Disk metrics
        let disk_metrics: DisksMetrics = DisksMetrics::collect(&cfg.get_mount_points_as_str_slice());
        debug_println!("{:#?}", disk_metrics);
        if let Err(e) = disk_metrics.write_to_db(&mut conn) {
            eprintln!("Failed to write Disk metrics to database: {}", e);
        }
    
        // // Collect and insert Network metrics
        // let network_metrics = NetworkMetrics::collect();
        // network_metrics.write_to_db(&conn);
    
        debug_println!("All Metrics collected and stored in the database.");   
    }
}
