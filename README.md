# WishSim

A cinematic terminal wish simulator and early turn-based RPG built in Rust with Ratatui and Crossterm. It includes a growing archive of limited and standard banners, animated star flights, five-star cutscenes, original full-color art, inspectable inventory, persistent teams and equipment, an isolated 3v3 Battle Test, persistent pity, history, and deterministic command-line pulls.

WishSim uses original characters, weapons, lore, and artwork. It is an unofficial fan-made terminal game and is not affiliated with or endorsed by HoYoverse.

## Install a release

### Homebrew (macOS)

```sh
brew tap TMAH1602/wishsim
brew install wishsim
```

Then launch it with `wishsim`. **Ghostty is WishSim's primary supported terminal emulator** and the recommended environment for full-resolution artwork. Kitty is also supported through the same graphics protocol, while modern true-color terminals use the portable ANSI fallback. Terminal multiplexers such as Zellij may block protocol graphics.

### Download an archive

Download the archive for your system from the [latest GitHub release](../../releases/latest):

- `wishsim-macos-arm64` for Apple Silicon Macs
- `wishsim-macos-x86_64` for Intel Macs
- `wishsim-linux-x86_64` for 64-bit Linux
- `wishsim-windows-x86_64` for 64-bit Windows

On macOS or Linux, extract the archive and run:

```sh
chmod +x wishsim
./wishsim
```

On Windows, extract the ZIP and launch `wishsim.exe` from Windows Terminal. Ghostty provides the primary tested presentation, Kitty is also supported, and any modern true-color terminal should work through the ANSI renderer.

The included banner and item names are original. The probability model is inspired by familiar character-event wish systems: 0.6% base five-star rate, soft pity after pull 73, hard pity at 90, four-star-or-better within 10, and featured guarantees after losing a 50/50.

## Run it

```sh
cargo run --release
```

The interactive controls are shown on screen:

- Main menu: `↑` / `↓` and `Enter` choose Teams, Wish, Inventory, Characters, Battle Test, or Info / Tutorial
- Teams: `↑` / `↓` select one of five teams; `←` / `→` select a member slot; `Enter` attaches an owned character; `R` renames; `D` clears a slot
- Characters: `←` / `→` scroll the owned roster; `L` opens the quick roster; `W` opens compatible owned weapon copies; `Enter` equips and `D` unequips
- Character quick roster: `R` filters rarity, `E` filters element, and `T` filters weapon type
- Battle Test: choose a complete saved team of three; characters and equipped weapons are projected to level 50 without changing saved data
- Battle commands: every character has a named Basic available each turn, a Skill with a one-turn post-use wait, and an Ultimate with a three-to-five-turn post-use wait; healer Skills/Ultimates target teammates, and `Defend` reduces damage using DEF and POISE
- Enemies choose targets using health, guard state, and rotating position pressure; Thornbloom can inflict a three-turn poison
- Info / Tutorial: reviews the temporary combat rules and complete provisional elemental matchup table

- `1` or `Enter`: one wish
- `0`: ten wishes
- `←` / `→` on the home screen: change banners
- `B`: open the limited five-star banner grid
- `P` on the weapon banner: open the scrollable five-star weapon path list; art appears by default, `V` toggles details/stats, and `Enter` confirms it (changing paths resets Fate)
- `P` on the standard banner: change the selected standard character (resets Fate)
- `C`: open the scrollable character archive; unowned characters retain their names but display as locked `?` records
- `H`: history
- `I`: inventory
- `Space` or `Enter`: advance/skip an animation
- `S`: skip ordinary remaining reveals in a ten-pull; every 5-star still plays its cutscene and result card before the summary
- `←` / `→`: select a result in the summary
- `Enter`: inspect the selected character or weapon
- `Esc`: return from an inspection screen
- `Q`: quit

### Inventory controls

- `↑` / `↓`: move through owned items
- `Space`: toggle the focused item for multi-selection
- `A`: select or deselect every inventory entry
- `S`: cycle sorting by name, rarity, character/weapon type, or element
- `F`: filter all items, characters only, or weapons only
- `E`: cycle elemental filters
- `Enter`: inspect the focused character or weapon
- `D`: delete the selection, or the focused item when nothing is selected
- `Shift+D`: request deletion of the entire inventory
- `Y` / `N`: confirm or cancel deletion

Inventory deletion never changes pity or wish history. Every deletion, including individual and batch deletion, requires confirmation.

Every character and weapon now has an initial combat-stat profile: CRIT DMG, CRIT RATE, ATK, DEF, SPD, ELEMENTAL ATK, HP, and POISE. These are visible on inspection and form the foundation for the planned 3v3 battle system.

Character duplicates advance a visible Ascension roadmap from N0 toward N10. Five saved team slots can each hold three owned characters and a custom name. Every character can equip one owned weapon matching their weapon class.

Characters currently remain level 1 throughout collection and management. Battle Test is an intentionally isolated 3v3 prototype: it projects one complete team and three test enemies to level 50, displays both sides simultaneously, and resolves turns by SPD. It does not persist HP, levels, wins, losses, or combat progression. Character-specific Skill 1, Skill 2, and Ultimate abilities remain reserved for a later release.

Ghostty is the primary supported terminal and is detected automatically. Character and weapon art is sent directly through the Kitty graphics protocol implemented by Ghostty; Kitty is also detected and supported. Other terminals fall back to terminal half-block rendering. Artwork is embedded in the executable, so no separate asset folder or `kitten` helper is required. Run WishSim outside Zellij for protocol artwork.

If a shell or multiplexer replaces Ghostty's terminal markers, launch with `WISHSIM_GRAPHICS=ghostty wishsim` to force native artwork. Use `WISHSIM_GRAPHICS=ansi wishsim` to force the portable fallback.

## CLI mode

```sh
cargo run -- pull --count 10
cargo run -- pull --count 10 --seed 42
cargo run -- pull --count 10 --banner kaelis
cargo run -- pull --count 10 --banner seraphine
cargo run -- pull --count 10 --banner vaughn
cargo run -- pull --count 10 --banner steven
cargo run -- pull --count 10 --banner sergei
cargo run -- pull --count 10 --banner saif
cargo run -- pull --count 10 --banner yeoungin
cargo run -- pull --count 10 --banner standard
cargo run -- pull --count 10 --banner weapon
cargo run -- stats
cargo run -- reset
```

Seeded pulls are reproducible and deliberately do not modify the save file.

All limited character-event banners share pity and featured guarantees. The Standard Archive has independent pity and a selectable character Fate path. The weapon banner has its own 80-pull hard pity, a 75% featured check, a scrollable selection of limited signatures, and a one-point epitomized path: missing the selected weapon grants one Fate Point, making the next five-star the selected weapon. This is an original, simplified system inspired by modern weapon-banner mechanics.

## Learn the code

Start in [`src/model.rs`](src/model.rs) for Rust structs and enums, then read [`src/simulation.rs`](src/simulation.rs) for state mutation and tests. [`src/app.rs`](src/app.rs) is the event-driven state machine, and [`src/ui.rs`](src/ui.rs) contains the Ratatui rendering and animation work.

Before implementing new features, read [`docs/V0_2_0_CONSISTENCY_GUIDE.md`](docs/V0_2_0_CONSISTENCY_GUIDE.md). It documents the v0.2.0 architecture, visual language, banner and portrait requirements, integration checklists, and validation rules. Detailed post-v0.2.0 work history and safe backtracking notes are kept in [`logs/`](logs/README.md).

Run the quality checks with:

```sh
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets -- -D warnings
```

## Publishing a release

Releases are built automatically for macOS, Linux, and Windows. Update the version in `Cargo.toml`, commit the change, then push a matching tag:

```sh
git tag v0.1.0
git push origin main --tags
```

The release workflow creates the GitHub release, builds all four archives, and attaches them for download.

## License

WishSim is available under the [MIT License](LICENSE).
