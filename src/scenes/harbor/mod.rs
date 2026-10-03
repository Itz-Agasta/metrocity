//! Harbor scene: a city at night over water. A moon, twinkling stars,
//! windows that go on and off, a train crossing an elevated line, a tower
//! beacon, a shooting star now and then, a lighthouse sweeping its beam,
//! and the whole skyline reflected in the water. Pure character-grid, no sprites.
//! Ported from limoni's city backdrop (see THIRD-PARTY-NOTICES.md).
//!
//! Module map:
//! - `layout`      horizon, rail and moon position from the terminal size
//! - `palette`     hardcoded night colors
//! - `paint`       low-level cell helpers
//! - `background`  sky, moon, stars, shooting stars
//! - `components`  skyline, train, water, lighthouse

use rand::rngs::StdRng;
use rand::{thread_rng, Rng, SeedableRng};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use crate::scene::Scene;
use crate::theme::Theme;

mod background;
mod components;
mod layout;
mod paint;
mod palette;

use layout::Layout;

/// Animation frame rate the timings are written against.
const FPS: u32 = 30;
/// Fixed layout seed: a terminal size always gets the same city.
const SEED: u64 = 7;

pub struct HarborScene {
    layout: Layout,
    t: f64,
    /// Per-launch seed deciding which windows are lit and when stars shoot.
    seed: u32,
    moon_phase: background::MoonPhase,
    sky: background::Sky,
    skyline: components::skyline::Skyline,
}

impl HarborScene {
    pub fn new() -> Self {
        Self {
            layout: Layout::default(),
            t: 0.0,
            seed: thread_rng().gen(),
            moon_phase: background::MoonPhase::random(&mut thread_rng()),
            sky: background::Sky::default(),
            skyline: components::skyline::Skyline::default(),
        }
    }
}

impl Scene for HarborScene {
    fn name(&self) -> &str {
        "harbor"
    }

    fn init(&mut self, width: u16, height: u16, _theme: &Theme) {
        self.layout = Layout::new(width, height);
        let mut rng = StdRng::seed_from_u64(SEED);
        self.sky.generate(&self.layout, &mut rng);
        self.skyline.generate(&self.layout, &mut rng);
    }

    fn update(&mut self, dt: f64) {
        self.t += dt;
    }

    fn draw(&self, area: Rect, buf: &mut Buffer) {
        let l = &self.layout;
        if area.width != l.w || area.height != l.h || l.h < 6 {
            return;
        }
        let frame = (self.t * f64::from(FPS)) as u32;
        self.sky.draw(buf, l, frame, self.seed, &self.moon_phase);
        self.skyline.draw(buf, l, frame, self.seed);
        components::train::draw(buf, l, frame);
        components::water::draw(buf, l, frame);
        components::lighthouse::draw(buf, l, self.t as f32);
    }
}
