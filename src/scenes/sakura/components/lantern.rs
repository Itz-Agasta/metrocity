//! Stone lantern at the bank edge, warm flickering core.

use ratatui::buffer::Buffer;

use super::super::layout::Layout;
use super::super::paint;
use super::super::palette::*;

pub fn draw(buf: &mut Buffer, l: &Layout, t: f64) {
    let x = l.lantern_x;
    // Fully on the bank: base a third of the way down the grass.
    let base_y = l.ground_y + (l.h - l.ground_y) / 3 + 1;
    if base_y < 7 || x + 3 >= l.w {
        return;
    }

    let flick = 0.8 + 0.2 * (t * 2.3).sin() as f32;
    let glow = paint::mix(LANTERN_STONE, LANTERN_GLOW, flick);

    // Finial, cap, light chamber (2 wide), pillar, base.
    paint::glyph_over(buf, x + 1, base_y - 6, '▄', LANTERN_STONE);
    paint::glyph_over(buf, x + 2, base_y - 6, '▄', LANTERN_STONE);
    for dx in 0..4 {
        paint::glyph_over(buf, x + dx, base_y - 5, '▄', LANTERN_STONE);
    }
    paint::glyph_over(buf, x, base_y - 4, '▐', LANTERN_STONE);
    paint::fill(buf, x + 1, base_y - 4, glow);
    paint::fill(buf, x + 2, base_y - 4, glow);
    paint::glyph_over(buf, x + 3, base_y - 4, '▌', LANTERN_STONE);
    paint::glyph_over(buf, x + 1, base_y - 3, '█', LANTERN_STONE);
    paint::glyph_over(buf, x + 2, base_y - 3, '█', LANTERN_STONE);
    paint::glyph_over(buf, x + 1, base_y - 2, '█', LANTERN_STONE);
    paint::glyph_over(buf, x + 2, base_y - 2, '█', LANTERN_STONE);
    for dx in 0..4 {
        paint::glyph_over(buf, x + dx, base_y - 1, '▀', LANTERN_STONE);
    }

    // Warm spill onto the grass around it.
    let spill = paint::mix(GRASS, LANTERN_GLOW, 0.12 * flick);
    for dx in 0..6u16 {
        paint::fill(buf, (x + dx).saturating_sub(1), base_y, spill);
    }
}
