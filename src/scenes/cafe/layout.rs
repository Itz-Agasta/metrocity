//! Scene layout: every element's position derived from the terminal size.
//! Shared by the ratatui components (draw) and the Kitty sprite layer
//! (post_draw) so characters and pixel sprites always line up.

use ratatui::layout::Rect;

const COUNTER_TOP_FRAC: f32 = 0.60; // counter top edge at 60% of height
const FLOOR_FRAC: f32 = 0.85; // floor starts at 85% of height

pub const STOOL_COUNT: u16 = 5;

#[derive(Clone, Copy, Default)]
pub struct Layout {
    pub w: u16,
    pub h: u16,
    /// Row of the counter top highlight. Things "on the counter" have their
    /// bottom row at `counter_y - 1`.
    pub counter_y: u16,
    /// Row of the baseboard line where the floor begins.
    pub floor_y: u16,

    // Wall elements (ratatui)
    pub neon_face: Rect,  // sprite: glowing cat face
    pub neon_text_y: u16, // "CAT" / "CAFE" rows below the face
    pub shelf_x0: u16,
    pub shelf_x1: u16,
    pub shelf_rows: [u16; 2],
    pub menu: Rect,
    pub window: Rect,
    pub lamp_x: u16,

    // Counter elements
    pub terminal: Rect, // CRT box, bottom row = counter_y - 1
    pub mug_x: u16,
    pub register_x: u16,
    pub pastry_case: Rect,

    // Sprites on the counter (bottom row = counter_y - 1)
    pub plant_left: Rect,
    pub plant_right: Rect,

    // Cats (sprite cells)
    pub cat_rows: u16,
    pub cat_cols: u16,
    pub black_cat_range: (u16, u16), // walk range (min col, max col)
    pub white_cat_x: u16,

    pub stool_y: u16,
}

impl Layout {
    pub fn new(w: u16, h: u16) -> Self {
        let counter_y = (h as f32 * COUNTER_TOP_FRAC) as u16;
        let floor_y = (h as f32 * FLOOR_FRAC) as u16;
        let pct = |p: u16| -> u16 { (u32::from(w) * u32::from(p) / 100) as u16 };

        let neon_face = Rect::new(pct(3), 2, 10, 5);
        let neon_text_y = neon_face.bottom() + 1;

        let shelf_x0 = pct(15);
        let shelf_x1 = pct(32).max(shelf_x0 + 8);
        let shelf_rows = [
            (u32::from(counter_y) * 30 / 100) as u16,
            (u32::from(counter_y) * 60 / 100) as u16,
        ];

        let menu_w = 19;
        let menu_h = 11.min(counter_y.saturating_sub(4));
        let menu = Rect::new(pct(40).saturating_sub(menu_w / 2), 2, menu_w, menu_h);

        let win_x0 = pct(56);
        let win_x1 = pct(78);
        let window = Rect::new(
            win_x0,
            2,
            win_x1.saturating_sub(win_x0),
            counter_y.saturating_sub(5),
        );

        let terminal_w = 24.min(w / 3);
        let terminal_h = 7.min(counter_y / 2);
        let terminal = Rect::new(
            pct(48),
            counter_y.saturating_sub(terminal_h),
            terminal_w,
            terminal_h,
        );

        let case_w = 14.min(w / 5);
        let case_h = 8.min(counter_y.saturating_sub(2));
        let pastry_case = Rect::new(
            w.saturating_sub(case_w + 2),
            counter_y.saturating_sub(case_h),
            case_w,
            case_h,
        );

        let cat_rows = 4;
        let cat_cols = 8;
        let plant_left = Rect::new(1, counter_y.saturating_sub(5), 9, 5);
        let plant_right = Rect::new(pct(78), counter_y.saturating_sub(3), 6, 3);

        Self {
            w,
            h,
            counter_y,
            floor_y,
            neon_face,
            neon_text_y,
            shelf_x0,
            shelf_x1,
            shelf_rows,
            menu,
            window,
            lamp_x: pct(88),
            terminal,
            mug_x: pct(32),
            register_x: terminal.right() + 3,
            pastry_case,
            plant_left,
            plant_right,
            cat_rows,
            cat_cols,
            black_cat_range: (pct(12), pct(40)),
            white_cat_x: terminal.left().saturating_sub(cat_cols + 2),
            stool_y: counter_y + (floor_y.saturating_sub(counter_y)) / 2 + 1,
        }
    }

    /// X positions (left edge) of the bar stools, evenly spaced.
    pub fn stool_xs(&self) -> impl Iterator<Item = u16> + '_ {
        (1..=STOOL_COUNT)
            .map(|i| (u32::from(self.w) * u32::from(i) / (u32::from(STOOL_COUNT) + 1)) as u16)
    }
}
