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
    Sergei,
    Saif,
    Standard,
    Weapon,
}

impl Banner {
    pub const SELECTOR: [Self; 8] = [
        Self::Astraea,
        Self::Kaelis,
        Self::Seraphine,
        Self::Vaughn,
        Self::Steven,
        Self::Sergei,
        Self::Saif,
        Self::Standard,
    ];
    pub const ALL: [Self; 9] = [
        Self::Astraea,
        Self::Kaelis,
        Self::Seraphine,
        Self::Vaughn,
        Self::Steven,
        Self::Sergei,
        Self::Saif,
        Self::Standard,
        Self::Weapon,
    ];

    pub const fn title(self) -> &'static str {
        match self {
            Self::Astraea => "Starfall Archive",
            Self::Kaelis => "Embers of Rebellion",
            Self::Seraphine => "Whispers in Bloom",
            Self::Vaughn => "Violet Oath Eternal",
            Self::Steven => "Veilfire Covenant",
            Self::Sergei => "Winterfang's Vigil",
            Self::Saif => "Sovereign of Shifting Sands",
            Self::Standard => "The Everlasting Archive",
            Self::Weapon => "Incarnate Armaments",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum StandardPath {
    #[default]
    Veyra,
    Orin,
    Cinder,
    Pyrite,
    Jeanette,
}

impl StandardPath {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Veyra => "Veyra, Stormseeker",
            Self::Orin => "Orin, Keeper of Embers",
            Self::Cinder => "Cinder, Forgeheart",
            Self::Pyrite => "Pyrite, Gilded Step",
            Self::Jeanette => "Jeanette, Tidemender",
        }
    }

    pub const fn next(self) -> Self {
        match self {
            Self::Veyra => Self::Orin,
            Self::Orin => Self::Cinder,
            Self::Cinder => Self::Pyrite,
            Self::Pyrite => Self::Jeanette,
            Self::Jeanette => Self::Veyra,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponPath {
    #[default]
    PolarisEdge,
    NovaGrimoire,
    DreamwoodRecurve,
    OathbreakerThunder,
    VeilfireSutra,
    WhiteHuntReliquary,
    SandswornDominion,
}

impl WeaponPath {
    pub const ALL: [Self; 7] = [
        Self::PolarisEdge,
        Self::NovaGrimoire,
        Self::DreamwoodRecurve,
        Self::OathbreakerThunder,
        Self::VeilfireSutra,
        Self::WhiteHuntReliquary,
        Self::SandswornDominion,
    ];
    pub const fn name(self) -> &'static str {
        match self {
            Self::PolarisEdge => "Polaris Edge",
            Self::NovaGrimoire => "Nova Grimoire",
            Self::DreamwoodRecurve => "Dreamwood Recurve",
            Self::OathbreakerThunder => "Oathbreaker Thunder",
            Self::VeilfireSutra => "Veilfire Sutra",
            Self::WhiteHuntReliquary => "White Hunt Reliquary",
            Self::SandswornDominion => "Sandsworn Dominion",
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
pub struct StandardPityState {
    pub five_star: u8,
    pub four_star: u8,
    pub fate_points: u8,
    pub path: StandardPath,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct SaveData {
    pub pity: PityState,
    pub weapon_pity: WeaponPityState,
    pub standard_pity: StandardPityState,
    pub total_wishes: u64,
    pub inventory: BTreeMap<String, u32>,
    pub history: Vec<SavedWish>,
    pub teams: Vec<Team>,
    pub equipment: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Team {
    pub name: String,
    pub members: [Option<String>; 3],
}

impl Default for Team {
    fn default() -> Self {
        Self {
            name: "New Team".into(),
            members: [None, None, None],
        }
    }
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            pity: PityState::default(),
            weapon_pity: WeaponPityState::default(),
            standard_pity: StandardPityState::default(),
            total_wishes: 0,
            inventory: BTreeMap::new(),
            history: Vec::new(),
            teams: (1..=5)
                .map(|n| Team {
                    name: format!("Team {n}"),
                    ..Team::default()
                })
                .collect(),
            equipment: BTreeMap::new(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn older_save_without_game_fields_receives_five_teams() {
        let save: SaveData = serde_json::from_str("{}").unwrap();
        assert_eq!(save.teams.len(), 5);
        assert!(
            save.teams
                .iter()
                .all(|team| team.members.iter().all(Option::is_none))
        );
        assert!(save.equipment.is_empty());
    }
}
