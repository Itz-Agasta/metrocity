#!/usr/bin/env python3
"""Generate the meadow scene sprite strips (8-bit RGBA horizontal strips).

Each frame is FRAME px wide. Output goes to assets/meadow/. The metrocity
engine decodes these with sprite::load_strip(bytes, FRAME) and places them via
the Kitty graphics protocol, so the format contract is: 8-bit RGBA PNG, a
horizontal strip whose width is an exact multiple of FRAME.

Setup (only needed to regenerate art; the committed PNGs are enough to build):
    python3 -m venv .venv
    .venv/bin/pip install -r tools/requirements.txt   # just Pillow
Run:
    .venv/bin/python tools/gen_meadow_sprites.py
Regenerating is deterministic; committing the output PNGs is enough to build.
"""

import os
from PIL import Image, ImageDraw

FRAME = 64
H = 64
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "assets", "meadow")

OUTLINE = (58, 42, 30, 255)
SHADOW = (0, 0, 0, 70)


def new_frame():
    img = Image.new("RGBA", (FRAME, H), (0, 0, 0, 0))
    return img, ImageDraw.Draw(img)


def blob(d, box, fill, outline=OUTLINE, w=2):
    d.ellipse(box, fill=fill, outline=outline, width=w)


def rrect(d, box, r, fill, outline=OUTLINE, w=2):
    d.rounded_rectangle(box, radius=r, fill=fill, outline=outline, width=w)


def ground_shadow(d, cx, by, half):
    d.ellipse([cx - half, by - 4, cx + half, by + 4], fill=SHADOW)


def eye(d, x, y, closed):
    if closed:
        d.line([x - 3, y, x + 3, y], fill=OUTLINE, width=2)
    else:
        d.ellipse([x - 3, y - 3, x + 3, y + 3], fill=(30, 22, 18, 255))
        d.ellipse([x - 1, y - 2, x + 1, y], fill=(240, 240, 240, 220))


# ---------------------------------------------------------------- capybara ---
CAPY = dict(body=(176, 122, 69, 255), light=(201, 154, 104, 255),
            dark=(138, 90, 48, 255), snout=(110, 74, 42, 255))


def capybara(pose):
    img, d = new_frame()
    bob = pose["bob"]
    ground_shadow(d, 30, 60, 24)
    # haunch / body
    blob(d, [8, 26 + bob, 52, 60 + bob], CAPY["body"])
    blob(d, [12, 30 + bob, 40, 52 + bob], CAPY["light"], outline=None)
    # legs / feet
    foot = 2 if pose.get("step") else 0
    d.ellipse([16, 54 + bob, 26, 61 + bob], fill=CAPY["dark"], outline=OUTLINE, width=2)
    d.ellipse([34 - foot, 54 + bob, 44 - foot, 61 + bob], fill=CAPY["dark"], outline=OUTLINE, width=2)
    # head: capybara has a big blocky snout
    rrect(d, [36, 20 + bob, 60, 44 + bob], 7, CAPY["body"])
    # ears (small, round, near top)
    blob(d, [40, 16 + bob, 46, 22 + bob], CAPY["dark"], w=1)
    blob(d, [52, 16 + bob, 58, 22 + bob], CAPY["dark"], w=1)
    # blunt muzzle
    rrect(d, [52, 30 + bob, 62, 42 + bob], 4, CAPY["snout"], w=1)
    d.ellipse([57, 33 + bob, 61, 37 + bob], fill=(40, 28, 22, 255))  # nose
    eye(d, 46, 30 + bob, pose["blink"])
    return img


# -------------------------------------------------------------------- sloth ---
SLOTH = dict(fur=(169, 148, 117, 255), light=(198, 180, 150, 255),
             mask=(216, 201, 168, 255), patch=(107, 85, 64, 255),
             claw=(232, 220, 192, 255))


def sloth_foot(d, x, bob):
    """A small clawed foot at the bottom, top-left at (x, 53+bob)."""
    d.ellipse([x, 53 + bob, x + 11, 62 + bob], fill=SLOTH["fur"], outline=OUTLINE, width=2)
    for k in range(3):
        d.line([x + 2 + k * 3, 59 + bob, x + 2 + k * 3, 63 + bob], fill=SLOTH["claw"], width=2)


def sloth_arm(d, sx, sy, hx, hy):
    """An arm from shoulder (sx, sy) to a clawed hand at (hx, hy)."""
    d.line([sx, sy, hx, hy], fill=SLOTH["fur"], width=8)
    d.ellipse([hx - 6, hy - 6, hx + 6, hy + 6], fill=SLOTH["fur"], outline=OUTLINE, width=2)
    for k in range(3):
        d.line([hx - 4 + k * 4, hy + 2, hx - 4 + k * 4, hy + 8], fill=SLOTH["claw"], width=2)


def sloth_body(d, bob, blink):
    """Body, belly, two feet, and the resting arm plus face. The moving hand is
    on the viewer's right, which is the sloth's LEFT hand since it faces us, so
    this is a left-handed sloth. The caller draws that reaching arm."""
    ground_shadow(d, 32, 61, 22)
    blob(d, [12, 22 + bob, 52, 58 + bob], SLOTH["fur"])
    blob(d, [18, 28 + bob, 46, 52 + bob], SLOTH["light"], outline=None)
    # two feet planted at the bottom
    sloth_foot(d, 22, bob)
    sloth_foot(d, 33, bob)
    # resting arm on the viewer's left (the sloth's right hand)
    sloth_arm(d, 16, 40 + bob, 9, 52 + bob)
    # round face
    blob(d, [22, 14 + bob, 44, 38 + bob], SLOTH["fur"])
    blob(d, [25, 18 + bob, 41, 36 + bob], SLOTH["mask"], outline=None)
    # dark eye patches (the signature sloth mask)
    d.ellipse([26, 22 + bob, 32, 30 + bob], fill=SLOTH["patch"])
    d.ellipse([34, 22 + bob, 40, 30 + bob], fill=SLOTH["patch"])
    eye(d, 29, 26 + bob, blink)
    eye(d, 37, 26 + bob, blink)
    d.ellipse([31, 29 + bob, 35, 33 + bob], fill=(60, 44, 34, 255))  # nose
    d.arc([28, 30 + bob, 38, 37 + bob], 20, 160, fill=OUTLINE, width=2)  # smile


def sloth(pose):
    img, d = new_frame()
    bob = pose["bob"]
    sloth_body(d, bob, pose["blink"])
    # reaching arm at rest, viewer's right (the sloth's left hand)
    sloth_arm(d, 48, 40 + bob, 55, 52 + bob)
    return img


# The sloth's LEFT hand (viewer's right) path for the eat cycle: from the side,
# out and down to the hunny pot at its front-right, up to the mouth, munch, and
# back. The reach angles toward the pot rather than straight down. Very slow.
SLOTH_EAT_PATH = [
    (55, 50, False),  # resting at the side
    (59, 56, False),  # reaching out toward the pot
    (62, 61, False),  # at the pot (out to the right, where the hunny sits)
    (55, 52, False),  # lifting away with the honey
    (43, 40, False),  # rising to the face
    (34, 32, True),   # at the mouth, eyes closed
    (35, 34, True),   # munch
    (50, 46, False),  # back toward rest
]


def sloth_eat(hand_x, hand_y, blink):
    img, d = new_frame()
    sloth_body(d, 0, blink)
    sloth_arm(d, 48, 40, hand_x, hand_y)  # the sloth's left hand reaching
    return img


# ------------------------------------------------------------- honey badger ---
# Grizzled GREY mantle (not white) over a long low black body with a stubby
# tail: the three cues that separate a honey badger from a skunk (skunk = white
# stripe, bushy tail, rounder body).
BADGER = dict(mantle=(171, 169, 159, 255), mantle_hi=(198, 196, 186, 255),
              grizzle=(130, 130, 124, 255), body=(40, 40, 44, 255),
              face=(25, 25, 29, 255))


def badger(pose):
    img, d = new_frame()
    bob = pose["bob"]
    ground_shadow(d, 33, 60, 28)
    # Long, low, all-black body (wider than tall, set low: badger proportions).
    blob(d, [4, 34 + bob, 58, 60 + bob], BADGER["body"])
    # Short stubby tail at the back-left (a skunk's is big and bushy).
    d.ellipse([1, 41 + bob, 11, 51 + bob], fill=BADGER["body"], outline=OUTLINE, width=2)
    # Dark face/head pushing forward at the front-right, no neck.
    d.ellipse([42, 35 + bob, 60, 56 + bob], fill=BADGER["face"], outline=OUTLINE, width=2)
    # Grizzled grey dorsal saddle: broad, from the crown over the back, but
    # ending well above the belly so black flanks stay visible on the sides.
    d.pieslice([6, 30 + bob, 52, 50 + bob], 178, 362, fill=BADGER["mantle"], outline=OUTLINE, width=2)
    d.pieslice([12, 32 + bob, 46, 46 + bob], 180, 360, fill=BADGER["mantle_hi"], outline=None)
    d.pieslice([40, 31 + bob, 58, 43 + bob], 180, 360, fill=BADGER["mantle"], outline=None)  # over crown
    # salt-and-pepper grizzle speckles across the saddle
    for gx, gy in [(16, 36), (22, 33), (28, 35), (34, 33), (40, 35), (24, 39), (32, 38)]:
        d.rectangle([gx, gy + bob, gx + 1, gy + 1 + bob], fill=BADGER["grizzle"])
    # Dark legs, low to the ground.
    foot = 2 if pose.get("step") else 0
    d.ellipse([14, 53 + bob, 24, 61 + bob], fill=BADGER["face"], outline=OUTLINE, width=2)
    d.ellipse([38 + foot, 53 + bob, 48 + foot, 61 + bob], fill=BADGER["face"], outline=OUTLINE, width=2)
    # Blunt snout, nose and eye on the dark face.
    d.ellipse([55, 43 + bob, 61, 49 + bob], fill=BADGER["face"], outline=OUTLINE, width=1)
    d.ellipse([57, 44 + bob, 61, 48 + bob], fill=(14, 14, 17, 255))
    eye(d, 49, 42 + bob, pose["blink"])
    return img


# ------------------------------------------------------------------- props ---

def beehive():
    img, d = new_frame()
    # hanging skep hive: stacked golden bands, tapering
    bands = [(20, 10, 44, 20), (16, 18, 48, 30), (14, 28, 50, 42), (18, 40, 46, 52)]
    for i, b in enumerate(bands):
        shade = (217 - i * 8, 164 - i * 6, 65 - i * 4, 255)
        rrect(d, list(b), 8, shade, outline=(120, 88, 40, 255), w=2)
    d.ellipse([28, 42, 36, 50], fill=(60, 44, 22, 255))  # entrance hole
    d.line([32, 4, 32, 10], fill=(90, 66, 34, 255), width=2)  # attach point
    return img


def bee():
    img, d = new_frame()
    d.ellipse([28, 28, 40, 38], fill=(240, 196, 64, 255), outline=OUTLINE, width=2)
    d.line([33, 28, 33, 38], fill=OUTLINE, width=2)
    d.ellipse([24, 26, 32, 32], fill=(230, 235, 245, 180), outline=OUTLINE, width=1)  # wing
    return img


def hunny():
    img, d = new_frame()
    ground_shadow(d, 32, 60, 16)
    rrect(d, [20, 30, 44, 58], 7, (200, 135, 63, 255))  # pot
    rrect(d, [18, 26, 46, 34], 4, (168, 108, 48, 255))  # rim
    d.ellipse([30, 20, 40, 30], fill=(238, 196, 96, 255), outline=(150, 108, 40, 255), width=2)  # honey drip
    # crude label band
    rrect(d, [24, 40, 40, 50], 2, (232, 214, 176, 255), outline=(150, 120, 80, 255), w=1)
    return img


def book():
    img, d = new_frame()
    ground_shadow(d, 32, 58, 18)
    d.polygon([(16, 52), (30, 44), (48, 48), (34, 56)], fill=(198, 176, 132, 255), outline=OUTLINE)
    d.line([30, 44, 34, 56], fill=(150, 130, 96, 255), width=1)
    return img


# ------------------------------------------------------------- strip build ---

def strip(frames):
    sheet = Image.new("RGBA", (FRAME * len(frames), H), (0, 0, 0, 0))
    for i, f in enumerate(frames):
        sheet.paste(f, (i * FRAME, 0))
    return sheet


def save(sheet, sub, name):
    d = os.path.join(OUT, sub)
    os.makedirs(d, exist_ok=True)
    sheet.save(os.path.join(d, name))


def poses_idle():
    return [dict(bob=0, blink=False), dict(bob=1, blink=False),
            dict(bob=0, blink=False), dict(bob=0, blink=True)]


def poses_asleep():
    return [dict(bob=0, blink=True), dict(bob=1, blink=True)]


def poses_walk():
    return [dict(bob=0, blink=False, step=True), dict(bob=1, blink=False, step=False),
            dict(bob=0, blink=False, step=False), dict(bob=1, blink=False, step=True)]


def build_animal(fn, sub):
    save(strip([fn(p) for p in poses_idle()]), sub, "idle.png")
    save(strip([fn(p) for p in poses_asleep()]), sub, "asleep.png")
    save(strip([fn(p) for p in poses_walk()]), sub, "walk.png")


def main():
    build_animal(capybara, "capybara")
    build_animal(sloth, "sloth")
    save(strip([sloth_eat(x, y, blink) for (x, y, blink) in SLOTH_EAT_PATH]), "sloth", "eat.png")
    build_animal(badger, "badger")
    save(strip([beehive()]), "props", "beehive.png")
    save(strip([bee(), new_frame()[0]]), "props", "bee.png")
    save(strip([hunny()]), "props", "hunny.png")
    save(strip([book()]), "props", "book.png")
    print("wrote sprites to", os.path.normpath(OUT))


if __name__ == "__main__":
    main()
