# 2026-07-23 — Klara Anemo release and Standard path selector

## Baseline and request

- Continues the uncommitted Battle Test and portrait work documented by the other 2026-07-23 logs. The published design baseline remains `v0.2.0` (`2c59836`); all work described here is in the working tree and is not committed.
- Requested a new limited five-star adult white woman with Anemo affinity, jade eyes, short dark-brown hair, mobile Russian-inspired white/jade attire, a scythe, and action-oriented full art showing an elemental air slash.
- Requested one dedicated four-star Anemo damage buffer, one five-star signature weapon on the selectable weapon banner, and a Standard character Fate-path selector matching the established weapon-path interaction.

## Catalog and banner

- Added `Klara, Jade Tempest` as a limited five-star Anemo Scythe striker with high ELEMENTAL ATK and SPD, solid offensive crit values, and deliberately lower DEF/POISE.
- Added `Banner::Klara`, CLI `--banner klara`, shared character-event pity integration, and the established four-field banner treatment under `The Wind Reaps White`.
- Expanded the banner archive to ten cards and four rows without introducing a separate selector implementation.
- Added `Taisia`, a four-star Anemo Catalyst buffer whose role-based Battle Test support targets an ally and increases damage used by both physical and elemental calculations. Her named abilities are `Road-Bell Chime`, `Blessing of the Open Gale`, and `Seven Winds Procession`.
- Added `Gale's Last Harvest`, a five-star Anemo Scythe, to `WeaponPath::ALL`, catalog metadata, the existing scrollable weapon Fate-path selector, and both raster registries.

## Standard Fate-path interaction

- Added `StandardPath::ALL` and `Phase::StandardSelect`.
- Pressing `P` on the Standard Archive now opens a scrollable list with the selected character's art by default; `V` swaps the right panel to profile/stats, `Enter` confirms and resets Standard Fate, and `Esc` returns without changing saved state.
- The screen deliberately reuses the weapon selector's panel proportions, focus treatment, selected-path marker, preview semantics, and help wording. The existing serialized `StandardPityState` did not change, so older saves remain compatible.

## Assets and registration

- Generated `assets/characters/klara.png`, `assets/characters/taisia.png`, and `assets/weapons/gales_last_harvest.png` with the built-in image generation workflow using existing WishSim sprites only as style references.
- Generated dedicated front-facing archive portraits at `assets/portraits/klara.png` and `assets/portraits/taisia.png`.
- All sources used a flat magenta chroma background because jade/Anemo effects conflict with the usual green key. The project outputs were processed through the imagegen chroma helper without despill, alpha-edge contracted by one pixel, and normalized to the established 1024×1536 full-art/weapon and 256×256 face-portrait dimensions.
- Registered every exact item name in both `src/art.rs` for ANSI rendering and `src/kitty.rs` for Ghostty/Kitty protocol rendering.

## Files changed

- Domain/state/catalog: `src/model.rs`, `src/simulation.rs`, `src/app.rs`, `src/main.rs`, `src/battle.rs`.
- Presentation/registries: `src/ui.rs`, `src/art.rs`, `src/kitty.rs`.
- Assets: `assets/characters/klara.png`, `assets/characters/taisia.png`, `assets/portraits/klara.png`, `assets/portraits/taisia.png`, `assets/weapons/gales_last_harvest.png`.
- Documentation: `README.md`, `CHANGELOG.md`, and this log.

## Validation

- `cargo fmt --all`
- `cargo test --locked`: 44 passed, 0 failed.
- Asset tests verify 1024×1536 transparent full art, 256×256 transparent face portraits, no retained opaque chroma pixels, gallery loading, and protocol-registry coverage.
- `git diff --check`
- `cargo check --locked` passed before removal of the now-unused Standard path cycling helper; full final lint/build validation remains to be run after documentation updates.

## Known limitations

- Taisia currently uses the shared provisional Buffer mechanic, whose ATK increase contributes to elemental ability power as well as basic attacks. Element-specific buff duration and stacking rules remain future Battle Test work.
- Manual checks in a real Ghostty window are still recommended for the dynamic scythe/wind silhouette, banner archive's fourth row, both new face portraits, and both Fate-path preview screens.

## Safe backtracking

- Remove only `Banner::Klara`, `KLARA`, its catalog/type/element/stats/profile/CLI/ability arms, the banner copy and art mapping, and `assets/characters/klara.png` plus `assets/portraits/klara.png` to remove the limited character.
- Remove only `Taisia` from `FEATURED_FOUR`, character typing/element/stats/profile/role/ability mappings and both registries, then delete `assets/characters/taisia.png` and `assets/portraits/taisia.png` to remove the companion.
- Remove only `WeaponPath::GalesLastHarvest`, `GALES_LAST_HARVEST`, its catalog/profile/path/registry arms, and `assets/weapons/gales_last_harvest.png` to remove the signature.
- To restore the prior Standard target interaction, remove `Phase::StandardSelect`, `StandardPath::ALL`, its renderer/native placement branches, and restore only the previous `P` handler that called `StandardPath::next()`. Do not revert other `app.rs`, `ui.rs`, or current uncommitted Battle Test changes.
