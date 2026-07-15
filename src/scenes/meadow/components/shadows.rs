//! Soft pools of shade under the tree, the sunflower patch, the windmill and
//! the animals, so everything stands on the grass instead of floating over it.

use ratatui::buffer::Buffer;

use crate::scenes::meadow::layout::Layout;
use crate::scenes::meadow::paint;

/// `animals`: (center col, half width) per critter, from the sprite layer.
pub fn draw(buf: &mut Buffer, l: &Layout, animals: &[(u16, u16)]) {
    // Tree: a wide pool around the trunk base, nudged left since the sun sits
    // in the upper right.
    let tx = i32::from(l.trunk.x) + i32::from(l.trunk.width) / 2;
    pool(buf, tx - 3, i32::from(l.ground_y) + 1, f32::from(l.canopy_rx) * 0.55, 2.4, 0.16);
    // Sunflower patch: covers both clusters at their base row.
    let s = l.sunflowers;
    pool(buf, i32::from(s.x), i32::from(s.bottom()), 11.0, 1.4, 0.14);
    // Sprite shadows only when the Kitty layer exists (empty `animals` means
    // no sprites, so no windmill either -- don't shade under nothing).
    if animals.is_empty() {
        return;
    }
    // Windmill: far away, so the pool is small and faint. Wider than the
    // sprite so it peeks out at the sides of the base.
    let m = l.windmill;
    pool(buf, i32::from(m.x) + i32::from(m.width) / 2, i32::from(m.bottom()), 6.0, 1.0, 0.10);
    // The friends, at their feet; follows them as they wander.
    for &(cx, hw) in animals {
        pool(buf, i32::from(cx), i32::from(l.ground_y), f32::from(hw) + 1.5, 1.3, 0.15);
    }
}

/// One elliptical pool of shade with a hash-ragged edge, strongest at the
/// center and fading out. paint::shade bounds-checks every cell.
fn pool(buf: &mut Buffer, cx: i32, cy: i32, rx: f32, ry: f32, amount: f32) {
    if rx <= 0.0 || ry <= 0.0 {
        return;
    }
    let (x0, x1) = (cx - rx as i32 - 1, cx + rx as i32 + 1);
    let (y0, y1) = (cy - ry as i32 - 1, cy + ry as i32 + 1);
    for y in y0..=y1 {
        for x in x0..=x1 {
            if x < 0 || y < 0 {
                continue;
            }
            let dx = (x - cx) as f32 / rx;
            let dy = (y - cy) as f32 / ry;
            let d = dx * dx + dy * dy;
            let n = (paint::hash(x as u32, (y as u32).wrapping_mul(5)) % 256) as f32 / 256.0;
            if d <= 1.0 + (n - 0.5) * 0.35 {
                paint::shade(buf, x as u16, y as u16, amount * (1.0 - d * 0.6));
            }
        }
    }
}
