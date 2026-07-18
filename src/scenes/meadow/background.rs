//! Sky, hills and grass: the character-grid base layers the components and
//! animal sprites sit on top of.

use ratatui::buffer::Buffer;
use ratatui::style::Color;

use super::layout::Layout;
use super::paint;
use super::palette::*;

pub fn render(buf: &mut Buffer, l: &Layout, t: f64) {
    sky(buf, l);
    hills(buf, l);
    grass(buf, l, t);
}

/// Sky gradient color at a cell, with a little per-cell dither so the blend
/// doesn't band into flat horizontal stripes.
fn sky_color(l: &Layout, x: u16, y: u16) -> Color {
    let f = y as f32 / l.horizon_y.max(1) as f32;
    let n = (paint::hash(u32::from(x), u32::from(y)) % 64) as f32 / 64.0 - 0.5;
    paint::mix(SKY_TOP, SKY_HORIZON, f + n * 0.05)
}

fn sky(buf: &mut Buffer, l: &Layout) {
    for y in 0..l.horizon_y {
        for x in 0..l.w {
            paint::fill(buf, x, y, sky_color(l, x, y));
        }
    }
    // Sun: a round disc with a soft glow halo, so it reads as a light source
    // rather than a yellow block. Cells are tested against an ellipse whose
    // radii match the rect; the halo is one ring wider and blends into the sky.
    let s = l.sun;
    let cx = f32::from(s.left()) + f32::from(s.width) / 2.0 - 0.5;
    let cy = f32::from(s.top()) + f32::from(s.height) / 2.0 - 0.5;
    let rx = f32::from(s.width) / 2.0;
    let ry = f32::from(s.height) / 2.0;
    for y in s.top().saturating_sub(1)..s.bottom() + 1 {
        for x in s.left().saturating_sub(1)..s.right() + 1 {
            let dx = (f32::from(x) - cx) / rx;
            let dy = (f32::from(y) - cy) / ry;
            let d = dx * dx + dy * dy;
            if d <= 1.0 {
                paint::fill(buf, x, y, SUN);
            } else if d <= 1.7 {
                // Glow ring: fade the warm halo into the sky behind it.
                paint::fill(
                    buf,
                    x,
                    y,
                    paint::mix(SUN_GLOW, sky_color(l, x, y), (d - 1.0) / 0.7),
                );
            }
        }
    }
}

fn hills(buf: &mut Buffer, l: &Layout) {
    // Two ridges above the horizon: a tall hazy one far away (color pulled
    // toward the sky, so it reads as distance) and the original nearer band
    // overlapping its foot.
    if l.horizon_y == 0 {
        return;
    }
    for x in 0..l.w {
        let xf = f64::from(x);
        let far = ((xf * 0.045).sin() * 2.2 + (xf * 0.011).sin() * 1.8 + 4.5) as u16;
        for dy in 0..=far {
            paint::fill(buf, x, l.horizon_y.saturating_sub(dy), HILL_FAR);
        }
        // Near bushline: two sine frequencies plus per-column jitter so the
        // clumps read as lumpy bushes, and a lightened top cell rounds them
        // off instead of leaving stepped rectangles.
        let bump = ((xf * 0.15).sin() * 1.5
            + (xf * 0.47).sin() * 0.9
            + 1.8
            + f64::from(paint::hash(u32::from(x), 3) % 2) * 0.5) as u16;
        for dy in 0..=bump {
            let c = if dy == bump {
                paint::mix(HILL, HILL_FAR, 0.45)
            } else {
                HILL
            };
            paint::fill(buf, x, l.horizon_y.saturating_sub(dy), c);
        }
    }
}

fn grass(buf: &mut Buffer, l: &Layout, t: f64) {
    let ts = t as f32;
    let depth = (l.h - l.horizon_y).max(1) as f32;
    for y in l.horizon_y..l.h {
        let f = (y - l.horizon_y) as f32 / depth;
        let base = paint::mix(GRASS_LIGHT, GRASS_DARK, f);
        // Atmospheric haze: the far quarter of the field fades toward the
        // hill tone, so the distance reads as depth instead of a flat green.
        let haze = (1.0 - f * 4.0).max(0.0) * 0.55;
        for x in 0..l.w {
            // A gentle lighter band of wind travels right across the whole
            // field, so every cell animates (keeps the screensaver moving).
            let wave = (f32::from(x) * 0.14 - ts * 1.6 + f32::from(y) * 0.12).sin();
            let mut c = paint::mix(base, GRASS_LIGHT, (wave * 0.5 + 0.5) * 0.16);
            if haze > 0.0 {
                c = paint::mix(c, HILL_FAR, haze);
            }
            paint::fill(buf, x, y, c);
        }
    }
    // Scattered blades that sway as each gust passes over them, plus tiny
    // still wildflowers. Both get denser toward the bottom (a perspective
    // cue); the farthest rows stay clean so the haze band reads as distance.
    for y in l.horizon_y..l.h {
        let f = (y - l.horizon_y) as f32 / depth;
        if f < 0.22 {
            continue;
        }
        let every = if f < 0.5 {
            23
        } else if f < 0.75 {
            13
        } else {
            9
        };
        for x in 0..l.w {
            let h = paint::hash(u32::from(x), u32::from(y));
            if h % every == 0 {
                let sway = (f32::from(x) * 0.28 - ts * 3.0 + f32::from(y) * 0.15).sin();
                let ch = if sway > 0.4 {
                    '/' // leaning into the gust
                } else if sway < -0.4 {
                    '\\'
                } else if (x + y) % 2 == 0 {
                    '\''
                } else {
                    ','
                };
                paint::glyph_over(buf, x, y, ch, GRASS_BLADE);
            } else if h % 89 == 0 {
                let color = if h % 3 == 0 {
                    WILDFLOWER_GOLD
                } else {
                    WILDFLOWER
                };
                paint::glyph_over(buf, x, y, '·', color);
            }
        }
    }
}
