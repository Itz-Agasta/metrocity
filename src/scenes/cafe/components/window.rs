//! Window: wooden frame, night sky with stars, city silhouette, animated rain.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use crate::scenes::cafe::layout::Layout;
use crate::scenes::cafe::paint;
use crate::scenes::cafe::palette::*;

const RAIN_DROPS: u32 = 22;

pub fn draw(buf: &mut Buffer, l: &Layout, t: f64) {
    let r = l.window;
    if r.width < 10 || r.height < 8 {
        return;
    }
    paint::fill_rect(buf, r, WOOD);
    let inner = Rect::new(r.x + 1, r.y + 1, r.width - 2, r.height - 2);

    draw_sky(buf, inner);
    draw_city(buf, inner);
    draw_rain(buf, inner, t);
    draw_mullion(buf, r);
}

fn draw_sky(buf: &mut Buffer, inner: Rect) {
    for y in inner.top()..inner.bottom() {
        let f = (y - inner.y) as f32 / inner.height.max(1) as f32;
        let color = paint::mix(NIGHT_DEEP, NIGHT, f);
        for x in inner.left()..inner.right() {
            paint::fill(buf, x, y, color);
            // Sparse stars in the upper half
            if y < inner.y + inner.height / 2 && paint::hash(x.into(), y.into()) % 41 == 0 {
                paint::glyph(buf, x, y, '·', RAIN, color);
            }
        }
    }
}

fn draw_city(buf: &mut Buffer, inner: Rect) {
    let max_h = (inner.height / 3).max(2);
    for x in inner.left()..inner.right() {
        // Blocky skyline: one height per 4-column building
        let block = u32::from(x - inner.x) / 4;
        let bh = 1 + paint::hash(block, 0xC17) % u32::from(max_h);
        for dy in 0..bh as u16 {
            let y = inner.bottom() - 1 - dy;
            paint::fill(buf, x, y, CITY);
            // A few lit windows
            if paint::hash(x.into(), y.into()) % 13 == 0 {
                paint::glyph(buf, x, y, '▪', CITY_LIT, CITY);
            }
        }
    }
}

fn draw_rain(buf: &mut Buffer, inner: Rect, t: f64) {
    for k in 0..RAIN_DROPS {
        let col = inner.x + (paint::hash(k, 0x5A1) % u32::from(inner.width)) as u16;
        let speed = 7.0 + f64::from(k % 5);
        let phase = f64::from(paint::hash(k, 0xF00) % 97);
        let y = inner.y + ((t * speed + phase) % f64::from(inner.height)) as u16;
        paint::glyph_over(buf, col, y, '╱', RAIN);
    }
}

fn draw_mullion(buf: &mut Buffer, r: Rect) {
    let mid_x = r.x + r.width / 2;
    let mid_y = r.y + r.height / 2;
    for y in r.top()..r.bottom() {
        paint::fill(buf, mid_x, y, WOOD_DARK);
    }
    for x in r.left()..r.right() {
        paint::fill(buf, x, mid_y, WOOD_DARK);
    }
}
