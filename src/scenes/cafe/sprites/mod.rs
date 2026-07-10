//! Kitty Graphics sprite layer: everything drawn in pixels on top of the
//! ratatui character grid (cats, neon face, plants, pastries).

mod cats;
mod decor;

use std::io::{self, Write};

use crate::kitty;
use crate::scenes::cafe::components::neon;
use crate::scenes::cafe::layout::Layout;

pub struct SpriteLayer {
    cats: cats::Cats,
    decor: decor::Decor,
    layout: Layout,
    t: f64,
}

impl SpriteLayer {
    pub fn new() -> Self {
        Self {
            cats: cats::Cats::new(),
            decor: decor::Decor::new(),
            layout: Layout::default(),
            t: 0.0,
        }
    }

    pub fn init(&mut self, l: &Layout) {
        self.layout = *l;
        self.cats.init(l);
        self.decor.init();
    }

    pub fn update(&mut self, dt: f64) {
        self.t += dt;
        self.cats.update(dt);
        // Keep the sprite face in sync with the ratatui text flicker
        self.decor.set_dim(neon::is_dim(self.t));
    }

    pub fn post_draw(&mut self, out: &mut dyn Write) -> io::Result<()> {
        if self.layout.w == 0 {
            return Ok(());
        }
        self.decor.post_draw(out, &self.layout)?;
        self.cats.post_draw(out, &self.layout)
    }

    pub fn cleanup(&mut self, out: &mut dyn Write) -> io::Result<()> {
        kitty::delete_all(out)
    }
}
