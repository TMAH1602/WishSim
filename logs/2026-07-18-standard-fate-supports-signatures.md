# 2026-07-18 — Standard Fate banner, support roster, and signatures

## Baseline and request

- Continues the uncommitted banner-grid/roster/weapon work documented in `2026-07-18-banner-grid-saif-weapon-art.md` from baseline `0c6b30f`.
- Requested a selectable standard-character banner with a Fate point on off-target five-stars, inclusion in the selector grid, more four-star characters, and signatures for limited five-stars that lacked them.
- All changes remain in the working tree until explicitly committed.

## Standard Archive rules

- Added `Banner::Standard` to the selector grid, Home banner presentation, CLI, and simulation routing.
- Added `StandardPityState` to `SaveData` under existing `#[serde(default)]`, preserving older saves.
- Standard pity is independent from limited character and weapon pity: 0.6% base, soft pity after 73, hard pity at 90.
- The standard target cycles with `P`. Changing it resets Standard Fate to zero, matching weapon-path behavior.
- Pulling the selected standard five-star clears Fate. Pulling any different standard five-star sets Fate to one. At one Fate, the next standard five-star is guaranteed to be the selected character.
- The selectable standard roster is Veyra, Orin, Cinder, Pyrite, and Jeanette. Sergei remains limited.

## New four-star roster

- `Farah`: Geo catalyst defensive support and Saif's thematic teammate.
- `Anya`: Cryo sword ward support and Sergei's thematic teammate.
- `Rook`: independent Electro gauntlet damage character.
- All three join the existing shared four-star character pool and have stats, profiles, transparent art, ANSI gallery entries, and Kitty/Ghostty registry entries.

## Signature weapons

- Astraea — `Nova Grimoire` (existing catalyst).
- Kaelis — `Polaris Edge` (existing sword).
- Seraphine — `Dreamwood Recurve` (new bow).
- Vaughn — `Oathbreaker Thunder` (new claymore).
- Steven — `Veilfire Sutra` (new catalyst).
- Sergei — `White Hunt Reliquary` (new catalyst).
- Saif — `Sandsworn Dominion` (new polearm).
- New signatures are obtainable as five-star off-path weapons and use the shared raster weapon rendering pipeline.

## Assets and implementation

- Built-in image generation used `example_sprites.png`, Saif, and existing weapon art as style references.
- Generated sources on flat magenta chroma and processed them with the imagegen skill's `remove_chroma_key.py`.
- Added `assets/characters/{farah,anya,rook}.png` and five signature PNGs under `assets/weapons/`.
- Updated `src/model.rs`, `src/simulation.rs`, `src/app.rs`, `src/ui.rs`, `src/main.rs`, `src/art.rs`, and `src/kitty.rs`.

## Validation and limitations

- Tests cover off-target Fate gain, selected-character guarantee/consumption, catalog metadata, weapon type/rarity, alpha/chroma integrity, gallery decoding, and protocol registry completeness.
- `cargo fmt --all -- --check` passed.
- `cargo check --locked` passed.
- `cargo test --locked` passed: 20 tests.
- `cargo clippy --locked --all-targets -- -D warnings` passed.
- `cargo build --release --locked` passed.
- `git diff --check` passed.
- Manual 80×34 ANSI and real Kitty/Ghostty interaction remain required before release.

## Safe backtracking

- Remove only Standard Fate by deleting `Banner::Standard`, `StandardPath`, `StandardPityState`, `pull_standard`, its UI/CLI/key branches, and focused tests. Older saves containing the added defaulted field remain readable if ignored.
- Remove a four-star by deleting its pool entry, stats/element/profile, both registries, asset, and expectations.
- Remove a signature by deleting its constant/pool entry, both registries, asset, and signature tests; this does not require removing its character.
