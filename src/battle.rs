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
pub enum BattleEncounter {
    RuinCourt,
    SomnialFrostwyrm,
    MadGoliath,
}

impl BattleEncounter {
    pub const ALL: [Self; 3] = [Self::RuinCourt, Self::SomnialFrostwyrm, Self::MadGoliath];

    pub const fn name(self) -> &'static str {
        match self {
            Self::RuinCourt => "Ruin Court",
            Self::SomnialFrostwyrm => "Somnial Frostwyrm",
            Self::MadGoliath => "Mad Goliath",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::RuinCourt => "Three-enemy boss formation with healing, poison, and Ruin Nova.",
            Self::SomnialFrostwyrm => {
                "Extreme solo boss: freezing breath, drowsiness, and escalating damage."
            }
            Self::MadGoliath => {
                "Extreme Geo siege: a barrier boss that summons and revives two Shardlings."
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BattleEffect {
    Attack,
    Heal,
    Shield,
    Status,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CombatRole {
    Striker,
    Healer,
    Guardian,
    Buffer,
    Battery,
    Hybrid,
}

impl CombatRole {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Striker => "STRIKER",
            Self::Healer => "HEALER",
            Self::Guardian => "GUARDIAN",
            Self::Buffer => "BUFFER",
            Self::Battery => "BATTERY",
            Self::Hybrid => "HYBRID SUPPORT",
        }
    }
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
    pub crit_rate: u16,
    pub crit_dmg: u16,
    pub support_atk_boost: bool,
    pub role: CombatRole,
    pub abilities: AbilityLoadout,
    pub bp: u8,
    pub shield: i32,
    pub weapon_level: Option<u8>,
    pub poison_turns: u8,
    pub burn_turns: u8,
    pub frozen_turns: u8,
    pub paralysis_turns: u8,
    pub drowsy_turns: u8,
    pub stun_turns: u8,
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
    pub encounter: BattleEncounter,
    pub visual_effect: Option<(BattleEffect, bool, usize, u8)>,
    pub show_history: bool,
    goliath_barrier_lock_turns: u8,
    shardling_defeated_round: [Option<u16>; 3],
    prepared_turn: Option<(u16, TurnRef)>,
}

impl BattleState {
    #[cfg(test)]
    pub fn new(team: &[String], equipment: &BTreeMap<String, String>) -> Option<Self> {
        Self::new_for(team, equipment, BattleEncounter::RuinCourt)
    }

    pub fn new_for(
        team: &[String],
        equipment: &BTreeMap<String, String>,
        encounter: BattleEncounter,
    ) -> Option<Self> {
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
        let enemies = match encounter {
            BattleEncounter::RuinCourt => vec![
                enemy_unit(
                    "Ember Wisp",
                    "Pyro",
                    Stats {
                        crit_dmg: 100,
                        crit_rate: 0,
                        atk: 104,
                        def: 78,
                        spd: 112,
                        elemental_atk: 126,
                        hp: 920,
                        poise: 62,
                    },
                ),
                enemy_unit(
                    "Astral Ruin Knight",
                    "Electro",
                    Stats {
                        crit_dmg: 100,
                        crit_rate: 0,
                        atk: 158,
                        def: 148,
                        spd: 65,
                        elemental_atk: 172,
                        hp: 3600,
                        poise: 180,
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
            ],
            BattleEncounter::SomnialFrostwyrm => vec![enemy_unit(
                "Somnial Frostwyrm",
                "Cryo",
                Stats {
                    crit_dmg: 170,
                    crit_rate: 20,
                    atk: 235,
                    def: 190,
                    spd: 104,
                    elemental_atk: 245,
                    hp: 7200,
                    poise: 240,
                },
            )],
            BattleEncounter::MadGoliath => vec![
                enemy_unit(
                    "Goliath Shardling",
                    "Geo",
                    Stats {
                        crit_dmg: 130,
                        crit_rate: 10,
                        atk: 145,
                        def: 150,
                        spd: 92,
                        elemental_atk: 135,
                        hp: 1750,
                        poise: 155,
                    },
                ),
                enemy_unit(
                    "Mad Goliath",
                    "Geo",
                    Stats {
                        crit_dmg: 185,
                        crit_rate: 25,
                        atk: 285,
                        def: 235,
                        spd: 72,
                        elemental_atk: 260,
                        hp: 10_500,
                        poise: 300,
                    },
                ),
                enemy_unit(
                    "Goliath Shardling",
                    "Geo",
                    Stats {
                        crit_dmg: 130,
                        crit_rate: 10,
                        atk: 145,
                        def: 150,
                        spd: 88,
                        elemental_atk: 135,
                        hp: 1750,
                        poise: 155,
                    },
                ),
            ],
        };
        let mut state = Self {
            allies,
            enemies,
            order: Vec::new(),
            turn: 0,
            round: 1,
            menu: BattleMenu::Action,
            cursor: 0,
            log: vec![format!("{}: {}", encounter.name(), encounter.description())],
            outcome: None,
            encounter,
            visual_effect: None,
            show_history: false,
            goliath_barrier_lock_turns: 0,
            shardling_defeated_round: [None; 3],
            prepared_turn: None,
        };
        state.rebuild_order();
        state.advance_enemy_turns();
        state.prepare_current_ally();
        Some(state)
    }

    pub fn tick_visual(&mut self) {
        if let Some((kind, ally, index, ticks)) = self.visual_effect {
            self.visual_effect = (ticks > 1).then_some((kind, ally, index, ticks - 1));
        }
    }

    pub fn toggle_history(&mut self) {
        self.show_history = !self.show_history;
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
                        && self
                            .current_ally()
                            .is_some_and(|unit| !matches!(unit.role, CombatRole::Striker))
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

    pub fn action_ready(&self, action: BattleAction) -> bool {
        self.current_ally()
            .is_some_and(|unit| unit.bp >= action_bp_cost(action, unit.abilities))
    }

    pub fn action_description(&self, action: BattleAction) -> &'static str {
        let Some(unit) = self.current_ally() else {
            return "";
        };
        ability_description(unit.role, action)
    }

    pub fn action_cost(&self, action: BattleAction) -> u8 {
        self.current_ally()
            .map_or(0, |unit| action_bp_cost(action, unit.abilities))
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
        self.spend_action_resources(actor.index, action);
        let defender = &mut self.enemies[target];
        let shield_before = defender.shield;
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
        let critical = (usize::from(self.round) * 17 + actor.index * 23 + target * 11) % 100
            < usize::from(attacker.crit_rate.min(75));
        let mut damage =
            damage(scaled_power, defender.def, defender.poise, false) * element_multiplier;
        if critical {
            damage = damage * i32::from(attacker.crit_dmg.max(100)) / 100;
        }
        let barrier_weakness =
            shield_before > 0 && !matches!(action, BattleAction::Basic) && element_multiplier == 2;
        if barrier_weakness {
            damage = damage.saturating_mul(2);
        }
        let absorbed = damage.min(defender.shield);
        defender.shield -= absorbed;
        damage -= absorbed;
        defender.hp = (defender.hp - damage).max(0);
        self.visual_effect = Some((BattleEffect::Attack, false, target, 28));
        let defender_name = defender.name.clone();
        let barrier_broken =
            defender_name == "Mad Goliath" && shield_before > 0 && defender.shield == 0;
        if defender.hp == 0 && defender_name == "Goliath Shardling" {
            self.shardling_defeated_round[target] = Some(self.round);
        }
        if !matches!(action, BattleAction::Basic) {
            self.try_inflict_status(attacker.element, target, action);
        }
        let effective = if element_multiplier == 2 {
            "  •  2× EFFECTIVE"
        } else {
            ""
        };
        self.push_log(format!(
            "{} used {label} [{}] on {} for {damage} damage{}{}.",
            attacker.name,
            if matches!(action, BattleAction::Basic) {
                "Physical"
            } else {
                attacker.element
            },
            defender_name,
            effective,
            if critical { "  •  CRITICAL" } else { "" },
        ));
        if absorbed > 0 {
            self.push_log(format!(
                "{defender_name}'s barrier absorbed {absorbed} damage.{}",
                if barrier_weakness {
                    "  Geo barrier weakness exploited."
                } else {
                    ""
                }
            ));
        }
        if barrier_broken {
            self.enemies[target].stun_turns = 3;
            self.goliath_barrier_lock_turns = 5;
            self.push_log(
                "Mad Goliath's Geo barrier shattered. It is STUNNED for 3 turns and cannot rebuild its barrier for 5 turns."
                    .into(),
            );
        }
        self.finish_turn();
    }

    fn perform_heal(&mut self, action: BattleAction, target: usize) {
        let Some(actor) = self.current() else { return };
        let healer = self.allies[actor.index].clone();
        self.allies[actor.index].defending = false;
        self.spend_action_resources(actor.index, action);
        let (label, scale) = match action {
            BattleAction::Skill => (healer.abilities.skill, 3),
            BattleAction::Ultimate => (healer.abilities.ultimate, 5),
            BattleAction::Basic => (healer.abilities.basic, 1),
        };
        let amount = (healer.elemental_atk as i32 * scale).max(180);
        let ally = &mut self.allies[target];
        let restored = amount.min(ally.max_hp - ally.hp).max(0);
        ally.hp += restored;
        let boost = if healer.support_atk_boost
            || matches!(healer.role, CombatRole::Buffer | CombatRole::Hybrid)
        {
            (healer.elemental_atk / 5).max(20)
        } else {
            0
        };
        ally.atk_boost = ally.atk_boost.max(boost);
        if matches!(healer.role, CombatRole::Guardian | CombatRole::Hybrid) {
            ally.shield = ally
                .shield
                .max(i32::from(healer.def + healer.poise) * scale / 2);
        }
        if matches!(healer.role, CombatRole::Battery | CombatRole::Hybrid) {
            ally.bp = ally
                .bp
                .saturating_add(if healer.role == CombatRole::Battery {
                    2
                } else {
                    1
                })
                .min(7);
        }
        let ally_name = ally.name.clone();
        self.visual_effect = Some((
            if ally.shield > 0 {
                BattleEffect::Shield
            } else {
                BattleEffect::Heal
            },
            true,
            target,
            36,
        ));
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

    fn spend_action_resources(&mut self, ally_index: usize, action: BattleAction) {
        let unit = &mut self.allies[ally_index];
        unit.bp = unit
            .bp
            .saturating_sub(action_bp_cost(action, unit.abilities));
        match action {
            BattleAction::Basic => {
                unit.bp = unit.bp.saturating_add(1).min(7);
            }
            BattleAction::Skill | BattleAction::Ultimate => {}
        }
    }

    fn perform_defend(&mut self) {
        let Some(actor) = self.current() else { return };
        let name = self.allies[actor.index].name.clone();
        self.allies[actor.index].defending = true;
        self.allies[actor.index].bp = self.allies[actor.index].bp.saturating_add(2).min(7);
        self.visual_effect = Some((BattleEffect::Shield, true, actor.index, 36));
        self.push_log(format!("{name} braces: +2 BP and 10% damage reduction."));
        self.finish_turn();
    }

    fn enemy_action(&mut self, enemy_index: usize) {
        if self.resolve_enemy_status(enemy_index) {
            return;
        }
        let Some(target) = self.choose_enemy_target(enemy_index) else {
            return;
        };
        let enemy = self.enemies[enemy_index].clone();
        if enemy.name == "Mad Goliath" && self.goliath_action() {
            return;
        }
        let uses_poison = enemy.name == "Thornbloom"
            && (usize::from(self.round) + enemy_index + target).is_multiple_of(3);
        let boss_burst = enemy.name == "Astral Ruin Knight" && self.round.is_multiple_of(3);
        let wisp_support = enemy.name == "Ember Wisp" && self.round.is_multiple_of(2);
        if wisp_support
            && let Some(boss) = self
                .enemies
                .iter_mut()
                .find(|unit| unit.name == "Astral Ruin Knight" && unit.alive())
        {
            let restored = 180.min(boss.max_hp - boss.hp).max(0);
            boss.hp += restored;
            boss.atk_boost = boss.atk_boost.max(24);
            self.push_log(format!(
                "Ember Wisp fed the ruin core: the boss restored {restored} HP and gained ATK."
            ));
            return;
        }
        let power = if uses_poison {
            enemy.elemental_atk
        } else {
            enemy.atk
        };
        let ally = &mut self.allies[target];
        let mut dealt = damage(
            power.saturating_add(enemy.atk_boost),
            ally.def,
            ally.poise,
            ally.defending,
        );
        if boss_burst {
            dealt = dealt * 3 / 2;
        }
        let absorbed = dealt.min(ally.shield);
        ally.shield -= absorbed;
        let hp_damage = dealt - absorbed;
        ally.hp = (ally.hp - hp_damage).max(0);
        let applied_poison = uses_poison && ally.alive() && ally.element != "Dendro";
        if applied_poison {
            ally.poison_turns = 3;
        }
        if enemy.name == "Somnial Frostwyrm" && ally.alive() {
            if self.round.is_multiple_of(4) && ally.element != "Cryo" {
                ally.frozen_turns = 1;
            } else if self.round.is_multiple_of(3) && ally.element != "Dendro" {
                ally.drowsy_turns = 2;
            }
        }
        let ally_name = ally.name.clone();
        let defended = ally.defending;
        self.push_log(if applied_poison {
            format!(
                "{} poisoned {} for {hp_damage} damage (3 turns){}.",
                enemy.name,
                ally_name,
                if defended { " through DEFEND" } else { "" }
            )
        } else {
            format!(
                "{} {} {} for {hp_damage} damage{}{}.",
                enemy.name,
                if boss_burst {
                    "unleashed RUIN NOVA on"
                } else {
                    "struck"
                },
                ally_name,
                if defended { " through DEFEND" } else { "" },
                if absorbed > 0 {
                    " (shield absorbed part)"
                } else {
                    ""
                },
            )
        });
        self.visual_effect = Some((BattleEffect::Attack, true, target, 28));
    }

    fn goliath_action(&mut self) -> bool {
        let Some(boss_index) = self
            .enemies
            .iter()
            .position(|unit| unit.name == "Mad Goliath")
        else {
            return false;
        };
        if self.goliath_barrier_lock_turns > 0 {
            self.goliath_barrier_lock_turns -= 1;
        }
        if self.enemies[boss_index].stun_turns > 0 {
            self.enemies[boss_index].stun_turns -= 1;
            let remaining = self.enemies[boss_index].stun_turns;
            self.push_log(format!(
                "Mad Goliath is STUNNED and loses its action ({remaining} turns remain)."
            ));
            return true;
        }
        if self.round.is_multiple_of(3) {
            let mut restored = 0;
            for (index, unit) in self.enemies.iter_mut().enumerate() {
                let waited_long_enough = self.shardling_defeated_round[index]
                    .is_some_and(|death_round| self.round.saturating_sub(death_round) >= 5);
                if index != boss_index
                    && unit.name == "Goliath Shardling"
                    && !unit.alive()
                    && waited_long_enough
                {
                    unit.hp = unit.max_hp / 2;
                    self.shardling_defeated_round[index] = None;
                    restored += 1;
                }
            }
            if restored > 0 {
                self.push_log(format!(
                    "Mad Goliath rebuilt {restored} fallen Shardling{}.",
                    if restored == 1 { "" } else { "s" }
                ));
                self.rebuild_order();
                return true;
            }
        }
        if self.round.is_multiple_of(2)
            && self.goliath_barrier_lock_turns == 0
            && self.enemies[boss_index].shield == 0
        {
            let barrier = 1_000;
            let boss = &mut self.enemies[boss_index];
            boss.shield = barrier;
            let total = boss.shield;
            self.visual_effect = Some((BattleEffect::Shield, false, boss_index, 36));
            self.push_log(format!(
                "Mad Goliath raised a {barrier}-point Geo barrier ({} total).",
                total
            ));
            return true;
        }
        false
    }

    fn try_inflict_status(&mut self, element: &str, target: usize, action: BattleAction) {
        let unit = &mut self.enemies[target];
        let chance = (usize::from(self.round) * 19 + target * 13 + action_index(action) * 29) % 100;
        let applied = match element {
            "Pyro" if unit.element != "Pyro" && chance < 55 => {
                unit.burn_turns = 3;
                Some("BURN")
            }
            "Cryo" if unit.element != "Cryo" && chance < 45 => {
                unit.frozen_turns = 1;
                Some("FROZEN")
            }
            "Electro" if unit.element != "Electro" && chance < 40 => {
                unit.paralysis_turns = 1;
                Some("PARALYSIS")
            }
            "Dendro" if unit.element != "Dendro" && chance < 45 => {
                unit.drowsy_turns = 2;
                Some("DROWSY")
            }
            _ => None,
        };
        if let Some(status) = applied {
            let name = unit.name.clone();
            self.push_log(format!("{name} is afflicted with {status}."));
            self.visual_effect = Some((BattleEffect::Status, false, target, 36));
        }
    }

    fn resolve_enemy_status(&mut self, enemy_index: usize) -> bool {
        let burn_event = {
            let enemy = &mut self.enemies[enemy_index];
            if enemy.burn_turns > 0 {
                let damage = (enemy.max_hp / 16).max(1);
                enemy.hp = (enemy.hp - damage).max(0);
                enemy.burn_turns -= 1;
                Some((enemy.name.clone(), damage))
            } else {
                None
            }
        };
        if let Some((name, damage)) = burn_event {
            self.push_log(format!("{name} suffered {damage} BURN damage."));
        }
        if !self.enemies[enemy_index].alive() {
            if self.enemies[enemy_index].name == "Goliath Shardling" {
                self.shardling_defeated_round[enemy_index] = Some(self.round);
            }
            return true;
        }
        let enemy = &mut self.enemies[enemy_index];
        let skipped = if enemy.frozen_turns > 0 {
            enemy.frozen_turns -= 1;
            Some("FROZEN")
        } else if enemy.paralysis_turns > 0 {
            enemy.paralysis_turns -= 1;
            Some("PARALYZED")
        } else if enemy.drowsy_turns > 0 {
            enemy.drowsy_turns -= 1;
            (enemy.drowsy_turns == 0).then_some("DROWSY")
        } else {
            None
        };
        if let Some(status) = skipped {
            let name = enemy.name.clone();
            self.push_log(format!("{name} lost its action while {status}."));
            return true;
        }
        false
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
            unit.defending = false;
            if unit.burn_turns > 0 {
                let burn_damage = (unit.max_hp / 16).max(1);
                unit.hp = (unit.hp - burn_damage).max(0);
                unit.burn_turns -= 1;
            }
            if unit.frozen_turns > 0 || unit.paralysis_turns > 0 {
                unit.frozen_turns = unit.frozen_turns.saturating_sub(1);
                unit.paralysis_turns = unit.paralysis_turns.saturating_sub(1);
                let name = unit.name.clone();
                self.push_log(format!("{name} cannot act due to a disabling status."));
                self.advance_turn();
                self.advance_enemy_turns();
                continue;
            }
            if unit.drowsy_turns > 0 {
                unit.drowsy_turns -= 1;
                if unit.drowsy_turns == 0 {
                    let name = unit.name.clone();
                    self.push_log(format!("{name} fell asleep and lost their action."));
                    self.advance_turn();
                    self.advance_enemy_turns();
                    continue;
                }
            }
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
        if self.log.len() > 100 {
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
        crit_rate: stats.crit_rate,
        crit_dmg: stats.crit_dmg,
        support_atk_boost: name == "Yeoungin, Winter's Grace",
        role: combat_role(name),
        abilities: ability_loadout(name),
        bp: 0,
        shield: 0,
        weapon_level: weapon_name.map(|_| BATTLE_LEVEL),
        poison_turns: 0,
        burn_turns: 0,
        frozen_turns: 0,
        paralysis_turns: 0,
        drowsy_turns: 0,
        stun_turns: 0,
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
        crit_rate: stats.crit_rate,
        crit_dmg: stats.crit_dmg,
        support_atk_boost: false,
        role: CombatRole::Striker,
        abilities: AbilityLoadout {
            basic: "Strike",
            skill: "Elemental Assault",
            ultimate: "Monstrous Onslaught",
            ultimate_wait: 3,
        },
        bp: 0,
        shield: match name {
            "Astral Ruin Knight" => 700,
            "Mad Goliath" => 2_400,
            _ => 0,
        },
        weapon_level: None,
        poison_turns: 0,
        burn_turns: 0,
        frozen_turns: 0,
        paralysis_turns: 0,
        drowsy_turns: 0,
        stun_turns: 0,
        defending: false,
    }
}

pub fn combat_role(name: &str) -> CombatRole {
    match name {
        "Jeanette, Tidemender" | "Mira" => CombatRole::Healer,
        "Lumen" | "Farah" | "Dolma" | "Ji-ho" => CombatRole::Guardian,
        "Kestrel" | "Brikka" | "Ysra" | "Taisia" => CombatRole::Buffer,
        "Seo-yeon" => CombatRole::Battery,
        "Yeoungin, Winter's Grace" | "Thorne" | "Corvin" | "Zephra" => CombatRole::Hybrid,
        _ => CombatRole::Striker,
    }
}

pub const fn action_bp_cost(action: BattleAction, abilities: AbilityLoadout) -> u8 {
    match action {
        BattleAction::Basic => 0,
        BattleAction::Skill => 2,
        BattleAction::Ultimate => match abilities.ultimate_wait {
            0..=3 => 4,
            4 => 5,
            _ => 6,
        },
    }
}

pub const fn ability_description(role: CombatRole, action: BattleAction) -> &'static str {
    match (role, action) {
        (_, BattleAction::Basic) => "Physical strike. Generates 1 BP.",
        (CombatRole::Striker, BattleAction::Skill) => {
            "Elemental attack with strong matchup and weapon scaling."
        }
        (CombatRole::Striker, BattleAction::Ultimate) => {
            "High-impact elemental finisher with boosted critical potential."
        }
        (CombatRole::Healer, BattleAction::Skill) => "Restore one ally's HP.",
        (CombatRole::Healer, BattleAction::Ultimate) => "Deliver a powerful emergency heal.",
        (CombatRole::Guardian, BattleAction::Skill) => "Grant an ally a DEF/POISE-scaled shield.",
        (CombatRole::Guardian, BattleAction::Ultimate) => {
            "Massive heal and reinforced shield for the selected ally."
        }
        (CombatRole::Buffer, BattleAction::Skill) => "Empower an ally with an ATK increase.",
        (CombatRole::Buffer, BattleAction::Ultimate) => {
            "Large heal and ATK increase for a chosen carry."
        }
        (CombatRole::Battery, BattleAction::Skill) => "Feed an ally 2 BP.",
        (CombatRole::Battery, BattleAction::Ultimate) => {
            "Restore HP while rapidly charging an ally's resources."
        }
        (CombatRole::Hybrid, BattleAction::Skill) => {
            "Heal, shield, and charge an ally for flexible support."
        }
        (CombatRole::Hybrid, BattleAction::Ultimate) => {
            "A potent recovery package with shield and resource support."
        }
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
        "Klara, Jade Tempest" => (
            "Whitewind Sweep",
            "Jade Severance",
            "Harvest of the Empty Sky",
            4,
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
        "Taisia" => (
            "Road-Bell Chime",
            "Blessing of the Open Gale",
            "Seven Winds Procession",
            4,
        ),
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
    let _ = poise;
    let mitigation = defense as i32;
    let mut dealt = power as i32 * 400 / (300 + mitigation);
    if defending {
        dealt = dealt * 9 / 10;
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
        assert_eq!(guarded, normal * 9 / 10);
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
        battle.allies[2].bp = 2;
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
        battle.allies[2].bp = 2;
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
    fn abilities_are_gated_only_by_bp() {
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
        assert!(!battle.action_ready(BattleAction::Skill));
        battle.allies[0].bp = 2;
        assert!(battle.action_ready(BattleAction::Skill));
        battle.allies[0].bp = 0;
        assert!(!battle.action_ready(BattleAction::Skill));
        battle.allies[0].bp = 2;
        assert!(battle.action_ready(BattleAction::Skill));
        assert_eq!(
            action_bp_cost(BattleAction::Ultimate, battle.allies[0].abilities),
            4
        );
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
                let target = battle.choose_enemy_target(2)?;
                (usize::from(round) + 2 + target)
                    .is_multiple_of(3)
                    .then_some((round, target))
            })
            .unwrap();
        battle.round = round;
        battle.enemy_action(2);
        assert_eq!(battle.allies[target].poison_turns, 3);
    }

    #[test]
    fn encounter_selector_supports_a_solo_extreme_boss() {
        let team = vec![
            "Pyrite, Gilded Step".into(),
            "Vaughn, Violet Oath".into(),
            "Jeanette, Tidemender".into(),
        ];
        let battle =
            BattleState::new_for(&team, &BTreeMap::new(), BattleEncounter::SomnialFrostwyrm)
                .unwrap();
        assert_eq!(battle.enemies.len(), 1);
        assert_eq!(battle.enemies[0].name, "Somnial Frostwyrm");
        assert!(battle.enemies[0].max_hp >= 7000);
    }

    #[test]
    fn matching_elements_are_immune_to_their_status_condition() {
        let team = vec![
            "Pyrite, Gilded Step".into(),
            "Vaughn, Violet Oath".into(),
            "Jeanette, Tidemender".into(),
        ];
        let mut pyro = BattleState::new(&team, &BTreeMap::new()).unwrap();
        for round in 1..=20 {
            pyro.round = round;
            pyro.try_inflict_status("Pyro", 0, BattleAction::Ultimate);
        }
        assert_eq!(pyro.enemies[0].burn_turns, 0);

        let mut cryo =
            BattleState::new_for(&team, &BTreeMap::new(), BattleEncounter::SomnialFrostwyrm)
                .unwrap();
        for round in 1..=20 {
            cryo.round = round;
            cryo.try_inflict_status("Cryo", 0, BattleAction::Ultimate);
        }
        assert_eq!(cryo.enemies[0].frozen_turns, 0);
    }

    #[test]
    fn mad_goliath_break_stuns_locks_barrier_and_delays_shardlings() {
        let team = vec![
            "Klara, Jade Tempest".into(),
            "Vaughn, Violet Oath".into(),
            "Jeanette, Tidemender".into(),
        ];
        let mut battle =
            BattleState::new_for(&team, &BTreeMap::new(), BattleEncounter::MadGoliath).unwrap();
        let boss = battle
            .enemies
            .iter()
            .position(|unit| unit.name == "Mad Goliath")
            .unwrap();
        battle.enemies[boss].shield = 100;
        battle.turn = battle
            .order
            .iter()
            .position(|turn| turn.ally && turn.index == 0)
            .unwrap();
        battle.allies[0].bp = 2;
        battle.cursor = 1;
        battle.confirm();
        battle.cursor = 1;
        battle.confirm();
        assert_eq!(battle.enemies[boss].shield, 0);
        assert_eq!(battle.enemies[boss].stun_turns, 3);
        assert_eq!(battle.goliath_barrier_lock_turns, 5);

        battle.round = 2;
        for _ in 0..4 {
            battle.goliath_action();
            assert_eq!(battle.enemies[boss].shield, 0);
        }
        assert!(battle.goliath_action());
        assert_eq!(battle.enemies[boss].shield, 1_000);

        battle.enemies[0].hp = 0;
        battle.enemies[2].hp = 0;
        battle.shardling_defeated_round[0] = Some(1);
        battle.shardling_defeated_round[2] = Some(1);
        battle.round = 3;
        battle.goliath_action();
        assert_eq!(battle.enemies[0].hp, 0);
        assert_eq!(battle.enemies[2].hp, 0);
        battle.round = 6;
        assert!(battle.goliath_action());
        assert!(battle.enemies[0].hp > 0);
        assert!(battle.enemies[2].hp > 0);
    }
}
