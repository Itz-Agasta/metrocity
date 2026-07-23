//! Stone lanterns: warm flickering pixel sprites on the grass bank, one near
//! the water on the left and one at the foot of the sakura tree.

use std::io::{self, Write};

use crate::kitty;
use crate::sprite::{self, Sprite};

const GLOW: &[u8] = include_bytes!("../../../../assets/sakura/lantern/glow.png");

const FRAME_W: u16 = 128;
/// Candle rhythm: mostly medium/bright with the occasional dip.
const FLICKER: [usize; 6] = [0, 1, 2, 1, 3, 1];
const FLICKER_DT: f64 = 0.45;

pub struct Lantern {
    frames: Vec<Sprite>,
    id_base: u32,
    /// Phase offset so two lanterns never flicker in sync.
    phase: f64,
    t: f64,
    // Placement geometry (cells), set by init
    col: u16,
    row: u16,
    cols: u16,
    rows: u16,
    last: Option<u32>,
    transmitted: bool,
}

impl Lantern {
    pub fn new(id_base: u32, phase: f64) -> Self {
        Self {
            frames: sprite::load_strip(GLOW, FRAME_W),
            id_base,
            phase,
            t: 0.0,
            col: 0,
            row: 0,
            cols: 0,
            rows: 0,
            last: None,
            transmitted: false,
        }
    }

    pub fn init(&mut self, col: u16, row: u16, cols: u16, rows: u16) {
        self.col = col;
        self.row = row;
        self.cols = cols;
        self.rows = rows;
        self.last = None;
    }

    pub fn update(&mut self, dt: f64) {
        self.t += dt;
    }

    pub fn post_draw(&mut self, out: &mut dyn Write) -> io::Result<()> {
        if self.cols == 0 {
            return Ok(());
        }
        if !self.transmitted {
            for (i, s) in self.frames.iter().enumerate() {
                kitty::transmit(out, self.id_base + i as u32, s.width, s.height, &s.rgba)?;
            }
            self.transmitted = true;
        }
        let step = ((self.t + self.phase) / FLICKER_DT) as usize % FLICKER.len();
        let id = self.id_base + FLICKER[step] as u32;
        if self.last != Some(id) {
            if let Some(old_id) = self.last {
                if old_id != id {
                    kitty::delete_placement(out, old_id)?;
                }
            }
            kitty::place(out, id, self.col, self.row, self.cols, self.rows)?;
            self.last = Some(id);
        }
        Ok(())
    }
}
