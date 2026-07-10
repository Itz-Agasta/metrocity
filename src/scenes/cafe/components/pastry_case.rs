//! Pastry display case on the right end of the counter.
//! The case itself is ratatui; the pastries inside are Kitty sprites.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use crate::scenes::cafe::layout::Layout;
use crate::scenes::cafe::paint;
use crate::scenes::cafe::palette::*;

const INTERIOR: ratatui::style::Color = ratatui::style::Color::Rgb(48, 32, 22);

/// Rows of the two glass shelves; pastry sprites sit right above them.
pub fn shelf_rows(l: &Layout) -> [u16; 2] {
    let r = l.pastry_case;
    [r.y + r.height / 2, r.y + r.height - 1]
}

pub fn draw(buf: &mut Buffer, l: &Layout) {
    let r = l.pastry_case;
    if r.width < 8 || r.height < 6 {
        return;
    }
    paint::fill_rect(buf, r, WOOD);
    let inner = Rect::new(r.x + 1, r.y + 1, r.width - 2, r.height - 1);
    paint::fill_rect(buf, inner, INTERIOR);

    // Glass shelf lines
    for row in shelf_rows(l) {
        if row > r.y && row < r.bottom() {
            for x in inner.left()..inner.right() {
                paint::glyph(buf, x, row, '─', GLASS, INTERIOR);
            }
        }
    }
}
