//! The lake: dark water with a drifting shimmer, and warm reflection
//! streaks mirroring the skyline windows, shore lamps and the moon.

use ratatui::buffer::Buffer;
use ratatui::style::Color;

use super::super::layout::Layout;
use super::super::paint;
use super::super::palette::*;
use super::skyline;

/// Base water color at a cell (darkens toward the near shore).
fn water(depth: f32) -> Color {
    paint::mix(LAKE_SHIMMER, LAKE, 0.35 + depth * 0.65)
}

pub fn draw(buf: &mut Buffer, l: &Layout, t: f64, seed: u32) {
    if l.ground_y <= l.horizon_y {
        return;
    }
    let depth_rows = f32::from(l.ground_y - l.horizon_y);

    for y in l.horizon_y..l.ground_y {
        let depth = f32::from(y - l.horizon_y) / depth_rows;
        for x in 0..l.w {
            let mut c = water(depth);
            // Drifting shimmer: time-bucketed hash so bright cells crawl.
            let bucket = (t * 2.5) as u32;
            let h = paint::hash(u32::from(x).wrapping_add(bucket), u32::from(y).wrapping_mul(31));
            if h % 19 == 0 {
                c = paint::mix(c, LAKE_SHIMMER, 0.7);
            }
            paint::fill(buf, x, y, c);
        }
    }

    // Warm streaks under the skyline lights, then the moon's shimmer path.
    for (x, s) in skyline::lights(l, seed) {
        streak(buf, l, t, x, REFLECT_WARM, s);
    }
    moon_path(buf, l, t);

    // Sparse wave glyphs on top so the streaks sit *in* the water.
    for y in l.horizon_y..l.ground_y {
        let depth = f32::from(y - l.horizon_y) / depth_rows;
        for x in 0..l.w {
            let bucket = (t * 2.5) as u32;
            let h = paint::hash(u32::from(x).wrapping_add(bucket), u32::from(y).wrapping_mul(31));
            if h % 53 == 0 {
                paint::glyph_over(buf, x, y, '~', paint::mix(water(depth), LAKE_SHIMMER, 0.9));
            }
        }
    }
}

/// The moon's reflection: a shimmering path that starts narrow at the
/// horizon and spreads as it comes toward the viewer, thinning into
/// scattered glints instead of a hard-edged column.
fn moon_path(buf: &mut Buffer, l: &Layout, t: f64) {
    let cx = i64::from(l.moon.left() + l.moon.width / 2);
    let depth_rows = f32::from(l.ground_y - l.horizon_y).max(1.0);
    for y in l.horizon_y..l.ground_y {
        let depth = f32::from(y - l.horizon_y) / depth_rows;
        // Perspective spread: ~1 cell wide at the horizon, ~7 near shore.
        let half = (0.8 + depth * 3.2) as i64;
        let sway = ((t * 0.9 + f64::from(y) * 0.5).sin() * f64::from(0.5 + depth * 1.5)) as i64;
        for dx in -half..=half {
            let x = cx + sway + dx;
            if x < 0 || x >= i64::from(l.w) {
                continue;
            }
            let edge = 1.0 - (dx.unsigned_abs() as f32 / (half as f32 + 1.0));
            // Sparser and dimmer with depth and toward the path's edges.
            let h = paint::hash(x as u32, u32::from(y).wrapping_mul(0x77));
            let keep = (1.0 - depth * 0.95) * (0.4 + 0.6 * edge);
            if (h % 100) as f32 / 100.0 >= keep {
                continue;
            }
            let dapple = 0.5 + (h % 50) as f32 / 100.0;
            let mut glow = 0.7 * (1.0 - depth * 0.65) * (0.4 + 0.6 * edge) * dapple;
            // Occasional hot glint riding the path.
            if h % 23 == 0 {
                glow = (glow + 0.25).min(0.9);
            }
            paint::fill(buf, x as u16, y, paint::mix(water(depth), MOON, glow));
        }
    }
}

/// One vertical reflection streak: wobbles sideways with a slow wave,
/// breaks up and fades as it nears the viewer.
fn streak(buf: &mut Buffer, l: &Layout, t: f64, x0: u16, color: Color, strength: f32) {
    let depth_rows = f32::from(l.ground_y - l.horizon_y).max(1.0);
    for y in l.horizon_y..l.ground_y {
        let depth = f32::from(y - l.horizon_y) / depth_rows;
        let wob = ((t * 1.2 + f64::from(y) * 0.55 + f64::from(x0) * 0.37).sin()
            * f64::from(0.6 + depth * 1.8)) as i32;
        let x = i64::from(x0) + i64::from(wob);
        if x < 0 || x >= i64::from(l.w) {
            continue;
        }
        let x = x as u16;
        // Streak survival decays with depth so it breaks into dashes and
        // dies out in the upper third of the lake; per-column length jitter.
        let len = 2.0 + (paint::hash(u32::from(x0), 0x1EE) % 80) as f32 / 100.0;
        let h = paint::hash(u32::from(x0).wrapping_mul(3), u32::from(y).wrapping_mul(17));
        let keep = strength * (1.0 - depth * len);
        if (h % 100) as f32 / 100.0 >= keep {
            continue;
        }
        // Hazy, not solid: each cell gets its own dimming so the streak
        // dapples into the water instead of reading as a painted bar.
        let dapple = 0.5 + (h % 50) as f32 / 100.0;
        let glow = 0.55 * strength * (1.0 - depth * 0.9) * dapple;
        let mut c = paint::mix(water(depth), color, glow);
        // The row right under the source glows a little hotter.
        if y == l.horizon_y {
            c = paint::mix(c, color, 0.25 * strength);
        }
        paint::fill(buf, x, y, c);
    }
}
