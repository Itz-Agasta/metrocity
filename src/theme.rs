// TODO: custom theme TOML parsing + apply_to + from_str not wired up yet (v2 feature)
#![allow(dead_code)]

use ratatui::style::Color;
use serde::Deserialize;

#[derive(Deserialize, Clone, Default, Debug)]
#[serde(default)]
pub struct ThemeMetadata {
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
}

#[derive(Deserialize, Clone, Default)]
#[serde(default)]
pub struct BuildingConfig {
    pub base_colors: Option<Vec<String>>,
    pub window_lit: Option<String>,
    pub window_unlit: Option<String>,
    pub window_dark: Option<String>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(default)]
pub struct NeonConfig {
    #[serde(alias = "main", alias = "neon_main")]
    pub primary: Option<String>,
    #[serde(alias = "sub1", alias = "neon_sub1")]
    pub secondary: Option<String>,
    #[serde(alias = "sub2", alias = "neon_sub2")]
    pub accent: Option<String>,
    #[serde(alias = "sub3", alias = "neon_sub3")]
    pub soft: Option<String>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(default)]
pub struct VehicleConfig {
    pub colors: Option<Vec<String>>,
    pub shuttle_colors: Option<Vec<String>>,
    pub police_red: Option<String>,
    pub police_blue: Option<String>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(default)]
pub struct EnvConfig {
    pub street_lamp_lit: Option<String>,
    pub street_lamp_dim: Option<String>,
    pub rain: Option<String>,
    pub rain_bg: Option<String>,
    pub snow: Option<String>,
    pub pedestrian: Option<String>,
    pub ground: Option<String>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(default)]
pub struct OverridesConfig {
    pub logo: Option<String>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(default)]
pub struct ThemeConfig {
    pub metadata: Option<ThemeMetadata>,
    pub building: Option<BuildingConfig>,
    #[serde(alias = "neon")]
    pub neon_metrics: Option<NeonConfig>,
    pub vehicles: Option<VehicleConfig>,
    pub env: Option<EnvConfig>,
    pub overrides: Option<OverridesConfig>,
}

impl ThemeConfig {
    pub fn apply_to(self, mut base: Theme) -> Theme {
        let p = |s: &str| parse_color(s);

        if let Some(c) = self.building {
            if let Some(colors) = c.base_colors {
                for (i, v) in colors.iter().enumerate().take(4) {
                    if let Some(clr) = p(v) {
                        base.building_base_colors[i] = clr;
                    }
                }
            }
            if let Some(v) = c.window_lit {
                if let Some(clr) = p(&v) {
                    base.window_lit = clr;
                }
            }
            if let Some(v) = c.window_unlit {
                if let Some(clr) = p(&v) {
                    base.window_unlit = clr;
                }
            }
            if let Some(v) = c.window_dark {
                if let Some(clr) = p(&v) {
                    base.window_dark = clr;
                }
            }
        }

        if let Some(c) = self.neon_metrics {
            if let Some(v) = c.primary {
                if let Some(clr) = p(&v) {
                    base.neon_main = clr;
                }
            }
            if let Some(v) = c.secondary {
                if let Some(clr) = p(&v) {
                    base.neon_sub1 = clr;
                }
            }
            if let Some(v) = c.accent {
                if let Some(clr) = p(&v) {
                    base.neon_sub2 = clr;
                }
            }
            if let Some(v) = c.soft {
                if let Some(clr) = p(&v) {
                    base.neon_sub3 = clr;
                }
            }
        }

        if let Some(c) = self.vehicles {
            if let Some(colors) = c.colors {
                for (i, v) in colors.iter().enumerate().take(5) {
                    if let Some(clr) = p(v) {
                        base.vehicle_colors[i] = clr;
                    }
                }
            }
            if let Some(colors) = c.shuttle_colors {
                for (i, v) in colors.iter().enumerate().take(3) {
                    if let Some(clr) = p(v) {
                        base.shuttle_colors[i] = clr;
                    }
                }
            }
            if let Some(v) = c.police_red {
                if let Some(clr) = p(&v) {
                    base.police_red = clr;
                }
            }
            if let Some(v) = c.police_blue {
                if let Some(clr) = p(&v) {
                    base.police_blue = clr;
                }
            }
        }

        if let Some(c) = self.env {
            if let Some(v) = c.street_lamp_lit {
                if let Some(clr) = p(&v) {
                    base.street_lamp_lit = clr;
                }
            }
            if let Some(v) = c.street_lamp_dim {
                if let Some(clr) = p(&v) {
                    base.street_lamp_dim = clr;
                }
            }
            if let Some(v) = c.rain {
                if let Some(clr) = p(&v) {
                    base.rain = clr;
                }
            }
            if let Some(v) = c.rain_bg {
                if let Some(clr) = p(&v) {
                    base.rain_bg = clr;
                }
            }
            if let Some(v) = c.snow {
                if let Some(clr) = p(&v) {
                    base.snow = clr;
                }
            }
            if let Some(v) = c.pedestrian {
                if let Some(clr) = p(&v) {
                    base.pedestrian = clr;
                }
            }
            if let Some(v) = c.ground {
                if let Some(clr) = p(&v) {
                    base.ground = clr;
                }
            }
        }

        if let Some(c) = self.overrides {
            if let Some(v) = c.logo {
                if let Some(clr) = p(&v) {
                    base.logo_override = Some(clr);
                }
            }
        }

        base
    }
}

pub fn parse_color(s: &str) -> Option<Color> {
    let mut clean_s = s.trim().to_lowercase();
    if clean_s.starts_with('#') {
        clean_s = clean_s[1..].to_string();
    }
    if clean_s.len() == 6 && clean_s.chars().all(|c| c.is_ascii_hexdigit()) {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&clean_s[0..2], 16),
            u8::from_str_radix(&clean_s[2..4], 16),
            u8::from_str_radix(&clean_s[4..6], 16),
        ) {
            return Some(Color::Rgb(r, g, b));
        }
    }
    match clean_s.as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" | "purple" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "gray" | "grey" => Some(Color::Gray),
        "dark_gray" => Some(Color::DarkGray),
        "light_red" => Some(Color::LightRed),
        "light_green" => Some(Color::LightGreen),
        "light_yellow" => Some(Color::LightYellow),
        "light_blue" => Some(Color::LightBlue),
        "light_magenta" | "pink" => Some(Color::LightMagenta),
        "light_cyan" => Some(Color::LightCyan),
        "white" => Some(Color::White),
        _ => {
            let parts: Vec<&str> = clean_s.split(',').collect();
            if parts.len() == 3 {
                Some(Color::Rgb(
                    parts[0].trim().parse().ok()?,
                    parts[1].trim().parse().ok()?,
                    parts[2].trim().parse().ok()?,
                ))
            } else {
                None
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Theme {
    pub building_base_colors: [Color; 4],
    pub window_lit: Color,
    pub window_unlit: Color,
    pub window_dark: Color,
    pub neon_main: Color,
    pub neon_sub1: Color,
    pub neon_sub2: Color,
    pub neon_sub3: Color,
    pub vehicle_colors: [Color; 5],
    pub shuttle_colors: [Color; 3], // Small, Medium, Large
    pub police_red: Color,
    pub police_blue: Color,
    pub street_lamp_lit: Color,
    pub street_lamp_dim: Color,
    pub rain: Color,
    pub rain_bg: Color,
    pub snow: Color,
    pub pedestrian: Color,
    pub ground: Color,
    pub logo_override: Option<Color>,
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_theme()
    }
}

impl Theme {
    pub fn default_theme() -> Self {
        Self {
            building_base_colors: [
                Color::Rgb(20, 20, 30),
                Color::Rgb(30, 20, 30),
                Color::Rgb(20, 30, 40),
                Color::Rgb(30, 30, 30),
            ],
            window_lit: Color::Rgb(255, 255, 85),
            window_unlit: Color::Rgb(40, 40, 40),
            window_dark: Color::Rgb(15, 15, 15),
            neon_main: Color::Rgb(85, 255, 255),
            neon_sub1: Color::Rgb(255, 85, 255),
            neon_sub2: Color::Rgb(85, 255, 85),
            neon_sub3: Color::Rgb(255, 255, 85),
            vehicle_colors: [
                Color::Red,
                Color::White,
                Color::Yellow,
                Color::Cyan,
                Color::Rgb(50, 150, 255),
            ],
            shuttle_colors: [
                Color::Rgb(255, 255, 85),  // Small: Yellow
                Color::Rgb(170, 170, 170), // Med: Light Gray
                Color::Rgb(85, 85, 85),    // Large: Dark Gray
            ],
            police_red: Color::Red,
            police_blue: Color::Blue,
            street_lamp_lit: Color::Rgb(255, 255, 150),
            street_lamp_dim: Color::Rgb(100, 100, 50),
            rain: Color::Rgb(0, 180, 180),
            rain_bg: Color::Rgb(0, 60, 60),
            snow: Color::White,
            pedestrian: Color::Rgb(170, 170, 170),
            ground: Color::Rgb(40, 40, 50),
            logo_override: None,
        }
    }

    pub fn cyberpunk() -> Self {
        Self {
            window_lit: Color::Rgb(0, 255, 255),
            neon_main: Color::Rgb(255, 0, 255),
            neon_sub1: Color::Rgb(0, 255, 255),
            vehicle_colors: [
                Color::Rgb(255, 0, 255),
                Color::Rgb(0, 255, 255),
                Color::Yellow,
                Color::Rgb(255, 0, 128),
                Color::White,
            ],
            ..Self::default_theme()
        }
    }

    pub fn matrix() -> Self {
        Self {
            building_base_colors: [
                Color::Rgb(0, 20, 0),
                Color::Rgb(0, 30, 0),
                Color::Rgb(0, 40, 0),
                Color::Rgb(0, 25, 0),
            ],
            window_lit: Color::Rgb(0, 255, 0),
            window_unlit: Color::Rgb(0, 50, 0),
            window_dark: Color::Rgb(0, 15, 0),
            neon_main: Color::Rgb(0, 255, 0),
            neon_sub1: Color::Rgb(0, 255, 0),
            neon_sub2: Color::Rgb(0, 150, 0),
            neon_sub3: Color::Rgb(0, 255, 0),
            vehicle_colors: [
                Color::Rgb(0, 255, 0),
                Color::Rgb(0, 200, 0),
                Color::Rgb(0, 150, 0),
                Color::Rgb(0, 255, 0),
                Color::Rgb(0, 180, 0),
            ],
            shuttle_colors: [
                Color::Rgb(0, 255, 0),
                Color::Rgb(0, 200, 0),
                Color::Rgb(0, 150, 0),
            ],
            police_red: Color::Rgb(0, 255, 0),
            police_blue: Color::Rgb(0, 200, 0),
            street_lamp_lit: Color::Rgb(0, 255, 0),
            street_lamp_dim: Color::Rgb(0, 100, 0),
            rain: Color::Rgb(0, 255, 0),
            rain_bg: Color::Rgb(0, 100, 0),
            snow: Color::Rgb(0, 255, 0),
            pedestrian: Color::Rgb(0, 180, 0),
            ground: Color::Rgb(0, 50, 0),
            logo_override: Some(Color::Rgb(0, 255, 0)),
        }
    }

    pub fn synthwave() -> Self {
        Self {
            building_base_colors: [
                Color::Rgb(30, 10, 40),
                Color::Rgb(40, 15, 50),
                Color::Rgb(50, 20, 60),
                Color::Rgb(25, 10, 35),
            ],
            window_lit: Color::Rgb(255, 100, 150),
            window_unlit: Color::Rgb(60, 20, 70),
            window_dark: Color::Rgb(30, 10, 40),
            neon_main: Color::Rgb(255, 150, 0),
            neon_sub1: Color::Rgb(255, 50, 150),
            neon_sub2: Color::Rgb(0, 255, 255),
            neon_sub3: Color::Rgb(255, 200, 0),
            vehicle_colors: [
                Color::Rgb(255, 50, 150),
                Color::Rgb(0, 255, 255),
                Color::Rgb(255, 150, 0),
                Color::White,
                Color::Rgb(150, 50, 255),
            ],
            street_lamp_lit: Color::Rgb(255, 100, 255),
            street_lamp_dim: Color::Rgb(150, 50, 150),
            ground: Color::Rgb(50, 20, 60),
            ..Self::default_theme()
        }
    }

    pub fn dracula() -> Self {
        Self {
            building_base_colors: [
                Color::Rgb(40, 42, 54),
                Color::Rgb(68, 71, 90),
                Color::Rgb(50, 52, 64),
                Color::Rgb(45, 47, 59),
            ],
            window_lit: Color::Rgb(241, 250, 140),
            window_unlit: Color::Rgb(98, 114, 164),
            window_dark: Color::Rgb(40, 42, 54),
            neon_main: Color::Rgb(189, 147, 249),
            neon_sub1: Color::Rgb(255, 121, 198),
            neon_sub2: Color::Rgb(139, 233, 253),
            neon_sub3: Color::Rgb(80, 250, 123),
            vehicle_colors: [
                Color::Rgb(255, 85, 85),
                Color::Rgb(139, 233, 253),
                Color::Rgb(189, 147, 249),
                Color::Rgb(255, 184, 108),
                Color::Rgb(248, 248, 242),
            ],
            shuttle_colors: [
                Color::Rgb(241, 250, 140), // Dracula Yellow
                Color::Rgb(98, 114, 164),  // Dracula Comment
                Color::Rgb(68, 71, 90),    // Dracula Current Line
            ],
            police_red: Color::Rgb(255, 85, 85),
            police_blue: Color::Rgb(139, 233, 253),
            street_lamp_lit: Color::Rgb(241, 250, 140),
            street_lamp_dim: Color::Rgb(255, 184, 108),
            rain: Color::Rgb(139, 233, 253),
            rain_bg: Color::Rgb(98, 114, 164),
            snow: Color::Rgb(248, 248, 242),
            pedestrian: Color::Rgb(170, 170, 170),
            ground: Color::Rgb(68, 71, 90),
            logo_override: None,
        }
    }

    pub fn sin_city() -> Self {
        Self {
            // High contrast grays and whites, piercing reds
            building_base_colors: [
                Color::Rgb(180, 180, 180),
                Color::Rgb(220, 220, 220),
                Color::Rgb(255, 255, 255),
                Color::Rgb(200, 200, 200),
            ],
            window_lit: Color::Rgb(255, 0, 0),
            window_unlit: Color::Rgb(100, 100, 100),
            window_dark: Color::Rgb(50, 50, 50),
            neon_main: Color::Rgb(255, 0, 0),
            neon_sub1: Color::Rgb(250, 0, 0),
            neon_sub2: Color::Rgb(200, 0, 0),
            neon_sub3: Color::Rgb(150, 0, 0),
            vehicle_colors: [
                Color::Rgb(255, 0, 0),
                Color::Rgb(200, 0, 0),
                Color::Rgb(255, 50, 50),
                Color::Rgb(150, 0, 0),
                Color::Rgb(255, 0, 0),
            ],
            shuttle_colors: [
                Color::Rgb(255, 0, 0),
                Color::Rgb(200, 0, 0),
                Color::Rgb(150, 0, 0),
            ],
            police_red: Color::Rgb(255, 0, 0),
            police_blue: Color::Rgb(255, 255, 255), // Flashing red and white
            street_lamp_lit: Color::Rgb(255, 0, 0),
            street_lamp_dim: Color::Rgb(150, 0, 0),
            rain: Color::Rgb(255, 0, 0),
            rain_bg: Color::Rgb(150, 0, 0),
            snow: Color::White,
            pedestrian: Color::Rgb(15, 15, 15), // Pitch black silhouettes
            ground: Color::Rgb(150, 150, 150),
            logo_override: Some(Color::Rgb(255, 0, 0)),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "cyberpunk" => Self::cyberpunk(),
            "matrix" => Self::matrix(),
            "synthwave" => Self::synthwave(),
            "dracula" => Self::dracula(),
            "sin_city" => Self::sin_city(),
            "default" | "" => Self::default_theme(),
            custom => {
                if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "metrocity") {
                    let mut path = proj_dirs.config_dir().to_path_buf();
                    path.push("themes");
                    path.push(format!("{}.toml", custom));
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        match toml::from_str::<ThemeConfig>(&content) {
                            Ok(cfg) => return cfg.apply_to(Self::default_theme()),
                            Err(e) => {
                                let mut err_file = proj_dirs.config_dir().to_path_buf();
                                err_file.push("error.log");
                                let err_msg =
                                    format!("Error parsing theme {}.toml: {}\n", custom, e);
                                let _ = std::fs::write(&err_file, &err_msg);
                                eprintln!("{}", err_msg);
                                std::thread::sleep(std::time::Duration::from_secs(2));
                            }
                        }
                    }
                }
                Self::default_theme()
            }
        }
    }
}
