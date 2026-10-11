# Attribution

Every asset bundled with or committed to SoundCraft is listed here with its author, source and
license. `cargo xtask assets` fails CI when a tracked asset file has no row.

**Rule:** no Avid, Pro Tools, Adobe or other proprietary icons, images, sounds or presets — ever.
Assets must be original, public domain (CC0) or permissively licensed. See `AGENTS.md`.

## Icons

All UI icons (tools, transport, track buttons) are drawn procedurally in
`crates/ui-egui/src/icons.rs` by the SoundCraft contributors (MIT OR Apache-2.0). No UI icon image
files are used. The app icon (`assets/app-icon/`, the hidden sun behind a waveform, drawn as original SVG for
Atelier) is listed under Files below.

## Audio

The demo session's audio (drums, bass, pad) and its MIDI parts are synthesised in code in
`crates/engine/src/demo.rs` by the SoundCraft contributors (MIT OR Apache-2.0). No sample files
are bundled.

## Fonts

SoundCraft bundles no font files. It uses the default fonts shipped inside the `egui` crate
(Ubuntu-Light: Ubuntu Font Licence 1.0; Hack: MIT; Noto Emoji: OFL-1.1; emoji-icon-font: OFL-1.1/MIT)
and, at runtime, the operating system's own UI fonts when available (never redistributed).

## Files

| Path | Author | Source | License |
|---|---|---|---|
| `assets/app-icon/icon.svg` | Aetherion Nation, for Atelier | original: the hidden sun behind a waveform, drawn as SVG (`assets/app-icon/README.md`) | MIT OR Apache-2.0 |
| `packaging/macos/dmg/` (all files) | @XusBadia | original (macOS DMG window background and layout, from the app icon; see `packaging/macos/dmg/README.md`) | MIT OR Apache-2.0 |
| `assets/app-icon/soundcraft-1024.png` | Aetherion Nation, for Atelier | original, rendered from `icon.svg` with cairosvg and Pillow | MIT OR Apache-2.0 |
| `assets/app-icon/soundcraft-macos-512.png` | Aetherion Nation, for Atelier | original, rendered from `icon.svg` with cairosvg and Pillow | MIT OR Apache-2.0 |
| `assets/app-icon/soundcraft.icns` | Aetherion Nation, for Atelier | original, rendered from `icon.svg` with cairosvg and Pillow | MIT OR Apache-2.0 |
| `assets/app-icon/soundcraft.ico` | Aetherion Nation, for Atelier | original, rendered from `icon.svg` with cairosvg and Pillow | MIT OR Apache-2.0 |
| `assets/app-icon/hicolor/` | Aetherion Nation, for Atelier | original, rendered from `icon.svg` with cairosvg and Pillow (16–512 px PNGs and the scalable SVG) | MIT OR Apache-2.0 |
| `docs/images/` | SoundCraft contributors | screenshots of SoundCraft rendered from the synthesised demo session | MIT OR Apache-2.0 |
