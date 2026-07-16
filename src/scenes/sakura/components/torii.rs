//! Torii gate on the right bank: two red posts, double lintel with upturned
//! ends, small tablet in the middle.

use ratatui::buffer::Buffer;

use super::super::layout::Layout;
use super::super::paint;
use super::super::palette::*;

pub fn draw(buf: &mut Buffer, l: &Layout) {
    let t = l.torii;
    if t.height < 5 || t.width < 6 {
        return;
    }
    let y0 = t.top();

    // Top lintel: full width plus overhang, dark cap over a red beam.
    for x in t.left().saturating_sub(1)..=(t.right()).min(l.w.saturating_sub(1)) {
        paint::fill(buf, x, y0, TORII_DARK);
    }
    paint::glyph_over(buf, t.left().saturating_sub(2), y0, '▄', TORII_DARK);
    paint::glyph_over(buf, t.right() + 1, y0, '▄', TORII_DARK);
    for x in t.left()..t.right().min(l.w) {
        paint::fill(buf, x, y0 + 1, TORII_RED);
    }

    // Second beam between the posts, with the tablet hanging from it.
    let beam_y = y0 + 3;
    for x in (t.left() + 1)..(t.right().saturating_sub(1)).min(l.w) {
        paint::fill(buf, x, beam_y, TORII_RED);
    }
    let cx = t.left() + t.width / 2;
    paint::fill(buf, cx, y0 + 2, TORII_RED);
    paint::glyph_over(buf, cx, y0 + 2, '≡', TORII_DARK);

    // Posts, slightly inset, two cells wide, sunk into the grass.
    for py in (y0 + 2)..t.bottom().min(l.h) {
        for (px, shade) in [
            (t.left() + 1, TORII_RED),
            (t.left() + 2, TORII_DARK),
            (t.right().saturating_sub(3), TORII_RED),
            (t.right().saturating_sub(2), TORII_DARK),
        ] {
            if py == y0 + 2 {
                continue; // gap row between the two lintels
            }
            paint::fill(buf, px, py, shade);
        }
    }
}
