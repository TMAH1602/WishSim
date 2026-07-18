# 2026-07-18 — inventory, roster, weapon classes, and stats

## Repository state

- v0.2.0 baseline: tag `v0.2.0`, commit `2c59836`.
- Branch tip before this conversation’s feature work: `a0f3b18` (`Document Homebrew installation`).
- Status at log creation: the changes below are present in the working tree and are not represented by a new commit in the visible history.

## Requested work

The request expanded inventory sorting/filtering, added a featured five-star Electro Claymore character named Vaughn, added standard five-star Pyro brawler Cinder, introduced several four-star characters and weapon classes, generated matching pixel-art portraits, and established base combat stats for a future 3v3 system.

Follow-up feedback required:

- Removing green chroma backgrounds from new portraits.
- Matching the old transparent embedded portrait pipeline.
- Restoring v0.2.0 UI and naming consistency.
- Fixing new character portraits in Kitty, which has a separate asset registry.
- Formatting Vaughn’s banner like the older character banners.

## Features currently implemented

### Inventory

- Sort modes: name A–Z, rarity descending, item type, and element.
- Filters: all/characters/weapons and all elemental affinities plus unaligned.
- Keys: `S` cycles sort, `F` cycles type, `E` cycles element.
- Styled filter badges and explicit shown/total/selected counts.
- Cursor returns to a valid starting row after filter/deletion changes.
- Existing selection, inspection, and confirmed deletion behavior remains.

Primary file: `src/app.rs`; rendering and labels: `src/ui.rs`; controls documented in `README.md`.

### Characters

- `Vaughn, Violet Oath`: featured five-star, Electro, Claymore; new `Vaughn` banner.
- `Cinder, Forgeheart`: standard five-star, Pyro, Gauntlet.
- `Zephra`: featured four-star, Anemo, Gauntlet.
- `Neris`: featured four-star, Cryo, Scythe.
- `Brikka`: featured four-star, Pyro, Dual Blades.

Catalog/pool definitions are in `src/simulation.rs`; banner model and CLI parsing are in `src/model.rs` and `src/main.rs`; profiles and banner copy are in `src/ui.rs`.

Vaughn’s home presentation was normalized to the v0.2.0 pattern:

- Hero: `V A U G H N`
- Subtitle: `VIOLET OATH  •  STORMBOUND KNIGHT`
- Quote: `Behind iron, even thunder learns to kneel.`
- Banner title: `Violet Oath Eternal`

### Weapons

- `Galegrip Knuckles`: four-star Anemo Gauntlet.
- `Winter's Requiem`: four-star Cryo Scythe.
- `Twin Cinderfangs`: four-star Pyro Dual Blades.

New weapon kinds have compact reveal sprites and larger inspection art in `src/ui.rs`.

### Stats

Added base fields: CRIT DMG, CRIT RATE, ATK, DEF, SPD, ELEMENTAL ATK, HP, and POISE. Values are intrinsic catalog/profile data rather than mutable saved progression. Inspection screens render a `COMBAT PROFILE` section using the existing dim-label/accent-value style.

## Assets

Added:

- `assets/characters/vaughn.png`
- `assets/characters/cinder.png`
- `assets/characters/zephra.png`
- `assets/characters/neris.png`
- `assets/characters/brikka.png`

The images were generated using Kaelis as a style reference. Final project files are `1024 × 1536` RGBA transparent cutouts. An initial mistake left opaque green backgrounds; a later pass reprocessed the untouched generated originals with a pixel-art chroma key. Tests now require transparent corners and opaque subject pixels.

Portraits must be registered twice under the current v0.2.0 architecture:

- `src/art.rs::PORTRAITS` for terminal half-block/ANSI rendering.
- `src/kitty.rs::portrait_bytes()` for full-resolution Kitty rendering.

The first implementation updated only `art.rs`, which is why new art still disappeared under Kitty. Both registries now include all five names, and a Kitty mapping test was added.

## Important fixes and decisions

- Corrected Astraea’s new metadata to Cryo to agree with her established v0.2.0 profile.
- Added `UNALIGNED` so affinity-less weapons are not unreachable when filtering.
- Replaced debug enum labels with authored UI labels.
- Kept nearest-neighbor portrait rasterization and the old `trim_transparent()` behavior.
- Kept character-event pity shared across all event banners.
- Added Vaughn to CLI banner parsing and README examples.
- Did not serialize stats into save data because combat/progression behavior is not defined yet.

## Files changed in this work stream

- `README.md`
- `src/app.rs`
- `src/art.rs`
- `src/kitty.rs`
- `src/main.rs`
- `src/model.rs`
- `src/simulation.rs`
- `src/ui.rs`
- Five PNGs listed above
- Documentation/logging files added alongside this entry

Use `git status --short` and `git diff -- <path>` to confirm the live scope; later work may have modified these same paths.

## Validation recorded

At the latest feature validation before documentation:

- `cargo test`: 14 tests passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `git diff --check`: passed.
- Portrait alpha metadata: all five new assets report alpha.
- Regression tests cover transparent cutouts, ANSI gallery loading, Kitty portrait mapping, and new catalog metadata.

## Known limitations and follow-up opportunities

- Portrait names/assets are duplicated between the ANSI and Kitty registries, matching v0.2.0 but making omissions possible. A future refactor could expose one shared immutable portrait registry, provided both rendering paths and tests remain unchanged.
- Stats are foundational values only; no combat formulas, equipment application, leveling, or save progression exists.
- New weapon assets are terminal symbolic art, not full raster weapon portraits.
- Manual Kitty testing still depends on running in a real Kitty terminal; automated tests validate byte mappings but cannot prove external `kitten icat` behavior.
- Item metadata is partly name-matched. Before a large roster expansion, consider a single catalog metadata representation while preserving serialized inventory names and pull behavior.

## Safe backtracking map

Do not reset the entire repository to v0.2.0 unless the user explicitly requests losing every accepted post-v0.2.0 change.

- Remove only inventory sort/filter work: inspect/revert the `InventorySort`, `InventoryKind`, `ELEMENT_FILTERS`, inventory key handling, visible-list ordering, and inventory footer/header hunks in `src/app.rs`, `src/ui.rs`, and the README control bullets.
- Remove only Vaughn: remove the banner enum/CLI/home match arms, `VAUGHN` catalog constant and featured selection, Vaughn profile and both portrait registrations, then remove `assets/characters/vaughn.png`. Re-run exhaustive-match compilation and banner tests.
- Remove only Cinder or a four-star: remove its pool entry, metadata/profile mapping, both portrait registrations, PNG, and associated tests. Existing saved inventory/history may retain its name; decide on migration before removal from a released build.
- Remove only new weapon classes: remove their pool entries, metadata, compact and detail sprites, and catalog tests.
- Remove only stats: remove `Stats`, `Item::stats`, stat catalog function/data, and the `COMBAT PROFILE` UI block. Save migration is not required because stats are not serialized.
- Restore exact v0.2.0 behavior for reference with `git show v0.2.0:<path>` and manually apply only the required hunks. Preserve unrelated current changes.

After any partial backtrack, run the complete validation gate and add a new dated log describing the reversal.
