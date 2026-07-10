//! Cat cafe scene: ratatui characters for structure and text, Kitty pixel
//! sprites for the cats and organic decor.
//!
//! Module map:
//! - `layout`      every element's position, shared by draw and post_draw
//! - `palette`     hardcoded colors
//! - `paint`       low-level cell helpers
//! - `background`  wall / counter / floor
//! - `components`  one file per ratatui-drawn element
//! - `sprites`     Kitty sprite layer (cats, neon face, plants, pastries)

use std::io::{self, Write};

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use crate::kitty;
use crate::scene::Scene;
use crate::theme::Theme;

mod background;
pub mod components;
pub mod layout;
mod paint;
mod palette;
mod sprites;

use layout::Layout;

pub struct CafeScene {
    layout: Layout,
    t: f64,
    sprites: Option<sprites::SpriteLayer>,
}

impl CafeScene {
    pub fn new() -> Self {
        Self {
            layout: Layout::default(),
            t: 0.0,
            sprites: kitty::supported().then(sprites::SpriteLayer::new),
        }
    }
}

impl Scene for CafeScene {
    fn name(&self) -> &str {
        "cafe"
    }

    fn init(&mut self, width: u16, height: u16, _theme: &Theme) {
        self.layout = Layout::new(width, height);
        if let Some(sprites) = &mut self.sprites {
            sprites.init(&self.layout);
        }
    }

    fn update(&mut self, dt: f64) {
        self.t += dt;
        if let Some(sprites) = &mut self.sprites {
            sprites.update(dt);
        }
    }

    fn draw(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        // init() keeps the layout in sync with resizes; rebuild locally if a
        // frame arrives with a different size anyway.
        let local;
        let l = if area.width == self.layout.w && area.height == self.layout.h {
            &self.layout
        } else {
            local = Layout::new(area.width, area.height);
            &local
        };

        background::render(buf, l);
        components::window::draw(buf, l, self.t);
        components::shelves::draw(buf, l);
        components::menu::draw(buf, l);
        components::neon::draw(buf, l, self.t);
        components::lamp::draw(buf, l);
        components::stools::draw(buf, l);
        components::terminal::draw(buf, l, self.t);
        components::counter_props::draw(buf, l, self.t);
        components::pastry_case::draw(buf, l);
        components::signboard::draw(buf, l);
    }

    fn post_draw(&mut self, out: &mut dyn Write) -> io::Result<()> {
        match &mut self.sprites {
            Some(sprites) => sprites.post_draw(out),
            None => Ok(()),
        }
    }

    fn cleanup(&mut self, out: &mut dyn Write) -> io::Result<()> {
        match &mut self.sprites {
            Some(sprites) => sprites.cleanup(out),
            None => Ok(()),
        }
    }
}
