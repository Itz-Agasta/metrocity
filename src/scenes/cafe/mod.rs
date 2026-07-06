use std::io::{self, Write};

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use crate::kitty;
use crate::scene::Scene;
use crate::theme::Theme;

mod background;
mod cat;
pub mod components;

pub struct CafeScene {
    width: u16,
    height: u16,
    cats: Option<cat::Cats>,
}

impl CafeScene {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            cats: kitty::supported().then(cat::Cats::new),
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
        if let Some(cats) = &mut self.cats {
            cats.init(width, background::counter_row(height));
        }
    }

    fn update(&mut self, dt: f64) {
        if let Some(cats) = &mut self.cats {
            cats.update(dt);
        }
    }

    fn draw(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        background::render(area, buf);
    }

    fn post_draw(&mut self, out: &mut dyn Write) -> io::Result<()> {
        match &mut self.cats {
            Some(cats) => cats.post_draw(out),
            None => Ok(()),
        }
    }

    fn cleanup(&mut self, out: &mut dyn Write) -> io::Result<()> {
        match &mut self.cats {
            Some(cats) => cats.cleanup(out),
            None => Ok(()),
        }
    }
}
