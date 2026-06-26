<div align="center">

# 🔥 Charmander

**A Wayland popup character picker — type any special symbol in seconds**

*The Linux/Wayland equivalent of PowerToys Character Map / Switch Accent*

[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Wayland](https://img.shields.io/badge/Wayland-only-blue?logo=wayland)](https://wayland.freedesktop.org/)
[![GTK4](https://img.shields.io/badge/GTK-4-green?logo=gnome)](https://gtk.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-lightgrey)](LICENSE)

<!-- Add a GIF/screenshot here once available -->
<!-- ![Charmander demo](assets/demo.gif) -->

</div>

---

## What is this?

Charmander is a lightweight popup utility for inserting special Unicode characters — arrows, Greek letters, math symbols, diacritics, and anything else you configure — into any application on Wayland.

Bind it to a hotkey in your window manager, press it, search, pick, done. The character appears wherever your cursor was.

```
Super + Shift + S  →  charmander popup  →  type "arrow"  →  ↔  →  pasted
```

Think of it as a always-ready character palette that gets out of your way.

---

## Features

- **Instant search** — filters by name, tags, and partial matches as you type
- **Ranked results** — characters you use more often float to the top automatically
- **Keyboard-first** — navigate entirely with arrows and Enter, no mouse required
- **Zero friction** — appears as an overlay above all windows, closes after picking
- **Fully customizable** — define your own character library in a plain TOML file
- **System theme** — follows your GTK4 theme, no custom styling to maintain
- **Small and fast** — single binary, no daemon, no background processes

---

## Requirements

| Dependency | Required | Notes |
|---|---|---|
| [wtype](https://github.com/atx/wtype) | **Yes** | Types the character into the active window |
| GTK 4.12+ | **Yes** | UI toolkit |
| A wlr-layer-shell compositor | **Yes** | See [compositor support](#compositor-support) |

### Compositor support

Charmander uses the [`wlr-layer-shell`](https://wayland.app/protocols/wlr-layer-shell-unstable-v1) Wayland protocol to appear as an overlay above all windows. This protocol is supported by:

| Compositor | Supported |
|---|---|
| [Hyprland](https://hyprland.org/) | ✅ |
| [Sway](https://swaywm.org/) | ✅ |
| [river](https://isaacfreund.com/software/river/) | ✅ |
| [niri](https://github.com/YaLTeR/niri) | ✅ |
| [labwc](https://labwc.github.io/) | ✅ |
| GNOME (45+) | ✅ |
| KDE Plasma (6+) | ✅ |
| X11 compositors | ❌ |

> **Note:** wtype also requires wlroots-based compositors. GNOME and KDE users may need [ydotool](https://github.com/ReimuNotMoe/ydotool) as an alternative — this is on the [roadmap](#roadmap).

---

## Installation

### From source

```bash
# 1. Install dependencies (Arch Linux example)
sudo pacman -S gtk4 gtk4-layer-shell wtype

# 2. Clone and build
git clone https://github.com/Szizoid/charmander.git
cd charmander
cargo build --release

# 3. Install the binary
sudo install -Dm755 target/release/charmander /usr/local/bin/charmander
```

### AUR *(coming soon)*

```bash
yay -S charmander
# or
paru -S charmander
```

---

## Setup

### Bind to a hotkey

Charmander is designed to be launched by a hotkey binding in your window manager. No daemon — just a keybind that runs `charmander`.

**Hyprland** (`~/.config/hypr/hyprland.conf`):
```ini
bind = SUPER SHIFT, S, exec, charmander
```

**Sway** (`~/.config/sway/config`):
```
bindsym $mod+Shift+s exec charmander
```

**niri** (`~/.config/niri/config.kdl`):
```
binds {
    Mod+Shift+S { spawn "charmander"; }
}
```

---

## Usage

| Key | Action |
|---|---|
| **Type anything** | Filter the character list in real time |
| **↓ / ↑** | Navigate the list |
| **↑** on first item | Return focus to the search field |
| **Enter** | Insert the selected character (or the first result) |
| **Double-click** | Insert a character with the mouse |
| **Escape** | Close without inserting |

---

## Configuration

The config file is created automatically at `~/.config/charmander/config.toml` on first launch.

### Settings

| Key | Default | Description |
|---|---|---|
| `max-results` | `0` | Max characters shown. `0` = no limit |
| `selection-indicator` | `"> "` | Prefix shown next to the selected row |
| `no-selection-indicator` | `"  "` | Prefix for non-selected rows (should match width) |

```toml
[settings]
max-results = 0
selection-indicator = "> "
no-selection-indicator = "  "
```

> **Tip:** `selection-indicator` and `no-selection-indicator` should have the same character count for the list to stay aligned.

### Adding characters

Each character is a `[[characters]]` block:

```toml
[[characters]]
symbol = "→"
name = "right arrow"
tags = ["arrow", "right", "->", "math"]

[[characters]]
symbol = "α"
name = "alpha"
tags = ["a", "greek", "math", "physics"]
```

| Field | Description |
|---|---|
| `symbol` | The Unicode character (or multi-character sequence) to insert |
| `name` | Full name — searched if no tag matches |
| `tags` | Keywords used for search; more tags = better discoverability |

### Search ranking

Results are ranked by a composite score:

```
score = match_priority × 1000 + usage_count
```

| Match type | Priority |
|---|---|
| Exact tag match | 4 (highest) |
| Tag starts with query | 3 |
| Tag contains query | 2 |
| Name contains query | 1 |

Characters you use more often rise within their match tier. With an empty search, the list shows **all characters sorted by usage** — effectively a "recently used" view.

---

## Roadmap

### CLI subcommands *(planned)*

```
charmander                    # open the popup window
charmander config default     # backup current config and restore defaults
charmander config edit        # open config in $EDITOR
charmander config path        # print the config file path
charmander config validate    # check config for syntax errors
charmander history clear      # reset usage history
charmander history show       # show top N most-used characters
charmander list               # print all configured characters (for scripting)
charmander version            # print version
```

### Planned features

- [ ] **Broader compositor support** — `ydotool` as an alternative to `wtype` for GNOME/KDE
- [ ] **Fuzzy search** — typo-tolerant matching
- [ ] **Categories / tabs** — group characters by Latin, Greek, Math, Arrows, etc.
- [ ] **Window customization** — size, padding, font via config
- [ ] **System config override** — `/etc/charmander/` + `~/.config/charmander/` layering
- [ ] **AUR package** — `charmander` and `charmander-git`

---

## Project structure

```
charmander/
├── src/
│   ├── main.rs         # Entry point: config loading, GTK app lifecycle
│   ├── config.rs       # Config file parsing and default creation
│   ├── history.rs      # Usage frequency tracking
│   ├── search.rs       # Filtering and ranking algorithm
│   ├── output.rs       # Character insertion via wtype
│   └── ui/
│       └── window.rs   # GTK4 overlay window with layer shell
├── config/
│   └── default.toml    # Default character library (embedded in binary)
├── ARCHITECTURE.md
└── README.md
```

---

## Contributing

Contributions are welcome — especially:

- New characters for the default library
- Bug reports with compositor/environment details
- Ideas for the CLI interface

Please open an issue before submitting a large PR so we can discuss the approach first.

---

## License

MIT — see [LICENSE](LICENSE) for details.
