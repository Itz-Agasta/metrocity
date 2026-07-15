<h1 align="center">Metrocity</h1>

<p align="center">
  <b>A terminal screensaver for Linux - animated pixel-art scenes that take over your idle shell.</b>
</p>

<p align="center">
  <a href="#install">Install</a> ·
  <a href="#scenes">Scenes</a> ·
  <a href="#setup">Shell setup</a> ·
  <a href="#configuration">Configuration</a> ·
  <a href="#themes">Themes</a>
</p>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.75+-orange?logo=rust">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-blue">
  <img alt="Binary size" src="https://img.shields.io/badge/binary-~1.4MB-green">
</p>

https://github.com/user-attachments/assets/e3bf0c0c-ef1e-4b0d-b89d-a4db6d16ea74

Metrocity activates when your shell has been idle, takes over the terminal with an animated scene (a cyberpunk city skyline or a cozy cat cafe), and exits instantly on any keypress - like a screensaver, but for your terminal. Flashes your distro logo. Goes hard in your Hyprland setup.

Built with [Rust](https://www.rust-lang.org/), [Ratatui](https://ratatui.rs/), [Crossterm](https://github.com/crossterm-rs/crossterm), and the [Kitty graphics protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/).

## Features

- **Animated scenes** with weather, traffic, wandering cats, and atmosphere
- **Pixel-art sprites** rendered over the character grid via the Kitty graphics protocol
- **Distro detection** - renders your distro's logo on a building
- **6 built-in color themes** plus custom themes via TOML
- **Shell integration** for zsh, bash and fish - activates automatically on idle
- **Instant exit** - any keypress restores your terminal exactly as it was
- **Tiny** - ~1.4MB release binary, no runtime dependencies

## Scenes

| Scene  | Description |
|--------|-------------|
| `city` | Cyberpunk skyline: neon buildings, flying traffic, rain, your distro logo - ported from [metropolis](https://github.com/5c0/metropolis) |
| `cafe` | Cozy cat cafe: two animated pixel cats, neon sign, rain on the window, pastries |

The cafe scene layers pixel-art sprites (cats, plants, pastries, neon sign) on top of the character grid. Sprites need a terminal that supports the Kitty graphics protocol - kitty, WezTerm, or Ghostty. On other terminals the scene still renders, just without the sprites.

## Install

### From source

```bash
git clone https://github.com/Itz-Agasta/metrocity.git
cd metrocity
cargo build --release
sudo cp target/release/metrocity /usr/local/bin/
```

### From crates.io

```bash
cargo install metrocity
```

### Arch Linux (AUR)

```bash
paru -S metrocity
```

## Setup

### Zsh

Add to your `~/.zshrc`:

```bash
export METROCITY_TIMEOUT=120  # seconds of idle before activation (default: 120)
export METROCITY_SCENE=cafe   # scene to launch: cafe (default) or city
eval "$(metrocity shell-init zsh)"
```

### Bash

Add to your `~/.bashrc`:

```bash
export METROCITY_TIMEOUT=120
export METROCITY_SCENE=cafe
eval "$(metrocity shell-init bash)"
```

### Fish

Add to your `~/.config/fish/config.fish`:

```fish
set -gx METROCITY_TIMEOUT 120
set -gx METROCITY_SCENE cafe
metrocity shell-init fish | source
```

`METROCITY_SCENE` is optional. If unset, metrocity uses the default scene (`cafe`). Set it to `city` for the cyberpunk skyline.


> **Add these lines to your shell config file, not just the current terminal.**
> `eval` (or `| source` in fish) only affects the shell that runs it, so pasting
> it into one terminal activates metrocity there only. Put it in `~/.zshrc` /
> `~/.bashrc` / `~/.config/fish/config.fish`, then open a new terminal (or
> re-source the file) so every shell picks it up.

### Manual launch

Just run `metrocity` - it takes over the terminal, press any key to exit.

## Usage

```
metrocity                              # Start immediately
metrocity --scene cafe                 # Lock to a specific scene (city, cafe)
metrocity --theme cyberpunk            # Override color theme
metrocity --weather rain               # Force weather mode (rain, snow, clear)
metrocity --fps 60                     # Target frame rate

metrocity shell-init zsh               # Print zsh integration snippet
metrocity shell-init bash              # Print bash integration snippet
metrocity shell-init fish              # Print fish integration snippet

metrocity list scenes                  # List available scenes
metrocity list themes                  # List available themes

metrocity config                       # Show effective configuration
metrocity config --init                # Write default config to disk

metrocity --version                    # Print version
```

## Configuration

Config file: `~/.config/metrocity/config.toml`. Generate it with `metrocity config --init`:

```toml
[engine]
fps = 30
scene = "cafe"  # cafe or city

[appearance]
theme = "default"
weather = "rain"

[simulation]
max_vehicles = 50
max_pedestrians = 15
vehicle_speed_multiplier = 1.0
weather_speed_multiplier = 1.0
weather_density_multiplier = 1.0

[monolith]
custom_text = ""
custom_color = ""
override_distro = ""
```

## Themes

| Theme | Description |
|-------|-------------|
| `cyberpunk` | Neon-drenched magenta and cyan |
| `matrix` | Green-on-black digital rain |
| `synthwave` | Retro sunset orange and pink |
| `dracula` | Purple-pink gothic palette |
| `sin_city` | Stark black, white, and red |
| `default` | Balanced dark tones |

### Custom themes

Place a TOML file at `~/.config/metrocity/themes/<name>.toml` and use it with `metrocity --theme <name>`:

```toml
[building]
base_colors = ["#14141e", "#1e141e", "#141e28", "#1e1e1e"]
window_lit = "#ffff55"
window_unlit = "#282828"
window_dark = "#0f0f0f"

[neon]
primary = "#55ffff"
secondary = "#ff55ff"
accent = "#55ff55"
soft = "#ffff55"

[vehicles]
colors = ["#ff0000", "#ffffff", "#ffff00", "#00ffff", "#3296ff"]
police_red = "#ff0000"
police_blue = "#0000ff"

[env]
street_lamp_lit = "#ffff96"
street_lamp_dim = "#646432"
rain = "#00b4b4"
rain_bg = "#003c3c"
snow = "#ffffff"
pedestrian = "#aaaaaa"
ground = "#282832"

[overrides]
logo = "#ff0000"
```

## How it works

1. **Shell integration** - after `METROCITY_TIMEOUT` seconds of idle time at the prompt, your shell launches `metrocity`.
2. **Terminal takeover** - it enters raw mode, switches to the alternate screen buffer, and hides the cursor.
3. **Render loop** - at the target FPS it updates the simulation and redraws the scene; on Kitty-capable terminals, pixel sprites are composited on top of the character cells.
4. **Any key exits** - a keypress breaks the loop, cleans up sprites, restores the terminal, and returns control to your shell.

Pairs well with Hyprland, fastfetch, and `~/.config` tinkering.
