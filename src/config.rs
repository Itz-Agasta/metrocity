use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;

// Appearance config
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Appearance {
    pub theme: String,
    pub weather: String, // "rain", "snow", "clear"
    pub solid_background_color: String,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            weather: "rain".to_string(),
            solid_background_color: String::new(),
        }
    }
}

// Engine config
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct EngineConfig {
    pub fps: u32,
    pub scene: String,
    pub cycle_seconds: u32,
    pub shuffle: bool,
    pub oled_shift: bool,
}
impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            fps: 30,
            scene: String::from("cafe"),
            cycle_seconds: 0,
            shuffle: false,
            oled_shift: false,
        }
    }
}

// Simulation config
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct SimulationConfig {
    pub max_vehicles: usize,
    pub max_pedestrians: usize,
    pub vehicle_speed_multiplier: f32,
    pub weather_speed_multiplier: f32,
    pub weather_density_multiplier: f32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            max_vehicles: 50,
            max_pedestrians: 15,
            vehicle_speed_multiplier: 1.0,
            weather_speed_multiplier: 1.0,
            weather_density_multiplier: 1.0,
        }
    }
}

// Monolith config
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
#[derive(Default)]
pub struct MonolithConfig {
    pub custom_text: String,
    pub custom_color: String,
    pub override_distro: String,
}

// Neon text config
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct NeonTextConfig {
    pub phrases: Vec<String>,
}

impl Default for NeonTextConfig {
    fn default() -> Self {
        Self {
            phrases: vec![
                "METROCITY".to_string(),
                "NEO TOKYO".to_string(),
                "CYBERCORP".to_string(),
                "GRID//LINK".to_string(),
                "SYNTH-WAVE".to_string(),
                "DATASTREAM".to_string(),
                "GRID_RUNNER".to_string(),
                "NETRUNNER".to_string(),
            ],
        }
    }
}

// Main config
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Config {
    pub engine: EngineConfig,
    pub appearance: Appearance,
    pub simulation: SimulationConfig,
    pub monolith: MonolithConfig,
    pub neon: NeonTextConfig,
}

impl Config {
    pub fn load() -> Self {
        if let Some(path) = config_path() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(config) = toml::from_str::<Config>(&content) {
                    return config;
                }
            }
        }
        Self::default()
    }

    pub fn write_default() -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = config_path() {
            if path.exists() {
                return Ok(()); // never overwrite
            }
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let default = Config::default();
            fs::write(&path, toml::to_string_pretty(&default)?)?;
        }
        Ok(())
    }
}

fn config_path() -> Option<std::path::PathBuf> {
    ProjectDirs::from("", "", "metrocity").map(|dirs| dirs.config_dir().join("config.toml"))
}
