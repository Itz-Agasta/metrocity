//! Green leaves shed from the tree, blown toward the animals on the breeze.

use ratatui::buffer::Buffer;

use crate::scenes::meadow::layout::Layout;
use crate::scenes::meadow::paint;
use crate::scenes::meadow::palette::*;

pub fn draw(buf: &mut Buffer, l: &Layout, t: f64) {
    if l.canopy_rx == 0 {
        return;
    }
    let top = l.canopy_cy;
    let bottom = (l.horizon_y + 5).min(l.h.saturating_sub(1));
    if bottom <= top {
        return;
    }
    let span = f32::from(bottom - top);
    // Drop points spread across the whole visible canopy (both sides of trunk).
    let canopy_r = (l.canopy_cx + l.canopy_rx).min(l.w);
    // A gusting breeze blowing to the right (toward the group).
    let gust = 14.0 * (1.0 + 0.3 * (t as f32 * 0.35).sin());
    let n = 6u16;
    for k in 0..n {
        let ph = ((t * 0.06 + f64::from(k) * 0.17) % 1.0) as f32;
        let y = top + (ph * span) as u16;
        let spawn = f32::from(k) / f32::from(n) * f32::from(canopy_r);
        let flutter = ((t * 1.8) as f32 + k as f32 * 1.7 + ph * 8.0).sin() * 2.5;
        let x = spawn + ph * gust + flutter;
        if x < 0.0 || x as u16 >= l.w || y >= l.h {
            continue;
        }
        // Green, to match the tree; a solid cell so it renders in any font.
        let color = if k % 2 == 0 { LEAF_LIGHT } else { LEAF };
        paint::fill(buf, x as u16, y, color);
    }
}
