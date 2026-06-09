<p align="center">
  <h1 align="center">Metrocity</h1>

  <p align="center">
    Terminal idle eye candy for people who treat their terminal like a desktop wallpaper.
  </p>
</p>



https://github.com/user-attachments/assets/cf53dfcd-e152-4222-a36a-8b0e2b7da1d5

Activates when your shell is idle, takes over the terminal with an animated scene, and exits instantly on any keypress. Flashes your distro logo, Goes hard in your Hyprland setup.

Built with [Rust](https://www.rust-lang.org/), [Ratatui](https://ratatui.rs/), [Crossterm](https://github.com/crossterm-rs/crossterm), and the [Kitty image protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/).

## Features

- Multiple animated scenes with weather, movement, and atmosphere
- Auto-detects your distro and renders its logo
- 6 built-in color themes: cyberpunk, matrix, synthwave, dracula, sin\_city, default
- Custom themes via TOML — match your terminal, rice it your way
- Shell integration for zsh and bash — activates automatically on idle

## Scenes

| Scene  | Description                    |
|--------|--------------------------------|
| `city` | Cyberpunk skyline with traffic |
| `cafe` | Cozy cat cafe interior         |
- Any keypress instantly exits and restores your terminal
- Under 1MB release binary

## Install

### From source

```bash
git clone https://github.com/Itz-Agasta/metrocity.git
cd metrocity
cargo build --release
sudo cp target/release/metrocity /usr/local/bin/
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
eval "$(metrocity shell-init zsh)"
```

### Bash

Add to your `~/.bashrc`:

```bash
export METROCITY_TIMEOUT=120
eval "$(metrocity shell-init bash)"
```

### Manual launch

Just run `metrocity` — it takes over the terminal, press any key to exit.

## Usage

```
metrocity                              # Start immediately
metrocity --theme cyberpunk            # Override color theme
metrocity --weather rain               # Force weather mode
metrocity --fps 60                     # Target frame rate
metrocity --scene city                 # Lock to a specific scene

metrocity shell-init zsh               # Print zsh integration snippet
metrocity shell-init bash              # Print bash integration snippet

metrocity list scenes                  # List available scenes
metrocity list themes                  # List available themes

metrocity config                       # Show effective configuration
metrocity config --init                # Write default config to disk

metrocity --version                    # Print version
```

## Configuration

Config file: `~/.config/metrocity/config.toml`

Generate with:

```bash
metrocity config --init
```

```toml
[engine]
fps = 30

[appearance]
theme = "cyberpunk"
weather = "rain"

[simulation]
max_vehicles = 30
max_pedestrians = 15
vehicle_speed_multiplier = 1.0
weather_speed_multiplier = 1.0
weather_density_multiplier = 1.0

[monolith]
custom_text = ""
custom_color = ""
override_distro = ""
```

### Custom themes

Place a TOML file at `~/.config/metrocity/themes/<name>.toml`:

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

Then use it with `metrocity --theme <name>`.

## Themes

| Theme | Description |
|-------|-------------|
| `cyberpunk` | Neon-drenched magenta and cyan |
| `matrix` | Green-on-black digital rain |
| `synthwave` | Retro sunset orange and pink |
| `dracula` | Purple-pink gothic palette |
| `sin_city` | Stark black, white, and red |
| `default` | Balanced dark tones |

## How it works

1. **Shell integration** — After `METROCITY_TIMEOUT` seconds of idle time at the prompt, your shell launches `metrocity`.
2. **Terminal takeover** — `metrocity` enters raw mode, switches to the alternate screen buffer, and hides the cursor.
3. **Render loop** — At the target FPS, it updates the simulation and redraws the scene.
4. **Any key exits** — Any keypress breaks the loop, restores the terminal, and returns control to your shell.

Pairs well with Hyprland, fastfetch, and ~/.config tinkering.
