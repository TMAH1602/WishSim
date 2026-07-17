use std::{
    collections::BTreeSet,
    time::{Duration, Instant},
};

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::{
    art::CharacterGallery,
    kitty::KittyRenderer,
    model::{Banner, SaveData, WishResult},
    simulation::WishEngine,
    storage, ui,
};

const FRAME_TIME: Duration = Duration::from_millis(33);
const FLIGHT_TIME: Duration = Duration::from_millis(1_650);
const FIVE_STAR_INTRO_TIME: Duration = Duration::from_millis(3_400);

pub struct App {
    pub save: SaveData,
    pub phase: Phase,
    pub kitty: bool,
    pub banner: Banner,
    pub gallery: CharacterGallery,
    pub confirm_quit: bool,
    engine: WishEngine,
    should_quit: bool,
}

pub enum Phase {
    Home,
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
    },
    FiveStarIntro {
        started: Instant,
        results: Vec<WishResult>,
        index: usize,
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
        phase: Phase::Home,
        kitty: std::env::var_os("KITTY_WINDOW_ID").is_some()
            || std::env::var("TERM").is_ok_and(|v| v.contains("kitty")),
        banner: Banner::Astraea,
        gallery: CharacterGallery::load()?,
        confirm_quit: false,
        engine: WishEngine::random(),
        should_quit: false,
    };

    let mut kitty_renderer = KittyRenderer::default();
    ratatui::run(|terminal| -> Result<()> {
        while !app.should_quit {
            let now = Instant::now();
            app.advance(now);
            let mut drawn_area = ratatui::layout::Rect::default();
            terminal.draw(|frame| {
                drawn_area = frame.area();
                ui::render(frame, &app, now);
            })?;
            if app.kitty {
                kitty_renderer.sync(ui::kitty_portrait(&app, drawn_area))?;
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
        kitty_renderer.clear()?;
        Ok(())
    })?;
    Ok(())
}

impl App {
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
        if key == KeyCode::Char('q') {
            self.confirm_quit = true;
            return Ok(());
        }
        match (&mut self.phase, key) {
            (Phase::Home, KeyCode::Char('1')) | (Phase::Home, KeyCode::Enter) => {
                self.begin_pull(1)?
            }
            (Phase::Home, KeyCode::Char('0')) => self.begin_pull(10)?,
            (Phase::Home, KeyCode::Left) => self.change_banner(-1),
            (Phase::Home, KeyCode::Right) => self.change_banner(1),
            (Phase::Home, KeyCode::Char('p')) if self.banner == Banner::Weapon => {
                self.save.weapon_pity.path = self.save.weapon_pity.path.toggled();
                self.save.weapon_pity.fate_points = 0;
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
                *cursor = (*cursor + 1).min(self.save.inventory.len().saturating_sub(1));
            }
            (Phase::Inventory { cursor, selected }, KeyCode::Char(' ')) => {
                if let Some(name) = self.save.inventory.keys().nth(*cursor).cloned()
                    && !selected.remove(&name)
                {
                    selected.insert(name);
                }
            }
            (Phase::Inventory { selected, .. }, KeyCode::Char('a')) => {
                if selected.len() == self.save.inventory.len() {
                    selected.clear();
                } else {
                    *selected = self.save.inventory.keys().cloned().collect();
                }
            }
            (Phase::Inventory { cursor, selected }, KeyCode::Enter) => {
                if let Some(name) = self.save.inventory.keys().nth(*cursor).cloned() {
                    self.phase = Phase::InventoryDetail {
                        cursor: *cursor,
                        selected: std::mem::take(selected),
                        name,
                    };
                }
            }
            (Phase::Inventory { cursor, selected }, KeyCode::Char('d')) => {
                let targets: Vec<String> = if selected.is_empty() {
                    self.save
                        .inventory
                        .keys()
                        .nth(*cursor)
                        .cloned()
                        .into_iter()
                        .collect()
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
                self.phase = Phase::Home
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
            (
                Phase::ConfirmInventoryDelete {
                    cursor, targets, ..
                },
                KeyCode::Char('y'),
            ) => {
                delete_inventory_entries(&mut self.save, targets);
                storage::save(&self.save)?;
                self.phase = Phase::Inventory {
                    cursor: (*cursor).min(self.save.inventory.len().saturating_sub(1)),
                    selected: BTreeSet::new(),
                };
            }
            (Phase::Flight { results, .. }, KeyCode::Char(' ') | KeyCode::Enter) => {
                self.phase = Self::reveal_phase(std::mem::take(results), 0, Instant::now());
            }
            (Phase::Flight { results, .. }, KeyCode::Char('s')) if results.len() > 1 => {
                self.phase = Phase::Summary {
                    results: std::mem::take(results),
                    selected: 0,
                };
            }
            (Phase::Reveal { results, index, .. }, KeyCode::Char(' ') | KeyCode::Enter) => {
                if *index + 1 < results.len() {
                    let next = *index + 1;
                    self.phase = Self::reveal_phase(std::mem::take(results), next, Instant::now());
                } else {
                    self.phase = Phase::Summary {
                        results: std::mem::take(results),
                        selected: 0,
                    };
                }
            }
            (Phase::FiveStarIntro { results, index, .. }, KeyCode::Char(' ') | KeyCode::Enter) => {
                self.phase = Phase::Reveal {
                    started: Instant::now(),
                    results: std::mem::take(results),
                    index: *index,
                };
            }
            (Phase::FiveStarIntro { results, .. }, KeyCode::Char('s')) if results.len() > 1 => {
                self.phase = Phase::Summary {
                    results: std::mem::take(results),
                    selected: 0,
                };
            }
            (Phase::Reveal { results, .. }, KeyCode::Char('s')) if results.len() > 1 => {
                self.phase = Phase::Summary {
                    results: std::mem::take(results),
                    selected: 0,
                };
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
            } if now.duration_since(*started) >= FIVE_STAR_INTRO_TIME => {
                self.phase = Phase::Reveal {
                    started: now,
                    results: std::mem::take(results),
                    index: *index,
                };
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
            }
        } else {
            Phase::Reveal {
                started: now,
                results,
                index,
            }
        }
    }
}

fn delete_inventory_entries(save: &mut SaveData, targets: &[String]) {
    for name in targets {
        save.inventory.remove(name);
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
    }
}
