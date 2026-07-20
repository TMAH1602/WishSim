# Changelog

All notable changes to WishSim will be documented here.

## 0.5.0 - 2026-07-20

- Ten-pull skip now preserves the cutscene and result card for every 5-star obtained before opening the summary.
- Equipment selection now reports each weapon's unequipped count without printing character-holder names.

- Added the first game-level main menu with Teams, Wish, Inventory, and Character Management destinations.
- Added five persistent, freely named three-character team slots.
- Added character management with a horizontal roster carousel, centered full art, N0–N10 ascension roadmap, combat stats, and type-safe weapon equipment.
- Added side-by-side character and weapon artwork in the equipment picker, including dual native images in Ghostty/Kitty.
- Added transparent pixel-art assets for every three-star weapon.
- Corrected Corvin's visible complexion to a natural light tan while preserving his design.
- Expanded team management into a three-member art gallery and enlarged/cleaned the root menu.
- Added a rarity, element, and weapon-filtered character quick selector.
- Made equipment selection copy-aware with rarity-colored weapon names and equipped-character indicators.
- Made weapon-path art the default view and moved weapon details/stats behind `V`; removed stale two-weapon banner copy.

## 0.4.0 - 2026-07-19

- Replaced the two-item weapon-path toggle with a scrollable selector covering every limited signature, including an optional art preview.
- Added a scrollable character archive with owned sprite thumbnails and obscured named records for unowned characters.
- Added five four-star characters: Kestrel, Mako, Ysra, Dolma, and Corvin.

- Added a grid-based limited five-star banner selector.
- Added limited five-star Geo polearm user Saif and moved Sergei to the limited roster.
- Added standard five-star characters Pyrite and Jeanette.
- Added transparent pixel-art assets for every five-star and four-star weapon.
- Corrected Lumen's lore to use masculine pronouns.
- Added a selectable Standard Archive banner with independent pity and a one-point chosen-character Fate guarantee.
- Added four-star characters Farah, Anya, and Rook with full transparent artwork and combat profiles.
- Added signature five-star weapons for Seraphine, Vaughn, Steven, Sergei, and Saif.
- Designated Ghostty as the primary supported terminal emulator while retaining Kitty protocol support and portable ANSI rendering.

## 0.3.1 - 2026-07-18

- Added native full-resolution character artwork in Ghostty through the Kitty graphics protocol.
- Removed the portable half-block portrait beneath full-resolution protocol artwork.
- Removed the runtime dependency on the external `kitten icat` helper.
- Added explicit native/ANSI graphics overrides for environments that replace terminal identity markers.

## 0.3.0 - 2026-07-18

- Added inventory sorting by name, rarity, item type, and element.
- Added character/weapon and elemental inventory filters.
- Added foundational CRIT DMG, CRIT RATE, ATK, DEF, SPD, ELEMENTAL ATK, HP, and POISE profiles.
- Added limited five-star banners for Vaughn and Steven.
- Added standard five-star characters Cinder and Sergei.
- Added four-star characters Zephra, Neris, and Brikka.
- Added Gauntlet, Scythe, and Dual Blades weapon classes and new four-star weapons.
- Added transparent pixel-art portraits for every new character in both Kitty and portable ANSI rendering paths.
- Added v0.2.0 consistency documentation and dated development logs for future feature work and selective backtracking.

## 0.2.0 - 2026-07-17

- Added an inspectable inventory with owned counts.
- Added individual, multi-select, and select-all inventory deletion with confirmation.
- Added original full-resolution character artwork with native Kitty rendering.
- Added a portable colored half-block portrait fallback.
- Enlarged wish-animation stars and added an earlier deceptive rarity-color tease.
- Made ten-pull reveals advance manually and added exit confirmation.

## 0.1.0 - 2026-07-17

- Added three shared-pity character-event banners.
- Added a separate weapon banner with Fate Points and an epitomized path.
- Added animated rarity flights and a dedicated five-star cutscene.
- Added one- and ten-wish flows, history, inventory, and persistent pity.
- Added compact reveal art and shaded JRPG-style inspection portraits.
- Added Kitty-enhanced true-color rendering with a portable ANSI fallback.
- Added deterministic seeded pulls and probability-boundary tests.
