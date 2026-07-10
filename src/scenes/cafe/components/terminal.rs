//! Retro CRT terminal sitting on the counter, with blinking cursor.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use crate::scenes::cafe::layout::Layout;
use crate::scenes::cafe::paint;
use crate::scenes::cafe::palette::*;

const LINES: [&str; 3] = ["WELCOME TO", "CAT CAFE TERMINAL", "> ORDER"];

pub fn draw(buf: &mut Buffer, l: &Layout, t: f64) {
    let r = l.terminal;
    if r.width < 21 || r.height < 6 {
        return;
    }
    paint::fill_rect(buf, r, CRT_BEZEL);
    let screen = Rect::new(r.x + 1, r.y + 1, r.width - 2, r.height - 2);
    paint::fill_rect(buf, screen, CRT_BG);

    for (i, line) in LINES.iter().enumerate() {
        let y = screen.y + i as u16;
        if y < screen.bottom() {
            paint::text(buf, screen.x + 1, y, line, CRT_GREEN, CRT_BG);
        }
    }
    // Blinking block cursor under the prompt
    if t % 1.0 < 0.6 {
        let y = screen.y + LINES.len() as u16;
        if y < screen.bottom() {
            paint::glyph(buf, screen.x + 1, y, '▉', CRT_GREEN, CRT_BG);
        }
    }
}
