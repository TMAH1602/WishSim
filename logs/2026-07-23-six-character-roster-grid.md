# 2026-07-23 — Six-character roster grid

## Baseline and request

- Continues the uncommitted Character Management portrait-grid work documented in `2026-07-23-battle-presentation-roster-portraits.md`. The published design baseline remains `v0.2.0` (`2c59836`); these changes remain in the working tree and are not committed.
- Requested six visible characters at a time and better use of available terminal screen real estate.

## Implementation

- Expanded the existing `CharacterQuickSelect` page from one row of three cards to two rows of three cards. No parallel selector or catalog was introduced.
- Changed page boundaries from three to six canonical filtered characters. Left/right still moves one card and up/down still moves one three-column row, preserving the established control language.
- Increased the shared roster panel cap from `92×31` to `112×38`. At the supported `80×34` minimum it uses the full terminal; on larger windows it remains centered and capped rather than becoming an unbounded dashboard.
- Preserved the existing filter header, rarity-colored card borders, double selected border, two-line character label, and fixed footer.
- Updated `graphics_portraits()` and `character_grid_portrait_areas()` to return the same six card portrait rectangles used by the Ratatui layout. This keeps Ghostty/Kitty face placements aligned with the ANSI fallback and prevents native images from using the old three-card page.
- Removed one excess inset from the native portrait-area helper so its geometry now mirrors the rendered card structure exactly.

## Files changed

- `src/ui.rs`: six-item paging, two-row card layout, larger panel, and matching six-image native placement geometry.
- `CHANGELOG.md`: unreleased UI summary and correction of the obsolete three-card description.
- `logs/2026-07-23-six-character-roster-grid.md`: this log.

## Validation

- Final formatting, tests, Clippy, release build, minimum-terminal geometry checks, and diff checks remain to be run after this log is written.

## Known limitations

- Six native face portraits increase protocol traffic when changing pages or filters. Images remain small 256×256 embedded portraits and use the existing clear/redraw path.
- Manual inspection in a real Ghostty window remains recommended at both `80×34` and a larger terminal size.

## Safe backtracking

- Restore the page divisor/take count from six to three, `visible_rows` from two to one, the single-row layout in both helpers, and the prior `92×31` panel cap. Do not revert portrait assets, filters, character catalogs, or unrelated Character Management changes.
