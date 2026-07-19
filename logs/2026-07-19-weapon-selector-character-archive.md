# 2026-07-19 — Weapon selector, character archive, and four-star roster

## Baseline and request

- Continues the uncommitted post-v0.3.1 work documented in `2026-07-18-standard-fate-supports-signatures.md` from working-tree baseline `0c6b30f`.
- Requested a scrollable weapon target selector with art preview, five new four-star characters, and a scrollable character archive that obscures unowned character art while retaining names.
- These changes remain uncommitted until explicitly committed.

## Implementation

- Expanded persisted `WeaponPath` to seven selectable limited five-star weapons. `P` opens `Phase::WeaponSelect`; arrows browse, `V` previews without mutation, `Enter` confirms and resets weapon Fate, and `Esc` cancels.
- Removed limited signatures from the true off-banner weapon pool. Celestial Atlas and Wolfsong Claymore remain off-banner five-stars.
- Added `Phase::CharacterArchive`, opened with `C`, using the canonical `all_characters()` catalog. It shows six records per page in a three-column grid and scrolls by row.
- Owned records use the shared raster gallery's archive-sized sprite. Unowned records remain named but use a dim question mark and `LOCKED` label. Ownership comes from saved inventory.
- Added Kestrel (Anemo Bow), Mako (Hydro Dual Blades), Ysra (Pyro Polearm support), Dolma (Geo Claymore defender), and Corvin (Cryo Gauntlet disruptor) to the shared four-star pool with metadata, stats, profiles, and both graphics registries.

## Assets

- Built-in image generation used Astraea, Mira, and Farah as live style references.
- Generated each character on a flat magenta chroma background and processed it through the imagegen skill `remove_chroma_key.py` helper with soft matte and despill.
- Added `assets/characters/{kestrel,mako,ysra,dolma,corvin}.png`, each 1024×1536 RGBA.

## Validation and limitations

- `cargo fmt --all -- --check` passed.
- `cargo check --locked` passed.
- `cargo test --locked` passed: 21 tests.
- `cargo clippy --locked --all-targets -- -D warnings` passed.
- `cargo build --release --locked` passed.
- `git diff --check` passed.
- Asset tests validated dimensions, alpha/chroma integrity, gallery decoding, and Kitty/Ghostty registry coverage for all five new sprites.
- Manual 80×34 ANSI and real Kitty/Ghostty interaction remain required before release, especially selector preview placement and archive thumbnail readability.

## Safe backtracking

- Remove only the weapon selector by deleting `Phase::WeaponSelect`, its key/render branches, the expanded `WeaponPath` variants, and corresponding documentation; restore signature pool placement only if the prior low-probability off-banner behavior is explicitly desired.
- Remove only the archive by deleting `Phase::CharacterArchive`, `all_characters()`, the archive raster size, its key/render branches, and controls copy.
- Remove one new four-star by deleting its shared-pool entry, element/stats/profile metadata, both raster registries, asset, and focused test expectations. Do not remove unrelated earlier roster assets.
