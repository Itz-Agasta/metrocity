//! A lighthouse on a rock in the near water, in front of the rail, with a
//! beam that sweeps side to side and flares when it faces the viewer.

use std::f32::consts::TAU;

use ratatui::buffer::Buffer;

use crate::scenes::harbor::layout::Layout;
use crate::scenes::harbor::palette::*;
use crate::scenes::harbor::{paint, FPS};

/// Seconds per full turn of the lamp.
const TURN: f32 = 10.0;

pub fn draw(buf: &mut Buffer, l: &Layout, t: f32) {
    let (cx, lamp, h) = (l.lighthouse_x, l.lamp_y, i32::from(l.h));
    let base = h - 3;
    if base - lamp < 4 {
        return; // too short a window for a tower
    }
    rock(buf, cx, h);
    foam(buf, cx, h, (t * FPS as f32) as u32);
    tower(buf, cx, lamp + 2, base - 1);
    // Stone plinth, wider than the tower, sitting on the rock.
    for x in cx - 3..=cx + 3 {
        let k = 0.6 + 0.4 * (x - cx + 3) as f32 / 6.0;
        paint::fill(buf, x, base, paint::scale(STONE, k));
    }

    // Gallery deck under the lamp room, roof above it.
    for x in cx - 2..=cx + 2 {
        let bg = paint::bg_at(buf, x, lamp + 1);
        paint::set(buf, x, lamp + 1, '▀', paint::scale(TOWER_WHITE, 0.55), bg);
    }
    let roof = paint::scale(TOWER_RED, 0.8);
    paint::glyph(buf, cx - 1, lamp - 1, '◢', roof);
    paint::fill(buf, cx, lamp - 1, roof);
    paint::glyph(buf, cx + 1, lamp - 1, '◣', roof);
    paint::glyph(buf, cx, lamp - 2, '▴', paint::scale(TOWER_WHITE, 0.6));

    let a = t / TURN * TAU;
    beam(buf, l, cx, lamp, a.cos(), a.sin());

    // Lamp room: glass either side of the lamp.
    paint::fill(buf, cx - 1, lamp, paint::mix(LAMP, TOWER_DARK, 0.45));
    paint::fill(buf, cx, lamp, LAMP);
    paint::fill(buf, cx + 1, lamp, paint::mix(LAMP, TOWER_DARK, 0.45));
}

fn rock(buf: &mut Buffer, cx: i32, h: i32) {
    // Each row is solid in the middle with a half-block slope at the ends.
    for (y, half) in [(h - 2, 3), (h - 1, 5)] {
        for x in cx - half - 1..=cx + half + 1 {
            if (x - cx).abs() > half {
                let bg = paint::bg_at(buf, x, y);
                paint::set(buf, x, y, '▄', ROCK, bg);
            } else {
                paint::fill(buf, x, y, ROCK);
            }
        }
    }
}

/// Waves breaking on the rock: low foam flickers against its slopes, and
/// spray is thrown up beside the plinth now and then.
fn foam(buf: &mut Buffer, cx: i32, h: i32, frame: u32) {
    let spots = [
        (5, h - 2),
        (6, h - 2),
        (7, h - 1),
        (8, h - 1),
        (4, h - 3),
        (5, h - 3),
    ];
    for (d, y) in spots {
        for x in [cx - d, cx + d] {
            let phase = paint::hash(x as u32 ^ (y as u32) << 12, (frame + x as u32 * 7) / 5) % 6;
            let ch = match (y == h - 3, phase) {
                (true, 5) => '·',
                (true, _) => continue,
                (false, 0) => continue,
                (false, 1 | 2) => '▁',
                (false, _) => '▂',
            };
            let bg = paint::bg_at(buf, x, y);
            paint::glyph(
                buf,
                x,
                y,
                ch,
                paint::mix(bg, FOAM, 0.25 + 0.06 * phase as f32),
            );
        }
    }
}

/// Banded tower, a cell wider near the base, lit from the moon's side.
fn tower(buf: &mut Buffer, cx: i32, top: i32, base: i32) {
    let flare = base - (base - top) / 3;
    for y in top..=base {
        let band = if ((y - top) / 2) % 2 == 0 {
            TOWER_WHITE
        } else {
            TOWER_RED
        };
        let half = if y > flare { 2 } else { 1 };
        for x in cx - half..=cx + half {
            let k = 0.45 + 0.55 * (x - cx + half) as f32 / (2 * half) as f32;
            paint::fill(buf, x, y, paint::scale(band, k));
        }
    }
}

/// The beam is a rotating cone seen from the side: its length on screen
/// follows cos(angle), and it is brighter while it faces the viewer.
fn beam(buf: &mut Buffer, l: &Layout, cx: i32, lamp: i32, c: f32, s: f32) {
    let toward = (-s).max(0.0);
    let strength = 0.22 + 0.2 * toward;
    let len = f32::from(l.w) * 0.6 * c.abs();
    let dir = if c >= 0.0 { 1 } else { -1 };
    for d in 2..len as i32 {
        let x = cx + dir * d;
        let fall = 1.0 - d as f32 / len;
        let half = 0.4 + d as f32 * 0.06;
        for y in lamp - half.ceil() as i32..=lamp + half.ceil() as i32 {
            let edge = (y - lamp).abs() as f32 / (half + 0.5);
            if edge < 1.0 {
                paint::tint(buf, x, y, BEAM, strength * fall * (1.0 - edge));
            }
        }
    }
    // Flare around the lamp while the beam points at the viewer.
    let flare = toward.powi(6) * (1.0 - c.abs());
    if flare > 0.02 {
        for y in lamp - 3..=lamp + 3 {
            for x in cx - 7..=cx + 7 {
                let dx = (x - cx) as f32 / 2.0;
                let d = (dx * dx + ((y - lamp) as f32).powi(2)).sqrt() / 3.5;
                if d < 1.0 {
                    paint::tint(buf, x, y, BEAM, flare * 0.7 * (1.0 - d));
                }
            }
        }
    }
}
