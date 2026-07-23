use std::{
    collections::BTreeSet,
    time::{Duration, Instant},
};

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::{
    art::CharacterGallery,
    battle::{BattleMenu, BattleState},
    kitty::GraphicsRenderer,
    model::{Banner, SaveData, WeaponPath, WishResult},
    simulation::WishEngine,
    storage, ui,
};

const FRAME_TIME: Duration = Duration::from_millis(33);
const FLIGHT_TIME: Duration = Duration::from_millis(1_650);
const FIVE_STAR_INTRO_TIME: Duration = Duration::from_millis(3_400);

pub struct App {
    pub save: SaveData,
    pub phase: Phase,
    pub graphics: bool,
    pub banner: Banner,
    pub gallery: CharacterGallery,
    pub confirm_quit: bool,
    pub inventory_sort: InventorySort,
    pub inventory_kind: InventoryKind,
    pub inventory_element: usize,
    engine: WishEngine,
    should_quit: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum InventorySort {
    Name,
    Rarity,
    Kind,
    Element,
}
impl InventorySort {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Name => "NAME A–Z",
            Self::Rarity => "RARITY 5★–3★",
            Self::Kind => "ITEM TYPE",
            Self::Element => "ELEMENT",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryKind {
    All,
    Character,
    Weapon,
}
impl InventoryKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "ALL ITEMS",
            Self::Character => "CHARACTERS",
            Self::Weapon => "WEAPONS",
        }
    }
}
pub const ELEMENT_FILTERS: [&str; 9] = [
    "ALL",
    "PYRO",
    "HYDRO",
    "ELECTRO",
    "CRYO",
    "ANEMO",
    "GEO",
    "DENDRO",
    "UNALIGNED",
];

pub enum Phase {
    MainMenu {
        cursor: usize,
    },
    Home,
    Teams {
        team: usize,
        slot: usize,
        editing_name: bool,
    },
    TeamCharacterSelect {
        team: usize,
        slot: usize,
        cursor: usize,
    },
    Characters {
        cursor: usize,
    },
    CharacterQuickSelect {
        cursor: usize,
        rarity: u8,
        element: usize,
        weapon: usize,
    },
    CharacterWeaponSelect {
        character_cursor: usize,
        weapon_cursor: usize,
    },
    BattleTeamSelect {
        cursor: usize,
    },
    Battle {
        state: BattleState,
    },
    Tutorial,
    BannerSelect {
        cursor: usize,
    },
    WeaponSelect {
        cursor: usize,
        preview: bool,
    },
    CharacterArchive {
        cursor: usize,
    },
    History,
    Inventory {
        cursor: usize,
        selected: BTreeSet<String>,
    },
    InventoryDetail {
        cursor: usize,
        selected: BTreeSet<String>,
        name: String,
    },
    ConfirmInventoryDelete {
        cursor: usize,
        selected: BTreeSet<String>,
        targets: Vec<String>,
    },
    Flight {
        started: Instant,
        results: Vec<WishResult>,
    },
    Reveal {
        started: Instant,
        results: Vec<WishResult>,
        index: usize,
        five_star_only: bool,
    },
    FiveStarIntro {
        started: Instant,
        results: Vec<WishResult>,
        index: usize,
        five_star_only: bool,
    },
    Summary {
        results: Vec<WishResult>,
        selected: usize,
    },
    Detail {
        results: Vec<WishResult>,
        selected: usize,
    },
}

pub fn run() -> Result<()> {
    let mut app = App {
        save: storage::load()?,
        phase: Phase::MainMenu { cursor: 0 },
        graphics: supports_graphics_protocol(),
        banner: Banner::Astraea,
        gallery: CharacterGallery::load()?,
        confirm_quit: false,
        inventory_sort: InventorySort::Name,
        inventory_kind: InventoryKind::All,
        inventory_element: 0,
        engine: WishEngine::random(),
        should_quit: false,
    };

    let mut graphics_renderer = GraphicsRenderer::default();
    ratatui::run(|terminal| -> Result<()> {
        while !app.should_quit {
            let now = Instant::now();
            app.advance(now);
            let mut drawn_area = ratatui::layout::Rect::default();
            terminal.draw(|frame| {
                drawn_area = frame.area();
                ui::render(frame, &app, now);
            })?;
            if app.graphics {
                graphics_renderer.sync(&ui::graphics_portraits(&app, drawn_area))?;
            }

            let timeout = if app.is_animating() {
                FRAME_TIME
            } else {
                Duration::from_millis(250)
            };
            if event::poll(timeout)?
                && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                app.handle_key(key.code)?;
            }
        }
        graphics_renderer.clear()?;
        Ok(())
    })?;
    Ok(())
}

fn supports_graphics_protocol() -> bool {
    if let Some(enabled) = graphics_override(std::env::var("WISHSIM_GRAPHICS").ok().as_deref()) {
        return enabled;
    }
    let term = std::env::var("TERM")
        .unwrap_or_default()
        .to_ascii_lowercase();
    let term_program = std::env::var("TERM_PROGRAM")
        .unwrap_or_default()
        .to_ascii_lowercase();
    detects_graphics_protocol(
        &term,
        &term_program,
        std::env::var_os("KITTY_WINDOW_ID").is_some(),
        std::env::var_os("GHOSTTY_RESOURCES_DIR").is_some()
            || std::env::var_os("GHOSTTY_BIN_DIR").is_some(),
    )
}

fn graphics_override(value: Option<&str>) -> Option<bool> {
    value.and_then(|value| match value.to_ascii_lowercase().as_str() {
        "1" | "on" | "true" | "kitty" | "ghostty" => Some(true),
        "0" | "off" | "false" | "ansi" => Some(false),
        _ => None,
    })
}

fn detects_graphics_protocol(
    term: &str,
    term_program: &str,
    kitty_window: bool,
    ghostty_environment: bool,
) -> bool {
    kitty_window
        || ghostty_environment
        || term.contains("kitty")
        || term.contains("ghostty")
        || term_program.contains("kitty")
        || term_program.contains("ghostty")
}

impl App {
    pub fn owned_character_names(&self) -> Vec<String> {
        crate::simulation::all_characters()
            .into_iter()
            .filter(|item| self.save.inventory.get(item.name).copied().unwrap_or(0) > 0)
            .map(|item| item.name.to_owned())
            .collect()
    }

    pub fn compatible_weapon_names(&self, character: &str) -> Vec<String> {
        let kind = crate::simulation::character_weapon_type(character);
        let mut names = self
            .save
            .inventory
            .iter()
            .filter_map(|(name, count)| {
                let held_elsewhere = self
                    .save
                    .equipment
                    .iter()
                    .filter(|(holder, equipped)| {
                        holder.as_str() != character && equipped.as_str() == name.as_str()
                    })
                    .count() as u32;
                crate::simulation::catalog_item(name)
                    .filter(|item| item.kind == kind && *count > held_elsewhere)
                    .map(|_| name.clone())
            })
            .collect::<Vec<_>>();
        names.sort();
        names
    }

    pub fn weapon_holders(&self, weapon: &str) -> Vec<String> {
        let mut holders = self
            .save
            .equipment
            .iter()
            .filter_map(|(character, equipped)| (equipped == weapon).then_some(character.clone()))
            .collect::<Vec<_>>();
        holders.sort();
        holders
    }

    pub fn inventory_names(&self) -> Vec<String> {
        let mut names = self
            .save
            .inventory
            .keys()
            .filter(|name| {
                let Some(item) = crate::simulation::catalog_item(name) else {
                    return true;
                };
                let kind_ok = match self.inventory_kind {
                    InventoryKind::All => true,
                    InventoryKind::Character => item.kind == "Character",
                    InventoryKind::Weapon => item.kind != "Character",
                };
                let element = ELEMENT_FILTERS[self.inventory_element];
                kind_ok && (element == "ALL" || item.element().eq_ignore_ascii_case(element))
            })
            .cloned()
            .collect::<Vec<_>>();
        names.sort_by(|a, b| {
            let aa = crate::simulation::catalog_item(a);
            let bb = crate::simulation::catalog_item(b);
            match self.inventory_sort {
                InventorySort::Name => a.cmp(b),
                InventorySort::Rarity => bb
                    .map_or(0, |i| i.rarity.value())
                    .cmp(&aa.map_or(0, |i| i.rarity.value()))
                    .then(a.cmp(b)),
                InventorySort::Kind => aa
                    .map_or("", |i| i.kind)
                    .cmp(bb.map_or("", |i| i.kind))
                    .then(a.cmp(b)),
                InventorySort::Element => aa
                    .map_or("", |i| i.element())
                    .cmp(bb.map_or("", |i| i.element()))
                    .then(a.cmp(b)),
            }
        });
        names
    }

    fn is_animating(&self) -> bool {
        match &self.phase {
            Phase::Flight { .. } | Phase::FiveStarIntro { .. } => true,
            Phase::Reveal { started, .. } => started.elapsed() < Duration::from_secs(2),
            _ => false,
        }
    }

    fn handle_key(&mut self, key: KeyCode) -> Result<()> {
        if self.confirm_quit {
            match key {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => self.should_quit = true,
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc | KeyCode::Char('q') => {
                    self.confirm_quit = false;
                }
                _ => {}
            }
            return Ok(());
        }
        if let Phase::Teams {
            team,
            editing_name: true,
            ..
        } = &mut self.phase
        {
            match key {
                KeyCode::Enter | KeyCode::Esc => {
                    storage::save(&self.save)?;
                    if let Phase::Teams { editing_name, .. } = &mut self.phase {
                        *editing_name = false;
                    }
                }
                KeyCode::Backspace => {
                    self.save.teams[*team].name.pop();
                }
                KeyCode::Char(ch)
                    if !ch.is_control() && self.save.teams[*team].name.chars().count() < 24 =>
                {
                    self.save.teams[*team].name.push(ch)
                }
                _ => {}
            }
            return Ok(());
        }
        if key == KeyCode::Char('q') {
            self.confirm_quit = true;
            return Ok(());
        }
        let inventory_names = self.inventory_names();
        let character_names = self.owned_character_names();
        let equipment_weapons = if let Phase::CharacterWeaponSelect {
            character_cursor, ..
        } = &self.phase
        {
            character_names
                .get(*character_cursor)
                .map(|name| self.compatible_weapon_names(name))
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        match (&mut self.phase, key) {
            (Phase::MainMenu { cursor }, KeyCode::Up) => *cursor = cursor.saturating_sub(1),
            (Phase::MainMenu { cursor }, KeyCode::Down) => *cursor = (*cursor + 1).min(5),
            (Phase::MainMenu { cursor }, KeyCode::Enter) => {
                self.phase = match *cursor {
                    0 => Phase::Teams {
                        team: 0,
                        slot: 0,
                        editing_name: false,
                    },
                    1 => Phase::Home,
                    2 => Phase::Inventory {
                        cursor: 0,
                        selected: BTreeSet::new(),
                    },
                    3 => Phase::Characters { cursor: 0 },
                    4 => Phase::BattleTeamSelect { cursor: 0 },
                    _ => Phase::Tutorial,
                }
            }
            (Phase::Teams { slot, .. }, KeyCode::Left) => *slot = slot.saturating_sub(1),
            (Phase::Teams { slot, .. }, KeyCode::Right) => *slot = (*slot + 1).min(2),
            (Phase::Teams { team, .. }, KeyCode::Up) => *team = team.saturating_sub(1),
            (Phase::Teams { team, .. }, KeyCode::Down) => *team = (*team + 1).min(4),
            (Phase::Teams { editing_name, .. }, KeyCode::Char('r')) => *editing_name = true,
            (Phase::Teams { team, slot, .. }, KeyCode::Enter) => {
                self.phase = Phase::TeamCharacterSelect {
                    team: *team,
                    slot: *slot,
                    cursor: 0,
                }
            }
            (Phase::Teams { team, slot, .. }, KeyCode::Char('d')) => {
                self.save.teams[*team].members[*slot] = None;
                storage::save(&self.save)?;
            }
            (Phase::Teams { .. }, KeyCode::Esc) => self.phase = Phase::MainMenu { cursor: 0 },
            (Phase::TeamCharacterSelect { cursor, .. }, KeyCode::Up) => {
                *cursor = cursor.saturating_sub(1)
            }
            (Phase::TeamCharacterSelect { cursor, .. }, KeyCode::Down) => {
                *cursor = (*cursor + 1).min(character_names.len().saturating_sub(1))
            }
            (Phase::TeamCharacterSelect { team, slot, cursor }, KeyCode::Enter) => {
                if let Some(name) = character_names.get(*cursor).cloned() {
                    for member in &mut self.save.teams[*team].members {
                        if member.as_deref() == Some(&name) {
                            *member = None;
                        }
                    }
                    self.save.teams[*team].members[*slot] = Some(name);
                    storage::save(&self.save)?;
                }
                self.phase = Phase::Teams {
                    team: *team,
                    slot: *slot,
                    editing_name: false,
                };
            }
            (Phase::TeamCharacterSelect { team, slot, .. }, KeyCode::Esc) => {
                self.phase = Phase::Teams {
                    team: *team,
                    slot: *slot,
                    editing_name: false,
                }
            }
            (Phase::Characters { cursor }, KeyCode::Left) => *cursor = cursor.saturating_sub(1),
            (Phase::Characters { cursor }, KeyCode::Right) => {
                *cursor = (*cursor + 1).min(character_names.len().saturating_sub(1))
            }
            (Phase::Characters { cursor }, KeyCode::Char('w')) if !character_names.is_empty() => {
                self.phase = Phase::CharacterWeaponSelect {
                    character_cursor: *cursor,
                    weapon_cursor: 0,
                }
            }
            (Phase::Characters { .. }, KeyCode::Char('l')) => {
                self.phase = Phase::CharacterQuickSelect {
                    cursor: 0,
                    rarity: 0,
                    element: 0,
                    weapon: 0,
                }
            }
            (
                Phase::CharacterQuickSelect {
                    cursor,
                    rarity,
                    element,
                    weapon,
                },
                key,
            ) => {
                let filtered = filtered_characters(&character_names, *rarity, *element, *weapon);
                match key {
                    KeyCode::Up => *cursor = cursor.saturating_sub(1),
                    KeyCode::Down => *cursor = (*cursor + 1).min(filtered.len().saturating_sub(1)),
                    KeyCode::Char('r') => {
                        *rarity = (*rarity + 1) % 3;
                        *cursor = 0;
                    }
                    KeyCode::Char('e') => {
                        *element = (*element + 1) % CHARACTER_ELEMENTS.len();
                        *cursor = 0;
                    }
                    KeyCode::Char('t') => {
                        *weapon = (*weapon + 1) % CHARACTER_WEAPONS.len();
                        *cursor = 0;
                    }
                    KeyCode::Enter => {
                        if let Some(name) = filtered.get(*cursor) {
                            let target = character_names
                                .iter()
                                .position(|owned| owned == name)
                                .unwrap_or(0);
                            self.phase = Phase::Characters { cursor: target };
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('l') => {
                        self.phase = Phase::Characters { cursor: 0 }
                    }
                    _ => {}
                }
            }
            (Phase::Characters { .. }, KeyCode::Esc) => self.phase = Phase::MainMenu { cursor: 3 },
            (Phase::BattleTeamSelect { cursor }, KeyCode::Up) => *cursor = cursor.saturating_sub(1),
            (Phase::BattleTeamSelect { cursor }, KeyCode::Down) => {
                *cursor = (*cursor + 1).min(self.save.teams.len().saturating_sub(1))
            }
            (Phase::BattleTeamSelect { cursor }, KeyCode::Enter) => {
                let members = self.save.teams[*cursor]
                    .members
                    .iter()
                    .cloned()
                    .collect::<Option<Vec<_>>>();
                if let Some(members) = members
                    && let Some(state) = BattleState::new(&members, &self.save.equipment)
                {
                    self.phase = Phase::Battle { state };
                }
            }
            (Phase::BattleTeamSelect { .. }, KeyCode::Esc) => {
                self.phase = Phase::MainMenu { cursor: 4 }
            }
            (Phase::Battle { state }, KeyCode::Up | KeyCode::Left) => state.move_cursor(-1),
            (Phase::Battle { state }, KeyCode::Down | KeyCode::Right) => state.move_cursor(1),
            (Phase::Battle { state }, KeyCode::Enter) if state.outcome.is_none() => state.confirm(),
            (Phase::Battle { state }, KeyCode::Enter) if state.outcome.is_some() => {
                self.phase = Phase::BattleTeamSelect { cursor: 0 }
            }
            (Phase::Battle { state }, KeyCode::Esc)
                if !matches!(state.menu, BattleMenu::Action) =>
            {
                state.back()
            }
            (Phase::Battle { .. }, KeyCode::Esc) => {
                self.phase = Phase::BattleTeamSelect { cursor: 0 }
            }
            (Phase::Tutorial, KeyCode::Esc | KeyCode::Enter) => {
                self.phase = Phase::MainMenu { cursor: 5 }
            }
            (
                Phase::CharacterWeaponSelect {
                    weapon_cursor,
                    character_cursor: _,
                },
                KeyCode::Up,
            ) => {
                *weapon_cursor = weapon_cursor
                    .saturating_sub(1)
                    .min(equipment_weapons.len().saturating_sub(1));
            }
            (
                Phase::CharacterWeaponSelect {
                    weapon_cursor,
                    character_cursor: _,
                },
                KeyCode::Down,
            ) => {
                *weapon_cursor =
                    (*weapon_cursor + 1).min(equipment_weapons.len().saturating_sub(1));
            }
            (
                Phase::CharacterWeaponSelect {
                    weapon_cursor,
                    character_cursor,
                },
                KeyCode::Enter,
            ) => {
                if let Some(character) = character_names.get(*character_cursor)
                    && let Some(weapon) = equipment_weapons.get(*weapon_cursor)
                {
                    self.save
                        .equipment
                        .insert(character.clone(), weapon.clone());
                    storage::save(&self.save)?;
                }
            }
            (
                Phase::CharacterWeaponSelect {
                    character_cursor, ..
                },
                KeyCode::Char('d'),
            ) => {
                if let Some(character) = character_names.get(*character_cursor) {
                    self.save.equipment.remove(character);
                    storage::save(&self.save)?;
                }
            }
            (
                Phase::CharacterWeaponSelect {
                    character_cursor, ..
                },
                KeyCode::Esc,
            ) => {
                self.phase = Phase::Characters {
                    cursor: *character_cursor,
                }
            }
            (Phase::Home, KeyCode::Char('1')) | (Phase::Home, KeyCode::Enter) => {
                self.begin_pull(1)?
            }
            (Phase::Home, KeyCode::Char('0')) => self.begin_pull(10)?,
            (Phase::Home, KeyCode::Left) => self.change_banner(-1),
            (Phase::Home, KeyCode::Right) => self.change_banner(1),
            (Phase::Home, KeyCode::Char('b')) => {
                let cursor = Banner::SELECTOR
                    .iter()
                    .position(|banner| *banner == self.banner)
                    .unwrap_or(0);
                self.phase = Phase::BannerSelect { cursor };
            }
            (Phase::Home, KeyCode::Char('c')) => {
                self.phase = Phase::CharacterArchive { cursor: 0 };
            }
            (Phase::BannerSelect { cursor }, KeyCode::Left) => {
                *cursor = cursor.saturating_sub(1);
            }
            (Phase::BannerSelect { cursor }, KeyCode::Right) => {
                *cursor = (*cursor + 1).min(Banner::SELECTOR.len() - 1);
            }
            (Phase::BannerSelect { cursor }, KeyCode::Up) => {
                *cursor = cursor.saturating_sub(3);
            }
            (Phase::BannerSelect { cursor }, KeyCode::Down) => {
                *cursor = (*cursor + 3).min(Banner::SELECTOR.len() - 1);
            }
            (Phase::BannerSelect { cursor }, KeyCode::Enter) => {
                self.banner = Banner::SELECTOR[*cursor];
                self.phase = Phase::Home;
            }
            (Phase::BannerSelect { .. }, KeyCode::Esc | KeyCode::Char('b')) => {
                self.phase = Phase::Home;
            }
            (Phase::Home, KeyCode::Char('p')) if self.banner == Banner::Weapon => {
                let cursor = WeaponPath::ALL
                    .iter()
                    .position(|path| *path == self.save.weapon_pity.path)
                    .unwrap_or(0);
                self.phase = Phase::WeaponSelect {
                    cursor,
                    preview: false,
                };
            }
            (Phase::WeaponSelect { cursor, .. }, KeyCode::Up) => {
                *cursor = cursor.saturating_sub(1);
            }
            (Phase::WeaponSelect { cursor, .. }, KeyCode::Down) => {
                *cursor = (*cursor + 1).min(WeaponPath::ALL.len() - 1);
            }
            (Phase::WeaponSelect { preview, .. }, KeyCode::Char('v')) => *preview = !*preview,
            (Phase::WeaponSelect { cursor, .. }, KeyCode::Enter) => {
                self.save.weapon_pity.path = WeaponPath::ALL[*cursor];
                self.save.weapon_pity.fate_points = 0;
                storage::save(&self.save)?;
                self.phase = Phase::Home;
            }
            (Phase::WeaponSelect { .. }, KeyCode::Esc | KeyCode::Char('p')) => {
                self.phase = Phase::Home;
            }
            (Phase::CharacterArchive { cursor }, KeyCode::Left) => {
                *cursor = cursor.saturating_sub(1);
            }
            (Phase::CharacterArchive { cursor }, KeyCode::Right) => {
                *cursor = (*cursor + 1).min(crate::simulation::all_characters().len() - 1);
            }
            (Phase::CharacterArchive { cursor }, KeyCode::Up) => {
                *cursor = cursor.saturating_sub(3);
            }
            (Phase::CharacterArchive { cursor }, KeyCode::Down) => {
                *cursor = (*cursor + 3).min(crate::simulation::all_characters().len() - 1);
            }
            (Phase::CharacterArchive { .. }, KeyCode::Esc | KeyCode::Char('c')) => {
                self.phase = Phase::Home;
            }
            (Phase::Home, KeyCode::Char('p')) if self.banner == Banner::Standard => {
                self.save.standard_pity.path = self.save.standard_pity.path.next();
                self.save.standard_pity.fate_points = 0;
                storage::save(&self.save)?;
            }
            (Phase::Home, KeyCode::Char('h')) => self.phase = Phase::History,
            (Phase::Home, KeyCode::Char('i')) => {
                self.phase = Phase::Inventory {
                    cursor: 0,
                    selected: BTreeSet::new(),
                }
            }
            (Phase::History, KeyCode::Esc | KeyCode::Char('h')) => self.phase = Phase::Home,
            (Phase::Inventory { cursor, .. }, KeyCode::Up) => *cursor = cursor.saturating_sub(1),
            (Phase::Inventory { cursor, .. }, KeyCode::Down) => {
                *cursor = (*cursor + 1).min(inventory_names.len().saturating_sub(1));
            }
            (Phase::Inventory { cursor, selected }, KeyCode::Char(' ')) => {
                if let Some(name) = inventory_names.get(*cursor).cloned()
                    && !selected.remove(&name)
                {
                    selected.insert(name);
                }
            }
            (Phase::Inventory { selected, .. }, KeyCode::Char('a')) => {
                if selected.len() == inventory_names.len() {
                    selected.clear();
                } else {
                    *selected = inventory_names.iter().cloned().collect();
                }
            }
            (Phase::Inventory { cursor, .. }, KeyCode::Char('s')) => {
                self.inventory_sort = match self.inventory_sort {
                    InventorySort::Name => InventorySort::Rarity,
                    InventorySort::Rarity => InventorySort::Kind,
                    InventorySort::Kind => InventorySort::Element,
                    InventorySort::Element => InventorySort::Name,
                };
                *cursor = 0;
            }
            (Phase::Inventory { cursor, selected }, KeyCode::Char('f')) => {
                self.inventory_kind = match self.inventory_kind {
                    InventoryKind::All => InventoryKind::Character,
                    InventoryKind::Character => InventoryKind::Weapon,
                    InventoryKind::Weapon => InventoryKind::All,
                };
                *cursor = 0;
                selected.clear();
            }
            (Phase::Inventory { cursor, selected }, KeyCode::Char('e')) => {
                self.inventory_element = (self.inventory_element + 1) % ELEMENT_FILTERS.len();
                *cursor = 0;
                selected.clear();
            }
            (Phase::Inventory { cursor, selected }, KeyCode::Enter) => {
                if let Some(name) = inventory_names.get(*cursor).cloned() {
                    self.phase = Phase::InventoryDetail {
                        cursor: *cursor,
                        selected: std::mem::take(selected),
                        name,
                    };
                }
            }
            (Phase::Inventory { cursor, selected }, KeyCode::Char('d')) => {
                let targets: Vec<String> = if selected.is_empty() {
                    inventory_names.get(*cursor).cloned().into_iter().collect()
                } else {
                    selected.iter().cloned().collect()
                };
                if !targets.is_empty() {
                    self.phase = Phase::ConfirmInventoryDelete {
                        cursor: *cursor,
                        selected: std::mem::take(selected),
                        targets,
                    };
                }
            }
            (Phase::Inventory { cursor, selected }, KeyCode::Char('D')) => {
                let targets = self.save.inventory.keys().cloned().collect::<Vec<_>>();
                if !targets.is_empty() {
                    self.phase = Phase::ConfirmInventoryDelete {
                        cursor: *cursor,
                        selected: std::mem::take(selected),
                        targets,
                    };
                }
            }
            (Phase::Inventory { .. }, KeyCode::Esc | KeyCode::Char('i')) => {
                self.phase = Phase::MainMenu { cursor: 2 }
            }
            (
                Phase::InventoryDetail {
                    cursor, selected, ..
                },
                KeyCode::Esc | KeyCode::Enter,
            ) => {
                self.phase = Phase::Inventory {
                    cursor: *cursor,
                    selected: std::mem::take(selected),
                };
            }
            (
                Phase::ConfirmInventoryDelete {
                    cursor, selected, ..
                },
                KeyCode::Char('n') | KeyCode::Esc,
            ) => {
                self.phase = Phase::Inventory {
                    cursor: *cursor,
                    selected: std::mem::take(selected),
                };
            }
            (Phase::ConfirmInventoryDelete { targets, .. }, KeyCode::Char('y')) => {
                delete_inventory_entries(&mut self.save, targets);
                storage::save(&self.save)?;
                self.phase = Phase::Inventory {
                    cursor: 0,
                    selected: BTreeSet::new(),
                };
            }
            (Phase::Flight { results, .. }, KeyCode::Char(' ') | KeyCode::Enter) => {
                self.phase = Self::reveal_phase(std::mem::take(results), 0, Instant::now());
            }
            (Phase::Flight { results, .. }, KeyCode::Char('s')) if results.len() > 1 => {
                self.phase = Self::next_five_star_phase(std::mem::take(results), 0, Instant::now());
            }
            (
                Phase::Reveal {
                    results,
                    index,
                    five_star_only,
                    ..
                },
                KeyCode::Char(' ') | KeyCode::Enter,
            ) => {
                if *five_star_only {
                    self.phase = Self::next_five_star_phase(
                        std::mem::take(results),
                        *index + 1,
                        Instant::now(),
                    );
                } else if *index + 1 < results.len() {
                    let next = *index + 1;
                    self.phase = Self::reveal_phase(std::mem::take(results), next, Instant::now());
                } else {
                    self.phase = Phase::Summary {
                        results: std::mem::take(results),
                        selected: 0,
                    };
                }
            }
            (
                Phase::FiveStarIntro {
                    results,
                    index,
                    five_star_only,
                    ..
                },
                KeyCode::Char(' ') | KeyCode::Enter,
            ) => {
                self.phase = Phase::Reveal {
                    started: Instant::now(),
                    results: std::mem::take(results),
                    index: *index,
                    five_star_only: *five_star_only,
                };
            }
            (
                Phase::FiveStarIntro {
                    five_star_only,
                    results,
                    ..
                },
                KeyCode::Char('s'),
            ) if results.len() > 1 => {
                *five_star_only = true;
            }
            (Phase::Reveal { results, index, .. }, KeyCode::Char('s')) if results.len() > 1 => {
                self.phase =
                    Self::next_five_star_phase(std::mem::take(results), *index + 1, Instant::now());
            }
            (Phase::Summary { selected, .. }, KeyCode::Left) => {
                *selected = selected.saturating_sub(1)
            }
            (Phase::Summary { selected, results }, KeyCode::Right) => {
                *selected = (*selected + 1).min(results.len() - 1)
            }
            (Phase::Summary { results, selected }, KeyCode::Enter) => {
                self.phase = Phase::Detail {
                    results: std::mem::take(results),
                    selected: *selected,
                };
            }
            (Phase::Summary { .. }, KeyCode::Esc | KeyCode::Char(' ')) => {
                self.phase = Phase::Home;
            }
            (Phase::Detail { results, selected }, KeyCode::Esc | KeyCode::Enter) => {
                self.phase = Phase::Summary {
                    results: std::mem::take(results),
                    selected: *selected,
                };
            }
            (Phase::Home, KeyCode::Esc) => self.phase = Phase::MainMenu { cursor: 1 },
            (_, KeyCode::Esc) => self.phase = Phase::Home,
            _ => {}
        }
        Ok(())
    }

    fn begin_pull(&mut self, count: usize) -> Result<()> {
        let results = self.engine.pull_many(&mut self.save, count, self.banner);
        storage::save(&self.save)?;
        self.phase = Phase::Flight {
            started: Instant::now(),
            results,
        };
        Ok(())
    }

    fn change_banner(&mut self, direction: isize) {
        let current = Banner::ALL
            .iter()
            .position(|banner| *banner == self.banner)
            .unwrap_or(0);
        let next = (current as isize + direction).rem_euclid(Banner::ALL.len() as isize) as usize;
        self.banner = Banner::ALL[next];
    }

    fn advance(&mut self, now: Instant) {
        match &mut self.phase {
            Phase::Flight { started, results } if now.duration_since(*started) >= FLIGHT_TIME => {
                self.phase = Self::reveal_phase(std::mem::take(results), 0, now);
            }
            Phase::FiveStarIntro {
                started,
                results,
                index,
                five_star_only,
            } if now.duration_since(*started) >= FIVE_STAR_INTRO_TIME => {
                self.phase = Phase::Reveal {
                    started: now,
                    results: std::mem::take(results),
                    index: *index,
                    five_star_only: *five_star_only,
                };
            }
            Phase::Reveal {
                started,
                results,
                index,
                five_star_only: true,
            } if now.duration_since(*started) >= Duration::from_secs(2) => {
                self.phase = Self::next_five_star_phase(std::mem::take(results), *index + 1, now);
            }
            _ => {}
        }
    }

    fn reveal_phase(results: Vec<WishResult>, index: usize, now: Instant) -> Phase {
        if results[index].rarity == crate::model::Rarity::Five {
            Phase::FiveStarIntro {
                started: now,
                results,
                index,
                five_star_only: false,
            }
        } else {
            Phase::Reveal {
                started: now,
                results,
                index,
                five_star_only: false,
            }
        }
    }

    fn next_five_star_phase(results: Vec<WishResult>, start: usize, now: Instant) -> Phase {
        if let Some(index) = results
            .iter()
            .enumerate()
            .skip(start)
            .find_map(|(index, result)| {
                (result.rarity == crate::model::Rarity::Five).then_some(index)
            })
        {
            Phase::FiveStarIntro {
                started: now,
                results,
                index,
                five_star_only: true,
            }
        } else {
            Phase::Summary {
                results,
                selected: 0,
            }
        }
    }
}

pub const CHARACTER_ELEMENTS: [&str; 8] = [
    "ALL", "PYRO", "HYDRO", "ELECTRO", "CRYO", "ANEMO", "GEO", "DENDRO",
];
pub const CHARACTER_WEAPONS: [&str; 10] = [
    "ALL",
    "SWORD",
    "CLAYMORE",
    "BOW",
    "POLEARM",
    "CATALYST",
    "GAUNTLET",
    "SCYTHE",
    "DUAL BLADES",
    "UNALIGNED",
];

pub fn filtered_characters(
    names: &[String],
    rarity: u8,
    element: usize,
    weapon: usize,
) -> Vec<String> {
    names
        .iter()
        .filter(|name| {
            crate::simulation::catalog_item(name).is_some_and(|item| {
                (rarity == 0 || item.rarity.value() == if rarity == 1 { 5 } else { 4 })
                    && (element == 0
                        || item
                            .element()
                            .eq_ignore_ascii_case(CHARACTER_ELEMENTS[element]))
                    && (weapon == 0
                        || crate::simulation::character_weapon_type(item.name)
                            .eq_ignore_ascii_case(CHARACTER_WEAPONS[weapon]))
            })
        })
        .cloned()
        .collect()
}

fn delete_inventory_entries(save: &mut SaveData, targets: &[String]) {
    for name in targets {
        save.inventory.remove(name);
        save.equipment.remove(name);
        save.equipment.retain(|_, weapon| weapon != name);
        for team in &mut save.teams {
            for member in &mut team.members {
                if member.as_deref() == Some(name) {
                    *member = None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Item, Rarity};

    #[test]
    fn five_star_gets_a_pre_reveal_cutscene() {
        let result = WishResult {
            item: Item {
                name: "Test Legend",
                kind: "Character",
                rarity: Rarity::Five,
            },
            rarity: Rarity::Five,
            featured: true,
            wish_number: 1,
        };
        assert!(matches!(
            App::reveal_phase(vec![result], 0, Instant::now()),
            Phase::FiveStarIntro { .. }
        ));
    }

    #[test]
    fn skip_routes_through_every_five_star_before_summary() {
        let item = |name, rarity| WishResult {
            item: Item {
                name,
                kind: "Character",
                rarity,
            },
            rarity,
            featured: false,
            wish_number: 1,
        };
        let results = vec![
            item("Three", Rarity::Three),
            item("First Five", Rarity::Five),
            item("Four", Rarity::Four),
            item("Second Five", Rarity::Five),
        ];

        let phase = App::next_five_star_phase(results, 0, Instant::now());
        let Phase::FiveStarIntro {
            results,
            index,
            five_star_only: true,
            ..
        } = phase
        else {
            panic!("skip should stop at the first five-star");
        };
        assert_eq!(index, 1);

        let phase = App::next_five_star_phase(results, index + 1, Instant::now());
        let Phase::FiveStarIntro {
            results,
            index,
            five_star_only: true,
            ..
        } = phase
        else {
            panic!("skip should stop at the second five-star");
        };
        assert_eq!(index, 3);
        assert!(matches!(
            App::next_five_star_phase(results, index + 1, Instant::now()),
            Phase::Summary { .. }
        ));
    }

    #[test]
    fn inventory_deletion_preserves_history_and_pity() {
        let mut save = SaveData::default();
        save.inventory.insert("Raven Bow".into(), 3);
        save.inventory.insert("Quartz Spear".into(), 2);
        save.pity.five_star = 47;
        save.total_wishes = 99;
        delete_inventory_entries(&mut save, &["Raven Bow".into()]);
        assert!(!save.inventory.contains_key("Raven Bow"));
        assert_eq!(save.inventory.get("Quartz Spear"), Some(&2));
        assert_eq!(save.pity.five_star, 47);
        assert_eq!(save.total_wishes, 99);
        assert_eq!(save.teams.len(), 5);
    }

    #[test]
    fn detects_kitty_and_ghostty_graphics_terminals() {
        assert!(detects_graphics_protocol("xterm-kitty", "", false, false));
        assert!(detects_graphics_protocol("xterm-ghostty", "", false, false));
        assert!(detects_graphics_protocol(
            "xterm-256color",
            "ghostty",
            false,
            false
        ));
        assert!(detects_graphics_protocol("xterm-256color", "", true, false));
        assert!(detects_graphics_protocol("xterm-256color", "", false, true));
        assert!(!detects_graphics_protocol(
            "xterm-256color",
            "apple_terminal",
            false,
            false
        ));
    }

    #[test]
    fn graphics_override_accepts_native_and_ansi_modes() {
        assert_eq!(graphics_override(Some("ghostty")), Some(true));
        assert_eq!(graphics_override(Some("1")), Some(true));
        assert_eq!(graphics_override(Some("ansi")), Some(false));
        assert_eq!(graphics_override(Some("0")), Some(false));
        assert_eq!(graphics_override(Some("unexpected")), None);
        assert_eq!(graphics_override(None), None);
    }

    #[test]
    fn character_quick_filters_combine_rarity_element_and_weapon() {
        let names = vec![
            "Astraea, Starbound".into(),
            "Anya".into(),
            "Kestrel".into(),
            "Mako".into(),
        ];
        assert_eq!(
            filtered_characters(&names, 1, 4, 5),
            vec!["Astraea, Starbound"]
        );
        assert_eq!(filtered_characters(&names, 2, 5, 3), vec!["Kestrel"]);
        assert!(filtered_characters(&names, 1, 2, 1).is_empty());
    }
}
