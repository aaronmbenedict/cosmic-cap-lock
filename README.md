# cosmic-applet-capslock

Caps Lock panel indicator for [Cosmic DE](https://github.com/pop-os/cosmic-epoch).

Shows a keyboard icon in the panel. Clicking it opens a popup showing whether
Caps Lock is On or Off.

---

## Requirements

- **Pop!_OS 24.04** (or any distro with Cosmic DE installed)
- **Rust stable** ≥ 1.85 — required by libcosmic's 2024 edition
- **just** — build tool
- Build dependencies (see below)

---

## 1. Install build dependencies

```bash
sudo apt update
sudo apt install \
    just \
    rustup \
    cmake \
    libexpat1-dev \
    libfontconfig-dev \
    libfreetype-dev \
    libxkbcommon-dev \
    pkgconf

# Rust 1.85+ is required
rustup default stable
rustup update stable
```

> These are exactly the dependencies listed in the official
> [libcosmic README](https://github.com/pop-os/libcosmic).

---

## 2. Build & Install

```bash
cd cosmic-applet-capslock

# Build release binary
just build-release

# Install to /usr/local/bin + register .desktop
just install
```

Then in Cosmic DE:
**Settings → Panel → Add Applet → "Caps Lock Indicator"**

---

## Quick sanity checks (no build needed)

**Check sysfs LED detection** (works on any Linux):
```bash
just check-sysfs
```

**Check X11 fallback** (requires active X11/XWayland session):
```bash
xset q | grep "Caps Lock"
```

---

## How Caps Lock detection works

The applet polls every 250 ms using this priority:

| Priority | Method | Works on |
|----------|--------|----------|
| 1 | `/sys/class/leds/input*::capslock/brightness` | Wayland + X11 |
| 2 | `xset q` output parsing | X11 only |
| 3 | Returns `false` | Safe fallback |

---

## Troubleshooting

**`error[E0053]` / `error[E0277]` / other type errors at compile time**

These mean the libcosmic API has changed since this code was written.
libcosmic is under heavy development and its git HEAD moves frequently.
Try:
```bash
# Force a fresh fetch of libcosmic
cargo update
cargo build --release
```
If that fails, file an issue — the `view()` or `popup_settings()` API may
need updating.

**Applet doesn't appear in panel settings**

Make sure the `.desktop` file is in place:
```bash
ls ~/.local/share/applications/com.system76.CosmicAppletCapslock.desktop
```
Then log out and back in, or restart the panel:
```bash
pkill cosmic-panel
```

**Binary runs but shows wrong state**

Run `just check-sysfs` to see which detection method will be used.
If neither sysfs nor xset works, open an issue with your distro info.

---

## Project structure

```
cosmic-applet-capslock/
├── Cargo.toml       — dependencies (libcosmic only, no cosmic-time)
├── justfile         — build/install recipes
├── data/
│   └── com.system76.CosmicAppletCapslock.desktop
└── src/
    ├── main.rs      — Application trait impl, UI, subscription
    └── keyboard.rs  — Caps Lock state detection
```

---

## License

GPL-3.0
