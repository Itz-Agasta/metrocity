pub mod buildings;
pub mod logos;
pub mod people;
pub mod utils;
pub mod vehicles;
pub mod weather;

use rand::prelude::*;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Widget;

use crate::scene::Scene;
use crate::theme::Theme;

use buildings::*;
use logos::*;
use people::*;
use utils::*;
use vehicles::*;
use weather::*;

/// Detect the current distro from /etc/os-release.
fn detect_distro() -> String {
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some(value) = line.strip_prefix("ID=") {
                return value.trim_matches('"').to_string();
            }
        }
    }
    "linux".to_string()
}

pub struct CityScene {
    vehicles: Vec<Vehicle>,
    raindrops: Vec<Raindrop>,
    splashes: Vec<Splash>,
    people: Vec<Person>,
    frame_count: u64,
    window_seed: u64,
    chase_cooldown: u32,
    pub distro: String,
    pub weather: Weather,
    theme: Theme,
    logo_asset: DistroLogo,
    pub simulation_config: crate::config::SimulationConfig,
    pub monolith_sign_text: String,
    buildings_cache: Vec<BuildingInfo>,
    cached_width: u16,
    cached_height: u16,
    width: u16,
    height: u16,
}

impl CityScene {
    pub fn new() -> Self {
        Self {
            vehicles: Vec::with_capacity(100),
            raindrops: Vec::with_capacity(250),
            splashes: Vec::with_capacity(50),
            people: Vec::with_capacity(30),
            frame_count: 0,
            window_seed: thread_rng().gen(),
            chase_cooldown: 0,
            distro: String::new(),
            weather: Weather::Rain,
            theme: Theme::default(),
            logo_asset: DistroLogo {
                grid: vec![vec![None; 32]; 20],
                is_compact: false,
                display_name: String::new(),
            },
            simulation_config: crate::config::SimulationConfig::default(),
            monolith_sign_text: String::from("METROCITY CORP"),
            buildings_cache: Vec::new(),
            cached_width: 0,
            cached_height: 0,
            width: 0,
            height: 0,
        }
    }

    fn rebuild_buildings_if_needed(&mut self, area: Rect) {
        if area.width != self.cached_width || area.height != self.cached_height {
            self.buildings_cache = compute_buildings(area.width, area.height);
            self.cached_width = area.width;
            self.cached_height = area.height;
        }
    }

    fn update_inner(&mut self, area: Rect) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        self.frame_count = self.frame_count.wrapping_add(1);

        if area.width < 32 || area.height < 12 {
            return;
        }

        self.rebuild_buildings_if_needed(area);

        let mut rng = thread_rng();

        vehicles::update_vehicles(
            &mut self.vehicles,
            &mut self.chase_cooldown,
            self.frame_count,
            area,
            &self.theme,
            &self.simulation_config,
            &mut rng,
        );
        let mut people = std::mem::take(&mut self.people);
        people::update_people(
            &mut people,
            self.frame_count,
            area,
            &self.theme,
            &self.simulation_config,
            &self.buildings_cache,
            &mut rng,
        );
        self.people = people;
        weather::update_weather(
            &self.weather,
            &mut self.raindrops,
            &mut self.splashes,
            self.frame_count,
            area,
            &self.simulation_config,
            &mut rng,
        );
    }

    // Render methods

    fn render_background(&self, area: Rect, buf: &mut Buffer) {
        let bg_color = self.theme.building_base_colors[0];
        // Terminal::draw resets the back buffer to blank cells before handing it
        // to the scene. Only the background color needs changing here. Avoiding
        // a redundant symbol assignment for every cell is significant for large
        // terminals and keeps symbol processing out of this full-screen pass.
        buf.set_style(area, Style::default().bg(bg_color));
    }

    fn render_stars(&self, area: Rect, buf: &mut Buffer) {
        use rand::rngs::StdRng;
        use rand::SeedableRng;
        let mut star_rng = StdRng::seed_from_u64(42);
        for i in 0..25 {
            let x = star_rng.gen_range(0..area.width);
            let y = star_rng.gen_range(0..area.height / 2);
            let mut p_rng = StdRng::seed_from_u64(i as u64);
            let star_type = p_rng.gen_range(0..4);
            let (symbol, dim_color) = match star_type {
                0 => ('.', Color::Rgb(60, 60, 80)),
                1 => ('·', Color::Rgb(50, 50, 70)),
                2 => ('*', Color::Rgb(70, 70, 90)),
                _ => ('+', Color::Rgb(40, 40, 60)),
            };
            let pulse = ((self.frame_count as f32 * 0.1 + i as f32).sin() + 1.0) / 2.0;
            let color = if pulse > 0.85 {
                match star_type {
                    0 => Color::Rgb(200, 200, 255),
                    1 => Color::Cyan,
                    2 => Color::Rgb(255, 150, 255),
                    _ => Color::White,
                }
            } else if pulse > 0.5 {
                dim_color
            } else {
                Color::Rgb(30, 30, 45)
            };
            safe_set_char(buf, area.x + x, area.y + y, symbol, color);
        }
    }

    fn render_skyline(&self, area: Rect, buf: &mut Buffer) {
        use rand::rngs::StdRng;
        use rand::SeedableRng;
        let logo_asset = &self.logo_asset;
        let b_base_color = self.theme.building_base_colors[1];
        let ground_y = area.height.saturating_sub(3);

        // Background buildings (dark silhouettes)
        let mut bg_rng = StdRng::seed_from_u64(12345);
        let bg_color = darken_color(self.theme.building_base_colors[0]);
        for x_bg in (0..area.width).step_by(15) {
            let bw = bg_rng.gen_range(6..15) as u16;
            let bh = bg_rng.gen_range(area.height / 5..area.height / 2);
            let start_x = area.x.saturating_add(x_bg);
            let start_y = ground_y.saturating_sub(bh);
            for y_rel in 0..bh {
                for x_rel in 0..bw {
                    let dx = start_x.saturating_add(x_rel);
                    let dy = start_y.saturating_add(y_rel);
                    if dx < area.x + area.width && dy < area.y + area.height {
                        safe_set_char_with_bg(buf, dx, dy, ' ', Color::Reset, bg_color);
                    }
                }
            }
        }

        // Foreground buildings
        for b in &self.buildings_cache {
            let i = b.index;
            let bw = b.width;
            let bh = b.height;
            let start_y = ground_y.saturating_sub(bh);
            let start_x = area.x.saturating_add(b.x_offset);

            for y_rel in 0..bh {
                for x_rel in 0..bw {
                    let dx = start_x.saturating_add(x_rel);
                    let dy = start_y.saturating_add(y_rel);
                    if dx < area.x + area.width && dy < area.y + area.height {
                        let mut symbol = " ";
                        let mut fg = b_base_color;
                        let mut bg = b_base_color;
                        let mut is_logo_pixel = false;

                        // Logo on main monolith (building index 1)
                        if i == 1 && y_rel < 20 && x_rel < 32 {
                            if let Some(pixel) = &logo_asset.grid[y_rel as usize][x_rel as usize] {
                                let logo_bg = if pixel.bg == Color::Reset {
                                    b_base_color
                                } else {
                                    pixel.bg
                                };
                                safe_set_char_with_bg(
                                    buf,
                                    dx,
                                    dy,
                                    pixel.ch,
                                    self.theme.logo_override.unwrap_or(pixel.color),
                                    logo_bg,
                                );
                                is_logo_pixel = true;
                            } else if !logo_asset.is_compact && x_rel > 4 && x_rel < 28 {
                                bg = b_base_color;
                                is_logo_pixel = true;
                            }
                            if is_logo_pixel
                                && logo_asset.grid[y_rel as usize][x_rel as usize].is_some()
                            {
                                continue;
                            }
                        }

                        if !is_logo_pixel {
                            // Building edges
                            if x_rel == 0 || x_rel == bw.saturating_sub(1) {
                                symbol = "┃";
                                fg = Color::Rgb(30, 30, 50);
                            }

                            // Windows
                            let has_sign = i % 2 == 1 && bh > 12;
                            let is_win_row =
                                y_rel > 2 && y_rel < bh.saturating_sub(4) && y_rel % 3 == 0;
                            let x_clearance = if has_sign {
                                bw.saturating_sub(2)
                            } else {
                                bw.saturating_sub(1)
                            };

                            // Check if near logo
                            let mut near_logo = false;
                            if i == 1 {
                                for dy_off in -1..=1 {
                                    for dx_off in -1..=1 {
                                        let check_y = (y_rel as i32 + dy_off) as usize;
                                        let check_x = (x_rel as i32 + dx_off) as usize;
                                        if check_y < 20
                                            && check_x < 32
                                            && logo_asset.grid[check_y][check_x].is_some()
                                        {
                                            near_logo = true;
                                            break;
                                        }
                                    }
                                    if near_logo {
                                        break;
                                    }
                                }
                            }

                            if !near_logo
                                && is_win_row
                                && x_rel > 0
                                && x_rel < x_clearance
                                && (dx.wrapping_add(dy)) % 4 == 0
                            {
                                let door_x = bw / 2;
                                if !(y_rel >= bh.saturating_sub(3)
                                    && x_rel >= door_x.saturating_sub(1)
                                    && x_rel <= door_x + 1)
                                {
                                    symbol = "▄";
                                    let seed = (dx as u64)
                                        .wrapping_mul(100)
                                        .wrapping_add(dy as u64)
                                        .wrapping_add(self.window_seed);
                                    let mut wr = StdRng::seed_from_u64(seed);

                                    // TODO: once set at scene creation a lit window stays lit forever
                                    // make them dynamically turn on/off
                                    fg = if wr.gen_bool(0.40) {
                                        self.theme.window_lit
                                    } else {
                                        self.theme.window_unlit
                                    };
                                    bg = self.theme.window_dark;
                                }
                            }

                            // Door
                            if y_rel >= bh.saturating_sub(3) {
                                let door_x = bw / 2;
                                if x_rel >= door_x.saturating_sub(1) && x_rel <= door_x + 1 {
                                    if y_rel == bh.saturating_sub(3) {
                                        symbol = "━";
                                        fg = if i % 2 == 0 {
                                            self.theme.neon_sub1
                                        } else {
                                            self.theme.neon_main
                                        };
                                    } else {
                                        symbol = "░";
                                        fg = self.theme.window_unlit;
                                    }
                                }
                                if x_rel == door_x + 2 && y_rel == bh.saturating_sub(2) {
                                    symbol = "·";
                                    fg = if self.frame_count % 20 < 10 {
                                        Color::Red
                                    } else {
                                        Color::Green
                                    };
                                }
                            }
                        }
                        safe_set_symbol_with_bg(buf, dx, dy, symbol, fg, bg);
                    }
                }
            }

            // Neon signs on buildings
            if i % 2 == 1 && bh > 12 {
                let sign_text: &str;
                let sign_color;
                if i == 1 {
                    sign_text = &self.monolith_sign_text;
                    sign_color = self.theme.neon_main;
                } else {
                    // Use frame_count to cycle through cyberpunk phrases
                    let phrases = [
                        "METROCITY",
                        "NEO TOKYO",
                        "CYBERCORP",
                        "GRID//LINK",
                        "SYNTH-WAVE",
                        "DATASTREAM",
                        "NETRUNNER",
                        "GRID_RUN",
                    ];
                    let idx =
                        ((i / 2).wrapping_add(self.frame_count as usize / 120)) % phrases.len();
                    sign_text = phrases[idx];
                    sign_color = match (i / 2) % 3 {
                        0 => self.theme.neon_sub1,
                        1 => self.theme.neon_sub2,
                        _ => self.theme.neon_sub3,
                    };
                }
                let sign_y = start_y.saturating_add(5);
                draw_neon_sign(
                    buf,
                    start_x + bw.saturating_sub(1),
                    sign_y,
                    sign_text,
                    sign_color,
                    self.frame_count,
                );
            }

            // Antennas
            if i != 1 && i != 3 {
                let ant_x = start_x.saturating_add(2);
                if ant_x < buf.area.width {
                    let ant_y = area.y.saturating_add(start_y.saturating_sub(1));
                    if ant_y < buf.area.height {
                        match i % 3 {
                            0 => {
                                safe_set_symbol(buf, ant_x, ant_y, "┷", Color::Rgb(60, 60, 80));
                                if ant_y > area.y {
                                    safe_set_symbol(
                                        buf,
                                        ant_x,
                                        ant_y - 1,
                                        "┃",
                                        Color::Rgb(50, 50, 70),
                                    );
                                    let beacon_color = if self.frame_count % 30 < 15 {
                                        Color::Red
                                    } else {
                                        Color::Rgb(60, 0, 0)
                                    };
                                    if ant_y > area.y + 1 {
                                        safe_set_symbol(buf, ant_x, ant_y - 2, "*", beacon_color);
                                    }
                                }
                            }
                            1 => {
                                safe_set_symbol(buf, ant_x, ant_y, "◆", Color::Rgb(100, 100, 120));
                            }
                            _ => {
                                safe_set_symbol(buf, ant_x, ant_y, "▝", Color::Rgb(40, 40, 50));
                            }
                        }
                    }
                }
            }
        }
    }

    fn render_street_lamps(&self, area: Rect, buf: &mut Buffer) {
        for x_lamp in (5..area.width).step_by(10) {
            let inside = self.buildings_cache.iter().any(|b| b.contains_x(x_lamp));
            if !inside {
                let lx = area.x + x_lamp;
                let ground_y = area.y + area.height - 3;
                let bulb_c = if (self.frame_count + lx as u64) % 40 < 2 {
                    self.theme.street_lamp_dim
                } else {
                    self.theme.street_lamp_lit
                };
                safe_set_symbol(buf, lx, ground_y, "┃", self.theme.ground);
                safe_set_symbol(buf, lx, ground_y.saturating_sub(1), "┃", self.theme.ground);
                safe_set_symbol(buf, lx, ground_y.saturating_sub(2), "┃", self.theme.ground);
                safe_set_string(
                    buf,
                    lx.saturating_sub(1),
                    ground_y.saturating_sub(3),
                    "(O)",
                    bulb_c,
                );
            }
        }
    }

    fn render_weather_bg(&self, area: Rect, buf: &mut Buffer) {
        if self.weather == Weather::Rain {
            for r in &self.raindrops {
                let rx = area.x + r.x as u16;
                let ry = area.y + r.y as u16;
                let sym = if r.z_index == 1 { "|" } else { ":" };
                let color = if r.z_index == 1 {
                    self.theme.rain
                } else {
                    self.theme.rain_bg
                };
                safe_set_symbol(buf, rx, ry, sym, color);
            }
        } else if self.weather == Weather::Snow {
            for r in &self.raindrops {
                let rx = area.x + r.x as u16;
                let ry = area.y + r.y as u16;
                let sym = match (self.frame_count + (r.x as u64)) % 30 {
                    0..=10 => "*",
                    11..=20 => "·",
                    _ => "❄",
                };
                let color = if r.z_index == 1 {
                    self.theme.snow
                } else {
                    darken_color(self.theme.snow)
                };
                safe_set_symbol(buf, rx, ry, sym, color);
            }
            let ground_y = area.y + area.height - 3;
            for rx in 0..area.width {
                let dx = area.x + rx;
                let sym = if (dx as u64 + self.frame_count / 100) % 7 == 0 {
                    "▆"
                } else {
                    "█"
                };
                safe_set_symbol(buf, dx, ground_y + 1, sym, self.theme.snow);
                safe_set_symbol(buf, dx, ground_y + 2, "█", self.theme.snow);
            }
        }
    }

    /// Time megaboard: shows current time and date on a rooftop billboard.
    fn render_time_megaboard(&self, area: Rect, buf: &mut Buffer) {
        let ground_y = area.height.saturating_sub(3);

        // Find building index 3 for the billboard placement
        if let Some(b3) = self.buildings_cache.iter().find(|b| b.index == 3) {
            let mb_x = area.x + 60;
            let mb_y = ground_y.saturating_sub(b3.height);

            let now = chrono_now();
            let time_str = format!("{:02}:{:02}:{:02}", now.hour, now.minute, now.second);
            let date_str = format!("{:02}/{:02}/{:04}", now.month, now.day, now.year);

            let width: u16 = 26;
            let height: u16 = 5;

            // Strut
            let strut_pulse = ((self.frame_count as f32 * 0.05).sin() + 1.0) * 0.5;
            let strut_color = if strut_pulse > 0.8 {
                Color::Rgb(170, 170, 170)
            } else {
                Color::Rgb(85, 85, 85)
            };
            safe_set_symbol(buf, mb_x + 4, mb_y.saturating_sub(1), "╨", strut_color);
            safe_set_symbol(buf, mb_x + 20, mb_y.saturating_sub(1), "╨", strut_color);

            let board_y = mb_y.saturating_sub(6);

            // Board border
            for dx in mb_x..mb_x + width {
                safe_set_symbol(buf, dx, board_y + height - 1, "─", Color::Rgb(85, 85, 85));
            }
            safe_set_symbol(buf, mb_x, board_y, "⌜", Color::Rgb(170, 170, 170));
            safe_set_symbol(
                buf,
                mb_x + width - 1,
                board_y,
                "⌝",
                Color::Rgb(170, 170, 170),
            );
            safe_set_symbol(
                buf,
                mb_x,
                board_y + height - 1,
                "⌞",
                Color::Rgb(170, 170, 170),
            );
            safe_set_symbol(
                buf,
                mb_x + width - 1,
                board_y + height - 1,
                "⌟",
                Color::Rgb(170, 170, 170),
            );

            // TIME label
            safe_set_string(
                buf,
                mb_x + 2,
                board_y + 1,
                "TIME",
                Color::Rgb(170, 170, 170),
            );

            // Time digits with neon glow
            let time_color = if (self.frame_count / 30) % 2 == 0 {
                self.theme.neon_main
            } else {
                self.theme.neon_sub1
            };
            safe_set_string(buf, mb_x + 7, board_y + 1, &time_str, time_color);

            // Colon blink effect
            if self.frame_count % 30 < 15 {
                safe_set_char(buf, mb_x + 9, board_y + 1, ':', time_color);
                safe_set_char(buf, mb_x + 12, board_y + 1, ':', time_color);
            }

            // DATE label
            safe_set_string(
                buf,
                mb_x + 2,
                board_y + 3,
                "DATE",
                Color::Rgb(170, 170, 170),
            );

            // Date
            safe_set_string(buf, mb_x + 7, board_y + 3, &date_str, Color::White);
        }
    }

    fn render_people(&self, area: Rect, buf: &mut Buffer) {
        let ground_y = area.height.saturating_sub(3);
        let b_base_color = self.theme.building_base_colors[1];

        for p in &self.people {
            if p.x < 0.0 {
                continue;
            }
            let px = area.x + p.x as u16;
            let py_l = area.y + ground_y;
            let py_h = py_l.saturating_sub(1);
            if px < area.x + area.width && py_l < area.y + area.height {
                let building_bg = self
                    .buildings_cache
                    .iter()
                    .find(|b| {
                        let bx = area.x + b.x_offset;
                        let top_y = ground_y.saturating_sub(b.height);
                        px >= bx && px < bx + b.width && py_l >= area.y + top_y
                    })
                    .map(|_| b_base_color);

                let gait = if p.is_entering && p.entry_pause_timer > 0 {
                    1
                } else {
                    ((self.frame_count + p.id_offset) / 4) % 3
                };
                let leg_char = match gait {
                    0 => 'Λ',
                    1 => '|',
                    _ => 'λ',
                };
                safe_set_char_with_bg(
                    buf,
                    px,
                    py_h,
                    'o',
                    p.color,
                    building_bg.unwrap_or(Color::Reset),
                );
                safe_set_char_with_bg(
                    buf,
                    px,
                    py_l,
                    leg_char,
                    p.color,
                    building_bg.unwrap_or(Color::Reset),
                );
            }
        }
    }

    fn render_vehicles(&self, area: Rect, buf: &mut Buffer) {
        for v in &self.vehicles {
            if v.x < -15.0 {
                continue;
            }
            let vx_f = area.x as f32 + v.x;
            let vy = area.y + v.y as u16;
            if vy >= area.y + area.height {
                continue;
            }

            let (body, tail_color) = match v.v_type {
                VehicleType::Spinner => (vec!['◢', '■', '◣'], Some(self.theme.police_red)),
                VehicleType::Shuttle => {
                    let mut b = vec!['▓'];
                    b.resize(1 + v.length.saturating_sub(2) as usize, '█');
                    b.push('▶');
                    (b, Some(self.theme.neon_main))
                }
                VehicleType::Police => (vec!['◤', '█', '◥'], None),
            };

            for (off, ch) in body.iter().enumerate() {
                let dx = (vx_f + off as f32) as u16;
                if dx >= area.x + area.width || vy >= area.y + area.height {
                    continue;
                }
                let final_fg = if v.v_type == VehicleType::Police {
                    match off {
                        0 => Color::White,
                        1 => Color::Rgb(60, 60, 70),
                        _ => Color::White,
                    }
                } else {
                    v.color
                };
                let sym = safe_get_symbol(buf, dx, vy);
                let fg_peek = safe_get_fg(buf, dx, vy);
                let bg_peek = safe_get_bg(buf, dx, vy);
                let effective_bg = if sym == "█" || sym == "▓" || sym == "▆" || sym == "▄" {
                    fg_peek
                } else {
                    bg_peek
                };
                safe_set_char_with_bg(buf, dx, vy, *ch, final_fg, effective_bg);
            }

            // Police lights
            if v.v_type == VehicleType::Police && vy > area.y {
                let sy = vy.saturating_sub(1);
                let flash = (self.frame_count / 2) % 2 == 0;
                for (sx_f, base_color, is_on) in [
                    (vx_f, self.theme.police_blue, flash),
                    (vx_f + 2.0, self.theme.police_red, !flash),
                ] {
                    let sx = sx_f as u16;
                    if sx < area.x + area.width {
                        let sym = safe_get_symbol(buf, sx, sy);
                        let l_bg = if sym == "█" || sym == "▓" || sym == "▆" || sym == "▄" {
                            safe_get_fg(buf, sx, sy)
                        } else {
                            safe_get_bg(buf, sx, sy)
                        };
                        safe_set_char_with_bg(
                            buf,
                            sx,
                            sy,
                            '═',
                            if is_on {
                                base_color
                            } else {
                                Color::Rgb(40, 40, 60)
                            },
                            l_bg,
                        );
                    }
                }
            }

            // Tail
            if let Some(t_color) = tail_color {
                let tx_f = v.x - 1.0;
                if tx_f >= 0.0 {
                    let tx = (area.x as f32 + tx_f) as u16;
                    if tx < area.x + area.width {
                        let sym = safe_get_symbol(buf, tx, vy);
                        let t_bg = if sym == "█" || sym == "▓" || sym == "▆" || sym == "▄" {
                            safe_get_fg(buf, tx, vy)
                        } else {
                            safe_get_bg(buf, tx, vy)
                        };
                        if v.v_type == VehicleType::Shuttle {
                            safe_set_char_with_bg(buf, tx, vy, ':', t_color, t_bg);
                            if tx > area.x {
                                let s2 = safe_get_symbol(buf, tx.saturating_sub(1), vy);
                                let t2_bg = if s2 == "█" || s2 == "▓" || s2 == "▆" || s2 == "▄"
                                {
                                    safe_get_fg(buf, tx.saturating_sub(1), vy)
                                } else {
                                    safe_get_bg(buf, tx.saturating_sub(1), vy)
                                };
                                safe_set_char_with_bg(
                                    buf,
                                    tx.saturating_sub(1),
                                    vy,
                                    '·',
                                    t_color,
                                    t2_bg,
                                );
                            }
                        } else {
                            safe_set_char_with_bg(buf, tx, vy, '·', t_color, t_bg);
                            if v.v_type == VehicleType::Spinner {
                                let t2x_f = v.x - 2.0;
                                if t2x_f >= 0.0 {
                                    let t2x = (area.x as f32 + t2x_f) as u16;
                                    if t2x < area.x + area.width {
                                        let s2 = safe_get_symbol(buf, t2x, vy);
                                        let t2_bg =
                                            if s2 == "█" || s2 == "▓" || s2 == "▆" || s2 == "▄"
                                            {
                                                safe_get_fg(buf, t2x, vy)
                                            } else {
                                                safe_get_bg(buf, t2x, vy)
                                            };
                                        safe_set_char_with_bg(
                                            buf,
                                            t2x,
                                            vy,
                                            '·',
                                            Color::Rgb(85, 255, 255),
                                            t2_bg,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn render_weather_fg(&self, area: Rect, buf: &mut Buffer) {
        if self.weather == Weather::Rain {
            let ground_y = area.y + area.height.saturating_sub(3);
            for ry in (ground_y + 1)..(area.y + area.height) {
                let dist = ry.saturating_sub(ground_y);
                let sy = ground_y.saturating_sub(dist);
                let ripple = ((self.frame_count as f32 * 0.2 + ry as f32 * 0.5).sin() * 1.2) as i16;
                for rx in 0..area.width {
                    let target_x = area.x + rx;
                    let source_x = (area.x as i16 + rx as i16 + ripple)
                        .max(area.x as i16)
                        .min((area.x + area.width - 1) as i16)
                        as u16;
                    let s_fg = safe_get_fg(buf, source_x, sy);
                    let s_bg = safe_get_bg(buf, source_x, sy);
                    let s_sym = safe_get_symbol(buf, source_x, sy);
                    if s_sym != " " || s_fg != Color::Reset {
                        let dim_fg = darken_color(s_fg);
                        let dim_bg = darken_color(s_bg);
                        let sym = if dist == 1 {
                            "█"
                        } else if dist == 2 {
                            "▓"
                        } else {
                            "▒"
                        };
                        safe_set_symbol_with_bg(buf, target_x, ry, sym, dim_fg, dim_bg);
                    }
                }
            }
        }
    }
}

impl Scene for CityScene {
    fn name(&self) -> &str {
        "city"
    }

    fn init(&mut self, width: u16, height: u16, theme: &Theme) {
        self.width = width;
        self.height = height;
        self.theme = *theme;

        // Detect distro once
        if self.distro.is_empty() {
            self.distro = detect_distro();
            self.logo_asset = get_logo(&self.distro);
            self.monolith_sign_text = format!("{} CORP", self.logo_asset.display_name);
        }

        // Rebuild buildings for new dimensions
        if width > 0 && height > 0 {
            self.buildings_cache = compute_buildings(width, height);
            self.cached_width = width;
            self.cached_height = height;
        }
    }

    fn update(&mut self, dt: f64) {
        let area = Rect::new(0, 0, self.width, self.height);
        // The original simulation was tuned for 30 updates per second. Keep
        // that motion speed when the renderer runs at a lower frame rate by
        // advancing multiple cheap simulation steps per rendered frame.
        let steps = (dt * 30.0).round().clamp(1.0, 3.0) as usize;
        for _ in 0..steps {
            self.update_inner(area);
        }
    }

    fn draw(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        self.render_background(area, buf);

        if area.width < 12 || area.height < 3 {
            return;
        }

        if area.width < 45 || area.height < 15 {
            // Mini display for tiny terminals
            let cx = area.x + area.width / 2;
            let cy = area.y + area.height / 2;
            let color = if self.frame_count % 20 < 10 {
                self.theme.neon_main
            } else {
                Color::Rgb(60, 60, 60)
            };
            safe_set_symbol(buf, cx, cy, "◆", color);
            return;
        }

        self.render_stars(area, buf);
        self.render_skyline(area, buf);
        self.render_street_lamps(area, buf);
        self.render_weather_bg(area, buf);
        self.render_time_megaboard(area, buf);
        self.render_people(area, buf);
        self.render_vehicles(area, buf);
        self.render_weather_fg(area, buf);
    }
}

impl Widget for &CityScene {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        self.render_background(area, buf);

        if area.width < 12 || area.height < 3 {
            return;
        }

        if area.width < 45 || area.height < 15 {
            let cx = area.x + area.width / 2;
            let cy = area.y + area.height / 2;
            let color = if self.frame_count % 20 < 10 {
                self.theme.neon_main
            } else {
                Color::Rgb(60, 60, 60)
            };
            safe_set_symbol(buf, cx, cy, "◆", color);
            return;
        }

        self.render_stars(area, buf);
        self.render_skyline(area, buf);
        self.render_street_lamps(area, buf);
        self.render_weather_bg(area, buf);
        self.render_time_megaboard(area, buf);
        self.render_people(area, buf);
        self.render_vehicles(area, buf);
        self.render_weather_fg(area, buf);
    }
}

// Minimal time helper (no chrono dependency)

struct SimpleTime {
    hour: u32,
    minute: u32,
    second: u32,
    day: u32,
    month: u32,
    year: i64,
}

fn chrono_now() -> SimpleTime {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut days = secs / 86400;
    let time_of_day = secs % 86400;
    let hour = time_of_day / 3600;
    let minute = (time_of_day % 3600) / 60;
    let second = time_of_day % 60;

    // Civil calendar from days since epoch
    let mut y = 1970i64;
    loop {
        let days_in_year = if is_leap(y) { 366 } else { 365 };
        if days < days_in_year as u64 {
            break;
        }
        days -= days_in_year as u64;
        y += 1;
    }

    let month_days: [u32; 12] = [
        31,
        if is_leap(y) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut m = 1u32;
    for &md in &month_days {
        if days < md as u64 {
            break;
        }
        days -= md as u64;
        m += 1;
    }

    SimpleTime {
        hour: hour as u32,
        minute: minute as u32,
        second: second as u32,
        day: days as u32 + 1,
        month: m,
        year: y,
    }
}

fn is_leap(y: i64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}
