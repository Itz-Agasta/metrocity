//! Sky, stars, moon, distant mountain ridges and the far hill band:
//! everything behind the water line, painted bottom of the z-order.

use ratatui::buffer::Buffer;
use ratatui::style::Color;

use super::layout::Layout;
use super::paint;
use super::palette::*;

pub fn render(buf: &mut Buffer, l: &Layout, t: f64) {
    sky(buf, l);
    stars(buf, l, t);
    moon(buf, l);
    mountains(buf, l);
    hills(buf, l);
}

/// Sky gradient color at a cell, with a little per-cell dither so the blend
/// doesn't band into flat horizontal stripes.
pub fn sky_color(l: &Layout, x: u16, y: u16) -> Color {
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
}

fn stars(buf: &mut Buffer, l: &Layout, t: f64) {
    for y in 0..l.horizon_y.saturating_sub(2) {
        for x in 0..l.w {
            let h = paint::hash(u32::from(x).wrapping_mul(7), u32::from(y).wrapping_mul(13));
            if h % 67 != 0 {
                continue;
            }
            // Per-star phase so they don't all twinkle in sync.
            let phase = f64::from(h % 628) / 100.0;
            let b = 0.35 + 0.65 * (0.5 + 0.5 * (t * 1.7 + phase).sin());
            let glyph = if h % 5 == 0 { '+' } else { '·' };
            paint::glyph_over(buf, x, y, glyph, paint::mix(sky_color(l, x, y), STAR, b as f32));
        }
    }
}

fn moon(buf: &mut Buffer, l: &Layout) {
    let m = l.moon;
    let cx = f32::from(m.left()) + f32::from(m.width) / 2.0;
    // Work in half-rows ('▀' sub-cells): a terminal cell is ~2x taller than
    // wide, so half-cells are roughly square and the disc stays round.
    let cyh = f32::from(m.top()) * 2.0 + f32::from(m.height);
    let rx = f32::from(m.width) / 2.0;
    let ryh = f32::from(m.height);

    // Crescent field: inside the disc but outside a second disc shifted
    // toward the upper-right, which bites the dark side out.
    let field = |dx: f32, dy: f32| -> (bool, f32) {
        let d = dx * dx + dy * dy;
        let bx = dx - 0.55;
        let by = dy - 0.25;
        let bite = bx * bx + by * by;
        let inside = d <= 1.0 && bite > 0.55;
        let glow = if bite > 0.55 {
            (1.0 - (d - 1.0) / 0.9).clamp(0.0, 1.0)
        } else {
            0.0
        };
        (inside, glow * glow * 0.30)
    };

    for y in m.top().saturating_sub(2)..(m.bottom() + 2).min(l.horizon_y) {
        for x in m.left().saturating_sub(2)..m.right() + 2 {
            let mut shade = [0.0f32; 2];
            for (half, s) in shade.iter_mut().enumerate() {
                let yh = f32::from(y) * 2.0 + half as f32;
                // 2x2 supersample per half-cell softens the pixel-stepped edge.
                let mut cov = 0.0;
                for (ox, oy) in [(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)] {
                    let dx = (f32::from(x) + ox - cx) / rx;
                    let dy = (yh + oy - cyh) / ryh;
                    if field(dx, dy).0 {
                        cov += 0.25;
                    }
                }
                let dx = (f32::from(x) + 0.5 - cx) / rx;
                let dy = (yh + 0.5 - cyh) / ryh;
                let (_, g) = field(dx, dy);
                *s = cov + (1.0 - cov) * g;
            }
            if shade[0] < 0.01 && shade[1] < 0.01 {
                continue; // pure sky: leave stars alone
            }
            let sky = sky_color(l, x, y);
            paint::half_fill(
                buf,
                x,
                y,
                paint::mix(sky, MOON, shade[0]),
                paint::mix(sky, MOON, shade[1]),
            );
        }
    }
}

/// Two hazy mountain ridge layers rolling across the whole horizon, the
/// far one paler (more sky mixed in), the near one a shade deeper.
fn mountains(buf: &mut Buffer, l: &Layout) {
    ridge(buf, l, 0.55, 0.45, MOUNTAIN_FAR, 0.055, 7.0);
    ridge(buf, l, 0.34, 0.18, MOUNTAIN_NEAR, 0.09, 2.0);
}

fn ridge(buf: &mut Buffer, l: &Layout, amp: f32, haze: f32, color: Color, freq: f64, phase: f64) {
    let max = f32::from(l.horizon_y) * amp;
    for x in 0..l.w {
        let xf = f64::from(x);
        // Layered sines: broad peaks with smaller shoulder bumps.
        let n = (xf * freq + phase).sin() * 0.6
            + (xf * freq * 2.7 + phase * 1.7).sin() * 0.3
            + (xf * freq * 6.1).sin() * 0.1;
        // Tiny per-column jitter roughens the quantized edge.
        let j = (paint::hash(u32::from(x), 0x81D) % 3) as f32 * 0.03;
        let hgt = (max * (0.55 + 0.45 * n as f32 + j)).max(0.0) as u16;
        for dy in 1..=hgt {
            let y = l.horizon_y.saturating_sub(dy);
            paint::fill(buf, x, y, paint::mix(color, sky_color(l, x, y), haze));
        }
    }
}

fn hills(buf: &mut Buffer, l: &Layout) {
    // Low dark ridge along the far shore, under Fuji and behind the lake.
    for x in 0..l.w {
        let xf = f64::from(x);
        let hgt = ((xf * 0.045).sin() * 1.4 + (xf * 0.013).sin() * 1.2 + 2.6) as u16;
        for dy in 0..hgt {
            let y = l.horizon_y.saturating_sub(dy + 1);
            paint::fill(buf, x, y, paint::mix(HILL_FAR, SKY_HORIZON, 0.15));
        }
    }
}
