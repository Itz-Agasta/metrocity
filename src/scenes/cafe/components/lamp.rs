//! Hanging lamp with a warm glow blended into the wall behind it.

use ratatui::buffer::Buffer;

use crate::scenes::cafe::layout::Layout;
use crate::scenes::cafe::paint;
use crate::scenes::cafe::palette::*;

const BULB_Y: u16 = 4;
const GLOW_RADIUS: f32 = 7.0;

pub fn draw(buf: &mut Buffer, l: &Layout) {
    let x = l.lamp_x;
    if x + 2 >= l.w || l.counter_y <= BULB_Y + 2 {
        return;
    }

    // Warm glow first, so the fixture draws on top of it
    for y in 0..l.counter_y {
        for dx in -10i32..=10 {
            let gx = i32::from(x) + dx;
            if gx < 0 || gx >= i32::from(l.w) {
                continue;
            }
            // Halve dx: terminal cells are roughly twice as tall as wide
            let dxf = dx as f32 / 2.0;
            let dyf = f32::from(y) - f32::from(BULB_Y);
            let dist = (dxf * dxf + dyf * dyf).sqrt();
            if dist < GLOW_RADIUS {
                let f = (1.0 - dist / GLOW_RADIUS) * 0.30;
                paint::tint_bg(buf, gx as u16, y, AMBER, f);
            }
        }
    }

    // Cord, shade, bulb
    for y in 0..BULB_Y - 1 {
        paint::glyph_over(buf, x, y, '│', WOOD_DARK);
    }
    for dx in 0..3 {
        paint::glyph_over(buf, x - 1 + dx, BULB_Y - 1, '▄', WOOD_DARK);
    }
    paint::glyph_over(buf, x, BULB_Y, '◉', AMBER);
}
