# 2026-07-18 — Limited banner grid, Saif, standard roster, and weapon art

## Baseline and request

- Starting source state: clean `main` at `0c6b30f`, after the v0.3.1 publication record.
- Requested a grid showing all limited five-star banners, limited Geo polearm user Saif, Sergei's move from standard to limited, standard five-stars Pyrite and Jeanette, masculine Lumen lore, and assets for every five-star and four-star weapon.
- Work remains in the working tree until explicitly committed.

## Architecture and UI decisions

- Added `Phase::BannerSelect { cursor }` to the existing application state machine. `B` opens it from Home; arrows move through a three-column grid; Enter selects; Esc/B returns.
- Preserved the established Home banner screen and left/right cycling. The grid is an additional scalable selection surface, not a parallel wish flow.
- Added an authoritative banner-selector roster. Sergei and Saif use the existing shared character-event pity and guarantee.
- The weapon banner remains outside the limited-character grid and continues using its separate weapon pity/path.
- Cards reuse Archive colors, double borders, authored profile labels, and the 80×34 layout contract.

## Roster and balance

- Added limited `Saif, Dune Sovereign`, a Geo polearm character with high attack/defense and sand-vortex presentation.
- Moved `Sergei, Winterfang` out of the standard pool and added his limited banner.
- Added standard `Pyrite, Gilded Step`, a high-speed Geo sword character.
- Added standard `Jeanette, Tidemender`, a high-HP Hydro bow healer profile.
- Updated Lumen's lore to use `he`/`his`.
- Existing string-based saves remain loadable. Sergei's inventory/history name is unchanged.

## Assets and rendering

- Generated new 1024×1536 pixel-art character sources for Saif, Pyrite, and Jeanette using `example_sprites.png` and Vaughn as style references.
- Generated isolated pixel-art assets for all four five-star weapons and all eight four-star weapons.
- Used the built-in image generation path. Flat chroma sources were processed with the imagegen skill's `remove_chroma_key.py` and saved as alpha PNGs.
- Character assets live under `assets/characters/`; weapon assets live under `assets/weapons/`.
- Extended `art.rs` and `kitty.rs` so four/five-star weapons use the same ANSI and Kitty/Ghostty reveal/detail paths as character art. Three-star weapons keep symbolic art.

## Files changed

- `src/model.rs`, `src/simulation.rs`, `src/main.rs`: banners, pools, CLI, metadata, and stats.
- `src/app.rs`, `src/ui.rs`: banner-grid state, navigation, rendering, lore, and raster weapon use.
- `src/art.rs`, `src/kitty.rs`: shared embedded registries and validation.
- `assets/characters/{saif,pyrite,jeanette}.png` and twelve PNGs under `assets/weapons/`.
- `README.md`, `CHANGELOG.md`, and the consistency guide.

## Validation and known limitations

- Chroma removal reported transparent backgrounds for every generated asset. Automated tests validate transparent corners, transparent pixels, opaque subject pixels, gallery decoding, and protocol registration.
- Long, narrow weapon silhouettes require a smaller minimum raster width than full-body characters; height and alpha checks remain strict.
- Manual visual checks in an interactive 80×34 ANSI window and real Kitty/Ghostty remain required before release.

## Safe backtracking

- To remove only the selector, reverse `BannerSelect`, its key arms, the selector roster, and `banner_select()` without changing banner pools.
- To restore Sergei as standard, remove his Banner/CLI arms and return the unchanged `Sergei, Winterfang` item to the standard five-star pool.
- To remove an added character, remove its pool/banner entry as applicable, metadata/stats/profile arms, both registries, PNG, CLI entry, and tests. Preserve names already present in saves unless a migration is designed.
- To remove raster weapon art, remove only weapon entries from both registries and `assets/weapons/`; restore reveal/detail weapon branches to symbolic rendering. Do not disturb character art.
