//! Distant city skyline along the far shore: dark building blocks with
//! scattered warm windows (a nod to the city scene), a string of shore
//! lights at the waterline, and a tall lit pagoda rising out of the
//! rooftops. The lake reads `lights()` to mirror the glow on the water.

use ratatui::buffer::Buffer;

use super::super::layout::Layout;
use super::super::paint;
use super::super::palette::*;

struct Building {
    x: u16,
    w: u16,
    h: u16,
    shade: f32,
}

fn buildings(l: &Layout) -> Vec<Building> {
    let mut v = Vec::new();
    let mut x: u16 = 0;
    let mut i: u32 = 0;
    while x < l.w {
        let w = 3 + (paint::hash(i, 0xB1) % 5) as u16;
        // Mostly low blocks, the occasional taller tower.
        let mut h = 2 + (paint::hash(i, 0xB2) % 4) as u16;
        if paint::hash(i, 0xB5) % 5 == 0 {
            h += 2;
        }
        v.push(Building {
            x,
            w,
            h,
            shade: (paint::hash(i, 0xB4) % 3) as f32 * 0.5,
        });
        x += w + (paint::hash(i, 0xB3) % 3 == 0) as u16;
        i += 1;
    }
    v
}

/// Window grid slot: every other row and column, like the city scene.
fn window_slot(xx: u16, dy: u16) -> bool {
    dy % 2 == 1 && xx % 2 == 0
}

/// Whether a window slot is lit this launch: seeded per scene creation
/// (city-scene style, ~40% of windows on), stable while the scene runs.
fn window_lit(xx: u16, dy: u16, seed: u32) -> bool {
    paint::hash(u32::from(xx) ^ seed, u32::from(dy).wrapping_mul(0x51)) % 100 < 40
}

/// Shoreline lamp columns (the string of lights at the waterline).
fn shore_light_at(x: u16) -> bool {
    paint::hash(u32::from(x), 0x5407) % 9 == 0
}

/// Columns that cast a warm reflection on the lake, with a 0..1 strength.
pub fn lights(l: &Layout, seed: u32) -> Vec<(u16, f32)> {
    let mut cols = vec![0.0f32; usize::from(l.w)];
    for b in buildings(l) {
        for xx in b.x..(b.x + b.w).min(l.w) {
            // Only lit windows right at the waterline reflect, and only
            // some of those, so the water doesn't clutter up.
            if window_slot(xx, 1)
                && window_lit(xx, 1, seed)
                && paint::hash(u32::from(xx), 0x2EF) % 2 == 0
            {
                cols[usize::from(xx)] += 0.45;
            }
        }
    }
    for x in 0..l.w {
        // Most (not all) shore lamps reflect, a touch softer than before
        // so the water keeps some dark stretches.
        if shore_light_at(x) && paint::hash(u32::from(x), 0x3A5) % 3 != 0 {
            cols[usize::from(x)] += 0.65;
        }
    }
    // The pagoda's stacked windows make the brightest column.
    if l.pagoda_x < l.w {
        cols[usize::from(l.pagoda_x)] = 1.0;
    }
    cols.into_iter()
        .enumerate()
        .filter(|&(_, s)| s >= 0.4)
        .map(|(x, s)| (x as u16, s.min(1.0)))
        .collect()
}

pub fn draw(buf: &mut Buffer, l: &Layout, t: f64, seed: u32) {
    let base = l.horizon_y; // building feet stand on the water line
    for b in buildings(l) {
        let body = paint::mix(SKYLINE, SKYLINE_LIT, b.shade);
        for xx in b.x..(b.x + b.w).min(l.w) {
            for dy in 1..=b.h {
                let y = base.saturating_sub(dy);
                paint::fill(buf, xx, y, body);
                if window_slot(xx, dy) {
                    if window_lit(xx, dy, seed) {
                        // Each lit window flickers slowly on its own phase.
                        let hw = paint::hash(u32::from(xx), u32::from(dy).wrapping_mul(0x51));
                        let phase = f64::from(hw % 628) / 100.0;
                        let f = 0.55 + 0.45 * (t * 0.4 + phase).sin();
                        let warm = paint::mix(body, WINDOW_WARM, 0.45 + 0.45 * f as f32);
                        paint::glyph_over(buf, xx, y, '▪', warm);
                    } else {
                        paint::glyph_over(buf, xx, y, '▪', paint::mix(body, WINDOW_UNLIT, 0.8));
                    }
                }
            }
        }
    }

    // String of shore lamps right at the waterline.
    for x in 0..l.w {
        if shore_light_at(x) {
            let y = base.saturating_sub(1);
            let c = paint::mix(WINDOW_WARM, LANTERN_GLOW, 0.5);
            paint::glyph_over(buf, x, y, '•', c);
        }
    }

    pagoda(buf, l, t);
}

/// Tall multi-tier pagoda with a row of warm windows per storey, the
/// skyline's focal point (like the reference art).
fn pagoda(buf: &mut Buffer, l: &Layout, t: f64) {
    let cx = l.pagoda_x;
    let base = l.horizon_y;
    if cx < 7 || cx + 7 >= l.w || base < 14 {
        // Not enough room: fall back to nothing, the skyline still reads.
        return;
    }

    // Stone base at the waterline.
    for x in cx - 5..=cx + 5 {
        paint::fill(buf, x, base - 1, SKYLINE);
    }

    // Tiers from bottom to top: (half-width of body, half-width of roof).
    let tiers: [(u16, u16); 4] = [(4, 6), (3, 5), (2, 4), (1, 3)];
    let mut y = base - 2;
    for (i, &(bhw, rhw)) in tiers.iter().enumerate() {
        // Body row with warm windows.
        for x in cx - bhw..=cx + bhw {
            paint::fill(buf, x, y, SKYLINE);
        }
        let f = 0.75 + 0.25 * (t * 0.5 + i as f64 * 1.3).sin() as f32;
        let warm = paint::mix(SKYLINE, WINDOW_WARM, 0.85 * f);
        let mut wx = cx - bhw + 1;
        while wx <= cx + bhw {
            if wx != cx || bhw < 2 {
                paint::glyph_over(buf, wx, y, '▮', warm);
            }
            wx += 2;
        }
        // Roof row: wider than the body, upswept eaves at the tips.
        let ry = y - 1;
        for x in cx - rhw..=cx + rhw {
            paint::fill(buf, x, ry, SKYLINE);
        }
        paint::glyph_over(buf, cx - rhw, ry, '▟', SKYLINE);
        paint::glyph_over(buf, cx + rhw, ry, '▙', SKYLINE);
        if ry < 2 {
            return;
        }
        y = ry - 1;
    }
    // Spire.
    paint::glyph_over(buf, cx, y, '╿', SKYLINE_LIT);
}
