use ratatui::style::Color;

#[derive(Debug, Clone, PartialEq)]
pub enum VehicleType {
    Spinner,
    Shuttle,
    Police,
}

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub color: Color,
    pub v_type: VehicleType,
    pub length: u16,
}

pub fn get_sky_lane(rng: &mut impl rand::Rng) -> f32 {
    let lanes = vec![5.0, 8.0, 11.0, 14.0];
    lanes[rng.gen_range(0..lanes.len())]
}

pub fn update_vehicles(
    vehicles: &mut Vec<Vehicle>,
    chase_cooldown: &mut u32,
    frame_count: u64,
    cpu: f32,
    disk_usage: f32,
    area: ratatui::layout::Rect,
    theme: &crate::theme::Theme,
    config: &crate::config::SimulationConfig,
    rng: &mut impl rand::Rng,
) {
    let pulse = (frame_count as f32 * 0.003).sin() * 0.5 + 0.5; // 0.0 to 1.0
    let c_max = config.max_vehicles as f32;
    // CPU usage maps from a baseline of 5% up to 15% of max_vehicles, creating a sparse sky
    let base_targets = ((cpu / 100.0) * (c_max * 0.15)).max(c_max * 0.05);
    // Pulse gently modulates traffic +/- 20%
    let target_vehicles = (base_targets * (0.8 + 0.4 * pulse)) as usize;
    
    if *chase_cooldown > 0 { *chase_cooldown -= 1; }

    if vehicles.len() < target_vehicles && frame_count % 3 == 0 {
        let y = get_sky_lane(rng);
        let roll = rng.gen_range(0.0..1.0);
        
        if roll < 0.02 && *chase_cooldown == 0 { 
            let speed = rng.gen_range(4.5..6.5);
            vehicles.push(Vehicle { x: -5.0, y, speed, color: theme.police_red, v_type: VehicleType::Spinner, length: 3 });
            vehicles.push(Vehicle { x: -15.0, y, speed, color: Color::White, v_type: VehicleType::Police, length: 3 });
            vehicles.push(Vehicle { x: -25.0, y, speed, color: Color::White, v_type: VehicleType::Police, length: 3 });
            vehicles.push(Vehicle { x: -35.0, y, speed, color: Color::White, v_type: VehicleType::Police, length: 3 });
            *chase_cooldown = 1200;
        } else if roll < 0.10 {
            let length: u16 = rng.gen_range(4..12);
            let mega_count = vehicles.iter().filter(|v| v.v_type == VehicleType::Shuttle && v.length > 9).count();
            let disk_bonus = (disk_usage / 10.0) as u16; 
            let adj_length = length.saturating_add(disk_bonus).min(25);

            if adj_length <= 9 || mega_count == 0 {
                let color = if adj_length <= 6 {
                    theme.shuttle_colors[0]
                } else if adj_length <= 9 {
                    theme.shuttle_colors[1]
                } else {
                    theme.shuttle_colors[2]
                };
                vehicles.push(Vehicle { x: -5.0, y, speed: rng.gen_range(0.3..0.6), color, v_type: VehicleType::Shuttle, length: adj_length });
            }
        } else {
            let color = theme.vehicle_colors[rng.gen_range(0..theme.vehicle_colors.len())];
            vehicles.push(Vehicle { x: -5.0, y, speed: rng.gen_range(0.8..2.2), color, v_type: VehicleType::Spinner, length: 3 });
        }
    }
    let speed_mod = (0.5 + (cpu / 80.0)) * config.vehicle_speed_multiplier;
    vehicles.retain_mut(|v| { v.x += v.speed * speed_mod; v.x < (area.width as f32 + 40.0) });
}
