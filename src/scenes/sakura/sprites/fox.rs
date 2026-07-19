//! The resident fox: a quiet state machine (asleep / moon-gazing / walk)
//! rendered as a Kitty pixel sprite on the grass bank.

use std::io::{self, Write};

use crate::kitty;
use crate::scenes::sakura::layout::Layout;
use crate::sprite::{self, Sprite};

const SIT: &[u8] = include_bytes!("../../../../assets/sakura/fox/sit.png");
const WALK: &[u8] = include_bytes!("../../../../assets/sakura/fox/walk.png");
const ASLEEP: &[u8] = include_bytes!("../../../../assets/sakura/fox/asleep.png");

const FRAME_W: u16 = 128;
/// The walk strip uses wider frames: a trotting fox is longer than tall,
/// so it gets a wider cell box at the same body scale as sit/asleep.
const WALK_FRAME_W: u16 = 176;
const WALK_SPEED: f64 = 2.5; // cells per second

// Kitty image id ranges (one image per animation frame)
const BASE: u32 = 10;
const ASLEEP_OFF: u32 = 0;
const SIT_OFF: u32 = 10;
const WALK_OFF: u32 = 20;
const WALK_FLIP_OFF: u32 = 30;

#[derive(Clone, Copy, PartialEq)]
enum State {
    Asleep,
    /// Sitting still, gazing up at the moon.
    Gazing,
    Walking,
}

pub struct Fox {
    asleep: Vec<Sprite>,
    sit: Vec<Sprite>,
    walk: Vec<Sprite>,
    walk_flip: Vec<Sprite>,

    state: State,
    state_time: f64,
    frame_time: f64,
    frame: usize,
    x: f64,
    dir: f64,
    range: (f64, f64),
    // (image id, col, row) of the current placement
    last: Option<(u32, u16, u16)>,
    transmitted: bool,
}

impl Fox {
    pub fn new() -> Self {
        let walk = sprite::load_strip(WALK, WALK_FRAME_W);
        let walk_flip = walk.iter().map(Sprite::flipped_h).collect();
        Self {
            asleep: sprite::load_strip(ASLEEP, FRAME_W),
            sit: sprite::load_strip(SIT, FRAME_W),
            walk,
            walk_flip,
            state: State::Gazing,
            state_time: 0.0,
            frame_time: 0.0,
            frame: 0,
            x: 0.0,
            dir: -1.0,
            range: (0.0, 0.0),
            last: None,
            transmitted: false,
        }
    }

    pub fn init(&mut self, l: &Layout) {
        let (min, max) = l.fox_range;
        self.range = (f64::from(min), f64::from(max));
        self.x = f64::from(max).max(f64::from(min));
        // Force re-placement at the new geometry on the next post_draw
        self.last = None;
    }

    pub fn update(&mut self, dt: f64) {
        self.state_time += dt;
        self.frame_time += dt;

        // Contemplative personality: long moon-gazes, naps, short strolls.
        let (duration, frame_dt, next) = match self.state {
            State::Gazing => (22.0, 0.9, State::Walking),
            State::Walking => (6.0, 0.22, State::Asleep),
            State::Asleep => (26.0, 0.8, State::Gazing),
        };

        if self.state_time >= duration {
            self.state = next;
            self.state_time = 0.0;
            self.frame_time = 0.0;
            self.frame = 0;
        } else if self.frame_time >= frame_dt {
            self.frame_time = 0.0;
            self.frame += 1;
        }

        if self.state == State::Walking {
            self.x += self.dir * WALK_SPEED * dt;
            let (min, max) = self.range;
            if self.x <= min || self.x >= max {
                self.x = self.x.clamp(min, max);
                self.dir = -self.dir;
            }
        }
    }

    fn frames(&self) -> (&[Sprite], u32) {
        match self.state {
            State::Asleep => (&self.asleep, BASE + ASLEEP_OFF),
            // Sit and sleep face left natively, toward the moon and the water.
            State::Gazing => (&self.sit, BASE + SIT_OFF),
            State::Walking => {
                if self.dir > 0.0 {
                    (&self.walk_flip, BASE + WALK_FLIP_OFF)
                } else {
                    (&self.walk, BASE + WALK_OFF)
                }
            }
        }
    }

    fn transmit_all(&self, out: &mut dyn Write) -> io::Result<()> {
        let sets: [(&[Sprite], u32); 4] = [
            (&self.asleep, BASE + ASLEEP_OFF),
            (&self.sit, BASE + SIT_OFF),
            (&self.walk, BASE + WALK_OFF),
            (&self.walk_flip, BASE + WALK_FLIP_OFF),
        ];
        for (frames, id_base) in sets {
            for (i, s) in frames.iter().enumerate() {
                kitty::transmit(out, id_base + i as u32, s.width, s.height, &s.rgba)?;
            }
        }
        Ok(())
    }

    pub fn post_draw(&mut self, out: &mut dyn Write, l: &Layout) -> io::Result<()> {
        if l.h < l.ground_y + l.fox_rows {
            return Ok(());
        }
        if !self.transmitted {
            self.transmit_all(out)?;
            self.transmitted = true;
        }
        // Feet on the upper bank, in front of the tree line.
        let row = l.ground_y;
        let (frames, id_base) = self.frames();
        let id = id_base + (self.frame % frames.len()) as u32;
        // The walk box is wider (same body scale, longer pose); keep the body
        // centered on x so the fox doesn't jump sideways on state changes.
        let (cols, col) = if self.state == State::Walking {
            let cols =
                (u32::from(l.fox_cols) * u32::from(WALK_FRAME_W) / u32::from(FRAME_W)) as u16;
            (
                cols,
                (self.x as u16).saturating_sub((cols - l.fox_cols) / 2),
            )
        } else {
            (l.fox_cols, self.x as u16)
        };
        if self.last != Some((id, col, row)) {
            if let Some((old_id, _, _)) = self.last {
                if old_id != id {
                    kitty::delete_placement(out, old_id)?;
                }
            }
            kitty::place(out, id, col, row, cols, l.fox_rows)?;
            self.last = Some((id, col, row));
        }
        Ok(())
    }
}
