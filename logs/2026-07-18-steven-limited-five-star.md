# 2026-07-18 — Steven limited five-star

## Repository state

- Design baseline: v0.2.0 (`2c59836`).
- This continues the uncommitted roster, inventory, and stat work recorded in earlier 2026-07-18 logs.
- Steven’s implementation is in the working tree at the time of this entry.

## Request

Add Steven as a limited five-star stealth-oriented fire fighter: a middle-aged white man with a beard and blue eyes, black/dark-blue ninja and samurai-inspired armor, exceptional speed, blue hand-cast fire, and a tiny but extremely powerful black maltipoo-like shoulder companion named Wick whose eyes emit blue flame. Emphasize ELEMENTAL ATK, SPD, and POISE while lowering other stats.

## Implementation

- Added `Steven, Azure Shade` as the featured character for the new `Steven` limited banner.
- Added `Banner::Steven` to banner cycling, title handling, simulation selection, CLI parsing, home copy, and README examples.
- Banner presentation follows the v0.2.0 character template:
  - Title: `Veilfire Covenant`
  - Hero: `S T E V E N`
  - Subtitle: `AZURE SHADE  •  VEILFIRE SHINOBI`
  - Quote: `By the time the flame is seen, the shadow has already moved.`
- Classified Steven as a Pyro character and Catalyst user; the catalyst represents his hand-cast veilfire rather than a conventional weapon.
- Added a custom base stat profile:
  - CRIT DMG 120
  - CRIT RATE 5
  - ATK 104
  - DEF 74
  - SPD 142
  - ELEMENTAL ATK 182
  - HP 840
  - POISE 158
- Added archive title, lore, quote, colors, and terminal fallback art in `src/ui.rs`.
- Registered the portrait in both `src/art.rs` and `src/kitty.rs` and extended mapping/catalog assertions.

## Asset

Added `assets/characters/steven.png`, a `1024 × 1536` transparent RGBA pixel-art cutout. The built-in image-generation tool used Kaelis as a style reference. The prompt requested an original layered stealth/lamellar design, visible middle-aged bearded face, black/navy palette, two spiraling blue hand flames, and exactly one small black curly-haired shoulder dog with blue-fire eyes.

The generated chroma-green source was converted locally to transparent RGBA before registration. The final asset must remain transparent for `trim_transparent()` and Kitty `icat` compatibility.

## Consistency decisions

- Used the same four-part banner copy structure as Astraea, Kaelis, Seraphine, and Vaughn.
- Kept Pyro as the gameplay element even though the character’s flames are visually blue.
- Used Catalyst to fit the existing weapon taxonomy and hand-cast elemental power.
- Kept Wick within Steven’s single character portrait and catalog entry rather than creating a second inventory unit.
- Kept Steven’s banner on shared event-character pity and guarantees.

## Validation

Run formatting, all tests, Clippy with warnings denied, `git diff --check`, portrait alpha validation, and a seeded CLI smoke test using `--banner steven`.

## Safe backtracking

To remove only Steven, remove `Banner::Steven` and every exhaustive match/CLI/home arm, the `STEVEN` catalog constant and featured selection, his element/stat/profile entries, both portrait registry entries, test expectations, README command, and `assets/characters/steven.png`. Do not remove Vaughn or the shared event pity system. Re-run compilation to catch every exhaustive banner match.
