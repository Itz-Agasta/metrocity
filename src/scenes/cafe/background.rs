//! Wall, counter and floor: the base layers every other component sits on.

use ratatui::buffer::Buffer;

use super::layout::Layout;
use super::paint;
use super::palette::*;

pub fn render(buf: &mut Buffer, l: &Layout) {
    draw_wall(buf, l);
    draw_counter(buf, l);
    draw_floor(buf, l);
}

fn draw_wall(buf: &mut Buffer, l: &Layout) {
    // Subtle gradient: darker at top, slightly lighter near the counter
    for y in 0..l.counter_y {
        let t = y as f32 / l.counter_y.max(1) as f32;
        let color = paint::mix(WALL_DARK, WALL, t);
        for x in 0..l.w {
            paint::fill(buf, x, y, color);
        }
    }
}

fn draw_counter(buf: &mut Buffer, l: &Layout) {
    // Counter top highlight (bright amber edge)
    for x in 0..l.w {
        paint::fill(buf, x, l.counter_y, COUNTER_TOP);
    }
    // Shadow just below the top edge
    if l.counter_y + 1 < l.floor_y {
        for x in 0..l.w {
            paint::fill(buf, x, l.counter_y + 1, COUNTER_SHADOW);
        }
    }
    // Counter body: the big front panel (solid brown)
    for y in l.counter_y + 2..l.floor_y {
        for x in 0..l.w {
            paint::fill(buf, x, y, COUNTER_BODY);
        }
    }
}

fn draw_floor(buf: &mut Buffer, l: &Layout) {
    if l.floor_y >= l.h {
        return;
    }
    // Baseboard line at the top of the floor
    for x in 0..l.w {
        paint::glyph(buf, x, l.floor_y, '─', FLOOR_LINE, FLOOR);
    }
    // Floor gradient: darker toward the bottom
    for y in l.floor_y + 1..l.h {
        let t = (y - l.floor_y) as f32 / (l.h - l.floor_y).max(1) as f32;
        let color = paint::mix(FLOOR, FLOOR_DARK, t);
        for x in 0..l.w {
            paint::fill(buf, x, y, color);
        }
    }
}
