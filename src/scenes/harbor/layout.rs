//! Scene layout: every element's position derived from the terminal size.

#[derive(Clone, Copy, Default)]
pub struct Layout {
    pub w: u16,
    pub h: u16,
    /// First row of the water (sky, skyline and rail end above it).
    pub horizon: u16,
    /// Row of the elevated rail, the last row above the water.
    pub rail: u16,
    /// Moon center (cells) and radius (rows).
    pub moon_x: f32,
    pub moon_y: f32,
    pub moon_r: f32,
    /// Lighthouse center column and lamp row.
    pub lighthouse_x: i32,
    pub lamp_y: i32,
}

impl Layout {
    pub fn new(w: u16, h: u16) -> Self {
        // A tiny window is mostly sky.
        let water = (h / 4).max(3).min(h / 2);
        let horizon = h - water;
        Self {
            w,
            h,
            horizon,
            rail: horizon.saturating_sub(1),
            moon_x: f32::from(w) * 0.8,
            moon_y: f32::from(h) * 0.2,
            moon_r: (f32::from(h) / 9.0).max(2.0),
            lighthouse_x: i32::from(w) * 12 / 100,
            lamp_y: (i32::from(horizon) - i32::from(h) / 4).max(1),
        }
    }
}
