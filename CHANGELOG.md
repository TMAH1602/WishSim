# Changelog

All notable changes to WishSim will be documented here.

## 0.7.0 - 2026-07-23

- Expanded Character Management's quick-selector to a six-character, two-row portrait grid and increased the panel to use substantially more of the available terminal area in both ANSI and Ghostty rendering.
- Renamed Nadezhda to Klara across the canonical catalog, banners, CLI, combat, profiles, portrait registries, and documentation; older saves migrate inventory copies, teams, equipment, and wish history safely. Replaced her full art with a corrected two-handed scythe pose and anatomically coherent left arm.
- Removed Battle Test ability cooldowns; Skills and Ultimates are now gated entirely by their BP costs.
- Rebalanced Mad Goliath so Hydro, Anemo, and Dendro rapidly break its Geo barrier; a barrier break stuns it for three turns and locks regeneration for five turns. Barriers no longer stack, and fallen Shardlings must remain defeated for at least five rounds before revival.
- Added limited five-star Anemo scythe striker Klara, Jade Tempest, her four-star Anemo buffer Taisia, and the selectable five-star signature scythe Gale's Last Harvest, with complete transparent full art, archive portraits, profiles, stats, abilities, and Ghostty/ANSI registration.
- Replaced the Standard Archive's one-key target cycle with a scrollable art/details Fate-path selector matching the weapon banner interaction.
- Reworked Battle Test around a seven-point BP economy, role-based support effects, critical hits, barriers, and element-labeled commands with contextual ability descriptions.
- Added the Astral Ruin Knight boss and Ember Wisp attendant, including transparent art and distinct boss/support tactics.
- Added unique, obtainable five-star signature weapons for all five standard characters.
- Added selectable Ruin Court, extreme Somnial Frostwyrm, and Mad Goliath encounters, elemental status conditions and immunities, animated combat feedback, barrier presentation, scaled enemy/active-character art, and per-unit BP meters.
- Added the Geo Mad Goliath boss and its Shardlings, including boss barriers, minion summoning/revival behavior, and transparent enemy art.
- Battle history now opens on demand with `H`; the reclaimed space enlarges combat art. Defend now grants 10% mitigation rather than a barrier.
- Character Management now documents each character's combat role and abilities, while its filtered roster uses a large six-card paged face-portrait grid backed by a complete character portrait collection.
- Character and standard banners now display centered currently featured/selected artwork beneath an independently centered banner title; all standard signatures are selectable on the weapon banner.

## 0.6.1 - 2026-07-23

- Repaired Yeoungin's portrait alpha matte so her face and other skin pixels remain visible in full-resolution and downsampled terminal rendering.
- Character attachment and Character Management roster lists now scroll to keep the highlighted row visible.

## 0.6.0 - 2026-07-23

- Added four-star Korean-inspired release companions Seo-yeon, an Electro catalyst tactician, and Ji-ho, a Pyro sword guardian, with complete profiles, abilities, stats, filters, archives, and transparent artwork.
- Added named Basic, Skill, and Ultimate abilities for every character, with one-turn Skill waits and character-specific three-to-five-turn Ultimate waits.
- Battle Test now projects equipped weapons to level 50, applies their scaled ATK/Elemental ATK/crit stats, uses tactical enemy targeting, and allows Thornbloom to inflict poison.
- Added limited five-star Cryo polearm healer Yeoungin, Winter's Grace, including her shared-pity banner, combat profile, Battle Test healing/ATK support, archive integration, and transparent character artwork.
- Added Yeoungin's five-star signature polearm, Rimebound Benediction, to the selectable weapon path with matching transparent artwork.
- Added an isolated level-50 3v3 Battle Test with saved-team selection, speed-based turns, character abilities, DEF/POISE-based Defend, support healing, battle logs, HP gauges, and victory/defeat handling.
- Added a main-menu field manual containing battle controls and provisional 2× elemental matchups.
- Added transparent Hydro Slime and Thornbloom pixel-art enemies to both ANSI and Ghostty/Kitty rendering paths.

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
