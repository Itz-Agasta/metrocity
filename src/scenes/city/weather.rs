#[derive(Debug, Clone, PartialEq)]
pub enum Weather {
    Clear,
    Rain,
    Snow,
}

#[derive(Debug, Clone)]
pub struct Raindrop {
    pub x: f32,
    pub y: f32,
    pub speed_y: f32,
    pub speed_x: f32,
    pub z_index: u8,
}

#[derive(Debug, Clone)]
pub struct Splash {
    #[allow(dead_code)] // TODO: use for splash position rendering
    pub x: u16,
    #[allow(dead_code)] // TODO: use for splash position rendering
    pub y: u16,
    pub frame: u8,
}

pub fn update_weather(
    weather: &Weather,
    raindrops: &mut Vec<Raindrop>,
    splashes: &mut Vec<Splash>,
    frame_count: u64,
    area: ratatui::layout::Rect,
    config: &crate::config::SimulationConfig,
    rng: &mut impl rand::Rng,
) {
    if *weather == Weather::Rain {
        let max_r = (100.0 * config.weather_density_multiplier) as usize;
        if raindrops.len() < max_r {
            let z = if rng.gen_bool(0.3) { 1 } else { 0 };
            raindrops.push(Raindrop {
                x: rng.gen_range(0..area.width as i32 + 40) as f32,
                y: -2.0,
                speed_y: if z == 1 {
                    rng.gen_range(1.2..1.8)
                } else {
                    rng.gen_range(0.6..1.0)
                },
                speed_x: -0.4,
                z_index: z,
            });
        }
        raindrops.retain_mut(|r| {
            r.y += r.speed_y * config.weather_speed_multiplier;
            r.x += r.speed_x * config.weather_speed_multiplier;
            let ground_y = area.height as f32 - 3.0;
            if r.y >= ground_y && rng.gen_bool(0.08) {
                splashes.push(Splash {
                    x: r.x as u16,
                    y: ground_y as u16,
                    frame: 0,
                });
                return false;
            }
            r.y < area.height as f32 && r.x > 0.0
        });
        splashes.retain_mut(|s| {
            s.frame += 1;
            s.frame < 4
        });
    } else if *weather == Weather::Snow {
        let max_s = (150.0 * config.weather_density_multiplier) as usize;
        if raindrops.len() < max_s {
            let z = if rng.gen_bool(0.3) { 1 } else { 0 };
            raindrops.push(Raindrop {
                x: rng.gen_range(0..area.width as i32 + 40) as f32,
                y: -2.0,
                speed_y: if z == 1 {
                    rng.gen_range(0.3..0.6)
                } else {
                    rng.gen_range(0.15..0.35)
                },
                speed_x: rng.gen_range(-0.3..0.1),
                z_index: z,
            });
        }
        raindrops.retain_mut(|r| {
            r.y += r.speed_y * config.weather_speed_multiplier;
            r.x += (r.speed_x + ((frame_count as f32 * 0.05).sin() * 0.1))
                * config.weather_speed_multiplier;
            r.y < area.height as f32 && r.x > 0.0 && r.x < area.width as f32 + 20.0
        });
        splashes.clear();
    } else {
        raindrops.clear();
        splashes.clear();
    }
}
