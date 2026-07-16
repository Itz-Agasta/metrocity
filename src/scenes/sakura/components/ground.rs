//! Foreground grass bank the tree, torii and lantern stand on.

use ratatui::buffer::Buffer;

use super::super::layout::Layout;
use super::super::paint;
use super::super::palette::*;

pub fn draw(buf: &mut Buffer, l: &Layout) {
    for y in l.ground_y..l.h {
        let depth = f32::from(y - l.ground_y) / f32::from((l.h - l.ground_y).max(1));
        for x in 0..l.w {
            let n = (paint::hash(u32::from(x), u32::from(y)) % 64) as f32 / 64.0 - 0.5;
            let c = paint::mix(GRASS, GRASS_DARK, depth + n * 0.15);
            paint::fill(buf, x, y, c);
        }
    }
}
