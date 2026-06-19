use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use crate::scene::Scene;
use crate::theme::Theme;

mod background;
pub mod components;

pub struct CafeScene {
    width: u16,
    height: u16,
}

impl CafeScene {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}

impl Scene for CafeScene {
    fn name(&self) -> &str {
        "cafe"
    }

    fn init(&mut self, width: u16, height: u16, _theme: &Theme) {
        self.width = width;
        self.height = height;
    }

    fn update(&mut self, _dt: f64) {}

    fn draw(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        background::render(area, buf);
    }
}
