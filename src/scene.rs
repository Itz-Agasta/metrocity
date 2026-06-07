use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use crate::theme::Theme;

/// A screensaver scene. The engine calls: init → (update → draw)* → resize → …
pub trait Scene {
    /// Human-readable name for CLI selection.
    #[allow(dead_code)] // TODO: used by scene cycling (v2) and `--list` output
    fn name(&self) -> &str;

    /// Initialize or reinitialize with new terminal dimensions and theme.
    fn init(&mut self, width: u16, height: u16, theme: &Theme);

    /// Advance the simulation by `dt` seconds.
    fn update(&mut self, dt: f64);

    /// Render the current state into the ratatui buffer.
    fn draw(&self, area: Rect, buf: &mut Buffer);
}
