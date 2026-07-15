//! The big tree: trunk, a branch reaching out to hold the hive, and a lumpy
//! elliptical canopy centered over the trunk.

use ratatui::buffer::Buffer;

use crate::scenes::meadow::layout::Layout;
use crate::scenes::meadow::paint;
use crate::scenes::meadow::palette::*;

pub fn draw(buf: &mut Buffer, l: &Layout) {
    if l.canopy_rx == 0 || l.canopy_ry == 0 {
        return;
    }
    // Trunk first, with a shaded right edge, so the canopy overlaps its top.
    let tr = l.trunk;
    for y in tr.top()..tr.bottom().min(l.h) {
        for x in tr.left()..tr.right() {
            let shade = x == tr.right() - 1;
            paint::fill(buf, x, y, if shade { TRUNK_DARK } else { TRUNK });
        }
    }
    // A short branch poking out of the foliage to hold the hive. Only the part
    // from the leaf edge outward is drawn, so it never slices the canopy.
    let leaf_edge = l.canopy_right(l.branch_y).unwrap_or(l.canopy_cx);
    for x in leaf_edge.saturating_sub(1)..=l.branch_x_end {
        paint::glyph_over(buf, x, l.branch_y, '━', TRUNK_DARK);
    }
    // Lumpy elliptical canopy, dappled with three leaf tones, drawn on top so
    // the branch appears to emerge from behind the leaves. Only the on-screen
    // part of the ellipse is scanned; the left side may run off the edge.
    let y0 = l.canopy_cy.saturating_sub(l.canopy_ry);
    let y1 = (l.canopy_cy + l.canopy_ry).min(l.h.saturating_sub(1));
    let x1 = (l.canopy_cx + l.canopy_rx).min(l.w.saturating_sub(1));
    let (cx, cy) = (f32::from(l.canopy_cx), f32::from(l.canopy_cy));
    let (rx, ry) = (f32::from(l.canopy_rx), f32::from(l.canopy_ry));
    for y in y0..=y1 {
        for x in 0..=x1 {
            // Ragged leaf edge: jitter the ellipse boundary per cell so the
            // silhouette (especially the bottom) breaks up instead of reading
            // as a razor-clean curve. Interior cells always fill.
            let dx = (f32::from(x) - cx) / rx;
            let dy = (f32::from(y) - cy) / ry;
            let n = (paint::hash(u32::from(x), u32::from(y).wrapping_mul(7)) % 256) as f32 / 256.0;
            if dx * dx + dy * dy <= 1.0 + (n - 0.5) * 0.22 {
                let h = paint::hash(u32::from(x), u32::from(y)) % 10;
                let leaf = if h < 2 {
                    LEAF_LIGHT
                } else if h < 7 {
                    LEAF
                } else {
                    LEAF_DARK
                };
                paint::fill(buf, x, y, leaf);
            }
        }
    }
}
