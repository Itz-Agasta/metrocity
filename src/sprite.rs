//! PNG sprite loading: decode embedded sprite sheets into RGBA frames.

use std::io::Cursor;

pub struct Sprite {
    pub width: u16,
    pub height: u16,
    pub rgba: Vec<u8>,
}

/// Decode a PNG (8-bit RGBA) into a single sprite.
/// Panics on malformed input: all sheets are embedded at compile time.
pub fn from_png_bytes(bytes: &[u8]) -> Sprite {
    let decoder = png::Decoder::new(Cursor::new(bytes));
    let mut reader = decoder.read_info().expect("embedded PNG: invalid header");
    let size = reader
        .output_buffer_size()
        .expect("embedded PNG: output size overflow");
    let mut buffer = vec![0u8; size];
    let info = reader
        .next_frame(&mut buffer)
        .expect("embedded PNG: decode failed");
    assert_eq!(info.color_type, png::ColorType::Rgba, "sprite must be RGBA");
    assert_eq!(info.bit_depth, png::BitDepth::Eight, "sprite must be 8-bit");
    buffer.truncate(info.buffer_size());
    Sprite {
        width: info.width as u16,
        height: info.height as u16,
        rgba: buffer,
    }
}

/// Decode a horizontal sprite strip and split it into frames of `frame_w` px.
pub fn load_strip(bytes: &[u8], frame_w: u16) -> Vec<Sprite> {
    let sheet = from_png_bytes(bytes);
    let count = sheet.width / frame_w;
    let row_bytes = usize::from(sheet.width) * 4;
    let frame_row_bytes = usize::from(frame_w) * 4;
    (0..count)
        .map(|f| {
            let x_off = usize::from(f * frame_w) * 4;
            let mut rgba = Vec::with_capacity(frame_row_bytes * usize::from(sheet.height));
            for y in 0..usize::from(sheet.height) {
                let start = y * row_bytes + x_off;
                rgba.extend_from_slice(&sheet.rgba[start..start + frame_row_bytes]);
            }
            Sprite {
                width: frame_w,
                height: sheet.height,
                rgba,
            }
        })
        .collect()
}
