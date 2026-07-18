# 2026-07-18 — Sergei standard five-star

## Repository state

- Design baseline: v0.2.0 (`2c59836`).
- This work continues the uncommitted post-v0.2.0 roster/stat changes described in `2026-07-18-inventory-roster-stats.md`.
- Sergei’s changes are working-tree changes at the time of this entry.

## Request

Add Sergei as a standard-pool five-star Cryo character: a short, lean young adult male Catalyst fighter in full ice armor over northern cold-weather clothing. His face is hidden by a predatory snouted ice helm with severe eye slits and deep-blue spectral smoke. He uses paired ice forearm shields and draws power from a subtle spectral snow-wolf guardian. His combat profile emphasizes ATK and SPD, keeps DEF moderate, and lowers other stats for balance.

## Implementation

- Added `Sergei, Winterfang` to `STANDARD_FIVE_CHARACTERS` in `src/simulation.rs`.
- Added Cryo metadata and a custom stat profile:
  - CRIT DMG 125
  - CRIT RATE 5
  - ATK 154
  - DEF 102
  - SPD 132
  - ELEMENTAL ATK 82
  - HP 880
  - POISE 64
- Added the `Heir of the White Hunt` archive profile in `src/ui.rs` with Catalyst weapon classification, original lore, quote, Cryo palette, and fallback terminal art.
- Added `assets/characters/sergei.png` to both required portrait consumers:
  - `src/art.rs::PORTRAITS` for ANSI rendering.
  - `src/kitty.rs::portrait_bytes()` for Kitty rendering.
- Extended existing catalog and Kitty registry tests.

## Asset production

The built-in image-generation tool used `assets/characters/kaelis.png` as a style reference. The prompt translated the user’s media references into an original winter-beast design rather than copying named characters or exact protected designs.

The selected output is a `1024 × 1536` full-body pixel-art PNG. It was generated on chroma green, copied to `assets/characters/sergei.png`, converted to transparent RGBA, and given a second green-hue edge cleanup around the translucent wolf. Visual inspection confirmed the crouched silhouette, two forearm shields, hidden face, ice armor/greaves, white hair, deep-blue eye smoke, winter clothing, catalyst core, and single spectral wolf.

## Consistency decisions

- Used a five-star standard-character display name with epithet, matching Veyra, Orin, and Cinder.
- Kept `Item.kind` as `Character`; Catalyst is profile weapon metadata, matching the existing character catalog model.
- Used the existing intrinsic stat system rather than adding serialized progression.
- Registered the portrait in both ANSI and Kitty mappings during the same change.
- Kept the character out of featured banners and added him only to the standard five-star loss pool.

## Validation

Run the full project validation gate after integration. Existing asset tests automatically include Sergei through `PORTRAITS`; the Kitty registry test and catalog metadata test were extended explicitly.

## Safe backtracking

To remove only Sergei, remove his standard-pool entry, element/stat arms, UI profile, ANSI registry entry, Kitty registry entry, test expectations, and `assets/characters/sergei.png`. Do not revert the broader roster/stat work. Existing released saves containing `Sergei, Winterfang` would require a migration decision before catalog removal.
