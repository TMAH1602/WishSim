use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rarity {
    Three,
    Four,
    Five,
}

impl Rarity {
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
    Weapon,
}

impl Banner {
    pub const ALL: [Self; 4] = [Self::Astraea, Self::Kaelis, Self::Seraphine, Self::Weapon];

    pub const fn title(self) -> &'static str {
        match self {
            Self::Astraea => "Starfall Archive",
            Self::Kaelis => "Embers of Rebellion",
            Self::Seraphine => "Whispers in Bloom",
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
