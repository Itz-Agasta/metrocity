//! The elevated rail and the train that crosses it right to left,
//! then waits two seconds before the next run.

use ratatui::buffer::Buffer;

use crate::scenes::harbor::layout::Layout;
use crate::scenes::harbor::palette::*;
use crate::scenes::harbor::{paint, FPS};

/// Cells per frame.
const SPEED: f32 = 0.9;

pub fn draw(buf: &mut Buffer, l: &Layout, frame: u32) {
    let rail = i32::from(l.rail);
    let w = i32::from(l.w);
    for x in 0..w {
        let bg = paint::bg_at(buf, x, rail);
        paint::set(buf, x, rail, '▀', RAIL, bg);
    }
    if rail < 3 {
        return;
    }

    let len = (w / 3).max(18);
    let travel = ((w + len + 10) as f32 / SPEED) as u32;
    let f = frame % (travel + FPS * 2);
    if f >= travel {
        return;
    }
    let tx = (w as f32 + 4.0 - SPEED * f as f32).round() as i32;
    let (roof, body) = (rail - 3, rail - 2);
    let lit = paint::mix(WINDOW_COOL, WINDOW_WARM, 0.3);
    for i in 0..len {
        let x = tx + i;
        let car = i % 12;
        if i == 0 {
            paint::set(buf, x, roof, '▗', TRAIN, paint::bg_at(buf, x, roof));
            paint::fill(buf, x, body, TRAIN);
        } else if car != 11 {
            // car == 11 is the gap between two cars.
            paint::set(buf, x, roof, '▄', TRAIN, paint::bg_at(buf, x, roof));
            if car % 3 == 1 {
                paint::set(buf, x, body, '█', lit, TRAIN);
            } else {
                paint::fill(buf, x, body, TRAIN);
            }
        }
        paint::set(buf, x, rail, '▀', RAIL, paint::scale(TRAIN, 0.7));
    }
    // Headlight glow fading ahead of the nose.
    for i in 1..=4 {
        let x = tx - i;
        let bg = paint::bg_at(buf, x, body);
        paint::fill(buf, x, body, paint::mix(bg, HEADLIGHT, 0.5 / i as f32));
    }
}
