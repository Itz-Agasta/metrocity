//! A couple of distant birds gliding across the sky.

use ratatui::buffer::Buffer;

use crate::scenes::meadow::layout::Layout;
use crate::scenes::meadow::paint;
use crate::scenes::meadow::palette::*;

pub fn draw(buf: &mut Buffer, l: &Layout, t: f64) {
    let drift = (t * 4.0) as i64;
    for (i, &cy) in [l.horizon_y / 3, l.horizon_y / 4].iter().enumerate() {
        let span = i64::from(l.w) + 6;
        let x = ((i64::from(l.w) / 2 + drift + i as i64 * 55) % span + span) % span - 3;
        if x >= 0 && x + 2 < i64::from(l.w) {
            let x = x as u16;
            paint::glyph_over(buf, x, cy, '⌒', BIRD);
            paint::glyph_over(buf, x + 1, cy, '⌒', BIRD);
        }
    }
}
