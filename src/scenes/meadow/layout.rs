//! Scene layout: every element's position derived from the terminal size.
//! Shared by the ratatui background (draw) and the Kitty sprite layer
//! (post_draw) so characters and pixel sprites always line up.

use ratatui::layout::Rect;

const HORIZON_FRAC: f32 = 0.50; // sky meets grass at 50% of height
const GROUND_FRAC: f32 = 0.80; // animals' feet rest here

/// A wandering critter's home: its walk range (min/max left-edge col) and
/// where it starts.
#[derive(Clone, Copy, Default)]
pub struct Slot {
    pub range: (u16, u16),
    pub start: u16,
}

#[derive(Clone, Copy, Default)]
pub struct Layout {
    pub w: u16,
    pub h: u16,
    pub horizon_y: u16,
    pub ground_y: u16,

    // Sky
    pub sun: Rect,

    // Tree (left). The canopy is an ellipse centered on the trunk (its left
    // side can run off-screen) so the foliage sits squarely over the trunk.
    pub trunk: Rect,
    pub canopy_cx: u16,
    pub canopy_cy: u16,
    pub canopy_rx: u16,
    pub canopy_ry: u16,
    pub branch_y: u16,
    pub branch_x_end: u16,

    // A little patch of sunflowers on the grass, lower left. x is the left of
    // the patch, bottom() is the base row, height is the flower height.
    pub sunflowers: Rect,

    // Distant cabin (right)
    pub cabin: Rect,

    // Picnic
    pub blanket: Rect,

    // Sprite cells (col, row, cols, rows)
    pub animal_rows: u16,
    pub capy: Slot,
    pub sloth: Slot,
    pub badger: Slot,
    pub hive: Rect,
    pub hunny: Rect,
    pub book: Rect,
}

impl Layout {
    pub fn new(w: u16, h: u16) -> Self {
        let horizon_y = (h as f32 * HORIZON_FRAC) as u16;
        let ground_y = (h as f32 * GROUND_FRAC) as u16;
        let pw = |p: u16| -> u16 { (u32::from(w) * u32::from(p) / 100) as u16 };

        let animal_rows = 5;

        // Sun in the upper right of the sky.
        let sun = Rect::new(pw(80), 2, 6, 3);

        // Tree on the left: trunk drops to the grass, canopy centered over it.
        let trunk = Rect::new(
            pw(4),
            horizon_y.saturating_sub(1),
            5,
            ground_y.saturating_sub(horizon_y) + 3,
        );
        let canopy_cx = trunk.x + trunk.width / 2;
        let canopy_cy = (f32::from(horizon_y) * 0.42) as u16;
        let canopy_rx = pw(20).max(12);
        let canopy_ry = (f32::from(horizon_y) * 0.60).max(4.0) as u16;

        // A branch reaches out of the right of the foliage; the hive hangs off
        // its tip. branch_x_end is anchored to the leaf edge at branch_y so the
        // branch and hive line up at any terminal size.
        let branch_y = (f32::from(horizon_y) * 0.30) as u16;
        let leaf_edge = ellipse_right(canopy_cx, canopy_cy, canopy_rx, canopy_ry, branch_y)
            .unwrap_or(canopy_cx + canopy_rx);
        let branch_x_end = (leaf_edge + 4).min(w.saturating_sub(4));

        // Beehive hangs from the branch tip (its top nub sits on the branch).
        let hive = Rect::new(branch_x_end.saturating_sub(3), branch_y, 6, 3);

        // A patch of sunflowers standing in the grass to the lower left. They
        // rise from a base row up to their heads; height is clamped to fit.
        let sf_base = (ground_y + 3).min(h.saturating_sub(1));
        let sf_h = 8u16.clamp(4, sf_base.saturating_sub(horizon_y + 1).max(4));
        let sunflowers = Rect::new(pw(11), sf_base.saturating_sub(sf_h), 11, sf_h);

        // Distant cabin near the horizon on the right.
        let cabin = Rect::new(pw(84), horizon_y.saturating_sub(4), 11, 5);

        // The three friends, grouped center: badger, sloth, capybara.
        let sloth = Slot {
            range: (pw(44), pw(48)),
            start: pw(46),
        };
        let badger = Slot {
            range: (pw(26), pw(38)),
            start: pw(32),
        };
        let capy = Slot {
            range: (pw(64), pw(76)),
            start: pw(70),
        };

        // Picnic anchored to the sloth (fixed cell offsets, not a percentage)
        // so the hunny pot always sits right where the sloth's hand reaches,
        // whatever the terminal width. The pot is at the sloth's front-right.
        let picnic_x = sloth.start;
        let hunny = Rect::new(picnic_x + 8, ground_y.saturating_sub(2), 5, 3);
        let book = Rect::new(picnic_x + 2, ground_y.saturating_sub(1), 5, 2);
        let blanket_x = picnic_x + 1;
        let blanket = Rect::new(blanket_x, ground_y, 15.min(w.saturating_sub(blanket_x)), 2);

        Self {
            w,
            h,
            horizon_y,
            ground_y,
            sun,
            trunk,
            canopy_cx,
            canopy_cy,
            canopy_rx,
            canopy_ry,
            branch_y,
            branch_x_end,
            sunflowers,
            cabin,
            blanket,
            animal_rows,
            capy,
            sloth,
            badger,
            hive,
            hunny,
            book,
        }
    }

    /// Top row where a ground-standing animal sprite is placed.
    pub fn animal_row(&self) -> u16 {
        self.ground_y.saturating_sub(self.animal_rows)
    }

    /// Whether cell (x, y) lies inside the leafy canopy ellipse.
    pub fn canopy_contains(&self, x: u16, y: u16) -> bool {
        if self.canopy_rx == 0 || self.canopy_ry == 0 {
            return false;
        }
        let dx = (f32::from(x) - f32::from(self.canopy_cx)) / f32::from(self.canopy_rx);
        let dy = (f32::from(y) - f32::from(self.canopy_cy)) / f32::from(self.canopy_ry);
        dx * dx + dy * dy <= 1.0
    }

    /// Rightmost leaf column at the given row, if the canopy reaches that row.
    pub fn canopy_right(&self, row: u16) -> Option<u16> {
        ellipse_right(
            self.canopy_cx,
            self.canopy_cy,
            self.canopy_rx,
            self.canopy_ry,
            row,
        )
    }
}

/// Rightmost column of the ellipse at `row`, or None if it does not span that
/// row. Free-standing so `Layout::new` can call it before the struct exists.
fn ellipse_right(cx: u16, cy: u16, rx: u16, ry: u16, row: u16) -> Option<u16> {
    if rx == 0 || ry == 0 {
        return None;
    }
    let dy = (f32::from(row) - f32::from(cy)) / f32::from(ry);
    if dy.abs() >= 1.0 {
        return None;
    }
    Some(cx + (f32::from(rx) * (1.0 - dy * dy).sqrt()) as u16)
}
