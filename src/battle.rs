use std::collections::BTreeMap;

use crate::{model::Stats, simulation::catalog_item};

pub const BASE_LEVEL: u8 = 1;
pub const BATTLE_LEVEL: u8 = 50;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BattleAction {
    Basic,
    Skill,
    Ultimate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BattleMenu {
    Action,
    EnemyTarget(BattleAction),
    AllyTarget(BattleAction),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BattleOutcome {
    Victory,
    Defeat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TurnRef {
    pub ally: bool,
    pub index: usize,
}

#[derive(Clone, Debug)]
pub struct BattleUnit {
    pub name: String,
    pub element: &'static str,
    pub level: u8,
    pub hp: i32,
    pub max_hp: i32,
    pub atk: u16,
    pub atk_boost: u16,
    pub def: u16,
    pub spd: u16,
    pub elemental_atk: u16,
    pub poise: u16,
    pub healer: bool,
    pub support_atk_boost: bool,
    pub abilities: AbilityLoadout,
    pub cooldowns: [u8; 3],
    pub weapon_level: Option<u8>,
    pub poison_turns: u8,
    pub defending: bool,
}

impl BattleUnit {
    pub fn alive(&self) -> bool {
        self.hp > 0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AbilityLoadout {
    pub basic: &'static str,
    pub skill: &'static str,
    pub ultimate: &'static str,
    pub ultimate_wait: u8,
}

#[derive(Clone, Debug)]
pub struct BattleState {
    pub allies: Vec<BattleUnit>,
    pub enemies: Vec<BattleUnit>,
    pub order: Vec<TurnRef>,
    pub turn: usize,
    pub round: u16,
    pub menu: BattleMenu,
    pub cursor: usize,
    pub log: Vec<String>,
    pub outcome: Option<BattleOutcome>,
    prepared_turn: Option<(u16, TurnRef)>,
}

impl BattleState {
    pub fn new(team: &[String], equipment: &BTreeMap<String, String>) -> Option<Self> {
        if team.len() != 3 {
            return None;
        }
        let allies = team
            .iter()
            .map(|name| {
                let item = catalog_item(name)?;
                let mut stats = item.stats();
                let mut weapon_name = None;
                if let Some(weapon) = equipment.get(name).and_then(|name| catalog_item(name)) {
                    let weapon_stats = projected_weapon_stats(weapon.stats(), BATTLE_LEVEL);
                    stats.atk = stats.atk.saturating_add(weapon_stats.atk);
                    stats.elemental_atk = stats
                        .elemental_atk
                        .saturating_add(weapon_stats.elemental_atk);
                    stats.crit_rate = stats.crit_rate.saturating_add(weapon_stats.crit_rate);
                    stats.crit_dmg = stats.crit_dmg.saturating_add(weapon_stats.crit_dmg);
                    weapon_name = Some(weapon.name);
                }
                Some(character_unit(name, item.element(), stats, weapon_name))
            })
            .collect::<Option<Vec<_>>>()?;
        let enemies = vec![
            enemy_unit(
                "Hydro Slime",
                "Hydro",
                Stats {
                    crit_dmg: 100,
                    crit_rate: 0,
                    atk: 95,
                    def: 82,
                    spd: 78,
                    elemental_atk: 96,
                    hp: 1050,
                    poise: 76,
                },
            ),
            enemy_unit(
                "Thornbloom",
                "Dendro",
                Stats {
                    crit_dmg: 100,
                    crit_rate: 0,
                    atk: 112,
                    def: 104,
                    spd: 68,
                    elemental_atk: 118,
                    hp: 1320,
                    poise: 112,
                },
            ),
            enemy_unit(
                "Hydro Slime",
                "Hydro",
                Stats {
                    crit_dmg: 100,
                    crit_rate: 0,
                    atk: 95,
                    def: 82,
                    spd: 78,
                    elemental_atk: 96,
                    hp: 1050,
                    poise: 76,
                },
            ),
        ];
        let mut state = Self {
            allies,
            enemies,
            order: Vec::new(),
            turn: 0,
            round: 1,
            menu: BattleMenu::Action,
            cursor: 0,
            log: vec![
                "Battle Test initialized. Characters and equipped weapons are level 50.".into(),
            ],
            outcome: None,
            prepared_turn: None,
        };
        state.rebuild_order();
        state.advance_enemy_turns();
        state.prepare_current_ally();
        Some(state)
    }

    pub fn current(&self) -> Option<TurnRef> {
        self.order.get(self.turn).copied()
    }

    pub fn current_name(&self) -> &str {
        self.current()
            .and_then(|turn| {
                if turn.ally {
                    self.allies.get(turn.index)
                } else {
                    self.enemies.get(turn.index)
                }
            })
            .map_or("—", |unit| unit.name.as_str())
    }

    pub fn target_count(&self) -> usize {
        match self.menu {
            BattleMenu::EnemyTarget(_) => self.enemies.iter().filter(|unit| unit.alive()).count(),
            BattleMenu::AllyTarget(_) => self.allies.iter().filter(|unit| unit.alive()).count(),
            BattleMenu::Action => 4,
        }
    }

    pub fn move_cursor(&mut self, delta: isize) {
        let count = self.target_count();
        if count == 0 {
            self.cursor = 0;
        } else if delta < 0 {
            self.cursor = self.cursor.saturating_sub(delta.unsigned_abs());
        } else {
            self.cursor = (self.cursor + delta as usize).min(count - 1);
        }
    }

    pub fn back(&mut self) {
        if !matches!(self.menu, BattleMenu::Action) {
            self.menu = BattleMenu::Action;
            self.cursor = 0;
        }
    }

    pub fn confirm(&mut self) {
        if self.outcome.is_some() || self.current().is_none_or(|turn| !turn.ally) {
            return;
        }
        match self.menu {
            BattleMenu::Action => match self.cursor {
                index @ 0..=2 => {
                    let action = [
                        BattleAction::Basic,
                        BattleAction::Skill,
                        BattleAction::Ultimate,
                    ][index];
                    if !self.action_ready(action) {
                        return;
                    }
                    if !matches!(action, BattleAction::Basic)
                        && self.current_ally().is_some_and(|unit| unit.healer)
                    {
                        self.menu = BattleMenu::AllyTarget(action);
                    } else {
                        self.menu = BattleMenu::EnemyTarget(action);
                    }
                    self.cursor = 0;
                }
                _ => self.perform_defend(),
            },
            BattleMenu::EnemyTarget(action) => {
                if let Some(target) = living_index(&self.enemies, self.cursor) {
                    self.perform_attack(action, target);
                }
            }
            BattleMenu::AllyTarget(action) => {
                if let Some(target) = living_index(&self.allies, self.cursor) {
                    self.perform_heal(action, target);
                }
            }
        }
    }

    fn action_ready(&self, action: BattleAction) -> bool {
        self.current_ally()
            .is_some_and(|unit| unit.cooldowns[action_index(action)] == 0)
    }

    pub fn current_ally(&self) -> Option<&BattleUnit> {
        let turn = self.current()?;
        if turn.ally {
            self.allies.get(turn.index)
        } else {
            None
        }
    }

    fn perform_attack(&mut self, action: BattleAction, target: usize) {
        let Some(actor) = self.current() else { return };
        let attacker = self.allies[actor.index].clone();
        self.allies[actor.index].defending = false;
        self.set_cooldown(actor.index, action);
        let defender = &mut self.enemies[target];
        let (power, ability_multiplier, element_multiplier, label) = match action {
            BattleAction::Basic => (
                attacker.atk.saturating_add(attacker.atk_boost),
                100,
                1,
                attacker.abilities.basic,
            ),
            BattleAction::Skill => (
                attacker.elemental_atk.saturating_add(attacker.atk_boost),
                145,
                effectiveness(attacker.element, defender.element),
                attacker.abilities.skill,
            ),
            BattleAction::Ultimate => (
                attacker.elemental_atk.saturating_add(attacker.atk_boost),
                220 + u16::from(attacker.abilities.ultimate_wait) * 15,
                effectiveness(attacker.element, defender.element),
                attacker.abilities.ultimate,
            ),
        };
        let scaled_power = (u32::from(power) * u32::from(ability_multiplier) / 100)
            .min(u32::from(u16::MAX)) as u16;
        let damage = damage(scaled_power, defender.def, defender.poise, false) * element_multiplier;
        defender.hp = (defender.hp - damage).max(0);
        let defender_name = defender.name.clone();
        let effective = if element_multiplier == 2 {
            "  •  2× EFFECTIVE"
        } else {
            ""
        };
        self.push_log(format!(
            "{} used {label} on {} for {damage} damage{effective}.",
            attacker.name, defender_name
        ));
        self.finish_turn();
    }

    fn perform_heal(&mut self, action: BattleAction, target: usize) {
        let Some(actor) = self.current() else { return };
        let healer = self.allies[actor.index].clone();
        self.allies[actor.index].defending = false;
        self.set_cooldown(actor.index, action);
        let (label, scale) = match action {
            BattleAction::Skill => (healer.abilities.skill, 3),
            BattleAction::Ultimate => (healer.abilities.ultimate, 5),
            BattleAction::Basic => (healer.abilities.basic, 1),
        };
        let amount = (healer.elemental_atk as i32 * scale).max(180);
        let ally = &mut self.allies[target];
        let restored = amount.min(ally.max_hp - ally.hp).max(0);
        ally.hp += restored;
        let boost = if healer.support_atk_boost {
            (healer.elemental_atk / 5).max(20)
        } else {
            0
        };
        ally.atk_boost = ally.atk_boost.max(boost);
        let ally_name = ally.name.clone();
        self.push_log(if boost > 0 {
            format!(
                "{} used {label}: restored {restored} HP to {ally_name} and granted +{boost} ATK.",
                healer.name,
            )
        } else {
            format!(
                "{} used {label} on {ally_name}, restoring {restored} HP.",
                healer.name,
            )
        });
        self.finish_turn();
    }

    fn set_cooldown(&mut self, ally_index: usize, action: BattleAction) {
        let wait = match action {
            BattleAction::Basic => 0,
            BattleAction::Skill => 2,
            BattleAction::Ultimate => self.allies[ally_index]
                .abilities
                .ultimate_wait
                .saturating_add(1),
        };
        self.allies[ally_index].cooldowns[action_index(action)] = wait;
    }

    fn perform_defend(&mut self) {
        let Some(actor) = self.current() else { return };
        let name = self.allies[actor.index].name.clone();
        self.allies[actor.index].defending = true;
        self.push_log(format!("{name} is defending until their next action."));
        self.finish_turn();
    }

    fn enemy_action(&mut self, enemy_index: usize) {
        let Some(target) = self.choose_enemy_target(enemy_index) else {
            return;
        };
        let enemy = self.enemies[enemy_index].clone();
        let ally = &mut self.allies[target];
        let uses_poison = enemy.name == "Thornbloom"
            && (usize::from(self.round) + enemy_index + target).is_multiple_of(3);
        let power = if uses_poison {
            enemy.elemental_atk
        } else {
            enemy.atk
        };
        let dealt = damage(power, ally.def, ally.poise, ally.defending);
        ally.hp = (ally.hp - dealt).max(0);
        if uses_poison && ally.alive() {
            ally.poison_turns = 3;
        }
        let ally_name = ally.name.clone();
        let defended = ally.defending;
        self.push_log(if uses_poison {
            format!(
                "{} poisoned {} for {dealt} damage (3 turns){}.",
                enemy.name,
                ally_name,
                if defended { " through DEFEND" } else { "" }
            )
        } else {
            format!(
                "{} struck {} for {dealt} damage{}.",
                enemy.name,
                ally_name,
                if defended { " through DEFEND" } else { "" }
            )
        });
    }

    fn choose_enemy_target(&self, enemy_index: usize) -> Option<usize> {
        self.allies
            .iter()
            .enumerate()
            .filter(|(_, unit)| unit.alive())
            .min_by_key(|(index, unit)| {
                let hp_percent = unit.hp * 100 / unit.max_hp.max(1);
                let guard_penalty = if unit.defending { 35 } else { 0 };
                let rotation = ((*index + enemy_index + usize::from(self.round)) % 3) as i32;
                hp_percent + guard_penalty + rotation
            })
            .map(|(index, _)| index)
    }

    fn finish_turn(&mut self) {
        self.menu = BattleMenu::Action;
        self.cursor = 0;
        self.advance_turn();
        self.check_outcome();
        self.advance_enemy_turns();
        self.prepare_current_ally();
    }

    fn prepare_current_ally(&mut self) {
        loop {
            let Some(turn) = self.current() else { return };
            if !turn.ally || self.outcome.is_some() {
                return;
            }
            let marker = (self.round, turn);
            if self.prepared_turn == Some(marker) {
                return;
            }
            self.prepared_turn = Some(marker);
            let unit = &mut self.allies[turn.index];
            for cooldown in &mut unit.cooldowns {
                *cooldown = cooldown.saturating_sub(1);
            }
            unit.defending = false;
            if unit.poison_turns == 0 {
                return;
            }
            let poison_damage = (unit.max_hp / 12).max(1);
            unit.hp = (unit.hp - poison_damage).max(0);
            unit.poison_turns -= 1;
            let name = unit.name.clone();
            self.push_log(format!(
                "{name} suffered {poison_damage} poison damage ({} turns remain).",
                self.allies[turn.index].poison_turns
            ));
            self.check_outcome();
            if self.allies[turn.index].alive() || self.outcome.is_some() {
                return;
            }
            self.advance_turn();
            self.advance_enemy_turns();
        }
    }

    fn advance_enemy_turns(&mut self) {
        loop {
            self.check_outcome();
            if self.outcome.is_some() {
                break;
            }
            let Some(turn) = self.current() else { break };
            if turn.ally {
                break;
            }
            if self.enemies[turn.index].alive() {
                self.enemy_action(turn.index);
            }
            self.advance_turn();
        }
    }

    fn advance_turn(&mut self) {
        if self.order.is_empty() {
            return;
        }
        self.turn += 1;
        if self.turn >= self.order.len() {
            self.turn = 0;
            self.round = self.round.saturating_add(1);
            self.rebuild_order();
        }
        while self.current().is_some_and(|turn| !self.unit(turn).alive()) {
            self.turn += 1;
            if self.turn >= self.order.len() {
                self.turn = 0;
                self.round = self.round.saturating_add(1);
                self.rebuild_order();
            }
        }
    }

    fn unit(&self, turn: TurnRef) -> &BattleUnit {
        if turn.ally {
            &self.allies[turn.index]
        } else {
            &self.enemies[turn.index]
        }
    }

    fn rebuild_order(&mut self) {
        self.order = self
            .allies
            .iter()
            .enumerate()
            .map(|(index, _)| TurnRef { ally: true, index })
            .chain(
                self.enemies
                    .iter()
                    .enumerate()
                    .map(|(index, _)| TurnRef { ally: false, index }),
            )
            .collect();
        let allies = &self.allies;
        let enemies = &self.enemies;
        self.order.sort_by(|a, b| {
            let speed = |turn: &TurnRef| {
                if turn.ally {
                    allies[turn.index].spd
                } else {
                    enemies[turn.index].spd
                }
            };
            speed(b).cmp(&speed(a)).then_with(|| b.ally.cmp(&a.ally))
        });
    }

    fn check_outcome(&mut self) {
        self.outcome = if self.enemies.iter().all(|unit| !unit.alive()) {
            Some(BattleOutcome::Victory)
        } else if self.allies.iter().all(|unit| !unit.alive()) {
            Some(BattleOutcome::Defeat)
        } else {
            None
        };
    }

    fn push_log(&mut self, message: String) {
        self.log.push(message);
        if self.log.len() > 5 {
            self.log.remove(0);
        }
    }
}

fn character_unit(
    name: &str,
    element: &'static str,
    stats: Stats,
    weapon_name: Option<&str>,
) -> BattleUnit {
    let max_hp = stats.hp as i32 * 2;
    BattleUnit {
        name: name.into(),
        element,
        level: BATTLE_LEVEL,
        hp: max_hp,
        max_hp,
        atk: stats.atk,
        atk_boost: 0,
        def: stats.def,
        spd: stats.spd,
        elemental_atk: stats.elemental_atk,
        poise: stats.poise,
        healer: matches!(name, "Jeanette, Tidemender" | "Yeoungin, Winter's Grace"),
        support_atk_boost: name == "Yeoungin, Winter's Grace",
        abilities: ability_loadout(name),
        cooldowns: [0; 3],
        weapon_level: weapon_name.map(|_| BATTLE_LEVEL),
        poison_turns: 0,
        defending: false,
    }
}

fn enemy_unit(name: &str, element: &'static str, stats: Stats) -> BattleUnit {
    let hp = stats.hp as i32;
    BattleUnit {
        name: name.into(),
        element,
        level: BATTLE_LEVEL,
        hp,
        max_hp: hp,
        atk: stats.atk,
        atk_boost: 0,
        def: stats.def,
        spd: stats.spd,
        elemental_atk: stats.elemental_atk,
        poise: stats.poise,
        healer: false,
        support_atk_boost: false,
        abilities: AbilityLoadout {
            basic: "Strike",
            skill: "Elemental Assault",
            ultimate: "Monstrous Onslaught",
            ultimate_wait: 3,
        },
        cooldowns: [0; 3],
        weapon_level: None,
        poison_turns: 0,
        defending: false,
    }
}

fn action_index(action: BattleAction) -> usize {
    match action {
        BattleAction::Basic => 0,
        BattleAction::Skill => 1,
        BattleAction::Ultimate => 2,
    }
}

fn projected_weapon_stats(mut stats: Stats, level: u8) -> Stats {
    let scale = 100_u32 + u32::from(level);
    let projected = |value: u16| (u32::from(value) * scale / 100).min(u32::from(u16::MAX)) as u16;
    stats.atk = projected(stats.atk);
    stats.elemental_atk = projected(stats.elemental_atk);
    stats.crit_rate = projected(stats.crit_rate);
    stats.crit_dmg = projected(stats.crit_dmg);
    stats
}

pub fn ability_loadout(name: &str) -> AbilityLoadout {
    let (basic, skill, ultimate, ultimate_wait) = match name {
        "Astraea, Starbound" => ("Frost Sigil", "Astral Rime", "Throne of Falling Stars", 4),
        "Kaelis, Ashen Vanguard" => ("Ashen Cut", "Cinder March", "Vanguard's Last Dawn", 3),
        "Seraphine, Verdant Oracle" => ("Thornshot", "Oracle's Bloom", "Worldroot Revelation", 4),
        "Vaughn, Violet Oath" => (
            "Oathbreaker",
            "Violet Tempest",
            "Knight of the Black Storm",
            5,
        ),
        "Steven, Azure Shade" => ("Shadeflame", "Wick's Ambush", "Azure Inferno", 4),
        "Sergei, Winterfang" => ("Frost Pounce", "Wolfguard Rush", "White Fang Revenant", 4),
        "Saif, Dune Sovereign" => ("Dune Thrust", "Sirocco Spiral", "Sovereign Sandstorm", 4),
        "Yeoungin, Winter's Grace" => (
            "Winter Edict",
            "Quiet Mercy",
            "Benediction of Endless Rime",
            5,
        ),
        "Orin, Keeper of Embers" => ("Ember Hew", "Keeper's Pyre", "Undying Hearth", 4),
        "Veyra, Stormseeker" => ("Storm Arrow", "Skybreak Volley", "Eye of the Tempest", 4),
        "Cinder, Forgeheart" => ("Forge Jab", "Meteor Clinch", "Heart of the Furnace", 4),
        "Pyrite, Gilded Step" => ("Gilded Cut", "Faultline Dash", "Golden Afterimage", 3),
        "Jeanette, Tidemender" => ("Silver Current", "Kindly Tide", "Grand Marée", 5),
        "Mira" => ("Tide Script", "Restoring Ripple", "Moonlit Deluge", 4),
        "Lumen" => ("Stoneflash", "Prismatic Ward", "Crown of Living Gold", 4),
        "Thorne" => ("Briar Point", "Root Snare", "Oldwood Dominion", 4),
        "Farah" => ("Quartz Spark", "Oasis Bulwark", "Citadel of Sand", 4),
        "Anya" => ("Rime Edge", "Glacial Feint", "Silent Whiteout", 3),
        "Rook" => ("Volt Knuckle", "Thunder Counter", "Storm-Crowned Gambit", 3),
        "Kestrel" => ("Gale Shot", "Crosswind Mark", "Horizon Breaker", 3),
        "Mako" => ("Riptide Fang", "Undertow Rush", "Abyssal Twinwake", 3),
        "Ysra" => ("Coalpoint", "Flare Lunge", "Crimson Procession", 4),
        "Dolma" => ("Granite Hew", "Mountain Stance", "Unmoving World", 5),
        "Corvin" => ("Icebreaker", "Permafrost Hook", "Siberian Maelstrom", 4),
        "Zephra" => ("Gale Palm", "Vacuum Step", "Heaven-Splitting Cyclone", 3),
        "Neris" => ("Cold Reap", "Gravefrost Arc", "Requiem of Pale Winter", 4),
        "Brikka" => ("Cinder Fang", "Blazing Cross", "Twin Comet Rupture", 3),
        "Seo-yeon" => (
            "Ward Spark",
            "Measured Thunder",
            "Edict of the Violet Court",
            4,
        ),
        "Ji-ho" => ("Ember Guard", "Hearthline Draw", "Oathfire Procession", 4),
        _ => ("Basic Strike", "Elemental Art", "Final Resonance", 4),
    };
    AbilityLoadout {
        basic,
        skill,
        ultimate,
        ultimate_wait,
    }
}

fn living_index(units: &[BattleUnit], selected: usize) -> Option<usize> {
    units
        .iter()
        .enumerate()
        .filter(|(_, unit)| unit.alive())
        .nth(selected)
        .map(|(index, _)| index)
}

fn damage(power: u16, defense: u16, poise: u16, defending: bool) -> i32 {
    let mitigation = defense as i32 + if defending { poise as i32 } else { 0 };
    let mut dealt = power as i32 * 400 / (300 + mitigation);
    if defending {
        dealt = dealt * 2 / 3;
    }
    dealt.max(1)
}

pub fn effectiveness(attacker: &str, defender: &str) -> i32 {
    if ELEMENT_MATCHUPS
        .iter()
        .any(|(strong, weak)| *strong == attacker && *weak == defender)
    {
        2
    } else {
        1
    }
}

pub const ELEMENT_MATCHUPS: [(&str, &str); 10] = [
    ("Pyro", "Dendro"),
    ("Pyro", "Cryo"),
    ("Hydro", "Pyro"),
    ("Hydro", "Geo"),
    ("Electro", "Hydro"),
    ("Cryo", "Electro"),
    ("Anemo", "Geo"),
    ("Geo", "Electro"),
    ("Dendro", "Hydro"),
    ("Dendro", "Geo"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elemental_matchups_apply_only_to_elemental_abilities() {
        assert_eq!(effectiveness("Hydro", "Pyro"), 2);
        assert_eq!(effectiveness("Pyro", "Hydro"), 1);
        assert_eq!(effectiveness("Unaligned", "Pyro"), 1);
    }

    #[test]
    fn speed_order_is_descending_and_all_units_are_level_fifty() {
        let team = vec![
            "Pyrite, Gilded Step".into(),
            "Vaughn, Violet Oath".into(),
            "Jeanette, Tidemender".into(),
        ];
        let battle = BattleState::new(&team, &BTreeMap::new()).unwrap();
        assert!(
            battle
                .allies
                .iter()
                .chain(&battle.enemies)
                .all(|unit| unit.level == BATTLE_LEVEL)
        );
        let speeds = battle
            .order
            .iter()
            .map(|turn| battle.unit(*turn).spd)
            .collect::<Vec<_>>();
        assert!(speeds.windows(2).all(|pair| pair[0] >= pair[1]));
    }

    #[test]
    fn defend_uses_defense_and_poise_to_reduce_damage() {
        let normal = damage(120, 90, 100, false);
        let guarded = damage(120, 90, 100, true);
        assert!(guarded < normal);
    }

    #[test]
    fn jeanette_skill_opens_living_ally_targeting() {
        let team = vec![
            "Pyrite, Gilded Step".into(),
            "Vaughn, Violet Oath".into(),
            "Jeanette, Tidemender".into(),
        ];
        let mut battle = BattleState::new(&team, &BTreeMap::new()).unwrap();
        battle.turn = battle
            .order
            .iter()
            .position(|turn| turn.ally && turn.index == 2)
            .unwrap();
        battle.cursor = 1;
        battle.confirm();
        assert_eq!(battle.menu, BattleMenu::AllyTarget(BattleAction::Skill));
        assert_eq!(battle.target_count(), 3);
    }

    #[test]
    fn yeoungin_skill_heals_and_grants_an_attack_boost() {
        let team = vec![
            "Pyrite, Gilded Step".into(),
            "Vaughn, Violet Oath".into(),
            "Yeoungin, Winter's Grace".into(),
        ];
        let mut battle = BattleState::new(&team, &BTreeMap::new()).unwrap();
        battle.allies[0].hp -= 300;
        battle.turn = battle
            .order
            .iter()
            .position(|turn| turn.ally && turn.index == 2)
            .unwrap();
        battle.cursor = 1;
        battle.confirm();
        battle.cursor = 0;
        battle.confirm();
        assert!(battle.allies[0].hp > battle.allies[0].max_hp - 300);
        assert!(battle.allies[0].atk_boost >= 20);
    }

    #[test]
    fn every_character_has_a_named_three_ability_loadout() {
        for character in crate::simulation::all_characters() {
            let loadout = ability_loadout(character.name);
            assert_ne!(loadout.basic, "Basic Strike", "{}", character.name);
            assert_ne!(loadout.skill, "Elemental Art", "{}", character.name);
            assert_ne!(loadout.ultimate, "Final Resonance", "{}", character.name);
            assert!((3..=5).contains(&loadout.ultimate_wait));
        }
    }

    #[test]
    fn skill_waits_one_owner_turn_and_ultimate_uses_variable_wait() {
        let team = vec![
            "Pyrite, Gilded Step".into(),
            "Vaughn, Violet Oath".into(),
            "Jeanette, Tidemender".into(),
        ];
        let mut battle = BattleState::new(&team, &BTreeMap::new()).unwrap();
        battle.turn = battle
            .order
            .iter()
            .position(|turn| turn.ally && turn.index == 0)
            .unwrap();
        battle.cursor = 1;
        battle.confirm();
        battle.confirm();
        assert_eq!(battle.allies[0].cooldowns[1], 2);
        assert_eq!(battle.allies[0].abilities.ultimate_wait, 3);
    }

    #[test]
    fn equipped_weapons_are_projected_to_level_fifty_and_raise_damage_stats() {
        let team = vec![
            "Pyrite, Gilded Step".into(),
            "Vaughn, Violet Oath".into(),
            "Jeanette, Tidemender".into(),
        ];
        let without = BattleState::new(&team, &BTreeMap::new()).unwrap();
        let equipment = BTreeMap::from([("Pyrite, Gilded Step".into(), "Dawncool Steel".into())]);
        let with = BattleState::new(&team, &equipment).unwrap();
        assert_eq!(with.allies[0].weapon_level, Some(BATTLE_LEVEL));
        assert!(with.allies[0].atk > without.allies[0].atk);
        assert!(with.allies[0].elemental_atk > without.allies[0].elemental_atk);
        assert_eq!(
            with.allies[0].atk - without.allies[0].atk,
            projected_weapon_stats(
                catalog_item("Dawncool Steel").unwrap().stats(),
                BATTLE_LEVEL
            )
            .atk
        );
    }

    #[test]
    fn enemy_targeting_considers_health_guard_and_rotation() {
        let team = vec![
            "Pyrite, Gilded Step".into(),
            "Vaughn, Violet Oath".into(),
            "Jeanette, Tidemender".into(),
        ];
        let mut battle = BattleState::new(&team, &BTreeMap::new()).unwrap();
        battle.allies[2].hp /= 4;
        assert_eq!(battle.choose_enemy_target(0), Some(2));
        battle.allies[2].defending = true;
        battle.allies[1].hp /= 3;
        assert_eq!(battle.choose_enemy_target(0), Some(1));
    }

    #[test]
    fn thornbloom_can_apply_three_turn_poison() {
        let team = vec![
            "Pyrite, Gilded Step".into(),
            "Vaughn, Violet Oath".into(),
            "Jeanette, Tidemender".into(),
        ];
        let mut battle = BattleState::new(&team, &BTreeMap::new()).unwrap();
        let (round, target) = (1..=6)
            .find_map(|round| {
                battle.round = round;
                let target = battle.choose_enemy_target(1)?;
                (usize::from(round) + 1 + target)
                    .is_multiple_of(3)
                    .then_some((round, target))
            })
            .unwrap();
        battle.round = round;
        battle.enemy_action(1);
        assert_eq!(battle.allies[target].poison_turns, 3);
    }
}
