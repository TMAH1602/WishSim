use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::model::{Banner, Item, Rarity, SaveData, SavedWish, Stats, WeaponPath, WishResult};

pub const ASTRAEA: Item = Item {
    name: "Astraea, Starbound",
    kind: "Character",
    rarity: Rarity::Five,
};
pub const KAELIS: Item = Item {
    name: "Kaelis, Ashen Vanguard",
    kind: "Character",
    rarity: Rarity::Five,
};
pub const SERAPHINE: Item = Item {
    name: "Seraphine, Verdant Oracle",
    kind: "Character",
    rarity: Rarity::Five,
};
pub const VAUGHN: Item = Item {
    name: "Vaughn, Violet Oath",
    kind: "Character",
    rarity: Rarity::Five,
};
pub const STEVEN: Item = Item {
    name: "Steven, Azure Shade",
    kind: "Character",
    rarity: Rarity::Five,
};
pub const POLARIS_EDGE: Item = Item {
    name: "Polaris Edge",
    kind: "Sword",
    rarity: Rarity::Five,
};
pub const NOVA_GRIMOIRE: Item = Item {
    name: "Nova Grimoire",
    kind: "Catalyst",
    rarity: Rarity::Five,
};

const STANDARD_FIVE_CHARACTERS: &[Item] = &[
    Item {
        name: "Veyra, Stormseeker",
        kind: "Character",
        rarity: Rarity::Five,
    },
    Item {
        name: "Orin, Keeper of Embers",
        kind: "Character",
        rarity: Rarity::Five,
    },
    Item {
        name: "Cinder, Forgeheart",
        kind: "Character",
        rarity: Rarity::Five,
    },
    Item {
        name: "Sergei, Winterfang",
        kind: "Character",
        rarity: Rarity::Five,
    },
];
const STANDARD_FIVE_WEAPONS: &[Item] = &[
    Item {
        name: "Celestial Atlas",
        kind: "Catalyst",
        rarity: Rarity::Five,
    },
    Item {
        name: "Wolfsong Claymore",
        kind: "Claymore",
        rarity: Rarity::Five,
    },
];
const FEATURED_FOUR: &[Item] = &[
    Item {
        name: "Mira",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Thorne",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Lumen",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Zephra",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Neris",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Brikka",
        kind: "Character",
        rarity: Rarity::Four,
    },
];
const FEATURED_FOUR_WEAPONS: &[Item] = &[
    Item {
        name: "Moonlit Longbow",
        kind: "Bow",
        rarity: Rarity::Four,
    },
    Item {
        name: "Sage's Codex",
        kind: "Catalyst",
        rarity: Rarity::Four,
    },
    Item {
        name: "Ironwind Blade",
        kind: "Sword",
        rarity: Rarity::Four,
    },
    Item {
        name: "Galegrip Knuckles",
        kind: "Gauntlet",
        rarity: Rarity::Four,
    },
    Item {
        name: "Winter's Requiem",
        kind: "Scythe",
        rarity: Rarity::Four,
    },
    Item {
        name: "Twin Cinderfangs",
        kind: "Dual Blades",
        rarity: Rarity::Four,
    },
];
const STANDARD_FOUR: &[Item] = &[
    Item {
        name: "Duskward Spear",
        kind: "Polearm",
        rarity: Rarity::Four,
    },
    Item {
        name: "Bellflower Greatsword",
        kind: "Claymore",
        rarity: Rarity::Four,
    },
];
const THREE_STAR: &[Item] = &[
    Item {
        name: "Dawncool Steel",
        kind: "Sword",
        rarity: Rarity::Three,
    },
    Item {
        name: "Raven Bow",
        kind: "Bow",
        rarity: Rarity::Three,
    },
    Item {
        name: "Quartz Spear",
        kind: "Polearm",
        rarity: Rarity::Three,
    },
    Item {
        name: "Wanderer's Notes",
        kind: "Catalyst",
        rarity: Rarity::Three,
    },
    Item {
        name: "Old Mercenary's Greatsword",
        kind: "Claymore",
        rarity: Rarity::Three,
    },
];

pub fn featured_character(banner: Banner) -> Item {
    match banner {
        Banner::Astraea => ASTRAEA,
        Banner::Kaelis => KAELIS,
        Banner::Seraphine => SERAPHINE,
        Banner::Vaughn => VAUGHN,
        Banner::Steven => STEVEN,
        Banner::Weapon => ASTRAEA,
    }
}

pub fn catalog_item(name: &str) -> Option<Item> {
    [
        ASTRAEA,
        KAELIS,
        SERAPHINE,
        VAUGHN,
        STEVEN,
        POLARIS_EDGE,
        NOVA_GRIMOIRE,
    ]
    .into_iter()
    .chain(STANDARD_FIVE_CHARACTERS.iter().copied())
    .chain(STANDARD_FIVE_WEAPONS.iter().copied())
    .chain(FEATURED_FOUR.iter().copied())
    .chain(FEATURED_FOUR_WEAPONS.iter().copied())
    .chain(STANDARD_FOUR.iter().copied())
    .chain(THREE_STAR.iter().copied())
    .find(|item| item.name == name)
}

pub fn item_element(name: &str) -> &'static str {
    match name {
        "Astraea, Starbound" | "Sergei, Winterfang" | "Neris" | "Winter's Requiem" => "Cryo",
        "Lumen" => "Geo",
        "Kaelis, Ashen Vanguard"
        | "Orin, Keeper of Embers"
        | "Cinder, Forgeheart"
        | "Steven, Azure Shade"
        | "Brikka"
        | "Twin Cinderfangs" => "Pyro",
        "Seraphine, Verdant Oracle" | "Thorne" => "Dendro",
        "Vaughn, Violet Oath" | "Veyra, Stormseeker" => "Electro",
        "Mira" => "Hydro",
        "Zephra" | "Galegrip Knuckles" => "Anemo",
        _ => "Unaligned",
    }
}

pub fn item_stats(item: Item) -> Stats {
    if item.kind == "Character" {
        match item.name {
            "Vaughn, Violet Oath" => Stats::character(item.rarity, 142, 138, 82, 1280, 150),
            "Cinder, Forgeheart" => Stats::character(item.rarity, 148, 105, 96, 1190, 135),
            "Sergei, Winterfang" => Stats {
                crit_dmg: 125,
                crit_rate: 5,
                atk: 154,
                def: 102,
                spd: 132,
                elemental_atk: 82,
                hp: 880,
                poise: 64,
            },
            "Steven, Azure Shade" => Stats {
                crit_dmg: 120,
                crit_rate: 5,
                atk: 104,
                def: 74,
                spd: 142,
                elemental_atk: 182,
                hp: 840,
                poise: 158,
            },
            "Zephra" => Stats::character(item.rarity, 112, 78, 128, 890, 70),
            "Neris" => Stats::character(item.rarity, 121, 82, 106, 930, 82),
            "Brikka" => Stats::character(item.rarity, 126, 91, 101, 980, 96),
            _ => Stats::character(
                item.rarity,
                if item.rarity == Rarity::Five {
                    132
                } else {
                    108
                },
                90,
                100,
                if item.rarity == Rarity::Five {
                    1120
                } else {
                    920
                },
                90,
            ),
        }
    } else {
        Stats::weapon(
            item.rarity,
            match item.rarity {
                Rarity::Five => 155,
                Rarity::Four => 120,
                Rarity::Three => 82,
            },
            if item.name.contains("Gale") { 12 } else { 5 },
            if item.name.contains("Requiem") {
                175
            } else {
                150
            },
        )
    }
}

pub struct WishEngine {
    rng: ChaCha8Rng,
}

impl WishEngine {
    pub fn random() -> Self {
        Self {
            rng: ChaCha8Rng::from_rng(&mut rand::rng()),
        }
    }
    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    pub fn pull_many(
        &mut self,
        save: &mut SaveData,
        count: usize,
        banner: Banner,
    ) -> Vec<WishResult> {
        (0..count).map(|_| self.pull(save, banner)).collect()
    }

    pub fn pull(&mut self, save: &mut SaveData, banner: Banner) -> WishResult {
        if banner == Banner::Weapon {
            self.pull_weapon(save)
        } else {
            self.pull_character(save, banner)
        }
    }

    fn pull_character(&mut self, save: &mut SaveData, banner: Banner) -> WishResult {
        let five_chance = if save.pity.five_star >= 89 {
            1.0
        } else if save.pity.five_star >= 73 {
            0.006 + 0.06 * f64::from(save.pity.five_star - 72)
        } else {
            0.006
        };
        let roll: f64 = self.rng.random();
        let (item, featured) = if roll < five_chance {
            let featured = save.pity.guaranteed_five || self.rng.random_bool(0.5);
            save.pity.guaranteed_five = !featured;
            if featured {
                (featured_character(banner), true)
            } else {
                (*self.choose(STANDARD_FIVE_CHARACTERS), false)
            }
        } else if save.pity.four_star >= 9 || roll < five_chance + 0.051 {
            let featured = save.pity.guaranteed_four || self.rng.random_bool(0.5);
            save.pity.guaranteed_four = !featured;
            if featured {
                (*self.choose(FEATURED_FOUR), true)
            } else {
                (*self.choose(STANDARD_FOUR), false)
            }
        } else {
            save.pity.five_star += 1;
            save.pity.four_star += 1;
            (*self.choose(THREE_STAR), false)
        };
        if item.rarity == Rarity::Five {
            save.pity.five_star = 0;
            save.pity.four_star += 1;
        } else if item.rarity == Rarity::Four {
            save.pity.five_star += 1;
            save.pity.four_star = 0;
        }
        Self::record(save, item, featured)
    }

    fn pull_weapon(&mut self, save: &mut SaveData) -> WishResult {
        let pity = &mut save.weapon_pity;
        let five_chance = if pity.five_star >= 79 {
            1.0
        } else if pity.five_star >= 62 {
            0.007 + 0.07 * f64::from(pity.five_star - 61)
        } else {
            0.007
        };
        let roll: f64 = self.rng.random();
        let (item, featured) = if roll < five_chance {
            let path = pity.path;
            let selected = weapon_for_path(path);
            let forced_path = pity.fate_points >= 1;
            let on_banner = forced_path || pity.guaranteed_featured || self.rng.random_bool(0.75);
            let item = if forced_path {
                selected
            } else if on_banner {
                if self.rng.random_bool(0.5) {
                    selected
                } else {
                    other_weapon(path)
                }
            } else {
                *self.choose(STANDARD_FIVE_WEAPONS)
            };
            if item.name == selected.name {
                pity.fate_points = 0;
                pity.guaranteed_featured = false;
            } else {
                pity.fate_points = 1;
                pity.guaranteed_featured = !on_banner;
            }
            (item, on_banner)
        } else if pity.four_star >= 9 || roll < five_chance + 0.06 {
            let featured = pity.guaranteed_four || self.rng.random_bool(0.75);
            pity.guaranteed_four = !featured;
            if featured {
                (*self.choose(FEATURED_FOUR_WEAPONS), true)
            } else {
                (*self.choose(STANDARD_FOUR), false)
            }
        } else {
            pity.five_star += 1;
            pity.four_star += 1;
            (*self.choose(THREE_STAR), false)
        };
        if item.rarity == Rarity::Five {
            pity.five_star = 0;
            pity.four_star += 1;
        } else if item.rarity == Rarity::Four {
            pity.five_star += 1;
            pity.four_star = 0;
        }
        Self::record(save, item, featured)
    }

    fn record(save: &mut SaveData, item: Item, featured: bool) -> WishResult {
        save.total_wishes += 1;
        *save.inventory.entry(item.name.into()).or_default() += 1;
        let result = WishResult {
            item,
            rarity: item.rarity,
            featured,
            wish_number: save.total_wishes,
        };
        save.history.push(SavedWish::from(&result));
        if save.history.len() > 200 {
            save.history.drain(..save.history.len() - 200);
        }
        result
    }

    fn choose<'a>(&mut self, items: &'a [Item]) -> &'a Item {
        &items[self.rng.random_range(0..items.len())]
    }
}

pub const fn weapon_for_path(path: WeaponPath) -> Item {
    match path {
        WeaponPath::PolarisEdge => POLARIS_EDGE,
        WeaponPath::NovaGrimoire => NOVA_GRIMOIRE,
    }
}
const fn other_weapon(path: WeaponPath) -> Item {
    match path {
        WeaponPath::PolarisEdge => NOVA_GRIMOIRE,
        WeaponPath::NovaGrimoire => POLARIS_EDGE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn character_hard_pity_is_90() {
        let mut s = SaveData::default();
        s.pity.five_star = 89;
        assert_eq!(
            WishEngine::seeded(7).pull(&mut s, Banner::Kaelis).rarity,
            Rarity::Five
        );
    }
    #[test]
    fn character_banners_share_pity() {
        let mut s = SaveData::default();
        s.pity.five_star = 89;
        let r = WishEngine::seeded(2).pull(&mut s, Banner::Seraphine);
        assert_eq!(r.rarity, Rarity::Five);
        assert_eq!(s.pity.five_star, 0);
    }
    #[test]
    fn four_star_or_better_is_guaranteed_at_10() {
        for seed in 0..100 {
            let mut s = SaveData::default();
            s.pity.four_star = 9;
            assert_ne!(
                WishEngine::seeded(seed)
                    .pull(&mut s, Banner::Astraea)
                    .rarity,
                Rarity::Three
            );
        }
    }
    #[test]
    fn weapon_hard_pity_is_80() {
        let mut s = SaveData::default();
        s.weapon_pity.five_star = 79;
        assert_eq!(
            WishEngine::seeded(9).pull(&mut s, Banner::Weapon).rarity,
            Rarity::Five
        );
    }
    #[test]
    fn fate_point_guarantees_selected_weapon() {
        let mut s = SaveData::default();
        s.weapon_pity.five_star = 79;
        s.weapon_pity.fate_points = 1;
        s.weapon_pity.path = WeaponPath::NovaGrimoire;
        let r = WishEngine::seeded(9).pull(&mut s, Banner::Weapon);
        assert_eq!(r.item.name, "Nova Grimoire");
        assert_eq!(s.weapon_pity.fate_points, 0);
    }
    #[test]
    fn seeded_pulls_are_reproducible() {
        let mut a = SaveData::default();
        let mut b = SaveData::default();
        let aa: Vec<_> = WishEngine::seeded(42)
            .pull_many(&mut a, 10, Banner::Astraea)
            .into_iter()
            .map(|r| r.item.name)
            .collect();
        let bb: Vec<_> = WishEngine::seeded(42)
            .pull_many(&mut b, 10, Banner::Astraea)
            .into_iter()
            .map(|r| r.item.name)
            .collect();
        assert_eq!(aa, bb);
    }

    #[test]
    fn new_catalog_metadata_matches_character_profiles() {
        for (name, element, kind) in [
            ("Vaughn, Violet Oath", "Electro", "Character"),
            ("Cinder, Forgeheart", "Pyro", "Character"),
            ("Sergei, Winterfang", "Cryo", "Character"),
            ("Steven, Azure Shade", "Pyro", "Character"),
            ("Zephra", "Anemo", "Character"),
            ("Neris", "Cryo", "Character"),
            ("Brikka", "Pyro", "Character"),
            ("Galegrip Knuckles", "Anemo", "Gauntlet"),
            ("Winter's Requiem", "Cryo", "Scythe"),
            ("Twin Cinderfangs", "Pyro", "Dual Blades"),
        ] {
            let item = catalog_item(name).unwrap();
            assert_eq!(item.element(), element, "wrong element for {name}");
            assert_eq!(item.kind, kind, "wrong item type for {name}");
        }
        assert_eq!(ASTRAEA.element(), "Cryo");

        let sergei = catalog_item("Sergei, Winterfang").unwrap().stats();
        assert!(sergei.atk > sergei.def);
        assert!(sergei.spd > sergei.def);
        assert!(sergei.def > sergei.hp / 10);
        assert!(sergei.crit_rate < 10);
        assert!(sergei.elemental_atk < 100);
        assert!(sergei.poise < 80);

        let steven = STEVEN.stats();
        assert!(steven.elemental_atk > steven.atk);
        assert!(steven.spd > 140);
        assert!(steven.poise > 150);
        assert!(steven.atk < 110);
        assert!(steven.def < 80);
        assert!(steven.hp < 900);
    }
}
