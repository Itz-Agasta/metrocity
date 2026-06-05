use ratatui::buffer::Buffer;
use ratatui::style::Color;

pub fn darken_color(c: Color) -> Color {
    match c {
        Color::Rgb(r, g, b) => Color::Rgb(r / 3, g / 3, b / 3),
        Color::Red => Color::Rgb(100, 0, 0),
        Color::Green => Color::Rgb(0, 100, 0),
        Color::Blue => Color::Rgb(0, 0, 100),
        Color::Yellow => Color::Rgb(100, 100, 0),
        Color::Cyan => Color::Rgb(0, 100, 100),
        Color::Magenta => Color::Rgb(100, 0, 100),
        Color::White => Color::Rgb(100, 100, 100),
        _ => Color::Rgb(10, 10, 20),
    }
}

pub fn safe_set_char(buf: &mut Buffer, x: u16, y: u16, ch: char, fg: Color) {
    if x < buf.area.width && y < buf.area.height { 
        buf.cell_mut((x, y)).unwrap().set_char(ch).set_fg(fg);
    }
}

pub fn safe_set_char_with_bg(buf: &mut Buffer, x: u16, y: u16, ch: char, fg: Color, bg: Color) {
    if x < buf.area.width && y < buf.area.height { 
        buf.cell_mut((x, y)).unwrap().set_char(ch).set_fg(fg).set_bg(bg);
    }
}

pub fn safe_set_symbol(buf: &mut Buffer, x: u16, y: u16, sym: &str, fg: Color) {
    if x < buf.area.width && y < buf.area.height { 
        buf.cell_mut((x, y)).unwrap().set_symbol(sym).set_fg(fg); 
    }
}

pub fn safe_set_symbol_with_bg(buf: &mut Buffer, x: u16, y: u16, sym: &str, fg: Color, bg: Color) {
    if x < buf.area.width && y < buf.area.height { 
        buf.cell_mut((x, y)).unwrap().set_symbol(sym).set_fg(fg).set_bg(bg); 
    }
}

pub fn safe_set_string(buf: &mut Buffer, x: u16, y: u16, s: &str, fg: Color) {
    if y < buf.area.height {
        for (i, ch) in s.chars().enumerate() {
            let dx = x.saturating_add(i as u16);
            if dx < buf.area.width { 
                buf.cell_mut((dx, y)).unwrap().set_char(ch).set_fg(fg); 
            }
        }
    }
}

pub fn safe_get_fg(buf: &Buffer, x: u16, y: u16) -> Color {
    if x < buf.area.width && y < buf.area.height {
        buf.cell((x, y)).unwrap().fg
    } else {
        Color::Reset
    }
}

pub fn safe_get_bg(buf: &Buffer, x: u16, y: u16) -> Color {
    if x < buf.area.width && y < buf.area.height {
        buf.cell((x, y)).unwrap().bg
    } else {
        Color::Reset
    }
}

pub fn safe_get_symbol(buf: &Buffer, x: u16, y: u16) -> &str {
    if x < buf.area.width && y < buf.area.height {
        buf.cell((x, y)).unwrap().symbol()
    } else {
        " "
    }
}
