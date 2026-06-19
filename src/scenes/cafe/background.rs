use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;

use crate::scenes::city::utils::safe_set_char;

// Hardcoded cafe palette
// Wall
const WALL: Color = Color::Rgb(54, 31, 31);
const WALL_DARK: Color = Color::Rgb(30, 18, 18);
// Counter
const COUNTER_TOP: Color = Color::Rgb(178, 101, 49);
const COUNTER_BODY: Color = Color::Rgb(85, 43, 30);
const COUNTER_SHADOW: Color = Color::Rgb(60, 30, 20);
// Floor
const FLOOR: Color = Color::Rgb(45, 22, 15);
const FLOOR_DARK: Color = Color::Rgb(30, 15, 10);
const FLOOR_LINE: Color = Color::Rgb(65, 35, 20);

// Layout constants (fraction of height)
// Wall:     rows 0..COUNTER_TOP_ROW
// Counter:  COUNTER_TOP_ROW (highlight) + 1 (shadow) + body down to FLOOR_ROW
// Floor:    FLOOR_ROW .. HEIGHT
const COUNTER_TOP_FRAC: f32 = 0.60; // counter top edge at 60%
const FLOOR_FRAC: f32 = 0.85; // floor starts at 85%

/// Returns the row index where the counter top edge sits.
pub fn counter_row(height: u16) -> u16 {
    (height as f32 * COUNTER_TOP_FRAC) as u16
}

/// Returns the row index where the floor begins.
pub fn floor_row(height: u16) -> u16 {
    (height as f32 * FLOOR_FRAC) as u16
}

// Public render entry point
pub fn render(area: Rect, buf: &mut Buffer) {
    let w = area.width;
    let h = area.height;
    if w == 0 || h == 0 {
        return;
    }
    let x0 = area.x;
    let y0 = area.y;

    let counter_y = counter_row(h);
    let floor_y = floor_row(h);

    draw_wall(x0, y0, w, counter_y, buf);
    draw_counter(x0, y0, w, counter_y, floor_y, buf);
    draw_floor(x0, y0, w, h, floor_y, buf);

    // Components
    super::components::signboard::draw(buf, x0, y0, w, floor_y);

    // TODO: draw window (right side, night sky + rain + city silhouette)
    // TODO: draw shelves on left wall (horizontal lines with mugs)
    // TODO: draw menu board (center, framed "MENU" + items + paw print)
    // TODO: draw neon cat sign (left side, cat face + "CAT CAFE")
    // TODO: draw hanging lamp (right side, warm glow)
    // TODO: draw bar stools (below counter, evenly spaced)
    // TODO: draw plants (left + right, green foliage)
    // TODO: draw terminal on counter ("WELCOME TO CAT CAFE TERMINAL")
    // TODO: draw pastry display (right side, cakes/cookies)
    // TODO: draw "FREE MEOW-FI" sign (bottom right)
}

// Wall
fn draw_wall(x0: u16, y0: u16, w: u16, counter_y: u16, buf: &mut Buffer) {
    for y in 0..counter_y {
        for x in 0..w {
            // Subtle gradient: darker at top, slightly lighter near counter
            let t = y as f32 / counter_y.max(1) as f32;
            let (r, g, b) = lerp(WALL_DARK, WALL, t);
            fill_cell(buf, x0 + x, y0 + y, r, g, b);
        }
    }
}

// Counter
fn draw_counter(x0: u16, y0: u16, w: u16, counter_y: u16, floor_y: u16, buf: &mut Buffer) {
    // Counter top highlight (bright amber edge)
    for x in 0..w {
        fill_cell_from_color(buf, x0 + x, y0 + counter_y, COUNTER_TOP);
    }
    // Shadow just below the top edge
    if counter_y + 1 < floor_y {
        for x in 0..w {
            fill_cell_from_color(buf, x0 + x, y0 + counter_y + 1, COUNTER_SHADOW);
        }
    }
    // Counter body — the big front panel (solid brown)
    for y in counter_y + 2..floor_y {
        for x in 0..w {
            fill_cell_from_color(buf, x0 + x, y0 + y, COUNTER_BODY);
        }
    }
}

// Floor
fn draw_floor(x0: u16, y0: u16, w: u16, h: u16, floor_y: u16, buf: &mut Buffer) {
    if floor_y >= h {
        return;
    }

    // Baseboard line at the top of the floor
    for x in 0..w {
        safe_set_char(buf, x0 + x, y0 + floor_y, '─', FLOOR_LINE);
    }

    // Floor gradient — darker toward the bottom
    for y in floor_y + 1..h {
        for x in 0..w {
            let t = (y - floor_y) as f32 / (h - floor_y).max(1) as f32;
            let (r, g, b) = lerp(FLOOR, FLOOR_DARK, t);
            fill_cell(buf, x0 + x, y0 + y, r, g, b);
        }
    }

    // TODO: draw floor tiles pattern (alternating subtle shade)
}

// Helpers
fn fill_cell(buf: &mut Buffer, x: u16, y: u16, r: u8, g: u8, b: u8) {
    if x < buf.area.width && y < buf.area.height {
        if let Some(cell) = buf.cell_mut((x, y)) {
            let color = Color::Rgb(r, g, b);
            cell.set_char(' ').set_fg(color).set_bg(color);
        }
    }
}

fn fill_cell_from_color(buf: &mut Buffer, x: u16, y: u16, color: Color) {
    if x < buf.area.width && y < buf.area.height {
        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_char(' ').set_fg(color).set_bg(color);
        }
    }
}

fn lerp(a: Color, b: Color, t: f32) -> (u8, u8, u8) {
    let (ar, ag, ab) = rgb(a);
    let (br, bg, bb) = rgb(b);
    let t = t.clamp(0.0, 1.0);
    (
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
