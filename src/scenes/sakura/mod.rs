//! Sakura scene: a night lakeside with a big procedural cherry-blossom tree,
//! a city skyline and pagoda across the water, and petals drifting on the wind.
//! Character-grid scenery; the Kitty sprites are the fox and the stone lanterns.
//!
//! Module map:
//! - `layout`      every element's position, shared by draw and post_draw
//! - `palette`     hardcoded night colors
//! - `paint`       low-level cell helpers
//! - `background`  sky / stars / moon / far hills
//! - `tree`        canopy, branches, trunk, petal carpet
//! - `petals`      falling-petal particles + wind
//! - `components`  one file per ratatui-drawn element (lake, pagoda, ...)
//! - `sprites`     Kitty sprite layer (fox, stone lanterns)

use std::io::{self, Write};

use rand::rngs::StdRng;
use rand::{thread_rng, Rng, SeedableRng};
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
mod petals;
mod sprites;
mod tree;

use layout::Layout;

pub struct SakuraScene {
    layout: Layout,
    t: f64,
    rng: StdRng,
    /// Per-launch seed deciding which skyline windows are lit (city-style).
    window_seed: u32,
    tree: tree::Tree,
    petals: petals::Petals,
    sprites: Option<sprites::SpriteLayer>,
}

impl SakuraScene {
    pub fn new() -> Self {
        Self {
            layout: Layout::default(),
            t: 0.0,
            rng: StdRng::from_entropy(),
            window_seed: thread_rng().gen(),
            tree: tree::Tree::default(),
            petals: petals::Petals::default(),
            sprites: kitty::supported().then(sprites::SpriteLayer::new),
        }
    }
}

impl Scene for SakuraScene {
    fn name(&self) -> &str {
        "sakura"
    }

    fn init(&mut self, width: u16, height: u16, _theme: &Theme) {
        self.layout = Layout::new(width, height);
        // Fixed seed: the tree is a designed composition, the same lush shape
        // on every launch (per terminal size). Petals stay truly random.
        let mut tree_rng = StdRng::seed_from_u64(tree::SEED);
        self.tree.generate(&self.layout, &mut tree_rng);
        self.petals
            .reset(&self.layout, &self.tree.sources, &mut self.rng, true);
        if let Some(sprites) = &mut self.sprites {
            sprites.init(&self.layout);
        }
    }

    fn update(&mut self, dt: f64) {
        self.t += dt;
        self.petals
            .update(dt, &self.layout, &self.tree, &mut self.rng);
        if let Some(sprites) = &mut self.sprites {
            sprites.update(dt);
        }
    }

    fn draw(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }
        // init() keeps the layout in sync with resizes; skip a frame that
        // arrives with a stale size (the tree grid can't be rebuilt here).
        if area.width != self.layout.w || area.height != self.layout.h {
            return;
        }
        let l = &self.layout;

        background::render(buf, l, self.t);
        components::skyline::draw(buf, l, self.t, self.window_seed);
        components::lake::draw(buf, l, self.t, self.window_seed);
        components::ground::draw(buf, l);
        self.tree.draw(buf);
        self.petals.draw(buf);
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
