
<div align="center">
    <h1 align="center"><b>FerrisKeys</b></h1>

![](./assets/images/FerrisKeys.ico)

</div>

<p align="center">
  <img src="https://img.shields.io/badge/Made%20with-Rust-CE412B?style=for-the-badge&logo=rust&logoColor=white" alt="Made with Rust">
  <img src="https://img.shields.io/github/stars/CompEng0001/FerrisKeys?style=for-the-badge" alt="GitHub Stars">
  <img src="https://img.shields.io/github/last-commit/CompEng0001/FerrisKeys?style=for-the-badge" alt="Last Commit">
  <a href="https://github.com/CompEng0001/FerrisKeys/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/CompEng0001/FerrisKeys/release.yml?style=for-the-badge&label=CI" alt="Build Status">
  </a>
  <img src="https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge" alt="License: MIT">
</p>

**FerrisKeys** is a cross-platform input visualizer for Rustaceans. Designed for screencasts, live presentations, and teaching environments, it displays real-time keyboard and mouse input with clean, customizable overlays using `egui` via `eframe`.

<div align=center>

![screenshot](assets/images/ferriskeys_demo.gif)

</div>

---

## Features

- **Keyboard visualisation** with modifier grouping (Shift, Ctrl, etc.)
- **Mouse click tracking** with overlay icons
- **Live theme reloading** via `config.toml`
- **Cross-platform**: tested on Linux (X11/Wayland) and Windows  
  *wayland is currently unsupported*
- **Customizable**: Fonts, icons, padding, opacity, display duration

---

## Releases

Latests releases can be found on the releases page

---

## Build from Source

### Prerequisites

- Rust (2021 edition or newer)
- A compatible Linux or Windows environment
- For Wayland input capture: `libinput`, `udev`, and `evdev` access

### Build & Run

```bash
git clone https://github.com/CompEng0001/FerrisKeys.git
cd FerrisKeys
cargo run --release
```

>[!IMPORTANT]
Linux dependencies:
>
>```
>sudo apt install -y \
>  libxdo-dev \
>  libglib2.0-dev \
>  libpango1.0-dev \
>  libatk1.0-dev \
>  libgtk-3-dev \
>  libgdk-pixbuf-xlib-2.0-dev \
>  libcairo2-dev \
>  pkg-config \
>  build-essential
>```

---

## Configuration

On first run, a `config.toml` will be generated in the `$HOME/.config/ferriskeys/` or `%APPDATA%\Roaming\ferriskeys\` directory.

You can view the source here -> [`default__config.rs`](./src/config/default_config.rs)

All avaiable fields are: 

- `timeout_ms` - time keys stay on screen once buffer is empty
- `window` - position/size of window and which monitor,
- `normal` - alpha
- `numeric` - numeric
- `modifier` - alt, shift etc
- `editor` - del, ins, backspace etc
- `navigation` - up, down, left, right
- `scrollable` - page up/down, home ,end, scroll lock
- `symbols` - !,",£,$, % , [  etc
- `escape` - escape key
- `unknown` - unknown keys not mapped yet!
- `function` - F1 etc
- `altfunction` - [fn + F1] etc
- `mouse` - left|right|middle
- `space` - spacebar

### Example

```toml
[window]
monitor = 0
position = [2050.0, 500.0]
size = [800, 120]

[styles.normal]
width = 90.0
height = 90.0
icon_size = 0.0
text_size = 20.0
bg_color = "#1e1e30"
fg_color = "#ffffff"
```

Changes are auto-reloaded on modification — no restart required.

>[!IMPORTANT]
> `[window] monitor = 0`
>
> Is not implemented yet, but will be soon.

### Global Config Paths

| OS        | Path                                                |
|-----------|-----------------------------------------------------|
| Linux     | `~/.config/ferriskeys/config.toml`                  |
| Windows   | `%APPDATA%\Roaming\FerrisKeys\config.toml`                |

---

## Platform Support

| OS      | Input Backend       | Status       |
|---------|---------------------|--------------|
| Linux   | `evdev` + `libinput`| Working |
| Windows | `winapi` raw input  | Working |
| Raspbian (framebuffer) | n/a | Not supported |
| macOS   | _Planned_           | Not yet supported |

>[!WARNING]
> Wayland support is currently limited due to upstream issues in the
> `winit` + `glutin` stack used by `eframe`.


---

## Use Cases

- Screencasting and tutorial recording
- Live coding demonstrations
- Accessibility and interaction debugging
- Classroom and teaching support

---

## Contributing

We welcome contributions! Please:
- File issues for bugs or feature requests
- Submit PRs with descriptive titles
- Format code with `rustfmt` and use `clippy`

---

## License

Licensed under the **MIT License**. See [`LICENSE`](LICENSE) for details.

---

## Author

Developed by [@CompEng0001](https://github.com/CompEng0001)