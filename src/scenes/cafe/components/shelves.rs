//! Wall shelves with coffee jars (top) and mugs (bottom).

use ratatui::buffer::Buffer;

use crate::scenes::cafe::layout::Layout;
use crate::scenes::cafe::paint;
use crate::scenes::cafe::palette::*;

pub fn draw(buf: &mut Buffer, l: &Layout) {
    let (x0, x1) = (l.shelf_x0, l.shelf_x1);
    if x1 <= x0 + 6 {
        return;
    }
    for (i, &row) in l.shelf_rows.iter().enumerate() {
        if row < 3 || row >= l.counter_y {
            continue;
        }
        // Shelf board with a darker underside
        for x in x0..x1 {
            paint::fill(buf, x, row, WOOD);
            paint::glyph_over(buf, x, row + 1, '▔', WOOD_DARK);
        }
        if i == 0 {
            draw_jars(buf, x0, x1, row);
        } else {
            draw_mugs(buf, x0, x1, row);
        }
    }
}

fn draw_jars(buf: &mut Buffer, x0: u16, x1: u16, shelf: u16) {
    // 2-cell wide jars with lids, spaced every 4 cells.
    // The right end of the shelf is left free for the potted-plant sprite.
    let mut x = x0 + 1;
    while x + 2 < x1.saturating_sub(5) {
        paint::glyph(buf, x, shelf - 2, '▄', JAR_LID, WALL);
        paint::glyph(buf, x + 1, shelf - 2, '▄', JAR_LID, WALL);
        paint::fill(buf, x, shelf - 1, JAR);
        paint::fill(buf, x + 1, shelf - 1, JAR);
        x += 4;
    }
}

fn draw_mugs(buf: &mut Buffer, x0: u16, x1: u16, shelf: u16) {
    // 2-cell wide mugs with a little handle, spaced every 5 cells.
    // The right end stays free for the sprout sprite.
    let mut x = x0 + 1;
    while x + 3 < x1.saturating_sub(4) {
        paint::fill(buf, x, shelf - 1, CREAM_DIM);
        paint::fill(buf, x + 1, shelf - 1, CREAM_DIM);
        paint::glyph_over(buf, x + 2, shelf - 1, ')', CREAM_DIM);
        x += 5;
    }
}
