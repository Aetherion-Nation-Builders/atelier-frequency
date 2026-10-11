# Atelier Frequency app icon

**The hidden sun behind a waveform**: an ivory disc with an ochre waveform crossing its lower
half, on a volcanic black field. Three colours, from the Nation's palette:

| Colour | Hex | Used for |
|---|---|---|
| Volcanic black | `#141210` | the field, and the gap that separates the waveform from the sun |
| Ivory | `#efe6d2` | the sun |
| Ochre | `#d9a441` | the waveform |

**Tile:** `viewBox="0 0 512 512"`, a rounded square with `rx=112`, as all the Atelier icons. The
macOS files add Apple's 824/1024 margin (`soundcraft-macos-512.png`, `.icns`); the others are the
full tile. `icon.svg` is the master; the file names keep their upstream `soundcraft` names so
packaging stays untouched.

**Provenance:** drawn for Atelier Frequency by hand as SVG (2026). It replaces SoundCraft's
placeholder nightingale, which stays in SoundCraft's own repository. Licence: MIT OR Apache-2.0,
like the code; the Atelier and Aetherion names are not covered by it.

**Regenerating:** `packaging/icons.sh` (needs resvg; iconutil on macOS for the .icns). The
committed renders were made from the same SVG with cairosvg and Pillow.
