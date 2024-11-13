use serde::Deserialize;
use std::path::PathBuf;
use dirs::config_dir;
use std::fs;
use clap::Parser;

/// Luced: A lightweight system metrics collector daemon
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to the config file (default: ~/.config/luced/config.toml)
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

impl CliArgs {
    // Method to get the database path, defaulting to `config_dir/database.sqlite`
    pub fn get_config_path(&self) -> PathBuf {
        if let Some(path) = &self.config {
            path.clone()
        } else {
            config_dir().unwrap_or_else(|| PathBuf::new()).join("luced").join("config.toml")
        }
    }
}


#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct LuceConfig {
    pub mount_points: Vec<String>,
    pub interval_sec: u64,
    pub database_path: PathBuf,
}

impl Default for LuceConfig {
    fn default() -> Self {
        let mount_points: Vec<String> = vec!["/".to_string()];
        let interval_sec: u64 = 3600;
        let database_path: PathBuf = config_dir().unwrap_or_else(|| PathBuf::new()).join("luced").join("database.sqlite");

        Self {
            mount_points : mount_points,
            interval_sec : interval_sec,
            database_path : database_path,
        }
    }
}

impl LuceConfig {
    /// Reads the configuration from a specified TOML file path.
    /// If the file is missing or some fields are absent, it will fall back to default values.
    pub fn read_config(config_file_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        if config_file_path.exists() {
            let config_content = fs::read_to_string(config_file_path)?;
            // Parse the TOML content into `LuceConfig`, using defaults for any missing fields
            let config: LuceConfig = toml::de::from_str(&config_content)?;
            Ok(config)
        } else {
            // If the file does not exist, return the default configuration
            println!("The config file {} does not exist, use default values instead.", config_file_path.to_string_lossy());
            Ok(LuceConfig::default())
        }
    }

    // This method prepares `mount_points` to be passed to functions expecting `&[&str]`.
    pub fn get_mount_points_as_str_slice(&self) -> Vec<&str> {
        self.mount_points.iter().map(|s| s.as_str()).collect()
    }
}