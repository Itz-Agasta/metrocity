//! Hardcoded night colors for the sakura scene.

use ratatui::style::Color;

// Sky
pub const SKY_TOP: Color = Color::Rgb(0x2a, 0x1a, 0x3e);
pub const SKY_HORIZON: Color = Color::Rgb(0x6b, 0x2d, 0x5c);
pub const STAR: Color = Color::Rgb(0xe8, 0xd8, 0xff);
pub const MOON: Color = Color::Rgb(0xf5, 0xe6, 0xa8);

// Distance
pub const MOUNTAIN_FAR: Color = Color::Rgb(0x4f, 0x35, 0x6d);
pub const MOUNTAIN_NEAR: Color = Color::Rgb(0x3d, 0x2b, 0x58);
pub const HILL_FAR: Color = Color::Rgb(0x3a, 0x2a, 0x52);

// Water
pub const LAKE: Color = Color::Rgb(0x24, 0x18, 0x38);
pub const LAKE_SHIMMER: Color = Color::Rgb(0x4a, 0x2e, 0x66);
pub const REFLECT_WARM: Color = Color::Rgb(0xc9, 0x82, 0x2e);

// Blossom ramp, lightest to deepest (csakura's 8-tone shade ramp),
// muted lavender-pink like the reference art.
pub const RAMP: [Color; 8] = [
    Color::Rgb(0xfc, 0xd9, 0xea),
    Color::Rgb(0xf6, 0xc2, 0xdc),
    Color::Rgb(0xe6, 0x9a, 0xc2),
    Color::Rgb(0xd8, 0x86, 0xb2),
    Color::Rgb(0xc4, 0x70, 0xa0),
    Color::Rgb(0xa8, 0x57, 0x88),
    Color::Rgb(0x87, 0x44, 0x6e),
    Color::Rgb(0x65, 0x33, 0x52),
];

// Tree
pub const TRUNK_D: Color = Color::Rgb(0x2f, 0x1c, 0x22);
pub const TRUNK_M: Color = Color::Rgb(0x4e, 0x32, 0x38);
pub const TRUNK_L: Color = Color::Rgb(0x70, 0x4c, 0x50);

// Structures
pub const LANTERN_GLOW: Color = Color::Rgb(0xff, 0xb3, 0x47);

// Far-shore city skyline
pub const SKYLINE: Color = Color::Rgb(0x1f, 0x16, 0x30);
pub const SKYLINE_LIT: Color = Color::Rgb(0x2a, 0x1e, 0x3c);
pub const WINDOW_WARM: Color = Color::Rgb(0xd9, 0x96, 0x3f);
pub const WINDOW_UNLIT: Color = Color::Rgb(0x45, 0x36, 0x60);

// Ground
pub const GRASS: Color = Color::Rgb(0x3e, 0x32, 0x52);
pub const GRASS_DARK: Color = Color::Rgb(0x30, 0x26, 0x42);
pub const GRASS_TUFT: Color = Color::Rgb(0x5a, 0x4a, 0x72);
