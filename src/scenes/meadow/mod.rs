//! Meadow scene: a capybara, a sloth and a honey badger lounging under a big
//! tree with a beehive, a patch of sunflowers and a pot of hunny. Character-grid
//! scenery, Kitty pixel sprites for the animals and props.
//!
//! Module map:
//! - `layout`      every element's position, shared by draw and post_draw
//! - `palette`     hardcoded colors
//! - `paint`       low-level cell helpers
//! - `background`  sky / hills / grass base layers
//! - `components`  one file per ratatui-drawn element (tree, cabin, ...)
//! - `sprites`     Kitty sprite layer (animals, hive, bee, hunny, book)

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

pub struct MeadowScene {
    layout: Layout,
    t: f64,
    sprites: Option<sprites::SpriteLayer>,
}

impl MeadowScene {
    pub fn new() -> Self {
        Self {
            layout: Layout::default(),
            t: 0.0,
            sprites: kitty::supported().then(sprites::SpriteLayer::new),
        }
    }
}

impl Scene for MeadowScene {
    fn name(&self) -> &str {
        "meadow"
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

        background::render(buf, l, self.t);
        components::clouds::draw(buf, l, self.t);
        components::birds::draw(buf, l, self.t);
        components::cabin::draw(buf, l, self.t);
        components::tree::draw(buf, l);
        components::sunflowers::draw(buf, l, self.t);
        components::leaves::draw(buf, l, self.t);
        components::blanket::draw(buf, l);
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
