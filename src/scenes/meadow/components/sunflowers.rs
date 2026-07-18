//! Two little clusters of sunflowers standing in the grass, nodding in the
//! breeze: a deep-orange trio tucked against the tree trunk and a yellow trio
//! to its right.

use ratatui::buffer::Buffer;
use ratatui::style::Color;

use crate::scenes::meadow::layout::Layout;
use crate::scenes::meadow::paint;
use crate::scenes::meadow::palette::*;

/// A flower colour scheme: petal face, darker outer petals, and center.
struct Bloom {
    petal: Color,
    dark: Color,
    center: Color,
}

pub fn draw(buf: &mut Buffer, l: &Layout, t: f64) {
    let s = l.sunflowers;
    if s.width == 0 || s.height == 0 {
        return;
    }
    let base = s.bottom();
    let orange = Bloom {
        petal: PETAL_ORANGE,
        dark: PETAL_ORANGE_DARK,
        center: FLOWER_CENTER,
    };
    let yellow = Bloom {
        petal: PETAL,
        dark: PETAL_DARK,
        center: FLOWER_CENTER,
    };
    // Orange trio tucked against the tree, then the yellow trio to its right.
    cluster(buf, s.x.saturating_sub(9), base, s.height, t, 0.9, &orange);
    cluster(buf, s.x, base, s.height, t, 0.0, &yellow);
}

/// Three flowers of slightly different height, nodding out of phase.
fn cluster(buf: &mut Buffer, bx0: u16, base: u16, h: u16, t: f64, phase_off: f32, bloom: &Bloom) {
    for &(dx, dh, phase) in &[(0i16, 0i16, 0.0f32), (4, -1, 2.1), (8, 1, 4.2)] {
        let bx = (bx0 as i16 + dx).max(0) as u16;
        let fh = (h as i16 + dh).clamp(4, 12) as u16;
        sunflower(buf, bx, base, fh, t, phase + phase_off, bloom);
    }
}

/// One sunflower rising from `base` at column `bx`: a stem that curves in the
/// breeze, broad leaves alternating up it, and a round head that nods.
/// paint::fill bounds-checks every cell.
fn sunflower(buf: &mut Buffer, bx: u16, base: u16, h: u16, t: f64, phase: f32, bloom: &Bloom) {
    // Heavy heads nod slowly; the sway grows toward the top of the stem.
    let sway = ((t as f32) * 1.2 + phase).sin() * 2.2;
    let stem_x = |frac: f32| f32::from(bx) + sway * frac;
    // Walk up the stalk, bridging each row to the previous column so the curve
    // stays a connected stem instead of leaving diagonal gaps as it sways.
    let mut prev: Option<u16> = None;
    for r in 0..h {
        let x = stem_x(f32::from(r) / f32::from(h.max(1)));
        if x < 0.0 {
            // Off-screen: forget prev so re-entry doesn't bridge a stale span.
            prev = None;
            continue;
        }
        let xc = x as u16;
        let row = base.saturating_sub(r);
        let (a, b) = match prev {
            Some(p) if p <= xc => (p, xc),
            Some(p) => (xc, p),
            None => (xc, xc),
        };
        for xx in a..=b {
            paint::fill(buf, xx, row, STEM);
        }
        prev = Some(xc);
    }
    // Broad leaves alternating up the stem (skip any that fall off a short stem).
    for &(frac, dir) in &[(0.34f32, 1i32), (0.54, -1), (0.72, 1)] {
        let r = (frac * f32::from(h)) as u16;
        if r == 0 || r >= h {
            continue;
        }
        stem_leaf(buf, stem_x(frac), base.saturating_sub(r), dir);
    }
    // The 3x3 head at the top: petals around a dark center.
    let hx = stem_x(1.0) as i32;
    let hy = base.saturating_sub(h) as i32;
    for dy in -1i32..=1 {
        for dx in -1i32..=1 {
            let (x, y) = (hx + dx, hy + dy);
            if x < 0 || y < 0 {
                continue;
            }
            let color = if dx == 0 && dy == 0 {
                bloom.center
            } else if dx.abs() == 1 && dy.abs() == 1 {
                bloom.dark
            } else {
                bloom.petal
            };
            paint::fill(buf, x as u16, y as u16, color);
        }
    }
}

/// A broad leaf reaching out from the stem and drooping at its tip. Uses the
/// brighter leaf green so it reads against the grass.
fn stem_leaf(buf: &mut Buffer, sx: f32, y: u16, dir: i32) {
    let sx = sx as i32;
    for i in 1..=2 {
        let x = sx + dir * i;
        if x >= 0 {
            paint::fill(buf, x as u16, y, LEAF_LIGHT);
        }
    }
    let tip = sx + dir * 2;
    if tip >= 0 {
        paint::fill(buf, tip as u16, y + 1, LEAF);
    }
}
