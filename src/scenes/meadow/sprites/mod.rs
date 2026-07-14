//! Kitty Graphics sprite layer: everything drawn in pixels on top of the
//! meadow character grid (the three animals, beehive, bee, hunny pot, book).

mod animals;
mod decor;

use std::io::{self, Write};

use crate::kitty;
use crate::scenes::meadow::layout::Layout;

pub struct SpriteLayer {
    animals: animals::Animals,
    decor: decor::Decor,
    layout: Layout,
    t: f64,
}

impl SpriteLayer {
    pub fn new() -> Self {
        Self {
            animals: animals::Animals::new(),
            decor: decor::Decor::new(),
            layout: Layout::default(),
            t: 0.0,
        }
    }

    pub fn init(&mut self, l: &Layout) {
        self.layout = *l;
        self.animals.init(l);
        self.decor.init();
    }

    pub fn update(&mut self, dt: f64) {
        self.t += dt;
        self.animals.update(dt);
        self.decor.set_time(self.t);
    }

    pub fn post_draw(&mut self, out: &mut dyn Write) -> io::Result<()> {
        if self.layout.w == 0 {
            return Ok(());
        }
        self.decor.post_draw(out, &self.layout)?;
        self.animals.post_draw(out, &self.layout)
    }

    pub fn cleanup(&mut self, out: &mut dyn Write) -> io::Result<()> {
        kitty::delete_all(out)
    }
}
