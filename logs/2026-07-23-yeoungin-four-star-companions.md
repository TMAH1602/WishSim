# 2026-07-23 — Yeoungin four-star companions

## Baseline and request

- Continues the uncommitted Battle Test, early ability, and Yeoungin work documented in the preceding 2026-07-22 and 2026-07-23 logs; published v0.5.0 documentation tip before this release remains `b26672b`.
- Requested two fully implemented four-star characters—one woman and one man—with Korean names and design language related to Yeoungin's release without duplicating her element, weapon, silhouette, or role.
- The user also authorized publishing the complete accumulated version to GitHub and the Homebrew tap after implementation.

## Character designs

- Added `Seo-yeon`, a four-star Electro catalyst tactician. Her indigo Korean-inspired court-mage layers, controlled ward slips, floating ledger, and measured violet lightning connect her to Yeoungin's restrained winter-court presentation while establishing a distinct intellectual combat identity.
- Seo-yeon's foundational stats emphasize ELEMENTAL ATK and measured SPD. Her early Battle Test loadout is `Ward Spark`, `Measured Thunder`, and `Edict of the Violet Court`.
- Added `Ji-ho`, a four-star Pyro sword guardian. His burgundy durumagi-inspired split coat, compact lamellar protection, warm cream accents, and controlled flame-wrapped blade establish him as a practical hearthward rather than another Cryo courtier.
- Ji-ho's foundational stats emphasize ATK, DEF, HP, and POISE. His early Battle Test loadout is `Ember Guard`, `Hearthline Draw`, and `Oathfire Procession`.
- Both are included in the shared featured four-star character pool, canonical archive, inventory/filter paths, Character Management, team selection, equipment typing, profiles, and Battle Test roster.

## Assets and rendering

- Generated `assets/characters/seo-yeon.png` and `assets/characters/ji-ho.png` with the built-in image-generation workflow, using Yeoungin only as a WishSim pixel-art and release-language reference.
- Seo-yeon's prompt specified an original adult Korean Electro catalyst tactician, indigo Korean-inspired layered court attire, shoulder-length dark hair with one understated pin, an open floating ledger, orbiting ward slips, controlled lightning, and a flat green chroma background.
- Ji-ho's prompt specified an original adult Korean Pyro sword guardian, burgundy/charcoal/cream Korean-inspired guard attire, short practical topknot, compact lamellar armor, one complete flame-wrapped sword, and a flat green chroma background.
- Both outputs were processed with the imagegen chroma helper using soft matte, despill, and a one-pixel edge contraction. The final project assets are 1024×1536 RGBA cutouts with transparent corners and were visually inspected.
- Registered both exact names in `src/art.rs` and `src/kitty.rs`, preserving portable ANSI and primary Ghostty/Kitty protocol rendering.

## Files and compatibility

- `src/simulation.rs`: four-star pool entries, weapon types, elements, stats, metadata tests, and roster count.
- `src/ui.rs`: complete lore, quotes, colors, weapon/element profiles, and shared raster presentation.
- `src/art.rs` and `src/kitty.rs`: exact-name image registries and protocol coverage.
- `src/battle.rs`: named three-tier provisional ability loadouts.
- `CHANGELOG.md`: user-facing summary.
- No save migration is required. Existing saves remain compatible because inventory, teams, and equipment continue to store stable catalog names.

## Validation and limitations

- Passed `cargo fmt --all -- --check`, `cargo check --locked`, `cargo test --locked` (37 passed, 0 failed), `cargo clippy --locked --all-targets -- -D warnings`, `cargo build --release --locked`, and `git diff --check`.
- The shared raster tests loaded and transparency-validated both portraits; protocol registry coverage includes both exact names. `sips` confirmed both final assets are 1024×1536 with alpha.
- Four-star rate and guarantee behavior are unchanged; the larger shared featured pool reduces the chance of any particular four-star name.
- Character-specific advanced ability effects remain future work beyond their current damage-oriented early loadouts.
- Manual reveal, archive, Character Management, team, Battle Test, ANSI, and real-Ghostty checks remain recommended.

## Safe backtracking

- Remove only the `Seo-yeon` and `Ji-ho` entries from `FEATURED_FOUR`, weapon/element/stat metadata, profiles, battle loadouts, both raster registries, focused test expectations, and their two PNG assets.
- Restore the canonical roster count from 29 to 27 only if both characters are removed.
- Do not remove Yeoungin, her signature weapon, the Battle Test, early abilities, enemy assets, or any preceding uncommitted work.
