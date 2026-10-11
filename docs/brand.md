# The brand layer

Atelier Frequency is SoundCraft with a different name on the door. We keep the change small on
purpose: upstream lands many changes a day, and every file we touch is a file that can conflict
when we pull them in. SoundCraft has no translation catalogs yet (English only), so the rename
is a plain rename of the strings a person reads.

**Renamed (what a person sees):** the window title, the app menu, the About box and its links,
the session file type name in open and save dialogs, the browser page title, Windows file
properties, installer and package names and vendor, and where settings live
(`~/Library/Application Support/Atelier Frequency` on macOS, `Atelier Frequency` under
`%APPDATA%` on Windows).

**Not renamed (machine names):** crate names (`soundcraft-*`), the binaries (`soundcraft`,
`soundcraft-cli`), command ids, the `ai.storyteller.soundcraft` app id and icon file names in
packaging, the `.scraft` session format, the log target names, environment variables, and the
docs under `docs/`. These change only when we decide to stop tracking upstream, in one
deliberate pass.

## Where the name lives

| Place | File |
|---|---|
| Window title, settings folder, file dialogs | `apps/soundcraft/src/main.rs` |
| macOS app menu | `apps/soundcraft/src/native_menu.rs` |
| In-window menu and About box, with the Nation links | `crates/ui-egui/src/menus.rs`, `crates/ui-egui/src/panels.rs` |
| Windows file properties | `apps/soundcraft/build.rs` |
| macOS bundle | `packaging/macos/Info.plist.in` (display name, copyright, microphone prompt; identifier unchanged) |
| Windows installer | `packaging/windows/soundcraft.wxs` (names, manufacturer, links; registry keys unchanged) |
| Linux desktop entry, AppStream, nfpm | `packaging/linux/ai.storyteller.soundcraft.*`, `nfpm.yaml` (names, vendor, links; ids unchanged) |
| Browser build | `apps/soundcraft-web/index.html` |
| README and NOTICE | README: our header above upstream's feature sections; NOTICE: ours on top of theirs, kept on merge by `.gitattributes` |
| App icon | `assets/app-icon/` (ours, see its README; rows in `ATTRIBUTION.md`) |

Still upstream's name, to be done in later passes: the `<software>` tag in exported MusicXML
(`crates/engine/src/score.rs`), the CLI's parity report wording, and the `docs/` manuals.

## Renaming the product

Search the tree for the name in string literals
(`grep -rn '"[^"]*Atelier Frequency' crates apps packaging`) and change each; there is no
translation layer to update yet. When upstream adds catalogs, add an `identity.rs` as in
Pigment and Quill.
