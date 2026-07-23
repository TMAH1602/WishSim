# 2026-07-23 — Early character abilities and enemy tactics

## Baseline and request

- Continues the uncommitted Battle Test and Yeoungin work documented in `2026-07-22-battle-test-foundation.md` and `2026-07-23-yeoungin-limited-five-star.md`; published v0.5.0 documentation tip remains `b26672b`.
- Requested an early three-tier ability system for every character, smarter enemy targeting, a poison effect for the Dendro enemy, equipped-weapon damage scaling, and level-50 weapons during Battle Test.
- This work is limited to transient Battle Test state. It does not alter saves, inventory counts, equipment ownership, wish behavior, or canonical progression.

## Ability system

- Replaced the provisional `FIGHT` and `MAGIC` commands with a named Basic, Skill, and Ultimate loadout for every one of the 27 canonical characters. `DEFEND` remains the fourth command.
- Basic abilities use ATK and are available every turn. Non-healer Skills and Ultimates use ELEMENTAL ATK and elemental effectiveness; Skills use a 145% power factor and Ultimates use a larger factor tied to their provisional wait.
- A used Skill waits one complete owner turn before becoming available. Each character's Ultimate has a three-, four-, or five-turn post-use wait based on its provisional strength.
- Jeanette and Yeoungin retain their support identity: Skills and Ultimates select living allies and scale healing from ELEMENTAL ATK. Yeoungin continues to grant her ATK increase.
- Cooldowns, poison, buffs, projected levels, and derived stats exist only inside `BattleState`.

## Weapon projection and damage

- Equipped weapons are looked up through the existing canonical catalog and equipment map. Battle Test marks them level 50 and projects ATK, ELEMENTAL ATK, CRIT RATE, and CRIT DMG to 150% of their catalog values before adding them to the character.
- Unequipped characters receive no weapon bonus. The battle card displays `WPN 50` only when a weapon is equipped.
- This is an explicit prototype formula, not persisted weapon progression. Crit resolution is not yet implemented even though projected crit stats are retained for the later combat pass.

## Enemy tactics and poison

- Enemies now select among all living allies using HP percentage, guard state, and rotating positional pressure. This makes wounded unguarded allies attractive without permanently pinning attacks to slot one.
- Thornbloom has a deterministic reproducible chance to use its Dendro poison strike. Poison lasts three affected-character turns and deals one-twelfth maximum HP at the beginning of each turn.
- Battle cards show remaining poison turns, and the combat log reports application and damage ticks.

## Presentation and files

- `src/battle.rs`: data-driven loadouts, cooldowns, ability resolution, level-50 weapon projection, target heuristic, poison state, and focused tests.
- `src/ui.rs`: named ability commands, cooldown counters, weapon-level and poison status markers, and revised field-manual text.
- `README.md`, `CHANGELOG.md`, and `docs/V0_2_0_CONSISTENCY_GUIDE.md`: user-facing controls and durable prototype formulas.
- No new visual assets or persistence fields were required.

## Validation and limitations

- Passed `cargo fmt --all -- --check`.
- Passed `cargo check --locked`.
- Passed `cargo test --locked`: 37 passed, 0 failed. Focused coverage includes all 27 named loadouts, Skill/Ultimate waits, level-50 weapon projection and stat contribution, tactical target selection, poison application, healer targeting, and Yeoungin's heal/ATK support.
- Passed `cargo clippy --locked --all-targets -- -D warnings`.
- Passed `cargo build --release --locked`.
- Passed `git diff --check`.
- Ability effects are intentionally archetypal: damage or healing plus Yeoungin's existing ATK support. Unique crowd control, shields, area targeting, passives, and cinematics remain future work.
- Critical hits, weapon passives, ascension scaling, status cleansing, poison resistance, and AI difficulty settings are not yet modeled.
- The deterministic poison trigger represents an early testable chance model and should be replaced by seeded battle RNG if Battle Test later gains its own replayable random stream.
- Manual checks at 80×34 and in a real Ghostty window remain recommended for long ability names and status combinations.

## Safe backtracking

- Restore only the ability/cooldown/loadout portions of `src/battle.rs` and the corresponding command rendering in `src/ui.rs` to return to `FIGHT`/`MAGIC`; retain the surrounding Battle Test and Yeoungin work.
- Remove only weapon projection and `weapon_level` handling to restore raw catalog weapon additions.
- Remove only `choose_enemy_target`, `poison_turns`, poison resolution, and their UI/log branches to restore the earlier first-living-target behavior.
- Revert the matching README, changelog, guide, and focused tests with those exact code paths. Do not reset or discard the unrelated uncommitted Battle Test, Yeoungin, enemy assets, or user work.
