//! Puffy clouds drifting across the sky at different heights and speeds.

use ratatui::buffer::Buffer;

use crate::scenes::meadow::layout::Layout;
use crate::scenes::meadow::paint;
use crate::scenes::meadow::palette::*;

pub fn draw(buf: &mut Buffer, l: &Layout, t: f64) {
    // A handful of puffy clouds at different heights, each drifting right at
    // its own speed so they parallax past one another and wrap around.
    // (row fraction of sky, width, speed in cells/sec, phase offset)
    let clouds = [
        (0.14_f32, 9u16, 1.1_f64, 0.0_f32),
        (0.28, 7, 1.8, 0.35),
        (0.10, 11, 0.7, 0.60),
        (0.36, 6, 2.4, 0.15),
        (0.22, 8, 1.4, 0.80),
    ];
    let sky = f32::from(l.horizon_y.max(1));
    for &(fy, cw, speed, phase) in &clouds {
        let cy = (sky * fy) as u16 + 1;
        let span = i64::from(l.w) + i64::from(cw) * 2;
        let start = (phase * span as f32) as i64;
        let x0 = ((start + (t * speed) as i64) % span + span) % span - i64::from(cw);
        puff(buf, l, x0, cy, cw);
    }
}

/// One rounded cloud: a full bottom row with a shorter, inset upper row.
fn puff(buf: &mut Buffer, l: &Layout, x0: i64, cy: u16, cw: u16) {
    for dx in 0..cw {
        let x = x0 + i64::from(dx);
        if x < 0 || x >= i64::from(l.w) {
            continue;
        }
        let x = x as u16;
        // Lit from above: bright bumps on top, a softly shaded base row.
        let edge = dx == 0 || dx == cw - 1;
        let base = if edge {
            CLOUD_SHADE
        } else {
            paint::mix(CLOUD, CLOUD_SHADE, 0.55)
        };
        paint::fill(buf, x, cy, base);
        // Upper bumps, inset from the ends so the top reads as rounded.
        if dx >= 2 && dx + 2 < cw {
            paint::fill(buf, x, cy.saturating_sub(1), CLOUD);
        }
    }
}
