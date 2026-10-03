//! Buildings standing on the rail, their windows and the tower beacon.
//! The layout is seeded per terminal size; which windows are lit varies
//! per launch.

use rand::rngs::StdRng;
use rand::Rng;
use ratatui::buffer::Buffer;
use ratatui::style::Color;

use crate::scenes::harbor::layout::Layout;
use crate::scenes::harbor::palette::*;
use crate::scenes::harbor::{paint, FPS};

struct Building {
    x0: i32,
    x1: i32,
    top: i32,
    body: Color,
}

struct Window {
    x: i32,
    y: i32,
    cool: bool,
    /// Frames between the window's chances to switch.
    period: u32,
    offset: u32,
}

#[derive(Default)]
pub struct Skyline {
    buildings: Vec<Building>,
    windows: Vec<Window>,
    antenna: (i32, i32),
}

impl Skyline {
    pub fn generate(&mut self, l: &Layout, rng: &mut StdRng) {
        let horizon = i32::from(l.horizon);
        let w = i32::from(l.w);
        self.buildings.clear();
        self.windows.clear();
        let (mut tallest, mut tallest_x) = (0, 0);
        let mut x = 0;
        while x < w {
            let bw = rng.gen_range(4..12);
            let min_h = horizon / 4;
            let mut bh = min_h + rng.gen_range(0..(horizon * 3 / 5 - min_h).max(1));
            if rng.gen_range(0..6) == 0 {
                bh = horizon * 3 / 4; // an occasional tower
            }
            let top = horizon - bh;
            let x1 = (x + bw).min(w);
            let s: u8 = rng.gen_range(16..30);
            self.buildings.push(Building {
                x0: x,
                x1,
                top,
                body: Color::Rgb(s, s + 2, s + 16),
            });
            if bh > tallest {
                (tallest, tallest_x) = (bh, x + bw / 2);
            }
            for wy in (top + 1..horizon - 3).step_by(2) {
                for wx in (x + 1..x1 - 1).step_by(2) {
                    self.windows.push(Window {
                        x: wx,
                        y: wy,
                        cool: rng.gen_range(0..5) == 0,
                        period: FPS * rng.gen_range(60..240),
                        offset: rng.gen_range(0..FPS * 240),
                    });
                }
            }
            x += bw + rng.gen_range(0..2);
        }
        self.antenna = (tallest_x, horizon - tallest - 3);
    }

    pub fn draw(&self, buf: &mut Buffer, l: &Layout, frame: u32, seed: u32) {
        for b in &self.buildings {
            for x in b.x0..b.x1 {
                // The roof edge catches a little moonlight against the sky.
                let sky = paint::bg_at(buf, x, b.top);
                paint::set(buf, x, b.top, '▁', paint::mix(b.body, MOON, 0.25), sky);
                for y in b.top + 1..i32::from(l.horizon) {
                    paint::fill(buf, x, y, b.body);
                }
            }
        }

        for (n, win) in self.windows.iter().enumerate() {
            let lit = paint::hash(n as u32 ^ seed, (frame + win.offset) / win.period) % 3 != 0;
            let color = match (lit, win.cool) {
                (true, true) => WINDOW_COOL,
                (true, false) => WINDOW_WARM,
                _ => WINDOW_OFF,
            };
            paint::glyph(buf, win.x, win.y, '▪', color);
        }

        // Antenna with a slow red beacon on the tallest tower.
        let (ax, ay) = self.antenna;
        for y in ay + 1..ay + 3 {
            paint::glyph(buf, ax, y, '│', ANTENNA);
        }
        let beacon = if (frame / FPS) % 2 == 0 {
            BEACON_ON
        } else {
            BEACON_OFF
        };
        paint::glyph(buf, ax, ay, '●', beacon);
    }
}
