# BP-only combat, Mad Goliath, and battle-history UI

Date: 2026-07-23
Baseline: continuation of the uncommitted combat and portrait work documented in `2026-07-23-battle-test-overhaul.md` and `2026-07-23-battle-presentation-roster-portraits.md`; published release remains `v0.6.1`.
Status: implemented in the working tree only; not committed or published.

## Requested behavior

- Make target selection visibly stronger than barrier styling.
- Reduce Defend to 10% damage mitigation and remove its generated barrier.
- Remove Energy because it overlaps conceptually with BP.
- Increase maximum BP to seven and price high-impact abilities differently.
- Hide the persistent combat log and expose history through a nonconflicting popup key.
- Show the active character name in the command-panel title.
- Make status conditions unmistakable on unit cards.
- Add an extremely difficult Geo construct boss that creates barriers and summons/revives two smaller constructs.
- Repair face portraits in the character grid and return that grid to a conventional centered size.

## BP economy

- Removed the Energy field, checks, gains, costs, and UI meter from transient battle units.
- Maximum BP is now seven.
- Basic attacks cost zero and generate one BP.
- Defend generates two BP.
- Skills cost two BP.
- Ultimates cost four, five, or six BP based on their existing three-, four-, or five-turn power/recharge classification.
- Battery supports feed two BP; Hybrid supports feed one BP.
- Recharge remains independently visible so BP does not erase turn-planning constraints.

## Defend and barriers

- Defend no longer creates a shield.
- Defend now applies exactly 10% damage reduction until the unit's next action.
- DEF remains part of normal mitigation; POISE no longer adds a second large mitigation term while defending.
- Real barriers remain an ability/enemy mechanic. Guardian and Hybrid abilities can still shield allies.
- Enemy barrier points now correctly absorb player damage before HP.

## Battle presentation

- Target selection is evaluated before effect/barrier styling.
- Selected targets use a gold double border and the explicit `▶▶ SELECTED ◀◀` title, even while a barrier is active.
- Statused units replace their secondary identity line with a purple `STATUS` indicator listing Guard, ATK boost, Poison, Burn, Frozen, Paralysis, Drowsy, or barrier value.
- The command panel title includes the active character name.
- The always-visible combat log was removed. `H` opens/closes a centered, double-bordered Battle History popup; `Esc` also closes it before leaving combat.
- Battle history retains up to 100 entries.
- The reclaimed row space is returned to the enemy and ally art lanes.

## Mad Goliath encounter

- Added the third encounter, **Mad Goliath**.
- Formation:
  - Goliath Shardling
  - Mad Goliath
  - Goliath Shardling
- Mad Goliath begins with 10,500 HP, very high DEF/POISE, high damage, and a 2,400-point barrier.
- On alternating scripted rounds it can:
  - raise a 1,200-point Geo barrier up to a 3,600-point cap;
  - rebuild every defeated Shardling at half HP.
- The encounter is deliberately tuned above the other Battle Tests.
- Shardlings use the smaller-enemy rendering scale while the boss receives the full central lane.

## Portrait-grid repair

The earlier grid rendered compact ANSI face rasters even in the primary Ghostty graphics mode. The grid now has an explicit native-protocol face registry:

- All 29 face assets have stable `face:<character>` protocol keys.
- `graphics_portraits()` returns the three visible face keys and exact card rectangles while the quick selector is active.
- Ghostty/Kitty receives the original `256×256` face PNG rather than full art or a low-resolution ANSI underlay.
- ANSI terminals continue to use the existing face raster fallback.
- The grid returned to a centered `92×31` maximum panel with three cards per page; portrait rectangles are shared between ANSI layout and native placement.

## Assets

- `assets/enemies/mad_goliath.png`
- `assets/enemies/goliath_shardling.png`

Both were generated with the built-in image-generation workflow using existing WishSim enemy art as style reference. The source used a flat `#FF00FF` key so Geo gold/stone colors remained separate from the background. Assets were processed with soft matte, despill, and one-pixel edge contraction, normalized to transparent `1024×1536` canvases, visually inspected, and registered in both `src/art.rs` and `src/kitty.rs`.

## Files changed

- `src/app.rs`
- `src/art.rs`
- `src/battle.rs`
- `src/kitty.rs`
- `src/ui.rs`
- `CHANGELOG.md`
- the two enemy assets listed above

## Validation

- `cargo fmt`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test` — 44 passed
- `git diff --check`

Focused tests cover:

- exact 10% Defend mitigation;
- Mad Goliath barrier growth and Shardling reconstruction;
- native face-protocol coverage for all canonical characters;
- prior encounter, status, asset, and equipment behavior.

## Compatibility and limitations

- No save-data schema changed.
- Energy existed only in transient Battle Test state, so its removal requires no migration.
- Boss scripting is deterministic for reproducible tests.
- Shardlings begin deployed and are subsequently reconstructed; a future wave system could support summoning into previously empty combat slots.

## Precise rollback guidance

1. Restore the prior Energy field/checks/meter only from the immediately preceding combat hunks in `src/battle.rs` and `src/ui.rs`; do not disturb unrelated saved stats.
2. Restore the prior Defend calculation and action text if the 10% rule is rejected. Reintroducing the old Defend barrier requires restoring only `perform_defend`.
3. Remove `BattleEncounter::MadGoliath`, its enemy formation, `goliath_action`, and focused test.
4. Remove Mad Goliath/Shardling entries from `src/art.rs` and `src/kitty.rs`, then delete only their two listed assets.
5. Remove `show_history`, the `H` key arm, and popup renderer to restore the previous log layout.
6. Remove only `face_portrait_key`, `face_portrait_bytes`, the quick-selector branch in `graphics_portraits`, and its focused protocol test to restore ANSI-only grid faces.
7. Run `cargo fmt`, strict Clippy, `cargo test`, asset validation, and `git diff --check`.
