//! Static decor sprites (beehive, hunny pot, book), a small swarm of bees that
//! bob around the hive and the hunny pot, and a distant windmill whose sails
//! turn slowly.

use std::io::{self, Write};

use ratatui::layout::Rect;

use crate::kitty;
use crate::scenes::meadow::layout::Layout;
use crate::sprite::{self, Sprite};

const HIVE: &[u8] = include_bytes!("../../../../assets/meadow/props/beehive.png");
const BEE: &[u8] = include_bytes!("../../../../assets/meadow/props/bee.png");
const HUNNY: &[u8] = include_bytes!("../../../../assets/meadow/props/hunny.png");
const BOOK: &[u8] = include_bytes!("../../../../assets/meadow/props/book.png");
const WINDMILL: &[u8] = include_bytes!("../../../../assets/meadow/props/windmill.png");

const FRAME_W: u16 = 64;
const WINDMILL_FRAME_W: u16 = 100;
const WINDMILL_SPIN_DT: f64 = 0.5; // seconds per rotor frame

const HIVE_ID: u32 = 400;
const HUNNY_ID: u32 = 420;
const BOOK_ID: u32 = 430;
const BEE_BASE: u32 = 410; // one image id per bee (410..)
const WINDMILL_BASE: u32 = 440; // one image id per rotor frame (440..)
const BEE_CELLS: (u16, u16) = (3, 2);

/// What a bee hovers around.
#[derive(Clone, Copy)]
enum Anchor {
    Hive,
    Hunny,
}

/// A bee bobbing in a little ellipse around its anchor.
struct Bee {
    id: u32,
    anchor: Anchor,
    off: (f32, f32),
    amp: (f32, f32),
    speed: f32,
    phase: f32,
    last: Option<(u16, u16)>,
}

/// The swarm: three bees at the hive, two at the hunny pot. Offsets and orbits
/// are relative to each anchor's top-left cell.
fn bees() -> Vec<Bee> {
    let specs = [
        (Anchor::Hive, (7.0, 0.0), (2.0, 1.3), 2.0, 0.0),
        (Anchor::Hive, (4.0, -2.5), (1.6, 1.6), 2.7, 1.7),
        (Anchor::Hive, (-2.5, 2.5), (2.3, 1.1), 1.7, 3.2),
        (Anchor::Hunny, (5.5, -2.0), (1.6, 1.1), 2.3, 0.6),
        (Anchor::Hunny, (-1.5, -1.5), (1.3, 1.3), 3.1, 2.4),
    ];
    specs
        .into_iter()
        .enumerate()
        .map(|(i, (anchor, off, amp, speed, phase))| Bee {
            id: BEE_BASE + i as u32,
            anchor,
            off,
            amp,
            speed,
            phase,
            last: None,
        })
        .collect()
}

/// Atmospheric perspective for the distant windmill: pull every pixel toward
/// the horizon haze tone so it reads as far away instead of popping forward
/// with the same contrast as the near animals.
fn hazed(mut s: Sprite) -> Sprite {
    const HAZE: [u8; 3] = [176, 196, 184];
    for px in s.rgba.chunks_exact_mut(4) {
        if px[3] > 0 {
            for (c, h) in px[..3].iter_mut().zip(HAZE) {
                *c = (f32::from(*c) + (f32::from(h) - f32::from(*c)) * 0.32) as u8;
            }
        }
    }
    s
}

pub struct Decor {
    hive: Sprite,
    bee: Sprite,
    hunny: Sprite,
    book: Sprite,
    windmill: Vec<Sprite>,
    bees: Vec<Bee>,
    transmitted: bool,
    placed: bool,
    windmill_last: Option<(u32, u16, u16)>,
    t: f64,
}

impl Decor {
    pub fn new() -> Self {
        Self {
            hive: sprite::from_png_bytes(HIVE),
            // The bee strip's first frame carries the bee; take just that.
            bee: sprite::load_strip(BEE, FRAME_W).swap_remove(0),
            hunny: sprite::from_png_bytes(HUNNY),
            book: sprite::from_png_bytes(BOOK),
            windmill: sprite::load_strip(WINDMILL, WINDMILL_FRAME_W)
                .into_iter()
                .map(hazed)
                .collect(),
            bees: bees(),
            transmitted: false,
            placed: false,
            windmill_last: None,
            t: 0.0,
        }
    }

    pub fn init(&mut self) {
        self.placed = false;
        // Resize drops transmitted images, so re-send them on the next draw.
        self.transmitted = false;
        self.windmill_last = None;
        for bee in &mut self.bees {
            bee.last = None;
        }
    }

    pub fn set_time(&mut self, t: f64) {
        self.t = t;
    }

    fn transmit(&mut self, out: &mut dyn Write) -> io::Result<()> {
        for (id, s) in [
            (HIVE_ID, &self.hive),
            (HUNNY_ID, &self.hunny),
            (BOOK_ID, &self.book),
        ] {
            kitty::transmit(out, id, s.width, s.height, &s.rgba)?;
        }
        // Every bee shares the same image, transmitted under its own id so the
        // placements are independent.
        let b = &self.bee;
        for bee in &self.bees {
            kitty::transmit(out, bee.id, b.width, b.height, &b.rgba)?;
        }
        // One image id per windmill rotor frame.
        for (i, s) in self.windmill.iter().enumerate() {
            kitty::transmit(out, WINDMILL_BASE + i as u32, s.width, s.height, &s.rgba)?;
        }
        self.transmitted = true;
        Ok(())
    }

    pub fn post_draw(&mut self, out: &mut dyn Write, l: &Layout) -> io::Result<()> {
        if !self.transmitted {
            self.transmit(out)?;
        }

        // Distant windmill: cycle the rotor frames so the sails turn. Drawn
        // first so it sits behind the foreground props and animals.
        if !self.windmill.is_empty() {
            let frame = (self.t / WINDMILL_SPIN_DT) as usize % self.windmill.len();
            let id = WINDMILL_BASE + frame as u32;
            let m = l.windmill;
            let cur = Some((id, m.x, m.y));
            if self.windmill_last != cur && m.right() <= l.w && m.bottom() <= l.h {
                if let Some((old, _, _)) = self.windmill_last {
                    if old != id {
                        kitty::delete_placement(out, old)?;
                    }
                }
                kitty::place(out, id, m.x, m.y, m.width, m.height)?;
                self.windmill_last = cur;
            }
        }

        if !self.placed {
            let fits = |r: Rect| r.right() <= l.w && r.bottom() <= l.h;
            for (id, r) in [(HIVE_ID, l.hive), (HUNNY_ID, l.hunny), (BOOK_ID, l.book)] {
                if fits(r) {
                    kitty::place(out, id, r.x, r.y, r.width, r.height)?;
                }
            }
            self.placed = true;
        }

        let (bw, bh) = BEE_CELLS;
        for bee in &mut self.bees {
            let (ax, ay) = match bee.anchor {
                Anchor::Hive => (l.hive.x, l.hive.y),
                Anchor::Hunny => (l.hunny.x, l.hunny.y),
            };
            let x = f32::from(ax)
                + bee.off.0
                + bee.amp.0 * (self.t as f32 * bee.speed + bee.phase).sin();
            let y = f32::from(ay)
                + bee.off.1
                + bee.amp.1 * (self.t as f32 * bee.speed * 1.3 + bee.phase).cos();
            let col = x.max(0.0) as u16;
            let row = y.max(0.0) as u16;
            if bee.last != Some((col, row)) && col + bw <= l.w && row + bh <= l.h {
                kitty::place(out, bee.id, col, row, bw, bh)?;
                bee.last = Some((col, row));
            }
        }
        Ok(())
    }
}
