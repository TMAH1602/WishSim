# 2026-07-23 — BP-only abilities and Mad Goliath balance

## Baseline and request

- Continues the uncommitted Battle Test work documented in `2026-07-23-bp-geo-boss-history.md` and the subsequent Klara release work. The published design baseline remains `v0.2.0` (`2c59836`); these changes remain in the working tree and are not committed.
- Requested removal of all character-ability cooldowns because BP now provides the intended resource gate.
- Requested a solvable Mad Goliath loop: explicit Geo barrier weaknesses, no immediate or stacking barrier replacement, a two-to-three-turn stun after barrier break, at least five turns without barrier regeneration, and a longer delay before fallen Goliath Shardlings revive.

## Ability resource changes

- Removed per-unit cooldown state, cooldown initialization, per-turn cooldown decrementing, and cooldown-setting calls from `src/battle.rs`.
- `BattleState::action_ready()` now checks only whether the active character has enough BP for the selected action.
- Basic attacks still generate 1 BP, Defend still generates 2 BP, Skills cost 2 BP, and Ultimates cost 4–6 BP according to their existing impact tier.
- Removed recharge diamonds from the command list and updated the field manual to state that abilities may be reused immediately whenever their BP cost is available.
- Retained `AbilityLoadout::ultimate_wait` as the current internal impact tier because it still determines Ultimate BP cost and damage scaling. It no longer represents or creates a cooldown.

## Mad Goliath mechanics

- The initial 2,400-point Geo barrier remains heavy, but elemental abilities using the existing Geo counters—Hydro, Anemo, and Dendro—receive an additional barrier-only damage multiplier. The combat log explicitly reports when the weakness is exploited.
- A Mad Goliath barrier transitioning from positive strength to zero now:
  - applies a deterministic three-Goliath-action `STUN`;
  - displays that stun in the unit status row;
  - starts a five-Goliath-action barrier regeneration lock;
  - records the break, stun, and lock duration in battle history.
- Mad Goliath loses each stunned action. The regeneration lock counts those actions and continues afterward until five scheduled boss actions have elapsed.
- The boss can create a new barrier only when the lock has expired and its current barrier is zero. New barriers are 1,000 points and cannot stack.
- Each Shardling now records its defeat round whether defeated by a direct ability or Burn. It cannot be rebuilt until at least five full battle rounds have elapsed since that death; the existing three-round reconstruction opportunity may delay revival longer but never shorten the five-round minimum.

## Files changed

- `src/battle.rs`: BP-only readiness, removal of cooldown state, barrier weakness/break handling, stun and regeneration lock state, Shardling death timestamps, revival delay, and focused tests.
- `src/ui.rs`: cooldown-free command rows, stun status display, and revised field-manual guidance.
- `CHANGELOG.md`: unreleased behavior summary.
- `logs/2026-07-23-bp-cooldown-goliath-balance.md`: this continuation record.

## Validation

- `cargo fmt --all`
- `cargo check --locked`
- `cargo test --locked`: 44 passed, 0 failed.
- Focused coverage proves abilities toggle readiness solely with BP, a weak-element ability breaks the Geo barrier, the break applies three stun actions and a five-action barrier lock, a new non-stacking barrier appears only after the lock, and Shardlings cannot revive before five rounds.
- Final Clippy, release build, and diff checks remain to be run after this log update.

## Known limitations

- The Ultimate field remains named `ultimate_wait` internally even though it now functions only as a provisional impact tier. A future data-model cleanup can rename it once ability definitions are stabilized.
- Turn durations are transient Battle Test state and are not persisted.
- Manual playtesting is still recommended to tune Mad Goliath's HP, starting barrier, and 1,000-point regenerated barrier against a broader range of saved teams.

## Safe backtracking

- To restore cooldowns, reintroduce only the removed `BattleUnit::cooldowns` state, initialization/decrement/set paths, readiness condition, command diamonds, and prior focused test. Do not revert BP, encounter, status, or unrelated character work.
- To restore the previous Goliath behavior, remove only `goliath_barrier_lock_turns`, `stun_turns`, `shardling_defeated_round`, the barrier-break branch, barrier weakness multiplier/log, and revised `goliath_action()` conditions. Restore the old even-round stacking barrier and three-round unconditional revival only if that exact difficulty is desired.
