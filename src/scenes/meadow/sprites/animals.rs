//! The three friends: capybara, sloth and honey badger. Each is an
//! independent state machine (asleep / idle / walk, plus a slow eat cycle for
//! the sloth) rendered as Kitty pixel sprites resting on the grass, mirroring
//! the cafe cats.

use std::io::{self, Write};

use crate::kitty;
use crate::scenes::meadow::layout::{Layout, Slot};
use crate::sprite::{self, Sprite};

const CAPY_IDLE: &[u8] = include_bytes!("../../../../assets/meadow/capybara/idle.png");
const CAPY_WALK: &[u8] = include_bytes!("../../../../assets/meadow/capybara/walk.png");
const CAPY_ASLEEP: &[u8] = include_bytes!("../../../../assets/meadow/capybara/asleep.png");
const SLOTH_IDLE: &[u8] = include_bytes!("../../../../assets/meadow/sloth/idle.png");
const SLOTH_ASLEEP: &[u8] = include_bytes!("../../../../assets/meadow/sloth/asleep.png");
const SLOTH_EAT: &[u8] = include_bytes!("../../../../assets/meadow/sloth/eat.png");
const BADGER_IDLE: &[u8] = include_bytes!("../../../../assets/meadow/badger/idle.png");
const BADGER_WALK: &[u8] = include_bytes!("../../../../assets/meadow/badger/walk.png");
const BADGER_ASLEEP: &[u8] = include_bytes!("../../../../assets/meadow/badger/asleep.png");

const FRAME_W: u16 = 64;
const WALK_SPEED: f64 = 2.0; // cells per second, a lazy meadow amble

// Kitty image id ranges, one image per animation frame. Each animal owns a
// 100-wide block; the offsets below never overlap (frame counts stay < 10).
const CAPY_BASE: u32 = 100;
const SLOTH_BASE: u32 = 200;
const BADGER_BASE: u32 = 300;
const ASLEEP_OFF: u32 = 0;
const IDLE_OFF: u32 = 10;
const WALK_OFF: u32 = 20;
const WALK_FLIP_OFF: u32 = 30;
const EAT_OFF: u32 = 40;

// The eat cycle plays slowly (sloth speed) and holds long enough to reach the
// jar, lift to the mouth, and settle back.
const EAT_FRAME_DT: f64 = 0.7;
const EAT_DURATION: f64 = 9.0;

#[derive(Clone, Copy, PartialEq)]
enum State {
    Asleep,
    Idle,
    Walking,
    Eating,
}

struct Critter {
    asleep: Vec<Sprite>,
    idle: Vec<Sprite>,
    walk: Vec<Sprite>,
    walk_flip: Vec<Sprite>,
    // Optional slow "reach for the hunny and eat" cycle (only the sloth has it).
    eat: Vec<Sprite>,
    base: u32,
    cols: u16,
    rows: u16,
    faces_right: bool,
    // Multiplies the asleep/idle/walk frame time. >1 slows the animal down
    // (the sloth breathes at half speed). The eat cycle keeps its own pace.
    frame_scale: f64,
    // (asleep, idle, walking) state durations: each animal's personality
    durations: (f64, f64, f64),

    state: State,
    state_time: f64,
    frame_time: f64,
    frame: usize,
    x: f64,
    dir: f64,
    range: (f64, f64),
    last: Option<(u32, u16, u16)>,
}

impl Critter {
    #[allow(clippy::too_many_arguments)]
    fn new(
        idle_png: &[u8],
        walk_png: Option<&[u8]>,
        asleep_png: &[u8],
        eat_png: Option<&[u8]>,
        base: u32,
        cols: u16,
        rows: u16,
        faces_right: bool,
        frame_scale: f64,
        durations: (f64, f64, f64),
        start: State,
    ) -> Self {
        // The sloth never walks (it idles, eats, then dozes), so it has no walk
        // strip; the wanderers do.
        let walk = walk_png
            .map(|p| sprite::load_strip(p, FRAME_W))
            .unwrap_or_default();
        let walk_flip = walk.iter().map(Sprite::flipped_h).collect();
        Self {
            asleep: sprite::load_strip(asleep_png, FRAME_W),
            idle: sprite::load_strip(idle_png, FRAME_W),
            walk,
            walk_flip,
            eat: eat_png
                .map(|p| sprite::load_strip(p, FRAME_W))
                .unwrap_or_default(),
            base,
            cols,
            rows,
            faces_right,
            frame_scale,
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

    fn set_home(&mut self, slot: Slot) {
        self.range = (f64::from(slot.range.0), f64::from(slot.range.1));
        self.x = f64::from(slot.start).clamp(self.range.0, self.range.1);
        self.last = None;
    }

    fn update(&mut self, dt: f64) {
        self.state_time += dt;
        self.frame_time += dt;

        // An animal with an eat cycle reaches for the hunny after idling;
        // the others go for a wander instead.
        let after_idle = if self.eat.is_empty() {
            State::Walking
        } else {
            State::Eating
        };
        let (duration, mut frame_dt, next) = match self.state {
            State::Asleep => (self.durations.0, 0.7, State::Idle),
            State::Idle => (self.durations.1, 0.4, after_idle),
            State::Walking => (self.durations.2, 0.22, State::Asleep),
            State::Eating => (EAT_DURATION, EAT_FRAME_DT, State::Asleep),
        };
        // Slow the breathing/wandering states per animal; the eat cycle is
        // already tuned, so it keeps its own timing.
        if self.state != State::Eating {
            frame_dt *= self.frame_scale;
        }

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
            State::Eating => (&self.eat, self.base + EAT_OFF),
        }
    }

    fn current_id(&self) -> u32 {
        let (frames, id_base) = self.frames();
        id_base + (self.frame % frames.len().max(1)) as u32
    }

    fn transmit_all(&self, out: &mut dyn Write) -> io::Result<()> {
        let sets: [(&[Sprite], u32); 5] = [
            (&self.asleep, self.base + ASLEEP_OFF),
            (&self.idle, self.base + IDLE_OFF),
            (&self.walk, self.base + WALK_OFF),
            (&self.walk_flip, self.base + WALK_FLIP_OFF),
            (&self.eat, self.base + EAT_OFF),
        ];
        for (frames, id_base) in sets {
            for (i, s) in frames.iter().enumerate() {
                kitty::transmit(out, id_base + i as u32, s.width, s.height, &s.rgba)?;
            }
        }
        Ok(())
    }

    fn place(&mut self, out: &mut dyn Write, l: &Layout) -> io::Result<()> {
        let row = l.animal_row();
        let id = self.current_id();
        let col = self.x as u16;
        if self.last != Some((id, col, row)) {
            if let Some((old_id, _, _)) = self.last {
                if old_id != id {
                    kitty::delete_placement(out, old_id)?;
                }
            }
            kitty::place(out, id, col, row, self.cols, self.rows)?;
            self.last = Some((id, col, row));
        }
        Ok(())
    }
}

pub struct Animals {
    capy: Critter,
    sloth: Critter,
    badger: Critter,
    transmitted: bool,
}

impl Animals {
    pub fn new() -> Self {
        let (cols, rows) = (10, 5);
        Self {
            // Capybara: unbothered, ambles a little.
            capy: Critter::new(
                CAPY_IDLE,
                Some(CAPY_WALK),
                CAPY_ASLEEP,
                None,
                CAPY_BASE,
                cols,
                rows,
                true,
                1.0,
                (8.0, 6.0, 7.0),
                State::Idle,
            ),
            // Sloth: mostly dozing, breathes at half speed, and slowly reaches
            // for the hunny pot.
            sloth: Critter::new(
                SLOTH_IDLE,
                None,
                SLOTH_ASLEEP,
                Some(SLOTH_EAT),
                SLOTH_BASE,
                cols,
                rows,
                true,
                2.0,
                (22.0, 10.0, 3.0),
                State::Asleep,
            ),
            // Honey badger: restless, patrols the widest range.
            badger: Critter::new(
                BADGER_IDLE,
                Some(BADGER_WALK),
                BADGER_ASLEEP,
                None,
                BADGER_BASE,
                cols,
                rows,
                true,
                1.0,
                (6.0, 5.0, 9.0),
                State::Walking,
            ),
            transmitted: false,
        }
    }

    pub fn init(&mut self, l: &Layout) {
        self.capy.set_home(l.capy);
        self.sloth.set_home(l.sloth);
        self.badger.set_home(l.badger);
        // A resize re-runs init(); terminals drop transmitted images on resize,
        // so re-send them (not just re-place) or the sprites vanish.
        self.transmitted = false;
    }

    pub fn update(&mut self, dt: f64) {
        self.capy.update(dt);
        self.sloth.update(dt);
        self.badger.update(dt);
    }

    /// (center col, half width) of each critter, for the shadow pools.
    pub fn spots(&self) -> [(u16, u16); 3] {
        [&self.badger, &self.sloth, &self.capy]
            .map(|c| (c.x as u16 + c.cols / 2, c.cols / 2))
    }

    pub fn post_draw(&mut self, out: &mut dyn Write, l: &Layout) -> io::Result<()> {
        if l.w == 0 || l.ground_y < l.animal_rows {
            return Ok(());
        }
        if !self.transmitted {
            self.capy.transmit_all(out)?;
            self.sloth.transmit_all(out)?;
            self.badger.transmit_all(out)?;
            self.transmitted = true;
        }
        // Draw sloth last so the middle friend sits in front where the group
        // overlaps.
        self.badger.place(out, l)?;
        self.capy.place(out, l)?;
        self.sloth.place(out, l)
    }
}
