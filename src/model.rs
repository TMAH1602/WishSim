use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rarity {
    Three,
    Four,
    Five,
}

impl Rarity {
    pub const fn value(self) -> u8 {
        match self {
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
        }
    }

    pub const fn stars(self) -> &'static str {
        match self {
            Self::Three => "★★★",
            Self::Four => "★★★★",
            Self::Five => "★★★★★",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Item {
    pub name: &'static str,
    pub kind: &'static str,
    pub rarity: Rarity,
}

impl Item {
    pub fn element(self) -> &'static str {
        crate::simulation::item_element(self.name)
    }
    pub fn stats(self) -> Stats {
        crate::simulation::item_stats(self)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Stats {
    pub crit_dmg: u16,
    pub crit_rate: u16,
    pub atk: u16,
    pub def: u16,
    pub spd: u16,
    pub elemental_atk: u16,
    pub hp: u16,
    pub poise: u16,
}

impl Stats {
    pub const fn character(
        rarity: Rarity,
        atk: u16,
        def: u16,
        spd: u16,
        hp: u16,
        poise: u16,
    ) -> Self {
        Self {
            crit_dmg: 150,
            crit_rate: if matches!(rarity, Rarity::Five) {
                10
            } else {
                5
            },
            atk,
            def,
            spd,
            elemental_atk: if matches!(rarity, Rarity::Five) {
                115
            } else {
                100
            },
            hp,
            poise,
        }
    }
    pub const fn weapon(rarity: Rarity, atk: u16, crit_rate: u16, crit_dmg: u16) -> Self {
        Self {
            crit_dmg,
            crit_rate,
            atk,
            def: 0,
            spd: 0,
            elemental_atk: if matches!(rarity, Rarity::Five) {
                18
            } else {
                12
            },
            hp: 0,
            poise: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WishResult {
    pub item: Item,
    pub rarity: Rarity,
    pub featured: bool,
    pub wish_number: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Banner {
    #[default]
    Astraea,
    Kaelis,
    Seraphine,
    Vaughn,
    Steven,
    Weapon,
}

impl Banner {
    pub const ALL: [Self; 6] = [
        Self::Astraea,
        Self::Kaelis,
        Self::Seraphine,
        Self::Vaughn,
        Self::Steven,
        Self::Weapon,
    ];

    pub const fn title(self) -> &'static str {
        match self {
            Self::Astraea => "Starfall Archive",
            Self::Kaelis => "Embers of Rebellion",
            Self::Seraphine => "Whispers in Bloom",
            Self::Vaughn => "Violet Oath Eternal",
            Self::Steven => "Veilfire Covenant",
            Self::Weapon => "Incarnate Armaments",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponPath {
    #[default]
    PolarisEdge,
    NovaGrimoire,
}

impl WeaponPath {
    pub const fn name(self) -> &'static str {
        match self {
            Self::PolarisEdge => "Polaris Edge",
            Self::NovaGrimoire => "Nova Grimoire",
        }
    }

    pub const fn toggled(self) -> Self {
        match self {
            Self::PolarisEdge => Self::NovaGrimoire,
            Self::NovaGrimoire => Self::PolarisEdge,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PityState {
    pub five_star: u8,
    pub four_star: u8,
    pub guaranteed_five: bool,
    pub guaranteed_four: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WeaponPityState {
    pub five_star: u8,
    pub four_star: u8,
    pub guaranteed_featured: bool,
    pub guaranteed_four: bool,
    pub fate_points: u8,
    pub path: WeaponPath,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SaveData {
    pub pity: PityState,
    pub weapon_pity: WeaponPityState,
    pub total_wishes: u64,
    pub inventory: BTreeMap<String, u32>,
    pub history: Vec<SavedWish>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedWish {
    pub name: String,
    pub rarity: Rarity,
    pub featured: bool,
    pub wish_number: u64,
}

impl From<&WishResult> for SavedWish {
    fn from(value: &WishResult) -> Self {
        Self {
            name: value.item.name.into(),
            rarity: value.rarity,
            featured: value.featured,
            wish_number: value.wish_number,
        }
    }
}
