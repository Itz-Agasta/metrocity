//! The two resident cats: independent state machines (asleep / idle / walk),
//! rendered as Kitty pixel sprites on the counter.

use std::io::{self, Write};

use crate::kitty;
use crate::scenes::cafe::layout::Layout;
use crate::sprite::{self, Sprite};

const BLACK_IDLE: &[u8] = include_bytes!("../../../../assets/cat_cafe/cats/black_idle.png");
const BLACK_WALK: &[u8] = include_bytes!("../../../../assets/cat_cafe/cats/black_walk.png");
const BLACK_ASLEEP: &[u8] = include_bytes!("../../../../assets/cat_cafe/cats/black_asleep.png");
const WHITE_IDLE: &[u8] = include_bytes!("../../../../assets/cat_cafe/cats/white_idle.png");
const WHITE_WALK: &[u8] = include_bytes!("../../../../assets/cat_cafe/cats/white_walk.png");
const WHITE_ASLEEP: &[u8] = include_bytes!("../../../../assets/cat_cafe/cats/white_asleep.png");

const FRAME_W: u16 = 64;
const WALK_SPEED: f64 = 3.0; // cells per second

// Kitty image id ranges (one image per animation frame)
const BLACK_BASE: u32 = 10;
const WHITE_BASE: u32 = 50;
const ASLEEP_OFF: u32 = 0;
const IDLE_OFF: u32 = 10;
const WALK_OFF: u32 = 20;
const WALK_FLIP_OFF: u32 = 30;

#[derive(Clone, Copy, PartialEq)]
enum State {
    Asleep,
    Idle,
    Walking,
}

struct Cat {
    asleep: Vec<Sprite>,
    idle: Vec<Sprite>,
    walk: Vec<Sprite>,
    walk_flip: Vec<Sprite>,
    base: u32,
    faces_right: bool, // native facing of the idle/walk sheets
    // (asleep, idle, walking) durations: gives each cat its own personality
    durations: (f64, f64, f64),

    state: State,
    state_time: f64,
    frame_time: f64,
    frame: usize,
    x: f64,
    dir: f64,
    range: (f64, f64),
    // (image id, col, row) of the current placement
    last: Option<(u32, u16, u16)>,
}

impl Cat {
    fn new(
        idle_png: &[u8],
        walk_png: &[u8],
        asleep_png: &[u8],
        base: u32,
        faces_right: bool,
        durations: (f64, f64, f64),
        start: State,
    ) -> Self {
        let walk = sprite::load_strip(walk_png, FRAME_W);
        let walk_flip = walk.iter().map(Sprite::flipped_h).collect();
        Self {
            asleep: sprite::load_strip(asleep_png, FRAME_W),
            idle: sprite::load_strip(idle_png, FRAME_W),
            walk,
            walk_flip,
            base,
            faces_right,
            durations,
            state: start,
            state_time: 0.0,
            frame_time: 0.0,
            frame: 0,
            x: 0.0,
            dir: 1.0,
            range: (0.0, 0.0),
            last: None,
        }
    }

    fn update(&mut self, dt: f64) {
        self.state_time += dt;
        self.frame_time += dt;

        let (duration, frame_dt, next) = match self.state {
            State::Asleep => (self.durations.0, 0.6, State::Idle),
            State::Idle => (self.durations.1, 0.4, State::Walking),
            State::Walking => (self.durations.2, 0.18, State::Asleep),
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
            State::Asleep => (&self.asleep, self.base + ASLEEP_OFF),
            State::Idle => (&self.idle, self.base + IDLE_OFF),
            State::Walking => {
                let flip = (self.dir > 0.0) != self.faces_right;
                if flip {
                    (&self.walk_flip, self.base + WALK_FLIP_OFF)
                } else {
                    (&self.walk, self.base + WALK_OFF)
                }
            }
        }
    }

    fn current_id(&self) -> u32 {
        let (frames, id_base) = self.frames();
        id_base + (self.frame % frames.len()) as u32
    }

    fn transmit_all(&self, out: &mut dyn Write) -> io::Result<()> {
        let sets: [(&[Sprite], u32); 4] = [
            (&self.asleep, self.base + ASLEEP_OFF),
            (&self.idle, self.base + IDLE_OFF),
            (&self.walk, self.base + WALK_OFF),
            (&self.walk_flip, self.base + WALK_FLIP_OFF),
        ];
        for (frames, id_base) in sets {
            for (i, s) in frames.iter().enumerate() {
                kitty::transmit(out, id_base + i as u32, s.width, s.height, &s.rgba)?;
            }
        }
        Ok(())
    }

    fn place(&mut self, out: &mut dyn Write, l: &Layout) -> io::Result<()> {
        let row = l.counter_y.saturating_sub(l.cat_rows);
        let id = self.current_id();
        let col = self.x as u16;
        if self.last != Some((id, col, row)) {
            if let Some((old_id, _, _)) = self.last {
                if old_id != id {
                    kitty::delete_placement(out, old_id)?;
                }
            }
            kitty::place(out, id, col, row, l.cat_cols, l.cat_rows)?;
            self.last = Some((id, col, row));
        }
        Ok(())
    }
}

pub struct Cats {
    black: Cat,
    white: Cat,
    transmitted: bool,
}

impl Cats {
    pub fn new() -> Self {
        Self {
            // Black cat is the wanderer, white cat mostly lounges
            black: Cat::new(
                BLACK_IDLE,
                BLACK_WALK,
                BLACK_ASLEEP,
                BLACK_BASE,
                true,
                (10.0, 5.0, 9.0),
                State::Idle,
            ),
            white: Cat::new(
                WHITE_IDLE,
                WHITE_WALK,
                WHITE_ASLEEP,
                WHITE_BASE,
                false,
                (16.0, 12.0, 4.0),
                State::Asleep,
            ),
            transmitted: false,
        }
    }

    pub fn init(&mut self, l: &Layout) {
        let (min, max) = l.black_cat_range;
        self.black.range = (f64::from(min), f64::from(max));
        self.black.x = f64::from(min + max) / 2.0;

        let wx = f64::from(l.white_cat_x);
        self.white.range = (wx - 5.0, wx + 5.0);
        self.white.x = wx;

        // Force re-placement at the new geometry on the next post_draw
        self.black.last = None;
        self.white.last = None;
    }

    pub fn update(&mut self, dt: f64) {
        self.black.update(dt);
        self.white.update(dt);
    }

    pub fn post_draw(&mut self, out: &mut dyn Write, l: &Layout) -> io::Result<()> {
        if l.w == 0 || l.counter_y < l.cat_rows {
            return Ok(());
        }
        if !self.transmitted {
            self.black.transmit_all(out)?;
            self.white.transmit_all(out)?;
            self.transmitted = true;
        }
        self.black.place(out, l)?;
        self.white.place(out, l)
    }
}
