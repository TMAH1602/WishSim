# 2026-07-23 — Yeoungin alpha and roster scrolling fix

## Baseline and request

- Begins from published v0.6.0 documentation tip `96dbb5d`, release commit `2fe8bd1`, and tag `v0.6.0`.
- Reported that Yeoungin's face rendered completely black and that character-selection lists did not scroll when the cursor moved below the final visible row.
- The fix is currently in the working tree and must not be described as committed or published until explicitly requested.

## Yeoungin portrait repair

- Compared the preserved built-in image-generation source with `assets/characters/yeoungin.png`. The source skin sample at `(512, 300)` was opaque `(230, 200, 170, 255)`, while the released asset had the same RGB value with alpha zero.
- The previous soft matte classified red-dominant skin as magenta spill, and alpha contraction further removed small pixel clusters. This explains why the face collapsed into dark hair/outline pixels at terminal size.
- Rebuilt the same generated portrait from its preserved source using the imagegen chroma helper in hard-key mode with the sampled magenta border and tolerance 42. No soft matte, despill, edge contraction, regeneration, or design change was applied.
- The repaired sample remains `(230, 200, 170, 255)`. The final asset remains a 1024×1536 RGBA cutout with transparent corners and was visually inspected.

## Roster viewport repair

- Added one shared `selected_list_scroll()` calculation in `src/ui.rs`.
- Applied it to both `ATTACH CHARACTER` team selection and the filtered `CHARACTER ROSTER` quick selector.
- The viewport remains at row zero while the cursor fits, then advances just enough to keep the selected entry on the final visible row.
- Added a focused boundary test covering the first row, exact viewport boundary, first overflow row, and deep roster selection.

## Files and compatibility

- `assets/characters/yeoungin.png`: corrected alpha matte only.
- `src/ui.rs`: shared selected-row viewport offset and focused test.
- `CHANGELOG.md` and `docs/V0_2_0_CONSISTENCY_GUIDE.md`: user-facing fix and durable scrolling requirement.
- No catalog names, saves, pity, inventory, teams, equipment, abilities, or combat formulas changed.

## Validation and limitations

- Passed `cargo fmt --all -- --check`, `cargo check --locked`, `cargo test --locked` (38 passed, 0 failed), `cargo clippy --locked --all-targets -- -D warnings`, `cargo build --release --locked`, and `git diff --check`.
- Passed explicit Yeoungin asset assertions: 1024×1536 RGBA, four transparent corners, and repaired face sample `(230, 200, 170, 255)`.
- Shared raster loading/transparency and Ghostty/Kitty registry tests remain green.
- Manual confirmation in a real Ghostty window remains recommended because Ghostty displays the original PNG while ANSI uses the downsampled gallery.

## Safe backtracking

- Restore only `assets/characters/yeoungin.png` to reverse the alpha repair; do not alter her registries or catalog entry.
- Remove only `selected_list_scroll()`, its two `.scroll(...)` applications, and its focused test to reverse roster viewport behavior.
- Do not revert the v0.6.0 character, Battle Test, ability, release-companion, or publication changes.
