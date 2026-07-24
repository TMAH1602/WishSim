use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::model::{
    Banner, Item, Rarity, SaveData, SavedWish, StandardPath, Stats, WeaponPath, WishResult,
};

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
pub const SERGEI: Item = Item {
    name: "Sergei, Winterfang",
    kind: "Character",
    rarity: Rarity::Five,
};
pub const SAIF: Item = Item {
    name: "Saif, Dune Sovereign",
    kind: "Character",
    rarity: Rarity::Five,
};
pub const YEOUNGIN: Item = Item {
    name: "Yeoungin, Winter's Grace",
    kind: "Character",
    rarity: Rarity::Five,
};
pub const KLARA: Item = Item {
    name: "Klara, Jade Tempest",
    kind: "Character",
    rarity: Rarity::Five,
};
pub const DREAMWOOD_RECURVE: Item = Item {
    name: "Dreamwood Recurve",
    kind: "Bow",
    rarity: Rarity::Five,
};
pub const OATHBREAKER_THUNDER: Item = Item {
    name: "Oathbreaker Thunder",
    kind: "Claymore",
    rarity: Rarity::Five,
};
pub const VEILFIRE_SUTRA: Item = Item {
    name: "Veilfire Sutra",
    kind: "Catalyst",
    rarity: Rarity::Five,
};
pub const WHITE_HUNT_RELIQUARY: Item = Item {
    name: "White Hunt Reliquary",
    kind: "Catalyst",
    rarity: Rarity::Five,
};
pub const SANDSWORN_DOMINION: Item = Item {
    name: "Sandsworn Dominion",
    kind: "Polearm",
    rarity: Rarity::Five,
};
pub const RIMEBOUND_BENEDICTION: Item = Item {
    name: "Rimebound Benediction",
    kind: "Polearm",
    rarity: Rarity::Five,
};
pub const GALES_LAST_HARVEST: Item = Item {
    name: "Gale's Last Harvest",
    kind: "Scythe",
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
pub const TEMPEST_MERIDIAN: Item = Item {
    name: "Tempest Meridian",
    kind: "Bow",
    rarity: Rarity::Five,
};
pub const EMBERKEEPERS_OATH: Item = Item {
    name: "Emberkeeper's Oath",
    kind: "Claymore",
    rarity: Rarity::Five,
};
pub const FURNACEHEART_BRACERS: Item = Item {
    name: "Furnaceheart Bracers",
    kind: "Gauntlet",
    rarity: Rarity::Five,
};
pub const AURUM_FLASH: Item = Item {
    name: "Aurum Flash",
    kind: "Sword",
    rarity: Rarity::Five,
};
pub const SILVER_TIDEMARK: Item = Item {
    name: "Silver Tidemark",
    kind: "Bow",
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
        name: "Pyrite, Gilded Step",
        kind: "Character",
        rarity: Rarity::Five,
    },
    Item {
        name: "Jeanette, Tidemender",
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
    TEMPEST_MERIDIAN,
    EMBERKEEPERS_OATH,
    FURNACEHEART_BRACERS,
    AURUM_FLASH,
    SILVER_TIDEMARK,
];

#[cfg(test)]
const STANDARD_SIGNATURES: [(&str, &str); 5] = [
    ("Veyra, Stormseeker", "Tempest Meridian"),
    ("Orin, Keeper of Embers", "Emberkeeper's Oath"),
    ("Cinder, Forgeheart", "Furnaceheart Bracers"),
    ("Pyrite, Gilded Step", "Aurum Flash"),
    ("Jeanette, Tidemender", "Silver Tidemark"),
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
    Item {
        name: "Farah",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Anya",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Rook",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Kestrel",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Mako",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Ysra",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Dolma",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Corvin",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Seo-yeon",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Ji-ho",
        kind: "Character",
        rarity: Rarity::Four,
    },
    Item {
        name: "Taisia",
        kind: "Character",
        rarity: Rarity::Four,
    },
];

pub fn all_characters() -> Vec<Item> {
    [
        ASTRAEA, KAELIS, SERAPHINE, VAUGHN, STEVEN, SERGEI, SAIF, YEOUNGIN, KLARA,
    ]
    .into_iter()
    .chain(STANDARD_FIVE_CHARACTERS.iter().copied())
    .chain(FEATURED_FOUR.iter().copied())
    .collect()
}

pub fn character_weapon_type(name: &str) -> &'static str {
    match name {
        "Astraea, Starbound"
        | "Steven, Azure Shade"
        | "Sergei, Winterfang"
        | "Mira"
        | "Farah"
        | "Seo-yeon" => "Catalyst",
        "Kaelis, Ashen Vanguard" | "Pyrite, Gilded Step" | "Lumen" | "Anya" | "Ji-ho" => "Sword",
        "Seraphine, Verdant Oracle" | "Veyra, Stormseeker" | "Jeanette, Tidemender" | "Kestrel" => {
            "Bow"
        }
        "Vaughn, Violet Oath" | "Orin, Keeper of Embers" | "Dolma" => "Claymore",
        "Saif, Dune Sovereign" | "Yeoungin, Winter's Grace" | "Thorne" | "Ysra" => "Polearm",
        "Cinder, Forgeheart" | "Rook" | "Corvin" => "Gauntlet",
        "Zephra" => "Gauntlet",
        "Neris" | "Klara, Jade Tempest" => "Scythe",
        "Taisia" => "Catalyst",
        "Brikka" | "Mako" => "Dual Blades",
        _ => "Unaligned",
    }
}
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
        Banner::Sergei => SERGEI,
        Banner::Saif => SAIF,
        Banner::Yeoungin => YEOUNGIN,
        Banner::Klara => KLARA,
        Banner::Standard => ASTRAEA,
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
        SERGEI,
        SAIF,
        YEOUNGIN,
        KLARA,
        DREAMWOOD_RECURVE,
        OATHBREAKER_THUNDER,
        VEILFIRE_SUTRA,
        WHITE_HUNT_RELIQUARY,
        SANDSWORN_DOMINION,
        RIMEBOUND_BENEDICTION,
        GALES_LAST_HARVEST,
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
        "Astraea, Starbound"
        | "Sergei, Winterfang"
        | "Neris"
        | "Anya"
        | "Corvin"
        | "Winter's Requiem"
        | "White Hunt Reliquary" => "Cryo",
        "Yeoungin, Winter's Grace" | "Rimebound Benediction" => "Cryo",
        "Klara, Jade Tempest" | "Taisia" | "Gale's Last Harvest" => "Anemo",
        "Lumen"
        | "Saif, Dune Sovereign"
        | "Pyrite, Gilded Step"
        | "Farah"
        | "Dolma"
        | "Sandsworn Dominion"
        | "Aurum Flash" => "Geo",
        "Kaelis, Ashen Vanguard"
        | "Orin, Keeper of Embers"
        | "Cinder, Forgeheart"
        | "Steven, Azure Shade"
        | "Brikka"
        | "Ysra"
        | "Twin Cinderfangs"
        | "Ji-ho"
        | "Emberkeeper's Oath"
        | "Furnaceheart Bracers" => "Pyro",
        "Seraphine, Verdant Oracle" | "Thorne" => "Dendro",
        "Vaughn, Violet Oath"
        | "Veyra, Stormseeker"
        | "Rook"
        | "Seo-yeon"
        | "Oathbreaker Thunder"
        | "Tempest Meridian" => "Electro",
        "Mira" | "Jeanette, Tidemender" | "Mako" | "Silver Tidemark" => "Hydro",
        "Zephra" | "Kestrel" | "Galegrip Knuckles" => "Anemo",
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
            "Saif, Dune Sovereign" => Stats::character(item.rarity, 146, 128, 104, 1160, 126),
            "Pyrite, Gilded Step" => Stats::character(item.rarity, 139, 92, 158, 1010, 72),
            "Jeanette, Tidemender" => Stats::character(item.rarity, 96, 104, 112, 1420, 88),
            "Yeoungin, Winter's Grace" => Stats {
                crit_dmg: 125,
                crit_rate: 5,
                atk: 92,
                def: 118,
                spd: 110,
                elemental_atk: 142,
                hp: 1510,
                poise: 104,
            },
            "Klara, Jade Tempest" => Stats {
                crit_dmg: 150,
                crit_rate: 12,
                atk: 118,
                def: 78,
                spd: 146,
                elemental_atk: 176,
                hp: 1040,
                poise: 68,
            },
            "Farah" => Stats::character(item.rarity, 94, 118, 98, 1040, 112),
            "Anya" => Stats::character(item.rarity, 101, 106, 112, 1100, 98),
            "Rook" => Stats::character(item.rarity, 126, 86, 119, 960, 91),
            "Kestrel" => Stats::character(item.rarity, 113, 78, 132, 900, 68),
            "Mako" => Stats::character(item.rarity, 124, 82, 125, 940, 76),
            "Ysra" => Stats::character(item.rarity, 96, 101, 105, 1060, 104),
            "Dolma" => Stats::character(item.rarity, 108, 132, 72, 1210, 142),
            "Corvin" => Stats::character(item.rarity, 116, 94, 118, 970, 110),
            "Zephra" => Stats::character(item.rarity, 112, 78, 128, 890, 70),
            "Neris" => Stats::character(item.rarity, 121, 82, 106, 930, 82),
            "Brikka" => Stats::character(item.rarity, 126, 91, 101, 980, 96),
            "Seo-yeon" => Stats {
                crit_dmg: 145,
                crit_rate: 7,
                atk: 91,
                def: 102,
                spd: 108,
                elemental_atk: 136,
                hp: 1030,
                poise: 86,
            },
            "Taisia" => Stats {
                crit_dmg: 120,
                crit_rate: 5,
                atk: 74,
                def: 92,
                spd: 118,
                elemental_atk: 142,
                hp: 1210,
                poise: 84,
            },
            "Ji-ho" => Stats {
                crit_dmg: 150,
                crit_rate: 6,
                atk: 126,
                def: 116,
                spd: 103,
                elemental_atk: 104,
                hp: 1090,
                poise: 121,
            },
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
        match banner {
            Banner::Weapon => self.pull_weapon(save),
            Banner::Standard => self.pull_standard(save),
            _ => self.pull_character(save, banner),
        }
    }

    fn pull_standard(&mut self, save: &mut SaveData) -> WishResult {
        let pity = &mut save.standard_pity;
        let five_chance = if pity.five_star >= 89 {
            1.0
        } else if pity.five_star >= 73 {
            0.006 + 0.06 * f64::from(pity.five_star - 72)
        } else {
            0.006
        };
        let roll: f64 = self.rng.random();
        let (item, targeted) = if roll < five_chance {
            let target = standard_character(pity.path);
            let item = if pity.fate_points >= 1 {
                target
            } else {
                *self.choose(STANDARD_FIVE_CHARACTERS)
            };
            if item.name == target.name {
                pity.fate_points = 0;
            } else {
                pity.fate_points = 1;
            }
            (item, item.name == target.name)
        } else if pity.four_star >= 9 || roll < five_chance + 0.051 {
            (*self.choose(FEATURED_FOUR), false)
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
        Self::record(save, item, targeted)
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
        WeaponPath::DreamwoodRecurve => DREAMWOOD_RECURVE,
        WeaponPath::OathbreakerThunder => OATHBREAKER_THUNDER,
        WeaponPath::VeilfireSutra => VEILFIRE_SUTRA,
        WeaponPath::WhiteHuntReliquary => WHITE_HUNT_RELIQUARY,
        WeaponPath::SandswornDominion => SANDSWORN_DOMINION,
        WeaponPath::RimeboundBenediction => RIMEBOUND_BENEDICTION,
        WeaponPath::GalesLastHarvest => GALES_LAST_HARVEST,
        WeaponPath::TempestMeridian => TEMPEST_MERIDIAN,
        WeaponPath::EmberkeepersOath => EMBERKEEPERS_OATH,
        WeaponPath::FurnaceheartBracers => FURNACEHEART_BRACERS,
        WeaponPath::AurumFlash => AURUM_FLASH,
        WeaponPath::SilverTidemark => SILVER_TIDEMARK,
    }
}

pub const fn standard_character(path: StandardPath) -> Item {
    match path {
        StandardPath::Veyra => STANDARD_FIVE_CHARACTERS[0],
        StandardPath::Orin => STANDARD_FIVE_CHARACTERS[1],
        StandardPath::Cinder => STANDARD_FIVE_CHARACTERS[2],
        StandardPath::Pyrite => STANDARD_FIVE_CHARACTERS[3],
        StandardPath::Jeanette => STANDARD_FIVE_CHARACTERS[4],
    }
}
const fn other_weapon(path: WeaponPath) -> Item {
    match path {
        WeaponPath::PolarisEdge => NOVA_GRIMOIRE,
        _ => POLARIS_EDGE,
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
    fn standard_fate_point_guarantees_selected_character() {
        let mut save = SaveData::default();
        save.standard_pity.five_star = 89;
        save.standard_pity.fate_points = 1;
        save.standard_pity.path = StandardPath::Jeanette;
        let result = WishEngine::seeded(4).pull(&mut save, Banner::Standard);
        assert_eq!(result.item.name, "Jeanette, Tidemender");
        assert_eq!(save.standard_pity.fate_points, 0);
    }

    #[test]
    fn off_target_standard_character_awards_fate_point() {
        let mut observed = false;
        for seed in 0..100 {
            let mut save = SaveData::default();
            save.standard_pity.five_star = 89;
            save.standard_pity.path = StandardPath::Jeanette;
            let result = WishEngine::seeded(seed).pull(&mut save, Banner::Standard);
            if result.item.name != "Jeanette, Tidemender" {
                assert_eq!(save.standard_pity.fate_points, 1);
                observed = true;
                break;
            }
        }
        assert!(
            observed,
            "seed range produced no off-target standard character"
        );
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
            ("Saif, Dune Sovereign", "Geo", "Character"),
            ("Pyrite, Gilded Step", "Geo", "Character"),
            ("Jeanette, Tidemender", "Hydro", "Character"),
            ("Yeoungin, Winter's Grace", "Cryo", "Character"),
            ("Farah", "Geo", "Character"),
            ("Anya", "Cryo", "Character"),
            ("Rook", "Electro", "Character"),
            ("Kestrel", "Anemo", "Character"),
            ("Mako", "Hydro", "Character"),
            ("Ysra", "Pyro", "Character"),
            ("Dolma", "Geo", "Character"),
            ("Corvin", "Cryo", "Character"),
            ("Seo-yeon", "Electro", "Character"),
            ("Ji-ho", "Pyro", "Character"),
            ("Galegrip Knuckles", "Anemo", "Gauntlet"),
            ("Winter's Requiem", "Cryo", "Scythe"),
            ("Twin Cinderfangs", "Pyro", "Dual Blades"),
        ] {
            let item = catalog_item(name).unwrap();
            assert_eq!(item.element(), element, "wrong element for {name}");
            assert_eq!(item.kind, kind, "wrong item type for {name}");
        }
        assert_eq!(ASTRAEA.element(), "Cryo");
        assert_eq!(featured_character(Banner::Sergei).name, SERGEI.name);
        assert_eq!(featured_character(Banner::Saif).name, SAIF.name);
        assert_eq!(featured_character(Banner::Yeoungin).name, YEOUNGIN.name);
        assert_eq!(featured_character(Banner::Klara).name, KLARA.name);
        for (weapon, kind) in [
            ("Dreamwood Recurve", "Bow"),
            ("Oathbreaker Thunder", "Claymore"),
            ("Veilfire Sutra", "Catalyst"),
            ("White Hunt Reliquary", "Catalyst"),
            ("Sandsworn Dominion", "Polearm"),
            ("Rimebound Benediction", "Polearm"),
            ("Gale's Last Harvest", "Scythe"),
        ] {
            let item = catalog_item(weapon).unwrap();
            assert_eq!(item.rarity, Rarity::Five);
            assert_eq!(item.kind, kind);
        }
        assert!(
            !STANDARD_FIVE_CHARACTERS
                .iter()
                .any(|item| item.name == SERGEI.name)
        );

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

        let yeoungin = YEOUNGIN.stats();
        assert!(yeoungin.hp > 1400);
        assert!(yeoungin.elemental_atk > yeoungin.atk);
        assert!(yeoungin.def > yeoungin.atk);

        let characters = all_characters();
        assert_eq!(characters.len(), 31);
        let mut names = characters.iter().map(|item| item.name).collect::<Vec<_>>();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), characters.len());
    }

    #[test]
    fn every_limited_signature_is_a_selectable_weapon_path() {
        let names = WeaponPath::ALL
            .iter()
            .map(|path| weapon_for_path(*path).name)
            .collect::<Vec<_>>();
        assert_eq!(names.len(), 14);
        for signature in [
            "Dreamwood Recurve",
            "Oathbreaker Thunder",
            "Veilfire Sutra",
            "White Hunt Reliquary",
            "Sandsworn Dominion",
            "Rimebound Benediction",
            "Gale's Last Harvest",
        ] {
            assert!(names.contains(&signature));
            assert!(
                !STANDARD_FIVE_WEAPONS
                    .iter()
                    .any(|item| item.name == signature)
            );
        }
    }

    #[test]
    fn every_standard_five_star_has_a_unique_obtainable_signature() {
        assert_eq!(STANDARD_SIGNATURES.len(), STANDARD_FIVE_CHARACTERS.len());
        for (character, signature) in STANDARD_SIGNATURES {
            assert!(
                STANDARD_FIVE_CHARACTERS
                    .iter()
                    .any(|item| item.name == character)
            );
            assert!(
                STANDARD_FIVE_WEAPONS
                    .iter()
                    .any(|item| item.name == signature),
                "{signature} must be obtainable from the standard weapon pool"
            );
            assert!(
                WeaponPath::ALL
                    .iter()
                    .any(|path| weapon_for_path(*path).name == signature),
                "{signature} must also be selectable on the weapon banner"
            );
            let character_kind = character_weapon_type(character);
            let weapon_kind = catalog_item(signature).unwrap().kind;
            assert_eq!(
                character_kind, weapon_kind,
                "{character} signature mismatch"
            );
        }
    }

    #[test]
    fn every_character_has_an_equipment_type() {
        for character in all_characters() {
            assert_ne!(
                character_weapon_type(character.name),
                "Unaligned",
                "{}",
                character.name
            );
        }
    }
}
