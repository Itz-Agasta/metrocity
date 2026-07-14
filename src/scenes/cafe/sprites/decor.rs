//! Static decor sprites: neon cat face (with flicker), plants and pastries.

use std::io::{self, Write};

use ratatui::layout::Rect;

use crate::kitty;
use crate::scenes::cafe::components::{pastry_case, signboard};
use crate::scenes::cafe::layout::Layout;
use crate::sprite::{self, Sprite};

const NEON_FACE: &[u8] = include_bytes!("../../../../assets/cat_cafe/props/neon_face.png");
const PLANT_TALL: &[u8] = include_bytes!("../../../../assets/cat_cafe/plants/plant_tall.png");
const PLANT_POINTY: &[u8] = include_bytes!("../../../../assets/cat_cafe/plants/plant_pointy.png");
const PLANT_SMALL: &[u8] = include_bytes!("../../../../assets/cat_cafe/plants/plant_small.png");
const PLANT_SPROUT: &[u8] = include_bytes!("../../../../assets/cat_cafe/plants/plant_sprout.png");
const PASTRY_CROISSANT: &[u8] =
    include_bytes!("../../../../assets/cat_cafe/pastries/pastry_croissant.png");
const PASTRY_CAKE: &[u8] = include_bytes!("../../../../assets/cat_cafe/pastries/pastry_cake.png");
const PASTRY_COOKIES: &[u8] =
    include_bytes!("../../../../assets/cat_cafe/pastries/pastry_cookies.png");
const PASTRY_ROLL: &[u8] = include_bytes!("../../../../assets/cat_cafe/pastries/pastry_roll.png");
const REGISTER: &[u8] = include_bytes!("../../../../assets/cat_cafe/props/register.png");
const PAW: &[u8] = include_bytes!("../../../../assets/cat_cafe/props/paw.png");

// Cats own ids 10..=83 (base 10/50 + per-animation offsets); decor starts at 90
const NEON_BRIGHT_ID: u32 = 90;
const NEON_DIM_ID: u32 = 91;
const STATIC_BASE: u32 = 92; // plants + pastries, consecutive ids

pub struct Decor {
    neon_bright: Sprite,
    neon_dim: Sprite,
    statics: Vec<Sprite>,
    transmitted: bool,
    placed: bool,
    dim: bool,
    last_dim: Option<bool>,
}

impl Decor {
    pub fn new() -> Self {
        let neon_bright = sprite::from_png_bytes(NEON_FACE);
        let neon_dim = neon_bright.dimmed(0.45);
        let statics = [
            PLANT_TALL,
            PLANT_POINTY,
            PLANT_SMALL,
            PLANT_SPROUT,
            PASTRY_CROISSANT,
            PASTRY_CAKE,
            PASTRY_COOKIES,
            PASTRY_ROLL,
            REGISTER,
            PAW,
        ]
        .iter()
        .map(|png| sprite::from_png_bytes(png))
        .collect();
        Self {
            neon_bright,
            neon_dim,
            statics,
            transmitted: false,
            placed: false,
            dim: false,
            last_dim: None,
        }
    }

    pub fn init(&mut self) {
        // Force re-placement at the new geometry on the next post_draw.
        self.placed = false;
        self.last_dim = None;
        // Resize drops transmitted images, so re-send them on the next draw.
        self.transmitted = false;
    }

    pub fn set_dim(&mut self, dim: bool) {
        self.dim = dim;
    }

    /// Where each static sprite goes, in cells (same order as `statics`).
    fn static_slots(l: &Layout) -> [Rect; 10] {
        let case = l.pastry_case;
        let [upper, lower] = pastry_case::shelf_rows(l);
        let item_w = 4;
        let left_x = case.x + 2;
        let right_x = case.x + case.width / 2 + 1;
        [
            l.plant_left,
            l.plant_right,
            Rect::new(
                l.shelf_x1.saturating_sub(5),
                l.shelf_rows[0].saturating_sub(2),
                4,
                2,
            ),
            Rect::new(
                l.shelf_x1.saturating_sub(3),
                l.shelf_rows[1].saturating_sub(2),
                3,
                2,
            ),
            Rect::new(left_x, upper.saturating_sub(2), item_w, 2),
            Rect::new(right_x, upper.saturating_sub(2), item_w, 2),
            Rect::new(left_x, lower.saturating_sub(2), item_w, 2),
            Rect::new(right_x, lower.saturating_sub(2), item_w, 2),
            // Billing machine on the counter, right of the terminal
            Rect::new(l.register_x, l.counter_y.saturating_sub(3), 5, 3),
            signboard::paw_slot(l),
        ]
    }

    pub fn post_draw(&mut self, out: &mut dyn Write, l: &Layout) -> io::Result<()> {
        if !self.transmitted {
            let nb = &self.neon_bright;
            let nd = &self.neon_dim;
            kitty::transmit(out, NEON_BRIGHT_ID, nb.width, nb.height, &nb.rgba)?;
            kitty::transmit(out, NEON_DIM_ID, nd.width, nd.height, &nd.rgba)?;
            for (i, s) in self.statics.iter().enumerate() {
                kitty::transmit(out, STATIC_BASE + i as u32, s.width, s.height, &s.rgba)?;
            }
            self.transmitted = true;
        }

        if !self.placed {
            for (i, slot) in Self::static_slots(l).iter().enumerate() {
                let id = STATIC_BASE + i as u32;
                kitty::delete_placement(out, id)?;
                if slot.right() <= l.w && slot.bottom() <= l.h {
                    kitty::place(out, id, slot.x, slot.y, slot.width, slot.height)?;
                }
            }
            self.placed = true;
        }

        // Neon face: swap bright/dim placements on flicker
        if self.last_dim != Some(self.dim) {
            let (show, hide) = if self.dim {
                (NEON_DIM_ID, NEON_BRIGHT_ID)
            } else {
                (NEON_BRIGHT_ID, NEON_DIM_ID)
            };
            kitty::delete_placement(out, hide)?;
            let f = l.neon_face;
            if f.right() <= l.w && f.bottom() <= l.h {
                kitty::place(out, show, f.x, f.y, f.width, f.height)?;
            }
            self.last_dim = Some(self.dim);
        }
        Ok(())
    }
}
