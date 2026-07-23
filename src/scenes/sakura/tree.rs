//! The sakura tree, ported from csakura (C/ncurses): a metaball canopy of
//! blossom clouds with depth shading, a recursive branch skeleton whose tips
//! grow the clouds, a thick tapering trunk, and a petal carpet on the bank.
//! Generated once into a cell grid at init/resize; draw just blits it.

use rand::Rng;
use ratatui::buffer::Buffer;
use ratatui::style::Color;

use super::layout::Layout;
use super::paint;
use super::palette::*;

/// Fixed generation seed: one designed tree, identical on every launch.
pub const SEED: u64 = 7;

const MAX_BLOBS: usize = 64;
const MAX_TIPS: usize = 64;

const BLOOMS: [char; 4] = ['❀', '✿', '❁', '✽'];

struct Blob {
    x: f64,
    y: f64,
    rx: f64,
    ry: f64,
}

#[derive(Default)]
pub struct Tree {
    w: u16,
    h: u16,
    cells: Vec<Option<(char, Color)>>,
    /// Canopy cells petals can spawn from (edges and lump undersides).
    pub sources: Vec<(u16, u16)>,
}

fn frange(rng: &mut impl Rng, a: f64, b: f64) -> f64 {
    rng.gen_range(a..b)
}

impl Tree {
    fn put(&mut self, x: i32, y: i32, ch: char, color: Color) {
        if x < 0 || y < 0 || x >= i32::from(self.w) || y >= i32::from(self.h) {
            return;
        }
        self.cells[y as usize * usize::from(self.w) + x as usize] = Some((ch, color));
    }

    pub fn generate(&mut self, l: &Layout, rng: &mut impl Rng) {
        self.w = l.w;
        self.h = l.h;
        self.cells.clear();
        self.cells.resize(usize::from(l.w) * usize::from(l.h), None);
        self.sources.clear();

        if l.w < 20 || l.h < 12 {
            return;
        }

        let w = f64::from(l.w);
        let h = f64::from(l.h);

        // A leaning trunk on the right; the canopy spreads wide to the left
        // across the top of the screen, like the reference art.
        let base_x = f64::from(l.tree_cx) + frange(rng, -1.0, 1.0);
        let base_y = h - 2.0;
        let fork_y = h * 0.40;
        let lean = (w * 0.05).clamp(2.0, 10.0);
        let fork_x = base_x - lean;

        // Branches must stay inside this box.
        let br_xmin = w * 0.48;
        let br_xmax = w - 2.0;
        let br_ymin = (h * 0.08).max(2.0);

        // Petal carpet under the whole spread of the crown.
        self.carpet(l, w * 0.72, w * 0.16, rng); // first, so the tree overdraws it
        self.trunk(base_x, base_y, fork_x, fork_y, rng);

        // Limbs fan from up-right to hard left; nodes are collected along the
        // way so blossom clouds follow the branches (true = branch tip).
        let mut tips: Vec<(f64, f64, bool)> = Vec::with_capacity(MAX_TIPS);
        let limbs = 4 + rng.gen_range(0..2);
        let reach = (w * 0.10).clamp(8.0, 26.0);
        for i in 0..limbs {
            let f = f64::from(i) / f64::from((limbs - 1).max(1));
            let na = 1.55 + f * 1.35 + frange(rng, -0.12, 0.12);
            let sy = fork_y + frange(rng, 0.0, h * 0.08);
            let t = ((base_y - sy) / (base_y - fork_y).max(1.0)).clamp(0.0, 1.0);
            let sx = base_x - lean * t.powf(1.35);
            self.branch(
                sx,
                sy,
                na,
                reach * frange(rng, 0.75, 1.05),
                0,
                (br_xmin, br_xmax, br_ymin),
                &mut tips,
                rng,
            );
        }

        // One limb reaches out to the right of the trunk so the crown doesn't
        // stop dead at the fork; its clouds fill the upper right.
        {
            let sy = fork_y + frange(rng, 0.0, h * 0.05);
            let t = ((base_y - sy) / (base_y - fork_y).max(1.0)).clamp(0.0, 1.0);
            let sx = base_x - lean * t.powf(1.35);
            self.branch(
                sx,
                sy,
                0.45 + frange(rng, -0.10, 0.10),
                reach * frange(rng, 0.45, 0.60),
                0,
                (br_xmin, br_xmax, br_ymin),
                &mut tips,
                rng,
            );
        }

        // Blossom clouds: a soft core over the fork plus big clouds along the
        // branches, largest at the tips. Few LARGE blobs keep the metaball
        // field smooth so the canopy shades like one lush mass (csakura
        // style) instead of dissolving into noise.
        let mut blobs: Vec<Blob> = Vec::with_capacity(MAX_BLOBS);
        blobs.push(Blob {
            x: fork_x - w * 0.02,
            y: h * 0.20,
            rx: w * 0.07,
            ry: w * 0.040,
        });
        for &(nx, ny, is_tip) in tips.iter() {
            if blobs.len() >= MAX_BLOBS {
                break;
            }
            let (bl, bu) = if is_tip {
                (0.050, 0.080)
            } else {
                (0.035, 0.055)
            };
            let rx = w * frange(rng, bl, bu);
            blobs.push(Blob {
                x: nx + frange(rng, -1.5, 1.5),
                y: ny + frange(rng, -2.0, 1.5), // clouds wrap the wood
                rx,
                ry: rx * frange(rng, 0.45, 0.60), // cell aspect: wider than tall
            });
        }

        // Hanging clusters: small blobs dropped below existing clouds so the
        // underside dips and scallops instead of ending in a straight line.
        let hangs = 6 + rng.gen_range(0..3);
        for _ in 0..hangs {
            if blobs.len() >= MAX_BLOBS {
                break;
            }
            let i = rng.gen_range(0..blobs.len());
            let (bx, by, brx, bry) = (blobs[i].x, blobs[i].y, blobs[i].rx, blobs[i].ry);
            let rx = brx * frange(rng, 0.35, 0.55);
            blobs.push(Blob {
                x: bx + brx * frange(rng, -0.6, 0.6),
                y: by + bry * frange(rng, 0.7, 1.1),
                rx,
                ry: rx * frange(rng, 0.45, 0.60),
            });
        }

        self.canopy(&blobs, rng); // canopy overdraws branch tops
    }

    /// True if the generated tree has a wood cell (trunk or branch) here.
    pub fn is_wood(&self, x: u16, y: u16) -> bool {
        if x >= self.w || y >= self.h {
            return false;
        }
        matches!(
            self.cells[usize::from(y) * usize::from(self.w) + usize::from(x)],
            Some((_, c)) if c == TRUNK_D || c == TRUNK_M || c == TRUNK_L
        )
    }

    pub fn draw(&self, buf: &mut Buffer) {
        let w = usize::from(self.w.min(buf.area.width));
        let h = usize::from(self.h.min(buf.area.height));
        for y in 0..h {
            for x in 0..w {
                if let Some((ch, color)) = self.cells[y * usize::from(self.w) + x] {
                    paint::glyph_over(buf, x as u16, y as u16, ch, color);
                }
            }
        }
    }

    fn trunk(&mut self, base_x: f64, base_y: f64, fork_x: f64, fork_y: f64, rng: &mut impl Rng) {
        let hgt = (base_y - fork_y).max(2.0);
        let maxw = (f64::from(self.w) * 0.028).clamp(2.5, 5.5);
        let wobble = frange(rng, 0.6, 1.6);

        let steps = (hgt * 2.0) as i32 + 2;
        for i in 0..=steps {
            let t = f64::from(i) / f64::from(steps);
            let y = base_y - t * hgt;
            // Lean left harder toward the fork, with a gnarled wiggle.
            let x = base_x + (fork_x - base_x) * t.powf(1.35) + (t * 7.0).sin() * wobble * t;

            // Taper upward, flare at the roots.
            let w = maxw * (1.0 - 0.50 * t) * (1.0 + 0.7 * (-t * 9.0).exp()) + 0.5;

            let mut dx = -w as i32;
            while f64::from(dx) <= w {
                let mut color = if f64::from(dx) < -w * 0.35 {
                    TRUNK_D
                } else if f64::from(dx) > w * 0.45 {
                    TRUNK_L
                } else {
                    TRUNK_M
                };
                // Bark streaks so the wide trunk doesn't read flat.
                if rng.gen_bool(0.08) {
                    color = TRUNK_D;
                }
                self.put((x + f64::from(dx)) as i32, y as i32, '█', color);
                dx += 1;
            }
        }
    }

    /// Recursive limb: wanders upward, bounces off the crown box, splits into
    /// 2-3 children; every few steps a node is recorded so blossom clouds can
    /// trail the branch (endpoints are marked as tips and grow bigger clouds).
    #[allow(clippy::too_many_arguments)]
    fn branch(
        &mut self,
        mut x: f64,
        mut y: f64,
        mut angle: f64,
        len: f64,
        depth: u32,
        bbox: (f64, f64, f64),
        tips: &mut Vec<(f64, f64, bool)>,
        rng: &mut impl Rng,
    ) {
        let (xmin, xmax, ymin) = bbox;
        let mut t = 0.0;
        while t < len {
            x += angle.cos() * 1.7;
            y -= angle.sin() * 0.62;
            t += 1.0;
            // Blossoms trail the limbs; main limbs stay bare near the trunk
            // so the gnarled wood shows, like the reference.
            let covered = depth >= 1 || t > len * 0.15;
            if covered && (t as u32) % 5 == 0 && tips.len() < MAX_TIPS {
                tips.push((x, y, false));
            }
            angle += frange(rng, -0.10, 0.10);
            angle = angle.clamp(0.15, std::f64::consts::PI - 0.15);

            if x < xmin {
                x = xmin;
                angle = std::f64::consts::PI - angle;
            }
            if x > xmax {
                x = xmax;
                angle = std::f64::consts::PI - angle;
            }
            if y < ymin {
                y = ymin;
                angle = if angle > std::f64::consts::FRAC_PI_2 {
                    std::f64::consts::PI - 0.20
                } else {
                    0.20
                };
            }

            let color = if depth == 0 { TRUNK_M } else { TRUNK_L };
            self.put(x as i32, y as i32, '█', color);
            if depth == 0 {
                // Main limbs are two cells thick.
                self.put(x as i32 + 1, y as i32, '█', TRUNK_D);
            } else if rng.gen_bool(0.35) {
                let side = if rng.gen_bool(0.5) { -1 } else { 1 };
                self.put(x as i32 + side, y as i32, '█', TRUNK_M);
            }
        }

        if depth >= 2 || len < 3.0 {
            if tips.len() < MAX_TIPS {
                tips.push((x, y, true));
            }
            return;
        }

        let kids = 2 + usize::from(rng.gen_bool(0.5));
        for i in 0..kids {
            let spread = frange(rng, 0.40, 0.80);
            let na = match i {
                0 => angle + spread,
                1 => angle - spread,
                _ => angle + frange(rng, -0.25, 0.25),
            };
            self.branch(
                x,
                y,
                na,
                len * frange(rng, 0.55, 0.75),
                depth + 1,
                bbox,
                tips,
                rng,
            );
        }
    }

    fn canopy(&mut self, blobs: &[Blob], rng: &mut impl Rng) {
        let field = |x: f64, y: f64| -> f64 {
            blobs
                .iter()
                .map(|b| {
                    let dx = (x - b.x) / b.rx;
                    let dy = (y - b.y) / b.ry;
                    (-(dx * dx + dy * dy) * 2.2).exp()
                })
                .sum()
        };

        // Bounds of the actual blossom clouds.
        let mut bx0 = f64::MAX;
        let mut bx1 = f64::MIN;
        let mut by0 = f64::MAX;
        let mut by1 = f64::MIN;
        for b in blobs {
            bx0 = bx0.min(b.x - b.rx);
            bx1 = bx1.max(b.x + b.rx);
            by0 = by0.min(b.y - b.ry);
            by1 = by1.max(b.y + b.ry);
        }

        // Shade against the real vertical extent of the clouds.
        let cy = (by0 + by1) / 2.0;
        let ry = ((by1 - by0) / 2.0).max(2.0);

        let mut sources = Vec::new();
        for y in (by0 as i32 - 2)..=(by1 as i32 + 3) {
            if y < 0 || y >= i32::from(self.h) {
                continue;
            }
            for x in (bx0 as i32 - 3)..=(bx1 as i32 + 3) {
                if x < 0 || x >= i32::from(self.w) {
                    continue;
                }

                let xf = f64::from(x);
                let yf = f64::from(y);
                let f = field(xf, yf);
                if f < 0.30 {
                    continue;
                }
                // Scalloped, airy outline.
                if f < 0.42 && rng.gen_bool(0.35) {
                    continue;
                }

                // Vertical position inside the crown drives the base tone.
                let h = ((yf - (cy - ry)) / (2.0 * ry)).clamp(0.0, 1.0);

                // Slightly open underside so the branches peek through.
                if h > 0.62 && f < 0.85 {
                    let openness = (h - 0.62) * 0.85;
                    if rng.gen_bool(openness.clamp(0.0, 1.0)) {
                        continue;
                    }
                }

                let mut shade = h * 5.2 + 0.3 + frange(rng, -0.9, 0.9);

                // Per-lump shading: bright tops, shadowed undersides.
                let fu = field(xf, yf - 1.6);
                if fu > f * 1.12 {
                    shade += 1.7;
                } else if fu < f * 0.88 {
                    shade -= 1.5;
                }

                // csakura's fill: solid cores shading down through the ramp,
                // dithered blooms only toward the airy edges.
                let mut g;
                if f > 0.92 {
                    g = if rng.gen_bool(0.80) { '█' } else { '▓' };
                } else if f > 0.55 {
                    g = if rng.gen_bool(0.60) { '▓' } else { '▒' };
                } else {
                    let r: f64 = rng.gen_range(0.0..1.0);
                    g = if r < 0.45 {
                        '▒'
                    } else if r < 0.85 {
                        BLOOMS[rng.gen_range(0..4)]
                    } else {
                        '·'
                    };
                }

                // Blossom sparkle on the surface.
                if f > 0.55 && rng.gen_bool(0.07) {
                    g = BLOOMS[rng.gen_range(0..4)];
                    shade -= 2.0;
                }

                let idx = shade.clamp(0.0, 7.0) as usize;
                self.put(x, y, g, RAMP[idx]);

                // Petals detach from edges and lump undersides.
                if (f < 0.60 || fu > f * 1.12) && rng.gen_bool(0.5) {
                    sources.push((x as u16, y as u16));
                }
            }
        }
        self.sources = sources;
    }

    /// Petal carpet and grass tufts on the bank, densest under the crown.
    fn carpet(&mut self, l: &Layout, cx: f64, rx: f64, rng: &mut impl Rng) {
        for y in l.ground_y..l.h {
            // Carpet thins out toward the bottom of the bank.
            let depth = f64::from(y - l.ground_y) / f64::from((l.h - l.ground_y).max(1));
            for x in 0..l.w {
                let dx = (f64::from(x) - cx) / (rx * 1.4);
                let p = (-dx * dx * 2.2).exp() * (1.0 - depth * 0.5);
                let r: f64 = rng.gen_range(0.0..1.0);
                if r < p * 0.55 {
                    let rr: f64 = rng.gen_range(0.0..1.0);
                    let g = if rr < 0.25 {
                        BLOOMS[rng.gen_range(0..4)]
                    } else if rr < 0.60 {
                        '·'
                    } else {
                        ','
                    };
                    self.put(i32::from(x), i32::from(y), g, RAMP[3 + rng.gen_range(0..4)]);
                } else if r < p * 0.55 + 0.06 {
                    self.put(i32::from(x), i32::from(y), '"', GRASS_TUFT);
                } else if r < p * 0.55 + 0.10 {
                    self.put(i32::from(x), i32::from(y), ',', GRASS_TUFT);
                }
            }
        }
    }
}
