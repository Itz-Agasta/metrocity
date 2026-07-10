use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Style;

use crate::scenes::cafe::layout::Layout;

const BG: Color = Color::Rgb(25, 20, 18);
const FRAME: Color = Color::Rgb(100, 55, 28);
const BG_DARK: Color = Color::Rgb(15, 12, 10);
// const FRAME_DARK: Color = Color::Rgb(60, 33, 16);
const FRAME_DARK: Color = Color::Rgb(54, 29, 21);

const SIGN_H: u16 = 10;

/// Cells on the front sign face where the paw-print sprite sits,
/// centered under the "FREE MEOW-FI" text.
pub fn paw_slot(l: &Layout) -> Rect {
    let sx = l.w.saturating_sub(24);
    let sy = (l.floor_y + 4).saturating_sub(SIGN_H);
    Rect::new(sx + 7, sy + 6, 4, 2)
}

/// Draws both parallelogram signs (dark behind, bright on top).
pub fn draw(buf: &mut Buffer, l: &Layout) {
    let base_sx = l.w.saturating_sub(24);
    let sy = (l.floor_y + 4).saturating_sub(SIGN_H);

    // Back darker parallelogram
    draw_sign(buf, base_sx, sy, BG_DARK, FRAME_DARK, 15, 4, |row| row / 2);
    draw_leg(buf, base_sx, sy, FRAME_DARK, 4, 0);
    draw_leg(buf, base_sx, sy, FRAME_DARK, 4, 18);

    // Front brighter parallelogram
    draw_sign(buf, base_sx, sy, BG, FRAME, 16, 0, |row| {
        (SIGN_H - 1 - row) / 2
    });
    draw_leg(buf, base_sx, sy, FRAME, 0, 0);
    draw_leg(buf, base_sx, sy, FRAME_DARK, 0, 1);
    draw_leg(buf, base_sx, sy, FRAME, 0, 15);
    draw_leg(buf, base_sx, sy, FRAME_DARK, 0, 16);

    // Free Meow-fi logo
    draw_logo(buf, base_sx, sy, 16, 2);
}

/// Draws "FREE MEOW-FI" text inside the sign body.
fn draw_logo(buf: &mut Buffer, sx: u16, sy: u16, w: u16, x_off: u16) {
    let text_color = Color::Rgb(220, 180, 120);
    let sign_sx = sx + x_off;
    let style = Style::default().fg(text_color).bg(BG);

    // "FREE" centered, 2 rows down from top of sign
    let free_x = sign_sx + (w - 4) / 2;
    buf.set_string(free_x, sy + 2, "FREE", style);

    // "MEOW-FI" centered, 4 rows down
    let meow_x = sign_sx + (w - 7) / 2;
    buf.set_string(meow_x, sy + 4, "MEOW-FI", style);

    // Paw sprite sits below, see paw_slot()
}

#[allow(clippy::too_many_arguments)]
fn draw_sign(
    buf: &mut Buffer,
    sx: u16,
    sy: u16,
    bg: Color,
    frame: Color,
    w: u16,
    x_off: u16,
    shift_fn: impl Fn(u16) -> u16,
) {
    let sx = sx + x_off;

    if sx + w > buf.area.width || sy + SIGN_H > buf.area.height {
        return;
    }

    for row in 0..SIGN_H {
        let abs_y = sy + row;
        if abs_y >= buf.area.height {
            break;
        }
        let shift = shift_fn(row);
        for col in 0..w {
            let abs_x = sx + col + shift;
            if abs_x >= buf.area.width {
                break;
            }
            let color = if row == 0 || row == SIGN_H - 1 || col == 0 || col == w - 1 {
                frame
            } else {
                bg
            };
            if let Some(cell) = buf.cell_mut((abs_x, abs_y)) {
                cell.set_char(' ');
                cell.set_style(Style::default().bg(color));
            }
        }
    }
}

fn draw_leg(buf: &mut Buffer, sx: u16, sy: u16, color: Color, x_off: u16, leg_col: u16) {
    let abs_x = sx + x_off + leg_col;
    let leg_y = sy + SIGN_H;
    if leg_y < buf.area.height && abs_x < buf.area.width {
        if let Some(cell) = buf.cell_mut((abs_x, leg_y)) {
            cell.set_char(' ');
            cell.set_style(Style::default().bg(color));
        }
    }
}
