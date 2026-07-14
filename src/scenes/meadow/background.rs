//! Sky, hills and grass: the character-grid base layers the components and
//! animal sprites sit on top of.

use ratatui::buffer::Buffer;

use super::layout::Layout;
use super::paint;
use super::palette::*;

pub fn render(buf: &mut Buffer, l: &Layout, t: f64) {
    sky(buf, l);
    hills(buf, l);
    grass(buf, l, t);
}

fn sky(buf: &mut Buffer, l: &Layout) {
    for y in 0..l.horizon_y {
        let f = y as f32 / l.horizon_y.max(1) as f32;
        let c = paint::mix(SKY_TOP, SKY_HORIZON, f);
        for x in 0..l.w {
            paint::fill(buf, x, y, c);
        }
    }
    // Sun: a filled block with its four corners trimmed for a round look.
    let s = l.sun;
    for y in s.top()..s.bottom() {
        for x in s.left()..s.right() {
            let corner =
                (x == s.left() || x == s.right() - 1) && (y == s.top() || y == s.bottom() - 1);
            if !corner {
                paint::fill(buf, x, y, SUN);
            }
        }
    }
}

fn hills(buf: &mut Buffer, l: &Layout) {
    // A soft band of far hills just below the horizon.
    if l.horizon_y == 0 {
        return;
    }
    for x in 0..l.w {
        let bump = ((f64::from(x) * 0.15).sin() * 1.5 + 1.5) as u16;
        for dy in 0..=bump {
            paint::fill(buf, x, l.horizon_y.saturating_sub(dy), HILL);
        }
    }
}

fn grass(buf: &mut Buffer, l: &Layout, t: f64) {
    let ts = t as f32;
    for y in l.horizon_y..l.h {
        let f = (y - l.horizon_y) as f32 / (l.h - l.horizon_y).max(1) as f32;
        let base = paint::mix(GRASS_LIGHT, GRASS_DARK, f);
        for x in 0..l.w {
            // A gentle lighter band of wind travels right across the whole
            // field, so every cell animates (keeps the screensaver moving).
            let wave = (f32::from(x) * 0.14 - ts * 1.6 + f32::from(y) * 0.12).sin();
            let c = paint::mix(base, GRASS_LIGHT, (wave * 0.5 + 0.5) * 0.16);
            paint::fill(buf, x, y, c);
        }
    }
    // Scattered blades that sway as each gust passes over them.
    for y in l.horizon_y + 1..l.h {
        for x in 0..l.w {
            if paint::hash(u32::from(x), u32::from(y)) % 11 == 0 {
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
            }
        }
    }
}
