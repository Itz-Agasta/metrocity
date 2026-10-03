//! Water: the city mirrored, darkened and blued, broken up by ripples,
//! glints and a shimmering path of moonlight.

use ratatui::buffer::Buffer;

use crate::scenes::harbor::layout::Layout;
use crate::scenes::harbor::paint;
use crate::scenes::harbor::palette::*;

pub fn draw(buf: &mut Buffer, l: &Layout, frame: u32) {
    let (w, h, horizon) = (i32::from(l.w), i32::from(l.h), i32::from(l.horizon));
    reflection(buf, w, h, horizon);
    moon_path(buf, l, frame);

    // Ripples on fixed columns.
    for y in horizon..h {
        for x in ((y * 7) % 11..w).step_by(11) {
            let bg = paint::bg_at(buf, x, y);
            paint::set(buf, x, y, '─', paint::mix(bg, RIPPLE, 0.25), bg);
        }
    }

    // Glints: sixteen slots, each jumping to another spot every few frames.
    let cells = ((h - horizon) * w) as u32;
    if cells == 0 {
        return;
    }
    for j in 0..16u32 {
        let period = 5 + j % 7;
        let i = (paint::hash(j + 1000, (frame + j * 3) / period) % cells) as i32;
        let (x, y) = (i % w, horizon + i / w);
        let bg = paint::bg_at(buf, x, y);
        paint::set(buf, x, y, '━', paint::mix(bg, MOON, 0.55), bg);
    }
}

fn reflection(buf: &mut Buffer, w: i32, h: i32, horizon: i32) {
    for y in horizon..h {
        let src = (2 * horizon - 1 - y).max(0);
        let depth = (y - horizon + 1) as f32 / (h - horizon + 1) as f32;
        for x in 0..w {
            let s = &buf[(x as u16, src as u16)];
            let fg = paint::mix(s.fg, WATER_DEEP, 0.45 + 0.35 * depth);
            let bg = paint::mix(s.bg, WATER_DEEP, 0.5 + 0.4 * depth);
            let ch = match s.symbol().chars().next().unwrap_or(' ') {
                '▀' => '▄',
                '▄' => '▀',
                '▗' => '▝',
                '▁' => '▔',
                c => c,
            };
            if ch == ' ' || fg == bg {
                paint::fill(buf, x, y, bg);
            } else {
                paint::set(buf, x, y, ch, fg, bg);
            }
        }
    }
}

/// A broken column of moonlight under the moon, widening toward the viewer.
fn moon_path(buf: &mut Buffer, l: &Layout, frame: u32) {
    let (horizon, h) = (i32::from(l.horizon), i32::from(l.h));
    let cx = l.moon_x as i32;
    for y in horizon..h {
        let depth = (y - horizon + 1) as f32 / (h - horizon) as f32;
        let half = (l.moon_r * (0.6 + 1.4 * depth)) as i32;
        for x in cx - half..=cx + half {
            let edge = (x - cx).abs() as f32 / (half + 1) as f32;
            let k = paint::hash((x as u32) ^ ((y as u32) << 16), frame / 6) % 100;
            if (k as f32) < 45.0 * (1.0 - edge) {
                let bg = paint::bg_at(buf, x, y);
                let glow = paint::mix(bg, MOON, 0.35 * (1.0 - edge) + 0.1);
                paint::set(buf, x, y, '━', glow, bg);
            }
        }
    }
}
