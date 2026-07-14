//! Distant cabin with a chimney and wispy smoke curling into the sky.

use ratatui::buffer::Buffer;

use crate::scenes::meadow::layout::Layout;
use crate::scenes::meadow::paint;
use crate::scenes::meadow::palette::*;

pub fn draw(buf: &mut Buffer, l: &Layout, t: f64) {
    let c = l.cabin;
    // paint::fill bounds-checks every cell, so no explicit fit guard is needed.
    // Body.
    for y in c.top() + 2..c.bottom() {
        for x in c.left()..c.right() {
            paint::fill(buf, x, y, CABIN);
        }
    }
    // Pitched roof.
    let mid = c.left() + c.width / 2;
    for x in c.left()..c.right() {
        let dist = x.abs_diff(mid);
        let ry = c.top() + 2 - (c.width / 2 - dist).min(2);
        for yy in ry..c.top() + 2 {
            paint::fill(buf, x, yy, CABIN_ROOF);
        }
    }
    // Door.
    paint::fill(buf, mid, c.bottom() - 1, CABIN_DOOR);
    paint::fill(buf, mid, c.bottom() - 2, CABIN_DOOR);
    // Chimney on the right of the roof (smoke rises from here).
    let chim_x = c.right().saturating_sub(2);
    for y in c.top().saturating_sub(1)..c.top() + 1 {
        paint::fill(buf, chim_x, y, CHIMNEY);
    }

    smoke(buf, l, t);
}

/// Wispy smoke curling up from the chimney and fading into the sky.
fn smoke(buf: &mut Buffer, l: &Layout, t: f64) {
    let c = l.cabin;
    if c.top() < 3 {
        return;
    }
    let chim_x = c.right().saturating_sub(2);
    let base = c.top().saturating_sub(2); // just above the chimney
    let rise = base.min(9); // rows the smoke climbs before dissipating
    let puffs = 7;
    for k in 0..puffs {
        // Slow: a puff takes ~13s to climb and dissipate.
        let ph = ((t * 0.075 + k as f64 / f64::from(puffs)) % 1.0) as f32;
        let y = base.saturating_sub((ph * f32::from(rise)) as u16);
        // curl gently to the right and wobble as it rises
        let drift = ph * 3.0 + ((t * 0.3) as f32 + k as f32).sin() * 1.0;
        let x = f32::from(chim_x) + drift;
        if x < 0.0 || x as u16 >= l.w || y >= l.horizon_y {
            continue;
        }
        // fade from grey toward the sky colour at this row as it thins out
        let sky = paint::mix(
            SKY_TOP,
            SKY_HORIZON,
            f32::from(y) / f32::from(l.horizon_y.max(1)),
        );
        paint::fill(buf, x as u16, y, paint::mix(SMOKE, sky, ph));
    }
}
