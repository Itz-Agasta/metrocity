//! Low-level cell painting helpers shared by harbor components.
//! Coordinates are i32 so callers can draw shapes that run off screen.

use ratatui::buffer::Buffer;
use ratatui::style::Color;

/// Sets a glyph with explicit fg and bg; off-screen cells are ignored.
pub fn set(buf: &mut Buffer, x: i32, y: i32, ch: char, fg: Color, bg: Color) {
    if let Some(cell) = cell_mut(buf, x, y) {
        cell.set_char(ch).set_fg(fg).set_bg(bg);
    }
}

/// Fills one cell with a solid color.
pub fn fill(buf: &mut Buffer, x: i32, y: i32, color: Color) {
    set(buf, x, y, ' ', color, color);
}

/// Sets a glyph and fg, keeping the cell's current bg.
pub fn glyph(buf: &mut Buffer, x: i32, y: i32, ch: char, fg: Color) {
    if let Some(cell) = cell_mut(buf, x, y) {
        cell.set_char(ch).set_fg(fg);
    }
}

/// Blends a cell's fg and bg toward `color`, keeping its glyph.
pub fn tint(buf: &mut Buffer, x: i32, y: i32, color: Color, k: f32) {
    if let Some(cell) = cell_mut(buf, x, y) {
        let (fg, bg) = (mix(cell.fg, color, k), mix(cell.bg, color, k));
        cell.set_fg(fg).set_bg(bg);
    }
}

/// Background color of a cell (black when off screen).
pub fn bg_at(buf: &Buffer, x: i32, y: i32) -> Color {
    if x < 0 || y < 0 || x >= buf.area.width as i32 || y >= buf.area.height as i32 {
        return Color::Rgb(0, 0, 0);
    }
    buf[(x as u16, y as u16)].bg
}

fn cell_mut(buf: &mut Buffer, x: i32, y: i32) -> Option<&mut ratatui::buffer::Cell> {
    if x < 0 || y < 0 || x >= buf.area.width as i32 || y >= buf.area.height as i32 {
        return None;
    }
    buf.cell_mut((x as u16, y as u16))
}

/// Linear blend between two RGB colors.
pub fn mix(a: Color, b: Color, t: f32) -> Color {
    let (ar, ag, ab) = rgb(a);
    let (br, bg, bb) = rgb(b);
    let t = t.clamp(0.0, 1.0);
    let lerp = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t) as u8;
    Color::Rgb(lerp(ar, br), lerp(ag, bg), lerp(ab, bb))
}

/// Multiplies a color's brightness by `k`.
pub fn scale(c: Color, k: f32) -> Color {
    let (r, g, b) = rgb(c);
    let s = |v: u8| (v as f32 * k).clamp(0.0, 255.0) as u8;
    Color::Rgb(s(r), s(g), s(b))
}

pub fn rgb(c: Color) -> (u8, u8, u8) {
    match c {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => (0, 0, 0),
    }
}

/// Small deterministic hash for animation jitter (stable per frame/key).
pub fn hash(a: u32, b: u32) -> u32 {
    let mut h = a.wrapping_mul(0x9E37_79B1) ^ b.wrapping_mul(0x85EB_CA77);
    h ^= h >> 16;
    h = h.wrapping_mul(0x85EB_CA6B);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B2_AE35);
    h ^ (h >> 16)
}
