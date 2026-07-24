# Battle presentation, encounter selection, and roster portraits

Date: 2026-07-23
Baseline: continuation of the uncommitted `2026-07-23-battle-test-overhaul` working tree; repository release remains `v0.6.1`. Design baseline remains `v0.2.0` (`2c59836`).
Status: implemented in the working tree only; not committed or published.

## Requested behavior

- Expose every character's provisional Battle Test role and ability descriptions in Character Management.
- Enlarge Battle Test so command descriptions remain readable.
- Scale combat artwork by subject size and emphasize the active character.
- Give shields, attacks, healing, statuses, BP, and Energy stronger visual feedback.
- Add standard five-star signatures to the selectable weapon banner.
- Increase encounter difficulty and add status conditions with elemental immunities.
- Document the complete mechanics in the Field Manual.
- Show featured/selected character artwork on banners without moving the centered title.
- Replace the filtered one-dimensional roster with a face-icon grid.
- Establish a shoulders-up face-portrait asset set and sprite sheet for all 29 characters.
- Add a second boss and an encounter selector; very strong bosses may fight alone.

## Implementation

### Battle flow and encounters

- Added `BattleEncounterSelect` between team selection and combat.
- Added two encounters:
  - **Ruin Court:** Ember Wisp, Astral Ruin Knight, and Thornbloom.
  - **Somnial Frostwyrm:** an extreme solo Cryo boss with 7,200 HP, high defenses, freezing breath, drowsiness pressure, and stronger damage.
- The existing Battle Test state machine is reused; no parallel battle loop was introduced.
- The encounter name appears in the centered battle title.

### Status system and difficulty

- Added Burn, Frozen, Paralysis, and Drowsy alongside Poison.
- Pyro abilities can Burn, Cryo abilities can Freeze, Electro abilities can Paralyze, and Dendro abilities can induce Drowsiness.
- Matching Pyro, Cryo, Electro, and Dendro targets are immune to their corresponding condition.
- Dendro allies resist Thornbloom poison; Cryo allies resist Frostwyrm freeze; Dendro allies resist its drowsiness.
- Burn deals recurring max-HP damage. Frozen and Paralysis skip an action. Drowsiness counts down before forcing sleep.
- Deterministic proc checks preserve reproducible Battle Test behavior and tests.

### Battle presentation

- Increased the maximum battle panel from 34 to 42 rows and expanded the command area.
- Full command descriptions wrap below the selected action.
- Each unit card now has explicit BP diamonds and an Energy block meter.
- Shields display a cyan `BARRIER` frame and numeric shield status.
- Attack, heal, shield, and status events create temporary double-border/color pulses through time-driven battle effect state.
- The active ally uses the largest portrait allocation; inactive allies are reduced.
- Astral Ruin Knight and Somnial Frostwyrm use full-size portrait allocation, while Ember Wisp is intentionally rendered near half scale.
- A single Frostwyrm is placed in the central enemy slot.

### Character Management and roster

- Character Management now shows the character's role plus Basic, Skill, and Ultimate names and descriptions.
- The filtered roster is now a four-column, two-row paged portrait grid navigated with all arrow keys.
- Rarity borders, name, element, and weapon class remain visible on each icon.
- Added 29 individual `256×256` transparent face portraits and a canonical `1280×1536` five-column sprite sheet.
- The final face portraits were produced from the established full-art identities and then normalized into front-facing, neutral shoulders-up portraits. Sergei received a dedicated corrected portrait so his helmeted human identity is not replaced by the snow-wolf companion.
- Face icons have a dedicated raster size in the existing gallery loader; the full-art registry remains unchanged for all other screens.

### Banner artwork and weapon paths

- Home banners now render art beneath the existing centered banner title.
- Limited banners use their featured character.
- Standard Archive uses the currently selected Fate-path character.
- Weapon banner uses the currently selected weapon-path art.
- Added the five standard signatures to `WeaponPath::ALL`, bringing the selectable weapon paths from 8 to 13:
  - Tempest Meridian
  - Emberkeeper's Oath
  - Furnaceheart Bracers
  - Aurum Flash
  - Silver Tidemark
- These weapons remain obtainable from the standard five-star weapon pool as well.

### Field Manual

- Expanded the manual panel.
- Added BP/Energy costs, recharge rules, role definitions, both encounters, all status effects, and elemental status immunity guidance.

## Files changed

- `src/app.rs`
- `src/art.rs`
- `src/battle.rs`
- `src/kitty.rs`
- `src/model.rs`
- `src/simulation.rs`
- `src/ui.rs`
- `CHANGELOG.md`
- `assets/enemies/somnial_frostwyrm.png`
- `assets/portraits/character_face_sheet.png`
- `assets/portraits/*.png` for all 29 characters

This continues to include the enemy and signature assets listed in `logs/2026-07-23-battle-test-overhaul.md`.

## Asset workflow

- Built-in image generation was used for the Somnial Frostwyrm, the normalized face sheet, and Sergei's corrected helmet portrait.
- Existing WishSim full art and enemy art were supplied as visual/identity references.
- Flat `#00FF00` sources were removed with the shared chroma-key helper using soft matte, despill, and one-pixel edge contraction.
- Frostwyrm was normalized to the established `1024×1536` transparent enemy canvas.
- The face sheet was split into 29 named `256×256` transparent icons and reassembled as the canonical transparent sprite sheet.
- All generated project assets were visually inspected after alpha processing.

## Validation

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test` — 42 passed
- `git diff --check`

Tests added cover:

- solo encounter construction and Frostwyrm difficulty;
- matching-element status immunity;
- all 29 face icons having square dimensions, transparent corners, and visible content;
- all 13 weapon paths and standard-signature weapon-type compatibility.

## Compatibility and limitations

- No save schema changed.
- Encounter choice, conditions, animation pulses, BP, Energy, and shields remain transient Battle Test state.
- Face portraits are deliberately optimized for the terminal icon grid. Native Ghostty/Kitty full art remains active on screens that already use protocol graphics; the grid uses its compact raster portraits consistently in every terminal.
- Status proc rates and boss numbers remain provisional test tuning.

## Precise rollback guidance

1. Remove only `BattleEncounterSelect` and its key handling from `src/app.rs`.
2. Revert the encounter/status/effect fields and related resolution logic in `src/battle.rs`; preserve the earlier BP/Energy/role implementation if that work remains desired.
3. Revert the battle sizing, scaling, meters, effect borders, tutorial additions, Character Management ability copy, roster grid, and home-banner art hunks in `src/ui.rs`.
4. Remove only the five standard-signature variants added to `WeaponPath` in `src/model.rs`, plus their `weapon_for_path` arms and updated tests in `src/simulation.rs`.
5. Remove Frostwyrm entries from `src/art.rs` and `src/kitty.rs`, then delete only `assets/enemies/somnial_frostwyrm.png`.
6. Remove `FACE_PORTRAITS`, the `face` raster field, and its test from `src/art.rs`; delete only `assets/portraits/`.
7. Remove this work's Unreleased bullets from `CHANGELOG.md`.
8. Run `cargo fmt`, strict Clippy, `cargo test`, asset validation, and `git diff --check`.

## Visual-layout correction

After the first implementation was reviewed in a real terminal, three layout problems were identified and corrected:

- The portrait grid's four-column/two-row pagination made the new face assets too small. It now uses the full terminal, displays three large cards per page, allocates nearly the entire card to a `24×18` face raster, and navigates vertically in three-item rows.
- Battle Test no longer sits inside a capped `126×42` centered panel. Its outer panel consumes the full available terminal, enemy/ally rows grow with terminal height, and the control area retains a readable minimum. The Ruin Court enemy row now assigns 60% width to the central Astral Ruin Knight and 20% to each attendant; the solo Frostwyrm uses that large central slot.
- Banner artwork no longer owns a left-hand column that shifts all banner copy. Artwork is centered vertically beneath the still-centered border title, while hero name, subtitle, quote, and path copy occupy a separate full-width centered row beneath the art.

The correction changed `src/art.rs`, `src/app.rs`, and `src/ui.rs` only. Final correction validation passed:

- `cargo fmt`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test` — 42 passed
- `git diff --check`
