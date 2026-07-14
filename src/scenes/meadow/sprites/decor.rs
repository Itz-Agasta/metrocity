//! Static decor sprites (beehive, hunny pot, book) plus a small swarm of bees
//! that bob around the hive and the hunny pot.

use std::io::{self, Write};

use ratatui::layout::Rect;

use crate::kitty;
use crate::scenes::meadow::layout::Layout;
use crate::sprite::{self, Sprite};

const HIVE: &[u8] = include_bytes!("../../../../assets/meadow/props/beehive.png");
const BEE: &[u8] = include_bytes!("../../../../assets/meadow/props/bee.png");
const HUNNY: &[u8] = include_bytes!("../../../../assets/meadow/props/hunny.png");
const BOOK: &[u8] = include_bytes!("../../../../assets/meadow/props/book.png");

const FRAME_W: u16 = 64;

const HIVE_ID: u32 = 400;
const HUNNY_ID: u32 = 420;
const BOOK_ID: u32 = 430;
const BEE_BASE: u32 = 410; // one image id per bee (410..)
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

pub struct Decor {
    hive: Sprite,
    bee: Sprite,
    hunny: Sprite,
    book: Sprite,
    bees: Vec<Bee>,
    transmitted: bool,
    placed: bool,
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
            bees: bees(),
            transmitted: false,
            placed: false,
            t: 0.0,
        }
    }

    pub fn init(&mut self) {
        self.placed = false;
        // Resize drops transmitted images, so re-send them on the next draw.
        self.transmitted = false;
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
        self.transmitted = true;
        Ok(())
    }

    pub fn post_draw(&mut self, out: &mut dyn Write, l: &Layout) -> io::Result<()> {
        if !self.transmitted {
            self.transmit(out)?;
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
