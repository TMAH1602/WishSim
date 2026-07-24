# Battle Test combat overhaul

Date: 2026-07-23
Baseline: `v0.6.1` working tree based on commit `9080f08`; design baseline remains `v0.2.0` (`2c59836`).
Status: implemented in the working tree only; not committed or published.

## Request and acceptance criteria

- Make Battle Test more responsive, interactive, and mechanically engaging.
- Add charge/resource decisions so stronger abilities are not available immediately.
- Display elemental damage identity, recharge readiness, costs, and contextual ability descriptions.
- Diversify four-star combat roles and make release companions useful to their featured five-star.
- Add new enemies, including a boss, with appropriate transparent pixel art.
- Confirm that every limited and standard five-star has a signature weapon.

## Implementation

### Combat economy and roles

- Added a five-point BP pool and 100-point Energy pool to allied battle units.
- Basic attacks generate 1 BP and 25 Energy.
- Defend generates 2 BP and 20 Energy, retains DEF/POISE mitigation, and grants a temporary shield.
- Skills require 1 BP and an initial/recurring one-owner-turn recharge.
- Ultimates require 3 BP, 100 Energy, and their existing character-specific 3–5 turn recharge.
- Added Striker, Healer, Guardian, Buffer, Battery, and Hybrid roles.
- Four-star supports now have actual support targeting/effects:
  - Mira heals.
  - Lumen, Farah, Dolma, and Ji-ho shield.
  - Kestrel, Brikka, and Ysra buff ATK.
  - Seo-yeon supplies BP and Energy.
  - Thorne, Corvin, and Zephra provide mixed recovery, shielding, and charge support.
- Yeoungin remains a healer/ATK support and now also supplies hybrid protection and resources. Her release companions directly complement her: Seo-yeon accelerates her costly ultimate and Ji-ho protects the team while it charges.
- Equipped level-50 weapon stats continue to feed combat power; CRIT RATE and CRIT DMG now participate in deterministic Battle Test critical hits.

### Encounter behavior

- Replaced the duplicate-slime test formation with:
  - Ember Wisp, a fast Pyro attendant that periodically heals and empowers the boss.
  - Astral Ruin Knight, a high-HP Electro boss with an opening shield and Ruin Nova burst every third round.
  - Thornbloom, retaining its poison pressure.
- Enemy targeting continues to consider HP, guard state, and turn rotation.
- Shields absorb damage before HP and are reported in combat status/log feedback.

### Battle presentation

- Extended the existing Ratatui Battle Test render path.
- Command rows show:
  - physical or elemental damage identity in the associated color,
  - BP/Energy cost,
  - a three-diamond recharge meter,
  - ready/locked state.
- Moving the command cursor displays a short description of the selected ability or Defend action.
- Unit cards now show BP, Energy, shields, buffs, poison, and level-50 weapon projection.
- Updated the Field Manual with the new economy and boss tactics.

### Five-star signature audit

All eight limited five-stars already had selectable weapon-path signatures. Added unique signatures for every standard five-star, obtainable from the existing standard five-star weapon pool:

- Veyra — Tempest Meridian (Electro Bow)
- Orin — Emberkeeper's Oath (Pyro Claymore)
- Cinder — Furnaceheart Bracers (Pyro Gauntlet)
- Pyrite — Aurum Flash (Geo Sword)
- Jeanette — Silver Tidemark (Hydro Bow)

A test now validates character coverage, catalog availability, uniqueness by mapping, and weapon-type compatibility.

## Files changed

- `src/battle.rs`
- `src/ui.rs`
- `src/simulation.rs`
- `src/art.rs`
- `src/kitty.rs`
- `CHANGELOG.md`
- `assets/enemies/astral_ruin_knight.png`
- `assets/enemies/ember_wisp.png`
- `assets/weapons/tempest_meridian.png`
- `assets/weapons/emberkeepers_oath.png`
- `assets/weapons/furnaceheart_bracers.png`
- `assets/weapons/aurum_flash.png`
- `assets/weapons/silver_tidemark.png`

## Asset workflow

Enemy art was generated with the repository's existing enemy art as style reference. The standard signature weapons were generated as one consistently styled sprite sheet and separated into individual assets. All seven assets were chroma-keyed, visually inspected, normalized to the established 1024×1536 transparent portrait canvas, embedded in the ANSI gallery, and registered in the Ghostty/Kitty graphics path.

## Validation

- `cargo fmt`
- `cargo check`
- `cargo test` — 39 passed
- Embedded-art tests verify dimensions, transparent corners, gallery decoding, and protocol-image coverage.
- A first test run exposed obsolete cooldown assumptions and nonstandard source dimensions; tests were updated to assert the new resource contract, and assets were normalized before the passing run.

## Compatibility and limitations

- No save-data schema changed. BP, Energy, recharge, shields, and encounter state are temporary Battle Test state.
- Battle Test remains a deterministic test encounter and does not award progression or inventory.
- Support effects currently use role-level mechanics; later character kit work can specialize individual numerical effects without replacing the state machine.
- Ruin Nova is presented as a boss burst but currently resolves against the boss's selected tactical target rather than the full party.

## Precise rollback guidance

Do not use broad Git restoration. To remove only this feature:

1. Revert the BP/Energy/role/shield/boss hunks and their tests in `src/battle.rs`.
2. Revert the command-meter, status-card, element-color, and Field Manual hunks in `src/ui.rs`.
3. Remove only the five new standard signature entries, elemental mappings, signature mapping, and audit test from `src/simulation.rs`.
4. Remove the seven new registry entries from `src/art.rs` and `src/kitty.rs`.
5. Delete only the seven assets listed above after confirming no later feature references them.
6. Remove the Unreleased bullets associated with this work from `CHANGELOG.md`.
7. Run `cargo fmt`, `cargo test`, and `git diff --check`.
