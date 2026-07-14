//! The red-and-cream checkered picnic blanket the hunny pot sits on.

use ratatui::buffer::Buffer;

use crate::scenes::meadow::layout::Layout;
use crate::scenes::meadow::paint;
use crate::scenes::meadow::palette::*;

pub fn draw(buf: &mut Buffer, l: &Layout) {
    let b = l.blanket;
    for y in b.top()..b.bottom().min(l.h) {
        for x in b.left()..b.right().min(l.w) {
            let checker = (x + y) % 2 == 0;
            paint::fill(buf, x, y, if checker { BLANKET_RED } else { BLANKET_CREAM });
        }
    }
}
