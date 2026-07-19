//! Kitty Graphics sprite layer: the fox and the stone lanterns.

mod fox;
mod lantern;

use std::io::{self, Write};

use crate::kitty;
use crate::scenes::sakura::layout::Layout;

pub struct SpriteLayer {
    fox: fox::Fox,
    lanterns: [lantern::Lantern; 2],
    layout: Layout,
}

impl SpriteLayer {
    pub fn new() -> Self {
        Self {
            fox: fox::Fox::new(),
            lanterns: [
                lantern::Lantern::new(50, 0.0),
                lantern::Lantern::new(60, 1.3),
            ],
            layout: Layout::default(),
        }
    }

    pub fn init(&mut self, l: &Layout) {
        self.layout = *l;
        self.fox.init(l);
        for (lantern, rect) in self.lanterns.iter_mut().zip([l.lantern_left, l.lantern_right]) {
            lantern.init(rect.x, rect.y, rect.width, rect.height);
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.fox.update(dt);
        for lantern in &mut self.lanterns {
            lantern.update(dt);
        }
    }

    pub fn post_draw(&mut self, out: &mut dyn Write) -> io::Result<()> {
        if self.layout.w == 0 {
            return Ok(());
        }
        for lantern in &mut self.lanterns {
            lantern.post_draw(out)?;
        }
        self.fox.post_draw(out, &self.layout)
    }

    pub fn cleanup(&mut self, out: &mut dyn Write) -> io::Result<()> {
        kitty::delete_all(out)
    }
}
