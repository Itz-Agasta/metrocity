//! Menu board: wooden frame, dark board, item list with dot leaders.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use crate::scenes::cafe::layout::Layout;
use crate::scenes::cafe::paint;
use crate::scenes::cafe::palette::*;

const ITEMS: [(&str, char); 5] = [
    ("COFFEE", '3'),
    ("LATTE", '4'),
    ("MOCHA", '5'),
    ("TEA", '3'),
    ("PAWTISSERIE", '4'),
];

pub fn draw(buf: &mut Buffer, l: &Layout) {
    let r = l.menu;
    if r.width < 17 || r.height < 9 {
        return;
    }
    paint::fill_rect(buf, r, WOOD);
    let inner = Rect::new(r.x + 1, r.y + 1, r.width - 2, r.height - 2);
    paint::fill_rect(buf, inner, BOARD_BG);

    let title_x = inner.x + (inner.width.saturating_sub(4)) / 2;
    paint::text(buf, title_x, inner.y + 1, "MENU", CREAM, BOARD_BG);

    let line_w = usize::from(inner.width.saturating_sub(2));
    for (i, (name, price)) in ITEMS.iter().enumerate() {
        let y = inner.y + 3 + i as u16;
        if y >= inner.bottom() {
            break;
        }
        let mut line = String::with_capacity(line_w);
        line.push_str(name);
        while line.len() < line_w - 1 {
            line.push('.');
        }
        line.push(*price);
        paint::text(buf, inner.x + 1, y, &line, CREAM_DIM, BOARD_BG);
    }
}
