//! "CAT CAFE" neon lettering under the sprite cat face, with shared flicker.

use ratatui::buffer::Buffer;
use ratatui::style::{Modifier, Style};

use crate::scenes::cafe::layout::Layout;
use crate::scenes::cafe::paint;
use crate::scenes::cafe::palette::*;

/// Shared flicker clock: true while the neon sign momentarily dims.
/// Used by both the text here and the sprite face placement.
pub fn is_dim(t: f64) -> bool {
    let phase = (t * 6.0) as u32;
    paint::hash(phase, 0x0E01) % 19 == 0
}

pub fn draw(buf: &mut Buffer, l: &Layout, t: f64) {
    let fg = if is_dim(t) { AMBER_DIM } else { AMBER };
    let style = Style::default().fg(fg).add_modifier(Modifier::BOLD);
    let cx = l.neon_face.x + l.neon_face.width / 2;
    for (i, word) in ["C A T", "C A F E"].iter().enumerate() {
        let x = cx.saturating_sub(word.len() as u16 / 2);
        let y = l.neon_text_y + i as u16;
        if y < l.counter_y && x + word.len() as u16 <= l.w {
            buf.set_string(x, y, *word, style);
        }
    }
}
