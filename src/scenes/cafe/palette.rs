//! Hardcoded cafe palette, shared by all cafe components.

use ratatui::style::Color;

// Wall
pub const WALL: Color = Color::Rgb(54, 31, 31);
pub const WALL_DARK: Color = Color::Rgb(30, 18, 18);
// Counter
pub const COUNTER_TOP: Color = Color::Rgb(178, 101, 49);
pub const COUNTER_BODY: Color = Color::Rgb(85, 43, 30);
pub const COUNTER_SHADOW: Color = Color::Rgb(60, 30, 20);
// Floor
pub const FLOOR: Color = Color::Rgb(45, 22, 15);
pub const FLOOR_DARK: Color = Color::Rgb(30, 15, 10);
pub const FLOOR_LINE: Color = Color::Rgb(65, 35, 20);
// Wood (frames, shelves, boards)
pub const WOOD: Color = Color::Rgb(110, 62, 32);
pub const WOOD_DARK: Color = Color::Rgb(70, 40, 22);
pub const BOARD_BG: Color = Color::Rgb(24, 17, 13);
// Neon / warm accents
pub const AMBER: Color = Color::Rgb(255, 176, 64);
pub const AMBER_DIM: Color = Color::Rgb(150, 95, 40);
pub const CREAM: Color = Color::Rgb(232, 200, 160);
pub const CREAM_DIM: Color = Color::Rgb(190, 158, 122);
// Window / night
pub const NIGHT: Color = Color::Rgb(24, 38, 70);
pub const NIGHT_DEEP: Color = Color::Rgb(10, 18, 38);
pub const CITY: Color = Color::Rgb(13, 17, 30);
pub const CITY_LIT: Color = Color::Rgb(235, 170, 80);
pub const RAIN: Color = Color::Rgb(110, 140, 190);
// CRT terminal
pub const CRT_GREEN: Color = Color::Rgb(110, 235, 110);
pub const CRT_BG: Color = Color::Rgb(10, 20, 12);
pub const CRT_BEZEL: Color = Color::Rgb(40, 32, 27);
// Stools
pub const STOOL_SEAT: Color = Color::Rgb(18, 11, 8);
pub const STOOL_LEG: Color = Color::Rgb(70, 40, 22);
// Small props
pub const MUG: Color = Color::Rgb(150, 110, 80);
pub const STEAM: Color = Color::Rgb(140, 125, 115);
pub const JAR: Color = Color::Rgb(196, 160, 110);
pub const JAR_LID: Color = Color::Rgb(120, 90, 50);
pub const GLASS: Color = Color::Rgb(92, 82, 74);
