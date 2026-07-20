# 2026-07-20 — Game shell, teams, equipment, and three-star art

## Baseline and request

- Begins from clean published v0.4.0 documentation tip `a84f364`.
- Requested a main menu separating Teams, Wish, Inventory, and Character Management; five named three-character teams; duplicate-based N0–N10 Ascension; type-matched weapon equipment; centered horizontal character browsing; side-by-side character/weapon art; three-star weapon assets; and a natural tan complexion correction for Corvin.
- The work was initially uncommitted and was approved for publication as part of v0.5.0 on 2026-07-20; final commit and release identifiers are recorded in the corresponding release log.

## Data and behavior

- Added backward-compatible `teams` and `equipment` fields to `SaveData`. Its custom default creates exactly five teams named Team 1 through Team 5, each with three empty slots.
- Team names accept up to 24 characters. Team members must be owned characters, and a character is unique within a given team.
- Equipment maps canonical character names to owned weapons filtered by `character_weapon_type()`. No abilities, damage formulas, or battle state were added.
- Inventory deletion now removes dangling team and equipment references while preserving pity and history.
- Ascension is derived from inventory copies. One copy displays N0; ten or more copies display the requested N10 cap. The two-row square roadmap fills upward from its lower row.

## UI and graphics

- Added `MainMenu`, `Teams`, `TeamCharacterSelect`, `Characters`, and `CharacterWeaponSelect` phases inside the existing event-driven state machine.
- Character Management uses a horizontal name carousel with selected full art centered between stats and Ascension/equipment panels.
- Equipment selection shows the character and highlighted compatible weapon side by side. `GraphicsRenderer` now tracks multiple image IDs so Ghostty and Kitty can place both embedded PNGs without ANSI underlays.

## Assets

- Built-in image generation created `assets/weapons/{dawncool_steel,raven_bow,quartz_spear,wanderers_notes,old_mercenarys_greatsword}.png` using existing weapon art as style references and a flat magenta chroma workflow.
- Built-in image editing revised `assets/characters/corvin.png`, changing only visible skin to a natural light warm tan while preserving pose, armor, hair, frost effects, and silhouette.
- All six sources were processed with the imagegen skill chroma helper into 1024×1536 RGBA cutouts and registered in ANSI and Ghostty/Kitty consumers.

## Validation and limitations

- `cargo fmt --all -- --check` passed.
- `cargo check --locked` passed.
- `cargo test --locked` passed after the interface continuation: 26 tests.
- `cargo clippy --locked --all-targets -- -D warnings` passed.
- `cargo build --release --locked` passed.
- `git diff --check` passed.
- Raster tests validated 1024×1536 RGBA dimensions, transparent corners, chroma cleanup, ANSI gallery loading, and Ghostty/Kitty registry coverage for Corvin and all five three-star weapons.
- Battle mechanics, abilities, leveling, and derived equipment-stat totals are intentionally not implemented.
- Manual 80×34 ANSI and real Ghostty dual-image validation remain recommended before release.

## Safe backtracking

- Remove the game shell by deleting only the new phase variants, their key/render branches, and the `teams`/`equipment` fields and helpers; retain Wish and Inventory phases.
- Remove three-star raster art by deleting the five registry entries and PNGs; symbolic fallback remains available.
- Restore only `assets/characters/corvin.png` from v0.4.0 if the complexion edit is rejected; do not touch other character assets.

## Same-day interface refinement

- Enlarged the root menu, removed inline option descriptions, and moved the selected description into a clean lower panel.
- Reworked Teams as three side-by-side full-art cards. Up/down changes among five teams, left/right changes member slots, and each member shows a name plus element symbol below the art.
- Added `CharacterQuickSelect`, opened with `L`, with composable rarity (`R`), element (`E`), and weapon (`T`) filters.
- Equipment initially expanded every owned copy into a separate row; this proved unusable for high-count three-star weapons. The corrected picker shows one row per compatible weapon name, reports `xN UNEQUIPPED`, hides fully occupied weapons, uses rarity color, marks the current weapon with `◆`, and marks weapons held elsewhere with `●` without printing character names.

## Final pre-release wish-flow correction

- Removed equipped-character names from weapon-picker rows so long names cannot overflow the list; each row now displays the number of unequipped copies instead.
- Reworked ten-pull skip into a five-star-only continuation: ordinary remaining results are skipped, while every remaining 5-star plays its intro cutscene and result card in pull order before the summary opens.
- Added a regression test covering multiple 5-stars in one skipped ten-pull.
- Removed the stale Polaris Edge/Nova Grimoire subtitle from the weapon banner. The path selector displays art immediately; `V` toggles to weapon details and stats.
- The final regression pass covered the combined original implementation and this continuation: formatting, locked checks, 26 tests, strict Clippy, locked release build, and `git diff --check` all passed.

## Same-day usability correction

- Centered root-menu option labels using paragraph alignment rather than manual padding.
- Reserved a spacer below each team portrait and moved the name/element label to the card bottom; native Ghostty/Kitty placement now uses exactly the same reduced art rectangle.
- Rarity-colored the equipped weapon name on the Character Management overview.
- Replaced duplicate-per-row equipment rendering with the aggregated available-count behavior described above.
