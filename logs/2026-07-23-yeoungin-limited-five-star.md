# 2026-07-23 — Yeoungin limited five-star

## Baseline and request

- Continues the uncommitted Battle Test work documented in `2026-07-22-battle-test-foundation.md` from published v0.5.0 documentation tip `b26672b`.
- Requested Yeoungin: an adult Asian limited five-star Cryo polearm support healer with a slender medium-height build, stern serious expression, medium straight black hair, one hairpin, brown eyes with blue pupils, elegant winter-court attire, an ice aura behind her, and a guarded personality that expresses care through protective action. Her support identity includes healing and an ATK boost.
- All Battle Test changes remain preserved and this continuation remains uncommitted until explicitly requested.

## Catalog, banner, and presentation

- Added canonical `YEOUNGIN` item name `Yeoungin, Winter's Grace` to the limited five-star catalog, character archive, inventory, filters, equipment typing, and shared character-event pity.
- Added `Banner::Yeoungin`, CLI `--banner yeoungin`, the ninth banner-selector card, and the established four-field banner treatment: `Mercy Beneath Winter`, spaced hero name, `WINTER'S GRACE  •  SILENT BENEDICTION`, and one restrained in-world quote.
- Added a complete Cryo/Polearm profile and foundational support-oriented stats: high HP and ELEMENTAL ATK, solid DEF/POISE, moderate SPD, and deliberately low physical ATK/crit values.
- Her lore preserves the requested contrast: severe and emotionally guarded in speech, but consistently attentive to teammates' wounds, strength, and safe return.

## Signature weapon

- The established WishSim contract gives every featured limited five-star a selectable signature. Added `Rimebound Benediction`, an original five-star Cryo polearm themed around healing and battlefield inspiration.
- Added `WeaponPath::RimeboundBenediction`, weapon catalog metadata, profile/lore, weapon-banner selection, both art registries, and expanded signature-path tests from seven to eight choices.

## Battle Test integration

- Expanded the temporary healer role mapping to include Yeoungin. Selecting Magic opens living-ally targeting, restores HP from ELEMENTAL ATK, and grants a persistent test-battle ATK increase.
- Battle cards display `ATK↑` while the support increase is active. Physical and offensive elemental damage include the temporary bonus.
- Jeanette retains healing-only behavior. Individual skills, ultimates, durations, stacking rules, and canonical role metadata remain future work.

## Assets

- Generated `assets/characters/yeoungin.png` with the built-in image generation workflow using Astraea, Jeanette, and Pyrite as style references only. The final prompt specified a stern adult Asian Cryo support, straight black hair, one icy hairpin, brown eyes with vivid blue pupils, original imperial winter-court attire, one complete spear, and a crystalline frost aura on a removable magenta background.
- Generated `assets/weapons/rimebound_benediction.png` separately using existing WishSim polearm and signature-weapon assets as style references only. The prompt specified one complete ceremonial navy, silver, and ice-blue support spear with a suspended frost crystal.
- Both outputs were processed through the imagegen chroma helper into 1024×1536 RGBA cutouts. The initial Yeoungin pass used despill and visibly damaged skin colors; the corrected pass removed despill, contracted the alpha edge by one pixel, preserved the source palette, and was visually inspected before registration.
- Registered both exact names in `src/art.rs` and `src/kitty.rs` for portable ANSI and Ghostty/Kitty protocol rendering.

## Validation and limitations

- Passed `cargo fmt --all -- --check`.
- Passed `cargo check --locked`.
- Passed `cargo test --locked`: 32 passed, 0 failed, including focused Yeoungin healing/ATK-boost behavior, catalog metadata, signature-path coverage, raster loading, transparency/chroma checks, and protocol registry coverage.
- Passed `cargo clippy --locked --all-targets -- -D warnings`.
- Passed `cargo build --release --locked`.
- Passed `git diff --check`.
- The ATK boost intentionally lasts for the current Battle Test encounter because buff durations and ability turns are not yet modeled.
- Yeoungin's healer/support identity remains temporary name-based battle metadata until the planned character ability system introduces canonical roles.
- Manual reveal, banner-grid, weapon-selector, Battle Test, and real-Ghostty visual checks remain recommended.

## Safe backtracking

- Remove only `Banner::Yeoungin`, `YEOUNGIN`, its catalog/type/element/stats/profile/CLI arms, its two portrait registry entries, and `assets/characters/yeoungin.png` to remove the character.
- Remove only `WeaponPath::RimeboundBenediction`, `RIMEBOUND_BENEDICTION`, its catalog/profile/path/registry arms, and `assets/weapons/rimebound_benediction.png` to remove the signature.
- Remove Yeoungin from the temporary Battle Test healer/ATK-boost mapping and the corresponding focused test without changing Jeanette or the underlying Battle Test system.
- Restore affected count assertions to 26 characters and seven weapon paths only if this feature is fully removed. Do not revert the unrelated uncommitted Battle Test files or published v0.5.0 history.
