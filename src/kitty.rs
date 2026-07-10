//! Kitty Graphics Protocol encoder.
//!
//! Safe to mix with ratatui/crossterm because:
//! - `q=2` suppresses terminal responses (otherwise they arrive as input
//!   events and trip the any-key-exit loop)
//! - `C=1` tells the terminal not to move the cursor after placing an image
//! - placements are wrapped in DECSC/DECRC (`ESC 7` / `ESC 8`) so the cursor
//!   ends up exactly where ratatui left it. ratatui re-issues an absolute
//!   `MoveTo` at the start of every draw anyway, so no state can desync.

use std::env;
use std::io::{self, Write};

use base64::{engine::general_purpose::STANDARD, Engine as _};

const CHUNK_SIZE: usize = 4096;

/// Best-effort detection of a Kitty-graphics-capable terminal.
pub fn supported() -> bool {
    if env::var_os("KITTY_WINDOW_ID").is_some() {
        return true;
    }
    match env::var("TERM") {
        Ok(term) => term.contains("kitty") || term.contains("ghostty") || term.contains("wezterm"),
        Err(_) => false,
    }
}

/// Transmit RGBA pixel data as image `id` (a=t: transmit only, no display).
pub fn transmit(
    out: &mut dyn Write,
    id: u32,
    width: u16,
    height: u16,
    rgba: &[u8],
) -> io::Result<()> {
    let payload = STANDARD.encode(rgba);
    let chunks: Vec<&[u8]> = payload.as_bytes().chunks(CHUNK_SIZE).collect();
    for (i, chunk) in chunks.iter().enumerate() {
        let more = usize::from(i + 1 < chunks.len());
        if i == 0 {
            write!(
                out,
                "\x1b_Ga=t,q=2,f=32,i={id},s={width},v={height},m={more};"
            )?;
        } else {
            write!(out, "\x1b_Gm={more};")?;
        }
        out.write_all(chunk)?;
        out.write_all(b"\x1b\\")?;
    }
    Ok(())
}

/// Place image `id` with its top-left at cell (col, row), scaled to fit
/// cols x rows cells. Re-placing the same id replaces the old placement.
pub fn place(
    out: &mut dyn Write,
    id: u32,
    col: u16,
    row: u16,
    cols: u16,
    rows: u16,
) -> io::Result<()> {
    write!(
        out,
        "\x1b7\x1b[{};{}H\x1b_Ga=p,q=2,i={id},p=1,c={cols},r={rows},C=1\x1b\\\x1b8",
        row + 1,
        col + 1
    )
}

/// Remove the on-screen placement of image `id` (pixel data stays cached).
pub fn delete_placement(out: &mut dyn Write, id: u32) -> io::Result<()> {
    write!(out, "\x1b_Ga=d,q=2,d=i,i={id};\x1b\\")
}

/// Delete all images and placements (exit cleanup).
pub fn delete_all(out: &mut dyn Write) -> io::Result<()> {
    write!(out, "\x1b_Ga=d,q=2,d=A;\x1b\\")
}
