# 2026-07-22 — Battle Test foundation

## Baseline and request

- Begins from clean published v0.5.0 documentation tip `b26672b` and release tag `v0.5.0` (`433e09a`).
- Requested a testing-only turn-based battle selection in the main menu, deployment from one saved three-character team, temporary level-50 combatants, speed-based turns, Fight/Magic/Defend actions, healer targeting, provisional 2× elemental matchups, a tutorial screen, and simultaneous art for three allies and three enemies.
- Work remains uncommitted until explicitly requested.

## Architecture and behavior

- Added `src/battle.rs` as the transient combat rules owner. `src/app.rs` retains phase transitions and input routing; `src/ui.rs` remains the renderer.
- Added `BattleTeamSelect`, `Battle`, and `Tutorial` phases to the existing Ratatui state machine. The main menu now has six consistently styled destinations.
- Character and team-management views identify the current roster baseline as level 1. Battle Test accepts only a complete saved team, derives canonical stats plus equipped weapon ATK and ELEMENTAL ATK, projects the session to level 50, doubles base HP for test pacing, and never saves battle state.
- Each encounter is 3v3 against two Hydro Slimes and one Dendro Thornbloom. SPD determines a stable descending round order; defeated units are skipped and enemy turns automatically resolve until the next living ally.
- `FIGHT` deals physical damage from ATK. Offensive `MAGIC` uses ELEMENTAL ATK and the centralized provisional 2× table. Jeanette is the current explicit healer and selects a living teammate with Magic. `DEFEND` reduces damage using DEF plus POISE until that character's next action.
- Victory and defeat return to team selection. No rewards, progression, level persistence, abilities, status effects, AI targeting strategy, or battle save migration were added.

## UI and assets

- The Battle Test screen displays three enemy cards, three ally cards, full art, level, element, HP gauges, active/target borders, a combat log, and the shared command panel at the existing 80×34 minimum.
- Added `Info / Tutorial` to the main menu with the three temporary commands and every active elemental matchup.
- Generated `assets/enemies/hydro_slime.png` and `assets/enemies/thornbloom.png` with the built-in image generation workflow using existing WishSim portraits as style references and flat magenta chroma sources.
- Removed chroma locally with the imagegen skill helper, normalized both final assets to 1024×1536 RGBA, and registered their exact names in `art.rs` and `kitty.rs` so ANSI, Ghostty, and Kitty share the same assets.
- Final generation prompts specified crisp full-body JRPG pixel art, hard pixel clusters, strong dark outlines, no text/scenery/shadows, and flat `#ff00ff` removable backgrounds. The slime was constrained to a blue Hydro body with exactly two circular eyes; Thornbloom was constrained to a faceless fantasy carnivorous-plant silhouette with thorns, vines, and Dendro colors.

## Validation and limitations

- `cargo fmt --all -- --check` passed.
- `cargo check --locked` passed.
- `cargo test --locked` passed: 31 tests, including battle rules, healer targeting, minimum-size portrait layout, transparent-cutout validation, gallery loading, and protocol-registry coverage.
- `cargo clippy --locked --all-targets -- -D warnings` passed.
- `cargo build --release --locked` passed.
- `git diff --check` passed.
- Both enemy assets passed the shared 1024×1536 RGBA, transparent-corner, opaque-subject, and chroma-residue tests.
- Elemental relationships and numeric formulas are intentionally provisional and are surfaced as such in the UI.
- Jeanette and Yeoungin are currently classified through temporary name-based healer/support mapping. Future character kits should replace this with canonical ability metadata.
- Manual real-Ghostty and minimum-size interaction checks remain recommended because automated tests cannot verify GPU placement or keyboard feel.

## Safe backtracking

- Remove only `src/battle.rs`, the three battle/tutorial phase variants and input/render branches, the two `assets/enemies/*.png` files, and their two `art.rs`/`kitty.rs` registry entries to remove this prototype.
- Restore the main menu from six entries to its previous four-entry cursor bound and copy. Remove only the Battle Test sections from README, changelog, consistency guide, and this dated log.
- Do not change saved teams, equipment, inventory, pity, wish history, or the v0.5.0 release tag; Battle Test does not migrate any of them.
