//! Falling petals, ported from csakura: spawn from canopy edge cells, drift
//! on a slowly wandering wind (biased left, matching the reference), land on
//! the bank, rest a few seconds as faded dots, then respawn.

use rand::Rng;
use ratatui::buffer::Buffer;
use ratatui::style::Color;

use super::layout::Layout;
use super::paint;
use super::palette::*;

const MAX_PETALS: usize = 512;
const GLYPHS: [char; 5] = ['❀', '✿', '*', '·', '∘'];

struct Petal {
    x: f64,
    y: f64,
    vy: f64, // fall speed, cells per frame at 20fps
    phase: f64,
    freq: f64,
    amp: f64, // horizontal sway
    glyph: char,
    color: Color,
    land_y: f64, // bank row this petal settles on
    rest: f64,   // seconds left on the ground, <0 = falling
    active: bool,
}

#[derive(Default)]
pub struct Petals {
    v: Vec<Petal>,
    wind: f64,
    wind_target: f64,
}

fn frange(rng: &mut impl Rng, a: f64, b: f64) -> f64 {
    rng.gen_range(a..b)
}

fn spawn(p: &mut Petal, l: &Layout, sources: &[(u16, u16)], rng: &mut impl Rng, scatter: bool) {
    p.active = true;
    p.rest = -1.0;

    if !sources.is_empty() && rng.gen_bool(0.85) {
        let (sx, sy) = sources[rng.gen_range(0..sources.len())];
        p.x = f64::from(sx) + frange(rng, -1.0, 1.0);
        p.y = f64::from(sy) + frange(rng, 0.0, 1.0);
    } else {
        p.x = frange(rng, 0.0, f64::from(l.w));
        p.y = -frange(rng, 0.0, 3.0);
    }
    if scatter {
        // Initial fill so the screen isn't empty at startup.
        p.y = frange(rng, 0.0, f64::from(l.h) - 2.0);
    }

    p.vy = frange(rng, 0.10, 0.28);
    p.amp = frange(rng, 0.10, 0.45);
    p.freq = frange(rng, 0.05, 0.18);
    p.phase = frange(rng, 0.0, std::f64::consts::TAU);
    p.glyph = GLYPHS[rng.gen_range(0..GLYPHS.len())];
    p.color = RAMP[1 + rng.gen_range(0..5)];
    p.land_y = f64::from(l.ground_y) + frange(rng, 0.0, f64::from((l.h - 1 - l.ground_y).max(1)));
}

impl Petals {
    pub fn reset(&mut self, l: &Layout, sources: &[(u16, u16)], rng: &mut impl Rng, scatter: bool) {
        let count = (usize::from(l.w) * 5 / 4).clamp(16, MAX_PETALS);
        self.v.clear();
        for _ in 0..count {
            let mut p = Petal {
                x: 0.0,
                y: 0.0,
                vy: 0.0,
                phase: 0.0,
                freq: 0.0,
                amp: 0.0,
                glyph: '·',
                color: RAMP[2],
                land_y: 0.0,
                rest: -1.0,
                active: false,
            };
            if rng.gen_bool(0.6) {
                spawn(&mut p, l, sources, rng, scatter);
            }
            self.v.push(p);
        }
    }

    pub fn update(&mut self, dt: f64, l: &Layout, tree: &super::tree::Tree, rng: &mut impl Rng) {
        let sources = &tree.sources;
        // Wind wanders slowly, biased leftward, with occasional gusts.
        if rng.gen_bool(0.008) {
            self.wind_target = frange(rng, -0.45, 0.12);
        }
        self.wind += (self.wind_target - self.wind) * 0.02;

        for p in &mut self.v {
            if !p.active {
                // Trickle respawns so it never pulses.
                if rng.gen_bool(0.03) {
                    spawn(p, l, sources, rng, false);
                }
                continue;
            }

            if p.rest >= 0.0 {
                // Lying on the ground.
                p.rest -= dt;
                if p.rest < 0.0 {
                    p.active = false;
                }
                continue;
            }

            p.phase += p.freq;
            p.x += self.wind + p.amp * p.phase.sin();
            p.y += p.vy;

            if p.x < -2.0 {
                p.x = f64::from(l.w) + 1.0;
            } else if p.x > f64::from(l.w) + 2.0 {
                p.x = -1.0;
            }

            if p.y >= p.land_y {
                // Touched down on the bank. Petals never pile up on the
                // trunk: over wood they just vanish and respawn.
                if tree.is_wood(p.x as u16, p.land_y as u16) {
                    p.active = false;
                    continue;
                }
                p.y = p.land_y;
                p.rest = frange(rng, 2.0, 7.0);
                p.color = RAMP[5];
                p.glyph = '·';
            }
        }
    }

    pub fn draw(&self, buf: &mut Buffer) {
        for p in &self.v {
            if !p.active || p.x < 0.0 || p.y < 0.0 {
                continue;
            }
            paint::glyph_over(buf, p.x as u16, p.y as u16, p.glyph, p.color);
        }
    }
}
