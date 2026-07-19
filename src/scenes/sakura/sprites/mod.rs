//! Kitty Graphics sprite layer: the red panda, the only pixel sprite in the
//! scene.

mod panda;

use std::io::{self, Write};

use crate::kitty;
use crate::scenes::sakura::layout::Layout;

pub struct SpriteLayer {
    panda: panda::Panda,
    layout: Layout,
}

impl SpriteLayer {
    pub fn new() -> Self {
        Self {
            panda: panda::Panda::new(),
            layout: Layout::default(),
        }
    }

    pub fn init(&mut self, l: &Layout) {
        self.layout = *l;
        self.panda.init(l);
    }

    pub fn update(&mut self, dt: f64) {
        self.panda.update(dt);
    }

    pub fn post_draw(&mut self, out: &mut dyn Write) -> io::Result<()> {
        if self.layout.w == 0 {
            return Ok(());
        }
        self.panda.post_draw(out, &self.layout)
    }

    pub fn cleanup(&mut self, out: &mut dyn Write) -> io::Result<()> {
        kitty::delete_all(out)
    }
}
