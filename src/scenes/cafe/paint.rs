//! Low-level cell painting helpers shared by cafe components.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;

/// Fills one cell with a solid color (space char, fg == bg).
pub fn fill(buf: &mut Buffer, x: u16, y: u16, color: Color) {
    if x < buf.area.width && y < buf.area.height {
        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_char(' ').set_fg(color).set_bg(color);
        }
    }
}

/// Fills every cell of `rect` with a solid color.
pub fn fill_rect(buf: &mut Buffer, rect: Rect, color: Color) {
    for y in rect.top()..rect.bottom() {
        for x in rect.left()..rect.right() {
            fill(buf, x, y, color);
        }
    }
}

/// Sets a char with fg over an explicit bg.
pub fn glyph(buf: &mut Buffer, x: u16, y: u16, ch: char, fg: Color, bg: Color) {
    if x < buf.area.width && y < buf.area.height {
        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_char(ch).set_fg(fg).set_bg(bg);
        }
    }
}

/// Sets a char with fg, keeping whatever bg the cell already has.
pub fn glyph_over(buf: &mut Buffer, x: u16, y: u16, ch: char, fg: Color) {
    if x < buf.area.width && y < buf.area.height {
        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_char(ch).set_fg(fg);
        }
    }
}

/// Writes a string with fg over an explicit bg.
pub fn text(buf: &mut Buffer, x: u16, y: u16, s: &str, fg: Color, bg: Color) {
    for (i, ch) in s.chars().enumerate() {
        glyph(buf, x.saturating_add(i as u16), y, ch, fg, bg);
    }
}

/// Blends the cell's bg (and solid-fill fg) toward `target` by factor `f` (0..1).
pub fn tint_bg(buf: &mut Buffer, x: u16, y: u16, target: Color, f: f32) {
    if x >= buf.area.width || y >= buf.area.height {
        return;
    }
    if let Some(cell) = buf.cell_mut((x, y)) {
        let bg = cell.bg;
        let solid = cell.fg == bg;
        let blended = mix(bg, target, f);
        cell.set_bg(blended);
        if solid {
            cell.set_fg(blended);
        }
    }
}

/// Linear blend between two RGB colors.
pub fn mix(a: Color, b: Color, t: f32) -> Color {
    let (ar, ag, ab) = rgb(a);
    let (br, bg, bb) = rgb(b);
    let t = t.clamp(0.0, 1.0);
    Color::Rgb(
        (ar as f32 + (br as f32 - ar as f32) * t) as u8,
        (ag as f32 + (bg as f32 - ag as f32) * t) as u8,
        (ab as f32 + (bb as f32 - ab as f32) * t) as u8,
    )
}

pub fn rgb(c: Color) -> (u8, u8, u8) {
    match c {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => (0, 0, 0),
    }
}

/// Small deterministic hash for animation jitter (stable per frame/key).
pub fn hash(a: u32, b: u32) -> u32 {
    let mut h = a.wrapping_mul(0x9E37_79B9) ^ b.wrapping_mul(0x85EB_CA6B);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B2_AE35);
    h ^ (h >> 16)
}
