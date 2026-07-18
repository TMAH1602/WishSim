# WishSim v0.2.0 consistency guide

Last reviewed: 2026-07-18

## Purpose and authority

This is the durable implementation and style contract for extending WishSim. It is intended for contributors and future coding-agent conversations that do not have access to earlier discussion.

The authoritative baseline is Git tag `v0.2.0`, commit `2c59836`. That version established the character-event and weapon banners, cinematic pull flow, inventory and deletion workflow, full-color embedded portraits, Kitty graphics support, portable ANSI fallback, persistent pity, CLI mode, and test conventions. The later commit `a0f3b18` only documented Homebrew installation and was the branch tip before the 2026-07-18 feature work began.

When this guide and the current code disagree, first compare the relevant code with `git show v0.2.0:<path>`. Preserve deliberate improvements documented in `logs/`; do not blindly replace current code with the tag.

## Product identity

WishSim is a cinematic, original-character terminal wish simulator. Its presentation is restrained fantasy-cosmic rather than a generic dashboard:

- Dark navy/near-black backgrounds and starfields.
- Gold for five-star emphasis and primary ceremonial accents.
- Purple for four-star/arcane emphasis.
- Blue for three-star/celestial emphasis.
- Muted blue-gray for help text and secondary information.
- Double borders for major cards/dialogs and plain separators for supporting structure.
- Sparse use of symbols such as `✦`, `✧`, `★`, `◇`, and `†`.
- Original names, lore, quotations, and artwork only.

New UI should feel like another room in the same Archive. Avoid framework-default widgets, debug formatting, emoji-heavy copy, verbose prose inside panels, or controls that look unrelated to existing buttons and help rows.

## Architecture and ownership

Respect the v0.2.0 module boundaries:

- `src/model.rs`: shared domain types and serialized save structures.
- `src/simulation.rs`: item catalog, probability rules, pity mutation, pull recording, and deterministic simulation tests.
- `src/app.rs`: interactive state machine, key handling, transitions, and persistence triggers.
- `src/ui.rs`: Ratatui layout, styling, animations, terminal fallback art, profiles, and rendering tests.
- `src/art.rs`: decoding, transparent trimming, nearest-neighbor rasterization, and ANSI portrait registry.
- `src/kitty.rs`: Kitty graphics placement/clearing and its embedded portrait registry.
- `src/storage.rs`: save location and atomic persistence.
- `src/main.rs`: CLI parsing, plain pull mode, stats, and reset commands.

Extend the owner of an existing behavior. Do not put UI state in the simulation engine, persistence rules in widgets, or catalog probability logic in key handling.

## Minimum terminal and layout contract

The full UI assumes at least `80 × 34`. Preserve the small-terminal fallback in `ui::render`.

Layout rules:

- Use `Layout`, `Constraint`, `Margin`, and `centered()` as existing screens do.
- Major content should be centered and capped to a readable width instead of stretching indefinitely.
- Reserve fixed-height footer/help areas so controls do not overlap content.
- Verify labels at 80 columns, not only on a wide development terminal.
- Prefer two short stable lines over one line that wraps differently by width.
- Use `Clear` before modal/detail panels layered over existing content.
- Retain consistent panel backgrounds (`Color::Rgb(7–10, 11–12, 25–28)` family), borders, padding, and title alignment.

Do not silently raise the minimum terminal size to accommodate a new feature.

## Shared visual language

Reuse the established constants in `ui.rs`:

- `GOLD = (255, 205, 90)`
- `PURPLE = (198, 120, 255)`
- `BLUE = (90, 180, 255)`
- `DIM = (100, 115, 145)`

Use item/element-specific colors only as accents. Major conventions:

- Focused inventory rows: dark foreground on `GOLD`, bold.
- Selected but unfocused rows: teal foreground.
- Labels: `DIM`; values: white, rarity, element, or panel accent color.
- Major headings: spaced uppercase or concise uppercase, often bold.
- Quotes: italic, muted or profile accent color.
- Help text: centered and `DIM`/muted blue-gray.
- Dangerous confirmation: red border/title with explicit `Y` and `N` actions.

Key badges should use the same pattern as home-screen actions: padded text, contrasting background, and bold key label. Never expose `Debug` output such as `InventorySort::Rarity` or raw enum variant names to users; provide deliberate display labels.

## Copy and naming conventions

### Character banners

Every character banner must follow the v0.2.0 four-field pattern in `ui::home`:

1. Border title from `Banner::title()`, uppercased by the renderer.
2. Hero name with letters separated by spaces: `V A U G H N`.
3. Uppercase epithet and role separated by `  •  `: `VIOLET OATH  •  STORMBOUND KNIGHT`.
4. One short in-world sentence rendered as an italic quote.

Do not insert rarity, element, weapon type, or phrases such as “Event Wish” into the hero subtitle unless all character banners are deliberately redesigned together. Banner-specific gameplay data belongs in details or pity UI.

Banner enum additions must be handled exhaustively in:

- `Banner::ALL` and `Banner::title()` in `model.rs`.
- `featured_character()` in `simulation.rs`.
- Home banner copy/color and any banner art mapping in `ui.rs`.
- CLI `BannerArg` and conversion in `main.rs`.
- README CLI examples when user-facing.

Character-event banners share the same character pity and guarantees. A new banner must not accidentally create separate pity unless the design explicitly changes.

### Characters and weapons

- Five-star characters use a full display name plus epithet, such as `Astraea, Starbound`.
- Existing four-stars use concise names (`Mira`, `Thorne`, `Lumen`); follow the active catalog convention unless intentionally revising all peers.
- Weapon names should be evocative, original, and title-cased.
- Use exactly `Character` for character `Item.kind`; weapon kinds are their readable class (`Sword`, `Claymore`, `Gauntlet`, etc.).
- The same exact item name must be used in the simulation catalog, inventory, history, UI profiles, ANSI gallery, Kitty registry, tests, and asset filenames/mappings.

## Adding a character: complete checklist

A character is not complete when only its catalog entry exists. Update every relevant path:

1. Add the `Item` to the appropriate featured or standard pool in `simulation.rs`.
2. Ensure `catalog_item()` can find it.
3. Add elemental/stat metadata and verify it agrees with the profile copy.
4. Add a complete `ItemProfile` in `ui.rs`: title, element/weapon agreement, lore, quote, color, accent, and fallback terminal art where applicable.
5. Add `assets/characters/<lowercase-name>.png` in the required asset format.
6. Add the exact display name and `include_bytes!` path to `PORTRAITS` in `art.rs` for portable ANSI rendering.
7. Add the same exact display name and `include_bytes!` path to `portrait_bytes()` in `kitty.rs` for full-resolution Kitty rendering.
8. If featured, update every banner location listed above.
9. Add or extend tests that prove both portrait registries contain the name.
10. Exercise reveal, result detail, and inventory detail paths. Kitty and ANSI are separate consumers; success in one does not prove success in the other.

The missing Kitty registry step caused new portraits to disappear during the 2026-07-18 work. Treat the dual-registry check as mandatory until the code is deliberately refactored to one shared source.

## Character portrait asset specification

The v0.2.0 portraits are embedded, full-body, crisp JRPG-style pixel art:

- PNG, `1024 × 1536`, RGBA.
- Transparent background, including all four corners.
- Character and carried signature weapon completely in frame.
- Centered full-body silhouette with readable padding.
- Hard pixel-art edges and pixel clusters; no smooth painterly rendering.
- Strong dark outline and readable color separation at terminal downsampling sizes.
- No text, watermark, floor, cast shadow, scenery, frame, or leftover chroma rectangle.

Generation may use a flat green chroma background, but the project asset must not. Remove chroma before copying the final PNG into `assets/characters/`. Validate more than `hasAlpha`: corners must be alpha 0, the image must contain opaque character pixels, and no faint semi-transparent full-canvas rectangle may remain.

`art.rs` deliberately performs:

1. `image::load_from_memory`
2. conversion to RGBA
3. `trim_transparent()` using alpha greater than 16
4. nearest-neighbor rasterization to reveal/detail sizes

Do not replace `FilterType::Nearest` with smoothing filters. Do not pre-crop so tightly that weapons/capes touch the image edge. Do not make the background merely black; it must be transparent.

Kitty sends the original embedded PNG bytes through `kitten icat`. ANSI fallback uses the pre-rasterized `TerminalRaster`. Both must be validated.

## Adding a weapon class

A new weapon class needs more than a catalog string:

- Add items to the correct weapon/standard pools.
- Add element/stat metadata if the design uses it.
- Add a compact five-row `item_sprite()` branch for reveal cards.
- Add a larger `weapon_profile()` art branch for inspection.
- Match existing symbolic construction and fixed-width whitespace.
- Verify inventory columns remain aligned with the longer kind name.
- Verify weapon pulls use weapon pity and the intended four-star pool.

If raster weapon art is later introduced, create one reusable weapon registry and rendering path rather than partially mixing bitmap and ASCII weapons.

## Inventory feature conventions

Inventory remains a phase in the existing `App` state machine. Preserve:

- `↑`/`↓` navigation.
- Space selection and `A` select-all behavior.
- Enter to inspect.
- Confirmed deletion with `D`, `Shift+D`, and `Y`/`N`.
- Deletion preserving pity and history.
- Cursor bounds based on the currently visible filtered list.

Sorting/filtering rules:

- Name is ascending A–Z.
- Rarity is descending 5★ to 3★ with name as a stable tie-breaker.
- Type groups characters/weapons predictably and uses name as a tie-breaker.
- Element uses name as a tie-breaker.
- Filters must include `ALL`; weapons without an affinity must remain reachable through `UNALIGNED`.
- Header copy must distinguish shown/filtered count from total unique inventory count.
- Changing filters should return the cursor to a valid row and must not leave invisible selected entries.
- UI labels must be authored labels such as `RARITY 5★–3★`, not enum debug strings.

New controls belong in the existing bordered footer and must remain readable at 80 columns.

## Stats conventions

The current foundational stats are:

- CRIT DMG
- CRIT RATE
- ATK
- DEF
- SPD
- ELEMENTAL ATK
- HP
- POISE

Until the 3v3 combat system defines formulas, treat these as catalog/base profile values, not saved mutable progression. Keep naming identical everywhere. In detail UI:

- Use `DIM` labels and accent-colored bold values.
- Use stable short rows that fit the narrow detail column.
- Keep the section heading consistent with `ARCHIVE LORE` styling.
- Do not hide zero-valued weapon fields without a deliberate global presentation decision.

When combat arrives, document derived formulas, caps, turn order, damage resolution, and migration behavior before changing stored save data.

## Persistence and compatibility

`SaveData` uses `#[serde(default)]`; retain this for backward-compatible additions. Existing saves contain inventory names and history names as strings, so renaming catalog items can orphan entries. Prefer stable names. If a rename is necessary, implement and test a migration.

Save only after deliberate state mutations, following existing `storage::save()` call sites. Seeded CLI pulls must remain reproducible and must not modify the save file. Inventory deletion must never change pity or wish history.

## Animation and state-machine conventions

Add screens as explicit `Phase` variants with owned state. Handle transitions centrally in `App::handle_key` and time-driven transitions in `advance`. Preserve:

- Space/Enter advance behavior.
- `S` skip-all behavior for ten-pulls.
- `Esc` returning to the appropriate prior screen.
- `Q` confirmation behavior across phases.
- Five-star pre-reveal cutscene routing.

Avoid widget-local hidden state or blocking animation loops. Rendering should be a pure view of `App` plus current time.

## Testing and validation gate

Before handing off any feature, run:

```sh
cargo fmt --all
cargo test
cargo clippy --all-targets -- -D warnings
git diff --check
```

Add focused regression tests for the failure modes touched. For character art, require transparent-cutout validation, gallery loading, and Kitty mapping. For catalog additions, verify name, element, type, and pool behavior. For pity changes, use seeded tests. For deletion or filtering, test preservation and cursor/selection behavior.

Also perform proportional manual checks:

- Minimum-size ANSI terminal (`80 × 34`).
- Wider ANSI terminal.
- Kitty reveal and detail/inventory detail when Kitty is available.
- Single pull and ten-pull.
- CLI banner parsing for new banners.
- Existing save load if serialized structures changed.

Tests passing does not replace visual inspection when layout or art changed.

## Change management and rollback

Before work:

1. Read `AGENTS.md`, this guide, and the newest file under `logs/`.
2. Run `git status --short`.
3. Treat existing modifications as user-owned unless the log proves they belong to the feature being continued.
4. Inspect v0.2.0 with `git show v0.2.0:<path>` when matching old behavior.

After work, update a dated log with exact files, tests, decisions, and limitations. If the user dislikes a change, do not broadly reset to v0.2.0: that would also erase accepted later work. Use the dated log and current diff to revert only the rejected feature.

## Definition of done

A feature is complete only when:

- Behavior is integrated into existing architecture and every relevant match/registry.
- Copy, colors, borders, spacing, controls, and fallback rendering match v0.2.0 conventions.
- It works in both Kitty and portable ANSI paths where applicable.
- It remains usable at `80 × 34`.
- Existing saves and CLI behavior remain compatible or have a tested migration.
- Tests, formatting, Clippy, and diff checks pass.
- README/help text is current when controls or commands changed.
- A dated log records the work and safe backtracking scope.
