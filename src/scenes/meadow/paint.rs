//! Low-level cell painting helpers shared by meadow components.

use ratatui::buffer::Buffer;
use ratatui::style::Color;

/// Fills one cell with a solid color (space char, fg == bg).
pub fn fill(buf: &mut Buffer, x: u16, y: u16, color: Color) {
    if x < buf.area.width && y < buf.area.height {
        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_char(' ').set_fg(color).set_bg(color);
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

/// Darkens the cell's current background toward a deep grass green: a soft
/// shadow that works over whatever is already painted there.
pub fn shade(buf: &mut Buffer, x: u16, y: u16, amount: f32) {
    if x < buf.area.width && y < buf.area.height {
        if let Some(cell) = buf.cell_mut((x, y)) {
            let c = mix(cell.bg, Color::Rgb(34, 46, 30), amount);
            cell.set_char(' ').set_fg(c).set_bg(c);
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

fn rgb(c: Color) -> (u8, u8, u8) {
    match c {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => (0, 0, 0),
    }
}

/// Small deterministic hash for stable per-cell variation.
pub fn hash(a: u32, b: u32) -> u32 {
    let mut h = a.wrapping_mul(0x9E37_79B9) ^ b.wrapping_mul(0x85EB_CA6B);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B2_AE35);
    h ^ (h >> 16)
}
