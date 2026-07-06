//! Cat sprites rendered via Kitty Graphics Protocol on top of the ratatui grid.

use std::io::{self, Write};

use crate::kitty;
use crate::sprite::{self, Sprite};

const ASLEEP_SHEET: &[u8] = include_bytes!("../../../assets/cat_cafe/Cat_sheets/Cat_asleep_1.png");
const IDLE_SHEET: &[u8] = include_bytes!("../../../assets/cat_cafe/Cat_sheets/Cat_idle_1.png");
const WALK_SHEET: &[u8] = include_bytes!("../../../assets/cat_cafe/Cat_sheets/Cat_walk_1.png");
const ALT_CAT: &[u8] = include_bytes!("../../../assets/cat_cafe/Cat_sheets/Cat_alt.png");

const FRAME_W: u16 = 32;

// Kitty image id ranges per animation (one image per frame)
const ASLEEP_BASE: u32 = 10;
const IDLE_BASE: u32 = 20;
const WALK_BASE: u32 = 30;
const ALT_ID: u32 = 40;

// On-screen size of a cat in cells (~square at typical 1:2 cell aspect)
const CAT_COLS: u16 = 8;
const CAT_ROWS: u16 = 4;

const WALK_SPEED: f64 = 3.0; // cells per second

#[derive(Clone, Copy, PartialEq)]
enum State {
    Asleep,
    Idle,
    Walking,
}

pub struct Cats {
    asleep: Vec<Sprite>,
    idle: Vec<Sprite>,
    walk: Vec<Sprite>,
    alt: Sprite,
    transmitted: bool,

    state: State,
    state_time: f64,
    frame_time: f64,
    frame: usize,
    x: f64,
    dir: f64,

    width: u16,
    counter_y: u16,
    // (image id, col, row) of the current main-cat placement
    last_main: Option<(u32, u16, u16)>,
    alt_placed: bool,
}

impl Cats {
    pub fn new() -> Self {
        Self {
            asleep: sprite::load_strip(ASLEEP_SHEET, FRAME_W),
            idle: sprite::load_strip(IDLE_SHEET, FRAME_W),
            walk: sprite::load_strip(WALK_SHEET, FRAME_W),
            alt: sprite::from_png_bytes(ALT_CAT),
            transmitted: false,
            state: State::Asleep,
            state_time: 0.0,
            frame_time: 0.0,
            frame: 0,
            x: 0.0,
            dir: 1.0,
            width: 0,
            counter_y: 0,
            last_main: None,
            alt_placed: false,
        }
    }

    pub fn init(&mut self, width: u16, counter_y: u16) {
        self.width = width;
        self.counter_y = counter_y;
        self.x = f64::from(width) / 3.0;
        // Force re-placement at the new geometry on the next post_draw
        self.last_main = None;
        self.alt_placed = false;
    }

    pub fn update(&mut self, dt: f64) {
        self.state_time += dt;
        self.frame_time += dt;

        let (duration, frame_dt, next) = match self.state {
            State::Asleep => (10.0, 0.5, State::Idle),
            State::Idle => (5.0, 0.35, State::Walking),
            State::Walking => (8.0, 0.15, State::Asleep),
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
            let min = 2.0;
            let max = f64::from(self.width.saturating_sub(CAT_COLS)) * 0.55;
            if self.x <= min || self.x >= max {
                self.x = self.x.clamp(min, max);
                self.dir = -self.dir;
            }
        }
    }

    fn main_frame(&self) -> (u32, &Sprite) {
        let (base, frames) = match self.state {
            State::Asleep => (ASLEEP_BASE, &self.asleep),
            State::Idle => (IDLE_BASE, &self.idle),
            State::Walking => (WALK_BASE, &self.walk),
        };
        let idx = self.frame % frames.len();
        (base + idx as u32, &frames[idx])
    }

    pub fn post_draw(&mut self, out: &mut dyn Write) -> io::Result<()> {
        if self.width == 0 || self.counter_y < CAT_ROWS {
            return Ok(());
        }

        if !self.transmitted {
            for (i, s) in self.asleep.iter().enumerate() {
                kitty::transmit(out, ASLEEP_BASE + i as u32, s.width, s.height, &s.rgba)?;
            }
            for (i, s) in self.idle.iter().enumerate() {
                kitty::transmit(out, IDLE_BASE + i as u32, s.width, s.height, &s.rgba)?;
            }
            for (i, s) in self.walk.iter().enumerate() {
                kitty::transmit(out, WALK_BASE + i as u32, s.width, s.height, &s.rgba)?;
            }
            kitty::transmit(out, ALT_ID, self.alt.width, self.alt.height, &self.alt.rgba)?;
            self.transmitted = true;
        }

        let row = self.counter_y - CAT_ROWS;

        // Main cat: re-place only when the frame image or position changed
        let (id, _) = self.main_frame();
        let col = self.x as u16;
        if self.last_main != Some((id, col, row)) {
            if let Some((old_id, _, _)) = self.last_main {
                if old_id != id {
                    kitty::delete_placement(out, old_id)?;
                }
            }
            kitty::place(out, id, col, row, CAT_COLS, CAT_ROWS)?;
            self.last_main = Some((id, col, row));
        }

        // Alt cat: static, placed once per geometry
        if !self.alt_placed {
            kitty::delete_placement(out, ALT_ID)?;
            let alt_col = (u32::from(self.width) * 62 / 100) as u16;
            kitty::place(out, ALT_ID, alt_col, row, CAT_COLS, CAT_ROWS)?;
            self.alt_placed = true;
        }

        Ok(())
    }

    pub fn cleanup(&mut self, out: &mut dyn Write) -> io::Result<()> {
        kitty::delete_all(out)
    }
}
