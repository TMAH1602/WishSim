# 2026-07-23 — Klara art correction and canonical rename

## Baseline and request

- Continues the uncommitted limited Anemo release documented in `2026-07-23-klara-anemo-release.md`. The published design baseline remains `v0.2.0` (`2c59836`); these changes are in the working tree and are not committed.
- Requested correction of the limited Anemo character's full art because the lower/left arm appeared broken and the scythe shaft appeared to emerge from the sleeve.
- Requested renaming the character from Nadezhda to the easier-to-pronounce `Klara` everywhere without breaking catalogs, rendering, combat, CLI access, or existing saves.

## Art correction

- Used the built-in image-editing workflow with the prior full art as the sole edit target and strict identity/style reference.
- Preserved the character's face, short dark-brown hair, jade eyes, white/jade Russian-inspired armor, airborne sweeping pose, complete scythe design, leg placement, and circular wind-slash composition.
- Redrew the arm/weapon interaction so the lower arm has a readable shoulder-to-elbow-to-wrist line, both gloved hands visibly grip one continuous scythe shaft, and the shaft passes in front of rather than emerging from either sleeve.
- Generated on a flat magenta key, removed the chroma locally without despill, contracted the alpha edge by one pixel, and saved the corrected 1024×1536 RGBA cutout as `assets/characters/klara.png`.
- Renamed the existing matching 256×256 face portrait to `assets/portraits/klara.png`; the face portrait itself required no anatomy correction.

## Canonical rename and compatibility

- Renamed the canonical item to `Klara, Jade Tempest`, the banner enum/CLI value to `Klara`/`klara`, and the item constant to `KLARA`.
- Updated the shared banner arrays, featured-character mapping, catalog, stats, element and weapon typing, ability loadout, Battle Test coverage, banner copy and spaced hero name, lore/profile lookup, ANSI registry, Ghostty/Kitty full-art and face registries, README CLI example, changelog, and prior uncommitted feature logs.
- Renamed the full-art and face files so all embedded paths use `klara.png`; no stale asset include path remains.
- Added `SaveData::migrate_klara_name()`. At load time it:
  - merges old inventory copies into the new canonical key without losing duplicates;
  - updates saved team slots;
  - transfers equipped weapons while preserving an already-present new-key assignment;
  - updates saved wish-history names.
- The legacy exact name remains only inside the migration and its focused regression test. Migration occurs in memory during load and is persisted by the next normal save mutation, preserving the existing rule that seeded CLI pulls do not modify the save file.

## Files changed

- Rename/catalog/runtime: `src/model.rs`, `src/storage.rs`, `src/simulation.rs`, `src/main.rs`, `src/battle.rs`.
- Presentation/registries: `src/ui.rs`, `src/art.rs`, `src/kitty.rs`.
- Assets: replaced `assets/characters/nadezhda.png` with `assets/characters/klara.png`; renamed `assets/portraits/nadezhda.png` to `assets/portraits/klara.png`.
- Documentation: `README.md`, `CHANGELOG.md`, the earlier Klara release log, the BP/Goliath continuation log, and this log.

## Validation

- Visual inspection confirms two coherent arms, two visible grips, one continuous shaft, and no sleeve/shaft merge.
- The focused migration test combines old and new inventory copies and verifies team, equipment, and history references.
- Final formatting, tests, Clippy, release build, alpha/chroma tests, registry tests, and diff checks remain to be run after this log is written.

## Known limitations

- The old exact display name intentionally remains in migration code and its regression test; removing it would orphan pre-rename working-tree saves.
- The corrected image retains the same highly dynamic airborne pose, so manual real-Ghostty inspection remains recommended at native banner and Battle Test scales.

## Safe backtracking

- To restore only the prior artwork, replace `assets/characters/klara.png` with the pre-correction full-art bytes while keeping the Klara filename and all rename work.
- To undo the rename, reverse only the Klara enum/constant/display/CLI/profile/registry keys and asset filenames, then remove `migrate_klara_name()` and its test. Do not revert unrelated Battle Test, Standard path, Taisia, signature weapon, or Goliath changes.
