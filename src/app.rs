use std::time::{Duration, Instant};

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::{
    model::{Banner, SaveData, WishResult},
    simulation::WishEngine,
    storage, ui,
};

const FRAME_TIME: Duration = Duration::from_millis(33);
const FLIGHT_TIME: Duration = Duration::from_millis(1_650);
const REVEAL_TIME: Duration = Duration::from_millis(1_050);
const FIVE_STAR_INTRO_TIME: Duration = Duration::from_millis(3_400);

pub struct App {
    pub save: SaveData,
    pub phase: Phase,
    pub kitty: bool,
    pub banner: Banner,
    engine: WishEngine,
    should_quit: bool,
}

pub enum Phase {
    Home,
    History,
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
        engine: WishEngine::random(),
        should_quit: false,
    };

    ratatui::run(|terminal| -> Result<()> {
        while !app.should_quit {
            let now = Instant::now();
            app.advance(now);
            terminal.draw(|frame| ui::render(frame, &app, now))?;

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
        Ok(())
    })?;
    Ok(())
}

impl App {
    fn is_animating(&self) -> bool {
        matches!(
            self.phase,
            Phase::Flight { .. } | Phase::Reveal { .. } | Phase::FiveStarIntro { .. }
        )
    }

    fn handle_key(&mut self, key: KeyCode) -> Result<()> {
        match (&mut self.phase, key) {
            (_, KeyCode::Char('q')) => self.should_quit = true,
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
            (Phase::History, KeyCode::Esc | KeyCode::Char('h')) => self.phase = Phase::Home,
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
            Phase::Reveal {
                started,
                results,
                index,
            } if now.duration_since(*started) >= REVEAL_TIME => {
                if *index + 1 < results.len() {
                    let next = *index + 1;
                    self.phase = Self::reveal_phase(std::mem::take(results), next, now);
                } else {
                    self.phase = Phase::Summary {
                        results: std::mem::take(results),
                        selected: 0,
                    };
                }
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
}
