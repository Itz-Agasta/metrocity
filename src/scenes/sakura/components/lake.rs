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

    // Single pass: base water + drifting ripple dashes + sparse '~' glyphs.
    for y in l.horizon_y..l.ground_y {
        let depth = f32::from(y - l.horizon_y) / depth_rows;
        // Near rows drift faster than far rows: a cheap perspective cue.
        let drift = (t * (1.5 + f64::from(depth) * 4.0)) as i64;
        let dash_max = 2 + (depth * 4.0) as u32;
        for x in 0..l.w {
            // Static dither breaks the depth gradient's banding; a light
            // sheen on the far rows reads as water catching the sky.
            let h = paint::hash(u32::from(x), u32::from(y));
            let n = (h % 64) as f32 / 64.0 - 0.5;
            let sheen = (1.0 - depth * 5.0).max(0.0) * 0.18;
            let mut c = paint::mix(LAKE_SHIMMER, LAKE, 0.35 + depth * 0.65 + n * 0.08 - sheen);
            // Horizontal ripple dashes: every 37th column starts a dash whose
            // length grows toward the viewer; drift slides them sideways.
            let k = (i64::from(x) + drift) as u32;
            let len = 2 + paint::hash(k / 43, u32::from(y).wrapping_mul(37)) % dash_max;
            if k % 43 < len {
                c = paint::mix(c, LAKE_SHIMMER, 0.18);
            }
            paint::fill(buf, x, y, c);
            if h.wrapping_add((t * 1.5) as u32) % 71 == 0 {
                paint::glyph_over(buf, x, y, '~', paint::mix(c, LAKE_SHIMMER, 0.8));
            }
        }
    }

    // Warm dashes under the skyline lights, then the moon's glint path.
    for (x, s) in skyline::lights(l, seed) {
        streak(buf, l, t, x, REFLECT_WARM, s);
    }
    moon_path(buf, l, t);
}

/// The moon's reflection: a glint path of horizontal dashes. Nearly solid
/// right under the horizon, then spreading wider, dimming and breaking into
/// scattered single sparkles as the water comes toward the viewer.
fn moon_path(buf: &mut Buffer, l: &Layout, t: f64) {
    let cx = f64::from(l.moon.left()) + f64::from(l.moon.width) / 2.0;
    let depth_rows = f32::from(l.ground_y - l.horizon_y).max(1.0);
    for y in l.horizon_y..l.ground_y {
        let depth = f32::from(y - l.horizon_y) / depth_rows;
        let hrow = paint::hash(u32::from(y).wrapping_mul(0x77), 0x5EA);
        // Fewer rows keep their glint as the path nears the shore.
        if (hrow % 100) as f32 / 100.0 >= 1.0 - depth * 0.55 {
            continue;
        }
        // One small dash per row: swaying under the moon, scattering wider
        // with depth, so the trail reads as a thin broken shimmer line.
        let sway = (t * 0.8 + f64::from(y) * 0.9).sin() * (0.6 + f64::from(depth) * 2.5);
        let jitter = (f64::from((hrow / 13) % 7) - 3.0) * f64::from(depth);
        let len = ((2.5 - depth * 1.5) as i64 + i64::from((hrow / 31) % 2)).max(1);
        let start = (cx + sway + jitter - len as f64 / 2.0).round() as i64;
        let glow =
            (0.42 - depth * 0.26) * (0.8 + 0.2 * ((t * 2.0 + f64::from(y) * 1.3).sin() as f32));
        for i in 0..len {
            let x = start + i;
            if (0..i64::from(l.w)).contains(&x) {
                paint::fill(buf, x as u16, y, paint::mix(water(depth), MOON, glow));
            }
        }
    }
}

/// One skyline light's reflection: a short broken dash column hugging the
/// far shore, wobbling sideways and dying out in the top of the lake.
fn streak(buf: &mut Buffer, l: &Layout, t: f64, x0: u16, color: Color, strength: f32) {
    let depth_rows = f32::from(l.ground_y - l.horizon_y).max(1.0);
    for y in l.horizon_y..l.ground_y {
        let depth = f32::from(y - l.horizon_y) / depth_rows;
        let fade = 1.0 - depth * 2.4;
        if fade <= 0.0 {
            break;
        }
        let h = paint::hash(u32::from(x0).wrapping_mul(3), u32::from(y).wrapping_mul(17));
        // Vertical gaps so the column reads as broken water, not an icicle.
        if h % 10 < 3 {
            continue;
        }
        let wob = ((t * 1.1 + f64::from(y) * 0.7 + f64::from(x0) * 0.37).sin()
            * f64::from(0.4 + depth * 1.6)) as i64;
        let x = i64::from(x0) + wob;
        if x < 0 || x >= i64::from(l.w) {
            continue;
        }
        let dapple = 0.5 + (h % 50) as f32 / 100.0;
        let mut c = paint::mix(water(depth), color, 0.5 * strength * fade * dapple);
        // The row right under the source glows a little hotter.
        if y == l.horizon_y {
            c = paint::mix(c, color, 0.3 * strength);
        }
        paint::fill(buf, x as u16, y, c);
    }
}
