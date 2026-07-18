# WishSim

A cinematic terminal wish simulator built in Rust with Ratatui and Crossterm. It includes four banners, animated star flights, five-star cutscenes, original full-color character portraits, inspectable inventory, ten-pull summaries, persistent pity, featured guarantees, history, and deterministic command-line pulls.

WishSim uses original characters, weapons, lore, and artwork. It is an unofficial fan-made terminal game and is not affiliated with or endorsed by HoYoverse.

## Install a release

### Homebrew (macOS)

```sh
brew tap TMAH1602/wishsim
brew install wishsim
```

Then launch it with `wishsim`. Run it directly in Kitty for full-resolution character artwork; terminal multiplexers such as Zellij may block Kitty's graphics protocol.

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

On Windows, extract the ZIP and launch `wishsim.exe` from Windows Terminal. Kitty provides the richest presentation, but any modern true-color terminal should work.

The included banner and item names are original. The probability model is inspired by familiar character-event wish systems: 0.6% base five-star rate, soft pity after pull 73, hard pity at 90, four-star-or-better within 10, and featured guarantees after losing a 50/50.

## Run it

```sh
cargo run --release
```

The interactive controls are shown on screen:

- `1` or `Enter`: one wish
- `0`: ten wishes
- `←` / `→` on the home screen: change banners
- `P` on the weapon banner: change the epitomized path (resets Fate)
- `H`: history
- `I`: inventory
- `Space` or `Enter`: advance/skip an animation
- `S`: skip every remaining reveal in a ten-pull
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

Kitty is detected automatically. Character portraits use Kitty's graphics protocol when available and fall back to terminal half-block rendering elsewhere. Portraits are embedded in the executable, so no separate asset folder is needed beside a release binary. Run WishSim outside Zellij for native Kitty artwork.

## CLI mode

```sh
cargo run -- pull --count 10
cargo run -- pull --count 10 --seed 42
cargo run -- pull --count 10 --banner kaelis
cargo run -- pull --count 10 --banner seraphine
cargo run -- pull --count 10 --banner vaughn
cargo run -- pull --count 10 --banner steven
cargo run -- pull --count 10 --banner weapon
cargo run -- stats
cargo run -- reset
```

Seeded pulls are reproducible and deliberately do not modify the save file.

The three character-event banners share pity and featured guarantees. The weapon banner has its own 80-pull hard pity, a 75% featured check, two featured weapons, and a one-point epitomized path: missing the selected weapon grants one Fate Point, making the next five-star the selected weapon. This is an original, simplified system inspired by modern weapon-banner mechanics.

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
