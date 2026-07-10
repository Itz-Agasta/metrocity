//! Bar stools in front of the counter.

use ratatui::buffer::Buffer;

use crate::scenes::cafe::layout::Layout;
use crate::scenes::cafe::paint;
use crate::scenes::cafe::palette::*;

pub fn draw(buf: &mut Buffer, l: &Layout) {
    if l.stool_y + 1 >= l.h || l.stool_y <= l.counter_y {
        return;
    }
    let xs: Vec<u16> = l.stool_xs().collect();
    for cx in xs {
        // Seat: odd width so the post sits dead center under it
        for dx in 0..7 {
            paint::fill(buf, cx.saturating_sub(3) + dx, l.stool_y, STOOL_SEAT);
        }
        // Center post down to the floor
        for y in l.stool_y + 1..=l.floor_y.min(l.h - 1) {
            paint::fill(buf, cx, y, STOOL_LEG);
        }
        // Base feet, also centered on the post
        let base_y = (l.floor_y + 1).min(l.h - 1);
        for dx in 0..5 {
            paint::fill(buf, cx.saturating_sub(2) + dx, base_y, STOOL_LEG);
        }
    }
}
