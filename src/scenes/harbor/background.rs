//! Sky, moon, stars and shooting stars: everything behind the skyline.

use rand::rngs::StdRng;
use rand::Rng;
use ratatui::buffer::Buffer;
use ratatui::style::Color;

use super::layout::Layout;
use super::paint;
use super::palette::*;
use super::FPS;

/// Moon craters: center and radius, in moon radii.
const CRATERS: [(f32, f32, f32); 5] = [
    (-0.35, -0.25, 0.24),
    (0.32, 0.2, 0.2),
    (0.05, -0.55, 0.13),
    (-0.25, 0.5, 0.16),
    (0.55, -0.3, 0.1),
];

struct Star {
    x: i32,
    y: i32,
    period: u32,
    offset: u32,
}

/// Which part of the moon is lit, picked once per launch.
pub struct MoonPhase {
    /// Terminator position: -1 full, 0 half, toward 1 a thin crescent.
    k: f32,
    /// Lit side: 1.0 right (waxing), -1.0 left (waning).
    side: f32,
}

impl MoonPhase {
    pub fn random(rng: &mut impl Rng) -> Self {
        const PHASES: [f32; 5] = [-1.0, -1.0, -0.5, 0.0, 0.55];
        Self {
            k: PHASES[rng.gen_range(0..PHASES.len())],
            side: if rng.gen() { 1.0 } else { -1.0 },
        }
    }

    /// Whether a point on the disc (in moon radii) is in sunlight.
    fn lit(&self, mx: f32, my: f32) -> bool {
        mx * self.side > self.k * (1.0 - my * my).max(0.0).sqrt()
    }

    /// Share of the disc that is lit, 0..1.
    fn fraction(&self) -> f32 {
        (1.0 - self.k) / 2.0
    }
}

#[derive(Default)]
pub struct Sky {
    stars: Vec<Star>,
}

impl Sky {
    pub fn generate(&mut self, l: &Layout, rng: &mut StdRng) {
        let n = usize::from(l.w) * usize::from(l.horizon) / 28;
        let rows = (u32::from(l.horizon) * 2 / 3).max(1) as i32;
        self.stars = (0..n)
            .map(|_| Star {
                x: rng.gen_range(0..i32::from(l.w.max(1))),
                y: rng.gen_range(0..rows),
                period: rng.gen_range(40..160),
                offset: rng.gen_range(0..160),
            })
            .collect();
    }

    pub fn draw(&self, buf: &mut Buffer, l: &Layout, frame: u32, seed: u32, phase: &MoonPhase) {
        sky(buf, l, phase);
        for s in &self.stars {
            let (ch, color) = match (frame + s.offset) % s.period {
                0..=2 => ('✦', STAR_BRIGHT),
                3..=5 => ('+', STAR_MID),
                _ => ('·', STAR_DIM),
            };
            paint::glyph(buf, s.x, s.y, ch, color);
        }
        shooting_star(buf, l, frame, seed);
        moon(buf, l, phase);
    }
}

/// Sky gradient at a row, before the moon halo.
fn sky_at(l: &Layout, y: i32) -> Color {
    let t = y as f32 / (l.horizon.max(2) - 1) as f32;
    paint::mix(SKY_TOP, SKY_HORIZON, t * t)
}

fn sky(buf: &mut Buffer, l: &Layout, phase: &MoonPhase) {
    let halo = l.moon_r * 3.2;
    let glow = 0.8 * (0.3 + 0.7 * phase.fraction());
    for y in 0..i32::from(l.horizon) {
        let base = sky_at(l, y);
        for x in 0..i32::from(l.w) {
            // Cells are about twice as tall as wide, so halve dx.
            let dx = (x as f32 - l.moon_x) / 2.0;
            let dy = y as f32 - l.moon_y;
            let d = (dx * dx + dy * dy).sqrt() / halo;
            let color = if d < 1.0 {
                paint::mix(base, MOON_HALO, (1.0 - d) * (1.0 - d) * glow)
            } else {
                base
            };
            paint::fill(buf, x, y, color);
        }
    }
}

/// In some six-second windows a star streaks down-right across the sky.
/// Drawn before the moon and skyline, so they cover it.
fn shooting_star(buf: &mut Buffer, l: &Layout, frame: u32, seed: u32) {
    const CYCLE: u32 = FPS * 6;
    let k = (frame / CYCLE) ^ seed;
    if paint::hash(k, 0) % 3 != 0 {
        return;
    }
    let start = paint::hash(k, 1) % (CYCLE - 30);
    let Some(age) = (frame % CYCLE).checked_sub(start) else {
        return;
    };
    if age > 26 {
        return;
    }
    let x0 = (paint::hash(k, 2) % u32::from(l.w / 2).max(1)) as f32;
    let y0 = (paint::hash(k, 3) % u32::from(l.horizon / 4).max(1)) as f32;
    for i in 0..7 {
        let x = (x0 + (age as f32 - i as f32) * 1.6) as i32;
        let y = (y0 + (age as f32 - i as f32) * 0.55) as i32;
        if y >= i32::from(l.horizon) - 2 {
            continue;
        }
        let ch = if i == 0 { '✦' } else { '━' };
        let k = 1.0 - i as f32 / 7.0;
        paint::glyph(buf, x, y, ch, paint::scale(SHOOTING_STAR, 0.35 + 0.65 * k));
    }
}

/// The moon at twice the vertical resolution with half blocks. The unlit
/// part still shows faintly against the sky (earthshine).
fn moon(buf: &mut Buffer, l: &Layout, phase: &MoonPhase) {
    let r = l.moon_r;
    let at = |x: i32, y: i32, sub: f32| {
        (
            (x as f32 - l.moon_x) / 2.0 / r,
            (y as f32 + sub - l.moon_y) / r,
        )
    };
    let shade = |mx: f32, my: f32| {
        let mut k = 1.0 - 0.2 * (mx * mx + my * my);
        for (cx, cy, cr) in CRATERS {
            let (dx, dy) = (mx - cx, my - cy);
            if dx * dx + dy * dy < cr * cr {
                k -= 0.13;
            }
        }
        let lit = paint::scale(MOON, k);
        if phase.lit(mx, my) {
            lit
        } else {
            paint::mix(SKY_TOP, lit, 0.12)
        }
    };
    let y0 = (l.moon_y - r - 1.0) as i32;
    let y1 = ((l.moon_y + r + 1.0) as i32).min(i32::from(l.horizon) - 1);
    let x0 = (l.moon_x - r * 2.0 - 2.0) as i32;
    let x1 = (l.moon_x + r * 2.0 + 2.0) as i32;
    for y in y0.max(0)..=y1 {
        for x in x0..=x1 {
            let (ux, uy) = at(x, y, 0.25);
            let (lx, ly) = at(x, y, 0.75);
            let upper = ux * ux + uy * uy <= 1.0;
            let lower = lx * lx + ly * ly <= 1.0;
            let bg = paint::bg_at(buf, x, y);
            match (upper, lower) {
                (true, true) => paint::set(buf, x, y, '▀', shade(ux, uy), shade(lx, ly)),
                (true, false) => paint::set(buf, x, y, '▀', shade(ux, uy), bg),
                (false, true) => paint::set(buf, x, y, '▄', shade(lx, ly), bg),
                _ => {}
            }
        }
    }
}
