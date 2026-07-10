//! Small counter props: the steaming coffee mug.
//! (The billing machine next to the terminal is a sprite, see sprites/decor.rs.)

use ratatui::buffer::Buffer;

use crate::scenes::cafe::layout::Layout;
use crate::scenes::cafe::paint;
use crate::scenes::cafe::palette::*;

pub fn draw(buf: &mut Buffer, l: &Layout, t: f64) {
    if l.counter_y < 4 {
        return;
    }
    draw_mug(buf, l, t);
}

fn draw_mug(buf: &mut Buffer, l: &Layout, t: f64) {
    let x = l.mug_x;
    let y = l.counter_y - 1;
    paint::fill(buf, x, y, MUG);
    paint::fill(buf, x + 1, y, MUG);
    paint::glyph_over(buf, x + 2, y, ')', MUG);
    // Wafting steam: two rows drifting with time
    let sway = ((t * 2.0) as u32 % 2) as u16;
    paint::glyph_over(buf, x + sway, y - 1, '(', STEAM);
    paint::glyph_over(buf, x + 1 - sway, y - 2, ')', STEAM);
}
