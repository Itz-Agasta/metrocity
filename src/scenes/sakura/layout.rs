//! Scene layout: every element's position derived from the terminal size.
//! Shared by the ratatui components (draw) and the Kitty sprite layer
//! (post_draw) so characters and pixel sprites always line up.

use ratatui::layout::Rect;

const HORIZON_FRAC: f32 = 0.52; // first lake row
const GROUND_FRAC: f32 = 0.78; // first grass-bank row

#[derive(Clone, Copy, Default)]
pub struct Layout {
    pub w: u16,
    pub h: u16,
    /// First row of the lake (sky and far shore end above it).
    pub horizon_y: u16,
    /// First row of the foreground grass bank.
    pub ground_y: u16,

    // Sky
    pub moon: Rect,

    // Far shore
    /// Small pagoda silhouette inside the skyline.
    pub pagoda_x: u16,

    // Right bank
    pub torii: Rect,
    pub lantern_x: u16,
    /// Trunk base column of the sakura tree.
    pub tree_cx: u16,

    // Fox (sprite cells)
    pub fox_cols: u16,
    pub fox_rows: u16,
    pub fox_range: (u16, u16), // walk range (min col, max col)

    // Stone lanterns (sprite cell boxes)
    pub lantern_left: Rect,
    pub lantern_right: Rect,
}

impl Layout {
    pub fn new(w: u16, h: u16) -> Self {
        let horizon_y = ((h as f32 * HORIZON_FRAC) as u16).max(6);
        let ground_y = ((h as f32 * GROUND_FRAC) as u16).clamp(horizon_y + 2, h.saturating_sub(3));
        let pct = |p: u16| -> u16 { (u32::from(w) * u32::from(p) / 100) as u16 };

        let moon = Rect::new(pct(19), 1, 8, 4);

        let torii_w = 12.min(w / 9).max(7);
        let torii_h = 10.min(ground_y.saturating_sub(horizon_y) + 6);
        let torii = Rect::new(
            w.saturating_sub(torii_w + 2),
            (ground_y + 2).saturating_sub(torii_h),
            torii_w,
            torii_h,
        );

        let tree_cx = pct(84);
        let fox_cols = 13;
        let fox_rows = 6;

        // Lanterns: square-ish cell boxes so the square frames keep their
        // proportions. Left one is foreground (bigger, low near the water),
        // right one stands at the foot of the tree.
        let bank_h = h.saturating_sub(ground_y);
        let lantern_left = Rect::new(
            pct(8),
            (ground_y + bank_h * 2 / 3).saturating_sub(4),
            8,
            4,
        );
        let lantern_right = Rect::new(
            tree_cx.saturating_sub(13),
            (ground_y + 5).saturating_sub(4),
            7,
            4,
        );

        Self {
            w,
            h,
            horizon_y,
            ground_y,
            moon,
            pagoda_x: pct(27),
            torii,
            lantern_x: pct(46),
            tree_cx,
            fox_cols,
            fox_rows,
            fox_range: (pct(48), pct(64)),
            lantern_left,
            lantern_right,
        }
    }
}
