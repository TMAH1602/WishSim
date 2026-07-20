use std::{collections::BTreeSet, time::Instant};

use crate::{
    app::{
        App, CHARACTER_ELEMENTS, CHARACTER_WEAPONS, ELEMENT_FILTERS, Phase, filtered_characters,
    },
    art::{CharacterGallery, TerminalRaster},
    model::{Banner, Rarity, WishResult},
    simulation::{all_characters, catalog_item, standard_character, weapon_for_path},
};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Gauge, Padding, Paragraph, Widget, Wrap},
};

const GOLD: Color = Color::Rgb(255, 205, 90);
const PURPLE: Color = Color::Rgb(198, 120, 255);
const BLUE: Color = Color::Rgb(90, 180, 255);
const DIM: Color = Color::Rgb(100, 115, 145);

pub fn graphics_portraits(app: &App, area: Rect) -> Vec<(&str, Rect)> {
    if app.confirm_quit || area.width < 80 || area.height < 34 {
        return Vec::new();
    }
    if let Phase::Characters { cursor } = &app.phase {
        let names = app.owned_character_names();
        if let Some(name) = names.get(*cursor) {
            let panel = centered(area, area.width.min(108), area.height.min(32));
            let inner = panel.inner(Margin {
                horizontal: 2,
                vertical: 1,
            });
            let [_, body, _] = Layout::vertical([
                Constraint::Length(3),
                Constraint::Min(23),
                Constraint::Length(2),
            ])
            .areas(inner);
            let [_, art, _] = Layout::horizontal([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .spacing(1)
            .areas(body);
            return vec![(catalog_item(name).unwrap().name, portrait_fit(art))];
        }
    }
    if let Phase::Teams { team, .. } = &app.phase {
        let panel = centered(area, area.width.min(112), area.height.min(32));
        let inner = panel.inner(Margin {
            horizontal: 2,
            vertical: 1,
        });
        let [_, body, _] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(24),
            Constraint::Length(2),
        ])
        .areas(inner);
        let cards = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .spacing(1)
        .split(body);
        return app.save.teams[*team]
            .members
            .iter()
            .enumerate()
            .filter_map(|(index, name)| {
                name.as_deref().and_then(catalog_item).map(|item| {
                    let inner = cards[index].inner(Margin {
                        horizontal: 1,
                        vertical: 1,
                    });
                    let [art, _, _] = Layout::vertical([
                        Constraint::Min(15),
                        Constraint::Length(2),
                        Constraint::Length(3),
                    ])
                    .areas(inner);
                    (item.name, portrait_fit(art))
                })
            })
            .collect();
    }
    if let Phase::CharacterWeaponSelect {
        character_cursor,
        weapon_cursor,
    } = &app.phase
    {
        let chars = app.owned_character_names();
        if let Some(character) = chars.get(*character_cursor) {
            let weapons = app.compatible_weapon_names(character);
            let panel = centered(area, area.width.min(112), area.height.min(32));
            let inner = panel.inner(Margin {
                horizontal: 2,
                vertical: 1,
            });
            let [body, _] =
                Layout::vertical([Constraint::Min(25), Constraint::Length(2)]).areas(inner);
            let [char_art, _, weapon_art] = Layout::horizontal([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .spacing(1)
            .areas(body);
            let mut out = vec![(
                catalog_item(character).unwrap().name,
                portrait_fit(char_art),
            )];
            if let Some(weapon) = weapons.get(*weapon_cursor) {
                out.push((catalog_item(weapon).unwrap().name, portrait_fit(weapon_art)));
            }
            return out;
        }
    }
    let portrait = match &app.phase {
        Phase::Reveal { results, index, .. }
            if app.gallery.get(results[*index].item.name).is_some() =>
        {
            let card = centered(area, 72.min(area.width - 8), 36.min(area.height - 4));
            let inner = card.inner(ratatui::layout::Margin {
                horizontal: 2,
                vertical: 1,
            });
            let [sprite_area, _] =
                Layout::vertical([Constraint::Min(8), Constraint::Length(6)]).areas(inner);
            Some((results[*index].item.name, portrait_fit(sprite_area)))
        }
        Phase::Detail { results, selected }
            if app.gallery.get(results[*selected].item.name).is_some() =>
        {
            Some((results[*selected].item.name, detail_portrait_area(area)))
        }
        Phase::InventoryDetail { name, .. } if app.gallery.get(name).is_some() => {
            Some((name.as_str(), detail_portrait_area(area)))
        }
        Phase::WeaponSelect {
            cursor,
            preview: false,
        } => {
            let item = weapon_for_path(crate::model::WeaponPath::ALL[*cursor]);
            Some((item.name, weapon_select_portrait_area(area)))
        }
        _ => None,
    };
    portrait.into_iter().collect()
}

fn detail_portrait_area(area: Rect) -> Rect {
    let panel = centered(area, area.width.min(112), area.height.min(38));
    let inner = panel.inner(ratatui::layout::Margin {
        horizontal: 2,
        vertical: 1,
    });
    let [art_area, _] =
        Layout::horizontal([Constraint::Percentage(42), Constraint::Percentage(58)])
            .spacing(2)
            .areas(inner);
    portrait_fit(art_area.inner(ratatui::layout::Margin {
        horizontal: 1,
        vertical: 2,
    }))
}

fn weapon_select_portrait_area(area: Rect) -> Rect {
    let panel = centered(area, area.width.min(94), area.height.min(31));
    let inner = panel.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });
    let [body, _] = Layout::vertical([Constraint::Min(20), Constraint::Length(2)]).areas(inner);
    let [_, art] = Layout::horizontal([Constraint::Percentage(46), Constraint::Percentage(54)])
        .spacing(2)
        .areas(body);
    portrait_fit(art.inner(Margin {
        horizontal: 1,
        vertical: 1,
    }))
}

fn portrait_fit(area: Rect) -> Rect {
    let width = area.width.min(area.height.saturating_mul(4) / 3).max(1);
    let height = area.height.min(width.saturating_mul(3) / 4).max(1);
    centered(area, width, height)
}

pub fn render(frame: &mut Frame, app: &App, now: Instant) {
    let area = frame.area();
    frame.render_widget(Block::new().bg(Color::Rgb(4, 7, 19)), area);
    if area.width < 80 || area.height < 34 {
        frame.render_widget(
            Paragraph::new("✦  WishSim needs a terminal at least 80 × 34 for full character art\n\nResize the window, or press Q to open the exit prompt.")
                .centered()
                .style(Style::new().fg(GOLD)),
            area,
        );
        if app.confirm_quit {
            confirm_quit(frame);
        }
        return;
    }

    match &app.phase {
        Phase::MainMenu { cursor } => main_menu(frame, app, *cursor),
        Phase::Home => home(frame, app),
        Phase::Teams {
            team,
            slot,
            editing_name,
        } => teams(frame, app, *team, *slot, *editing_name),
        Phase::TeamCharacterSelect { team, slot, cursor } => {
            team_character_select(frame, app, *team, *slot, *cursor)
        }
        Phase::Characters { cursor } => characters(frame, app, *cursor),
        Phase::CharacterQuickSelect {
            cursor,
            rarity,
            element,
            weapon,
        } => character_quick_select(frame, app, *cursor, *rarity, *element, *weapon),
        Phase::CharacterWeaponSelect {
            character_cursor,
            weapon_cursor,
        } => character_weapon_select(frame, app, *character_cursor, *weapon_cursor),
        Phase::BannerSelect { cursor } => banner_select(frame, app, *cursor),
        Phase::WeaponSelect { cursor, preview } => weapon_select(frame, app, *cursor, *preview),
        Phase::CharacterArchive { cursor } => character_archive(frame, app, *cursor),
        Phase::History => history(frame, app),
        Phase::Inventory { cursor, selected } => inventory(frame, app, *cursor, selected),
        Phase::InventoryDetail { name, .. } => {
            if let Some(item) = catalog_item(name) {
                let result = WishResult {
                    item,
                    rarity: item.rarity,
                    featured: false,
                    wish_number: 0,
                };
                detail(
                    frame,
                    &result,
                    app.save.inventory.get(name).copied(),
                    &app.gallery,
                    !app.graphics,
                );
            }
        }
        Phase::ConfirmInventoryDelete {
            cursor,
            selected,
            targets,
        } => {
            inventory(frame, app, *cursor, selected);
            confirm_inventory_delete(frame, targets);
        }
        Phase::Flight { started, results } => {
            flight(frame, now.duration_since(*started).as_secs_f32(), results)
        }
        Phase::Reveal {
            started,
            results,
            index,
            ..
        } => reveal(
            frame,
            now.duration_since(*started).as_secs_f32(),
            &results[*index],
            *index,
            results.len(),
            &app.gallery,
            !app.graphics,
        ),
        Phase::FiveStarIntro {
            started,
            results,
            index,
            ..
        } => five_star_intro(
            frame,
            now.duration_since(*started).as_secs_f32(),
            results.len(),
            *index,
        ),
        Phase::Summary { results, selected } => summary(frame, results, *selected),
        Phase::Detail { results, selected } => detail(
            frame,
            &results[*selected],
            None,
            &app.gallery,
            !app.graphics,
        ),
    }
    if app.confirm_quit {
        confirm_quit(frame);
    }
}

fn main_menu(frame: &mut Frame, app: &App, cursor: usize) {
    let area = frame.area();
    frame.render_widget(
        Starfield {
            time: app.save.total_wishes as f32 * 0.1,
            intensity: 1.0,
        },
        area,
    );
    let panel = centered(area, 78.min(area.width - 4), 30.min(area.height - 4));
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(GOLD))
            .title(" THE CELESTIAL ARCHIVE ")
            .title_alignment(Alignment::Center)
            .title_style(Style::new().fg(GOLD).bold()),
        panel,
    );
    let inner = panel.inner(Margin {
        horizontal: 4,
        vertical: 2,
    });
    let [title, menu, description, footer] = Layout::vertical([
        Constraint::Length(5),
        Constraint::Min(11),
        Constraint::Length(4),
        Constraint::Length(2),
    ])
    .areas(inner);
    frame.render_widget(
        Paragraph::new(Text::from(vec![
            Line::from("W I S H S I M").fg(Color::White).bold(),
            Line::from("ASSEMBLE  •  EQUIP  •  SEEK THE STARS").fg(DIM),
        ]))
        .centered(),
        title,
    );
    let entries = [
        ("TEAMS", "Create five squads of three"),
        ("WISH", "Acquire characters and armaments"),
        ("INVENTORY", "Review the Archive's holdings"),
        ("CHARACTERS", "Ascension, stats, and equipment"),
    ];
    let lines = entries
        .iter()
        .enumerate()
        .flat_map(|(i, (name, _))| {
            let style = if i == cursor {
                Style::new().fg(Color::Rgb(8, 12, 25)).bg(GOLD).bold()
            } else {
                Style::new().fg(Color::White)
            };
            [Line::from(*name).style(style), Line::from("")]
        })
        .collect::<Vec<_>>();
    frame.render_widget(
        Paragraph::new(lines).centered().block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(Color::Rgb(45, 55, 85))),
        ),
        menu,
    );
    frame.render_widget(
        Paragraph::new(entries[cursor].1).centered().fg(DIM).block(
            Block::new()
                .borders(Borders::TOP)
                .border_style(Style::new().fg(Color::Rgb(45, 55, 85))),
        ),
        description,
    );
    frame.render_widget(
        Paragraph::new("↑ / ↓ select  •  ENTER open  •  Q quit")
            .centered()
            .fg(DIM),
        footer,
    );
}

fn teams(frame: &mut Frame, app: &App, team: usize, slot: usize, editing: bool) {
    let area = frame.area();
    frame.render_widget(
        Starfield {
            time: 0.5,
            intensity: 0.7,
        },
        area,
    );
    let panel = centered(area, area.width.min(112), area.height.min(32));
    let record = &app.save.teams[team];
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(GOLD))
            .title(format!(
                " TEAM {}/5  •  {}{} ",
                team + 1,
                record.name,
                if editing { "_" } else { "" }
            ))
            .title_alignment(Alignment::Center),
        panel,
    );
    let inner = panel.inner(Margin {
        horizontal: 3,
        vertical: 2,
    });
    let [head, body, help] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(24),
        Constraint::Length(2),
    ])
    .areas(inner);
    frame.render_widget(
        Paragraph::new("Three resonances form one field team.")
            .centered()
            .fg(DIM),
        head,
    );
    let rows = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .spacing(1)
    .split(body);
    for (i, row) in rows.iter().enumerate() {
        let selected = i == slot;
        let border = if selected {
            GOLD
        } else {
            Color::Rgb(55, 65, 95)
        };
        let name = record.members[i].as_deref().unwrap_or("EMPTY RESONANCE");
        let detail = record.members[i]
            .as_deref()
            .and_then(catalog_item)
            .map(|c| {
                format!(
                    "{} {}  •  {}  •  N{}",
                    element_symbol(c.element()),
                    c.element(),
                    crate::simulation::character_weapon_type(c.name),
                    ascension_level(app.save.inventory.get(c.name).copied().unwrap_or(0))
                )
            })
            .unwrap_or_else(|| "ENTER to attach a character".into());
        let inner = row.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });
        let [art, _, label] = Layout::vertical([
            Constraint::Min(15),
            Constraint::Length(2),
            Constraint::Length(3),
        ])
        .areas(inner);
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(border)),
            *row,
        );
        if !app.graphics {
            if let Some(member) = record.members[i]
                .as_deref()
                .and_then(|n| app.gallery.get(n))
            {
                frame.render_widget(TerminalPortrait::new(&member.detail), art);
            } else {
                frame.render_widget(Paragraph::new("\n\n\n       ◇").centered().fg(DIM), art);
            }
        }
        frame.render_widget(
            Paragraph::new(Text::from(vec![
                Line::from(format!("SLOT {}  •  {}", i + 1, name))
                    .fg(if selected { GOLD } else { Color::White })
                    .bold(),
                Line::from(detail).fg(DIM),
            ]))
            .centered(),
            label,
        );
    }
    frame.render_widget(
        Paragraph::new(if editing {
            "TYPE NAME  •  ENTER save  •  BACKSPACE delete"
        } else {
            "↑ / ↓ team  •  ← / → slot  •  ENTER attach  •  D clear  •  R rename  •  ESC menu"
        })
        .centered()
        .fg(DIM),
        help,
    );
}

fn element_symbol(element: &str) -> &'static str {
    match element {
        "Pyro" => "△",
        "Hydro" => "≈",
        "Electro" => "ϟ",
        "Cryo" => "❄",
        "Anemo" => "≋",
        "Geo" => "◇",
        "Dendro" => "♧",
        _ => "·",
    }
}

fn team_character_select(frame: &mut Frame, app: &App, team: usize, slot: usize, cursor: usize) {
    teams(frame, app, team, slot, false);
    let area = centered(
        frame.area(),
        54.min(frame.area().width - 6),
        25.min(frame.area().height - 6),
    );
    frame.render_widget(Clear, area);
    let names = app.owned_character_names();
    let lines = names
        .iter()
        .enumerate()
        .map(|(i, n)| {
            Line::from(format!(
                " {:<38} N{} ",
                n,
                ascension_level(app.save.inventory.get(n).copied().unwrap_or(0))
            ))
            .style(if i == cursor {
                Style::new().fg(Color::Rgb(8, 12, 25)).bg(GOLD).bold()
            } else {
                Style::new().fg(Color::White)
            })
        })
        .collect::<Vec<_>>();
    frame.render_widget(
        Paragraph::new(if lines.is_empty() {
            vec![Line::from("No characters owned. Visit Wish first.").fg(DIM)]
        } else {
            lines
        })
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::new().fg(PURPLE))
                .title(" ATTACH CHARACTER "),
        ),
        area,
    );
}

fn ascension_level(copies: u32) -> u32 {
    if copies >= 10 {
        10
    } else {
        copies.saturating_sub(1)
    }
}

fn characters(frame: &mut Frame, app: &App, cursor: usize) {
    let area = frame.area();
    frame.render_widget(
        Starfield {
            time: 0.8,
            intensity: 0.65,
        },
        area,
    );
    let names = app.owned_character_names();
    let panel = centered(area, area.width.min(108), area.height.min(32));
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(GOLD))
            .title(" CHARACTER MANAGEMENT ")
            .title_alignment(Alignment::Center),
        panel,
    );
    let inner = panel.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });
    if names.is_empty() {
        frame.render_widget(
            Paragraph::new("No character resonances recorded. Make a wish to begin.")
                .centered()
                .fg(DIM),
            inner,
        );
        return;
    }
    let name = &names[cursor];
    let item = catalog_item(name).unwrap();
    let profile = item_profile(&WishResult {
        item,
        rarity: item.rarity,
        featured: false,
        wish_number: 0,
    });
    let [carousel, body, help] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(23),
        Constraint::Length(2),
    ])
    .areas(inner);
    let prev = cursor
        .checked_sub(1)
        .and_then(|i| names.get(i))
        .map_or("", String::as_str);
    let next = names.get(cursor + 1).map_or("", String::as_str);
    frame.render_widget(
        Paragraph::new(format!("‹ {prev}     ✦ {name} ✦     {next} ›"))
            .centered()
            .fg(profile.color)
            .bold(),
        carousel,
    );
    let [stats, art, resonance] = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(40),
        Constraint::Percentage(30),
    ])
    .spacing(1)
    .areas(body);
    if !app.graphics
        && let Some(p) = app.gallery.get(name)
    {
        frame.render_widget(TerminalPortrait::new(&p.detail), art);
    }
    let copies = app.save.inventory.get(name).copied().unwrap_or(0);
    let level = ascension_level(copies);
    let road_row = |range: std::ops::RangeInclusive<u32>| {
        range
            .map(|n| {
                if n <= level {
                    Span::styled("■ ", Style::new().fg(GOLD).bold())
                } else {
                    Span::styled("□ ", Style::new().fg(Color::Rgb(55, 65, 85)))
                }
            })
            .collect::<Vec<_>>()
    };
    let weapon = app
        .save
        .equipment
        .get(name)
        .map_or("UNEQUIPPED", String::as_str);
    let weapon_color = app
        .save
        .equipment
        .get(name)
        .and_then(|weapon| catalog_item(weapon))
        .map_or(DIM, |item| rarity_color(item.rarity));
    let s = item.stats();
    let stats_text = Text::from(vec![
        Line::from(name.as_str()).fg(Color::White).bold(),
        Line::from(format!(
            "{}  •  {}  •  {}",
            item.rarity.stars(),
            item.element(),
            profile.weapon
        ))
        .fg(profile.color),
        Line::from(""),
        Line::from("COMBAT PROFILE").fg(GOLD).bold(),
        stat_line(&[("ATK", s.atk), ("DEF", s.def)], profile.color),
        stat_line(&[("HP", s.hp), ("SPD", s.spd)], profile.color),
        stat_line(
            &[("CRIT RATE", s.crit_rate), ("CRIT DMG", s.crit_dmg)],
            profile.color,
        ),
        stat_line(
            &[("ELEMENTAL ATK", s.elemental_atk), ("POISE", s.poise)],
            profile.color,
        ),
    ]);
    frame.render_widget(
        Paragraph::new(stats_text)
            .block(
                Block::new()
                    .borders(Borders::RIGHT)
                    .padding(Padding::uniform(1)),
            )
            .wrap(Wrap { trim: true }),
        stats,
    );
    frame.render_widget(
        Paragraph::new(Text::from(vec![
            Line::from("ASCENSION ROAD").fg(GOLD).bold(),
            Line::from(format!("N{level}  •  {copies} COPIES")).fg(Color::White),
            Line::from(road_row(6..=10)),
            Line::from(road_row(1..=5)),
            Line::from(""),
            Line::from("EQUIPPED WEAPON").fg(DIM),
            Line::from(weapon).fg(weapon_color).bold(),
            Line::from(""),
            Line::from("Press W to manage this character's armament.").fg(DIM),
        ]))
        .block(
            Block::new()
                .borders(Borders::LEFT)
                .padding(Padding::uniform(1)),
        )
        .wrap(Wrap { trim: true }),
        resonance,
    );
    frame.render_widget(
        Paragraph::new("← / → character  •  L roster list  •  W select weapon  •  ESC menu")
            .centered()
            .fg(DIM),
        help,
    );
}

fn character_quick_select(
    frame: &mut Frame,
    app: &App,
    cursor: usize,
    rarity: u8,
    element: usize,
    weapon: usize,
) {
    let area = frame.area();
    frame.render_widget(
        Starfield {
            time: 0.9,
            intensity: 0.65,
        },
        area,
    );
    let owned = app.owned_character_names();
    let names = filtered_characters(&owned, rarity, element, weapon);
    let panel = centered(area, area.width.min(86), area.height.min(31));
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(GOLD))
            .title(" CHARACTER ROSTER ")
            .title_alignment(Alignment::Center),
        panel,
    );
    let inner = panel.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });
    let [filters, list, help] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(22),
        Constraint::Length(2),
    ])
    .areas(inner);
    let rarity_label = match rarity {
        1 => "5★",
        2 => "4★",
        _ => "ALL",
    };
    frame.render_widget(
        Paragraph::new(format!(
            "[R] RARITY {rarity_label}   •   [E] ELEMENT {}   •   [T] WEAPON {}",
            CHARACTER_ELEMENTS[element], CHARACTER_WEAPONS[weapon]
        ))
        .centered()
        .fg(PURPLE),
        filters,
    );
    let lines = names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let item = catalog_item(name).unwrap();
            let style = if i == cursor {
                Style::new().fg(Color::Rgb(8, 12, 25)).bg(GOLD).bold()
            } else {
                Style::new().fg(rarity_color(item.rarity))
            };
            Line::from(format!(
                " {:<30} {}  {:<9}  {} ",
                name,
                item.rarity.stars(),
                item.element(),
                crate::simulation::character_weapon_type(item.name)
            ))
            .style(style)
        })
        .collect::<Vec<_>>();
    frame.render_widget(
        Paragraph::new(if lines.is_empty() {
            vec![Line::from("No owned characters match these filters.").fg(DIM)]
        } else {
            lines
        })
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(Color::Rgb(45, 55, 85))),
        ),
        list,
    );
    frame.render_widget(
        Paragraph::new(format!(
            "↑ / ↓ select  •  ENTER open  •  ESC / L return  •  {} shown",
            names.len()
        ))
        .centered()
        .fg(DIM),
        help,
    );
}

fn character_weapon_select(
    frame: &mut Frame,
    app: &App,
    character_cursor: usize,
    weapon_cursor: usize,
) {
    let area = frame.area();
    frame.render_widget(
        Starfield {
            time: 1.0,
            intensity: 0.65,
        },
        area,
    );
    let chars = app.owned_character_names();
    if chars.is_empty() {
        return;
    }
    let character = &chars[character_cursor];
    let weapons = app.compatible_weapon_names(character);
    let selected = weapons.get(weapon_cursor);
    let panel = centered(area, area.width.min(112), area.height.min(32));
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(GOLD))
            .title(format!(
                " EQUIP {} ",
                crate::simulation::character_weapon_type(character)
            ))
            .title_alignment(Alignment::Center),
        panel,
    );
    let inner = panel.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });
    let [body, help] = Layout::vertical([Constraint::Min(25), Constraint::Length(2)]).areas(inner);
    let [char_art, list, weapon_art] = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(40),
        Constraint::Percentage(30),
    ])
    .spacing(1)
    .areas(body);
    if !app.graphics {
        if let Some(p) = app.gallery.get(character) {
            frame.render_widget(TerminalPortrait::new(&p.detail), char_art);
        }
        if let Some(w) = selected.and_then(|n| app.gallery.get(n)) {
            frame.render_widget(TerminalPortrait::new(&w.detail), weapon_art);
        }
    }
    let current = app.save.equipment.get(character);
    let start = weapon_cursor
        .saturating_sub(8)
        .min(weapons.len().saturating_sub(18));
    let lines = weapons
        .iter()
        .enumerate()
        .skip(start)
        .take(18)
        .map(|(i, n)| {
            let holders = app.weapon_holders(n);
            let equipped = current.is_some_and(|equipped| equipped == n);
            let marker = if equipped {
                "◆"
            } else if holders.is_empty() {
                " "
            } else {
                "●"
            };
            let owned = app.save.inventory.get(n).copied().unwrap_or(0);
            let unequipped = owned.saturating_sub(holders.len() as u32);
            let suffix = format!("x{unequipped} UNEQUIPPED");
            let rarity = catalog_item(n).unwrap().rarity;
            let style = if i == weapon_cursor {
                Style::new().fg(Color::Rgb(8, 12, 25)).bg(GOLD).bold()
            } else if equipped {
                Style::new().fg(GOLD).bold()
            } else {
                Style::new().fg(rarity_color(rarity))
            };
            Line::from(format!("{marker} {}  {suffix}", n)).style(style)
        })
        .collect::<Vec<_>>();
    frame.render_widget(
        Paragraph::new(if lines.is_empty() {
            vec![Line::from("No compatible weapons owned").fg(DIM)]
        } else {
            lines
        })
        .centered()
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(PURPLE))
                .title(" AVAILABLE WEAPONS  •  ◆ CURRENT  •  ● EQUIPPED ELSEWHERE "),
        ),
        list,
    );
    frame.render_widget(
        Paragraph::new("↑ / ↓ weapon  •  ENTER equip  •  D unequip  •  ESC character")
            .centered()
            .fg(DIM),
        help,
    );
}

fn home(frame: &mut Frame, app: &App) {
    let area = frame.area();
    frame.render_widget(
        Starfield {
            time: app.save.total_wishes as f32 * 0.13,
            intensity: 1.0,
        },
        area,
    );
    let [header, hero, pity, actions, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(8),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(2),
    ])
    .areas(area);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            " ✦ ".fg(GOLD),
            "W I S H S I M".bold().fg(Color::White),
        ]))
        .centered()
        .block(
            Block::new()
                .borders(Borders::BOTTOM)
                .border_style(Style::new().fg(Color::Rgb(35, 48, 80))),
        ),
        header,
    );

    let hero_width = area.width.min(86);
    let [hero_area] = Layout::horizontal([Constraint::Length(hero_width)])
        .flex(Flex::Center)
        .areas(hero);
    let (hero_name, hero_subtitle, hero_quote, hero_color) = match app.banner {
        Banner::Astraea => (
            "A S T R A E A",
            "STARBOUND  •  CELESTIAL NAVIGATOR",
            "Every falling light remembers where it came from.",
            GOLD,
        ),
        Banner::Kaelis => (
            "K A E L I S",
            "ASHEN VANGUARD  •  FLAMEBOUND DUELIST",
            "Let the old world burn only where a new one can grow.",
            Color::Rgb(255, 105, 60),
        ),
        Banner::Seraphine => (
            "S E R A P H I N E",
            "VERDANT ORACLE  •  DREAMWOOD SEER",
            "The smallest seed dreams of becoming a forest.",
            Color::Rgb(105, 220, 105),
        ),
        Banner::Vaughn => (
            "V A U G H N",
            "VIOLET OATH  •  STORMBOUND KNIGHT",
            "Behind iron, even thunder learns to kneel.",
            Color::Rgb(170, 95, 255),
        ),
        Banner::Steven => (
            "S T E V E N",
            "AZURE SHADE  •  VEILFIRE SHINOBI",
            "By the time the flame is seen, the shadow has already moved.",
            Color::Rgb(65, 155, 255),
        ),
        Banner::Sergei => (
            "S E R G E I",
            "WINTERFANG  •  WOLFBOUND HUNTER",
            "The old winter bares its fangs for those it remembers.",
            Color::Rgb(105, 185, 255),
        ),
        Banner::Saif => (
            "S A I F",
            "DUNE SOVEREIGN  •  SANDWARD LANCER",
            "Every kingdom is only stone waiting to become sand.",
            Color::Rgb(239, 234, 187),
        ),
        Banner::Standard => (
            "E V E R L A S T I N G   A R C H I V E",
            "STANDARD RESONANCE  •  CHOSEN DESTINY",
            "Name the star you seek, and every wandering light brings it closer.",
            Color::Rgb(125, 205, 225),
        ),
        Banner::Weapon => (
            "I N C A R N A T E   A R M A M E N T S",
            "SELECTED SIGNATURE  •  EPITOMIZED PATH",
            "Set a path, and let fate sharpen its answer.",
            PURPLE,
        ),
    };
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(hero_color))
            .title(format!("  {}  ", app.banner.title().to_uppercase()))
            .title_alignment(Alignment::Center)
            .title_style(Style::new().fg(hero_color).bold()),
        hero_area,
    );
    let hero_inner = hero_area.inner(ratatui::layout::Margin {
        horizontal: 2,
        vertical: 1,
    });
    frame.render_widget(
        Paragraph::new(Text::from(vec![
            Line::from("✧     ✦       ·        ✧").fg(BLUE),
            Line::from(hero_name).style(Style::new().fg(hero_color).add_modifier(Modifier::BOLD)),
            Line::from(hero_subtitle).fg(Color::Rgb(225, 220, 255)),
            Line::from(""),
            Line::from(hero_quote)
                .italic()
                .fg(Color::Rgb(165, 175, 210)),
            Line::from(if app.banner == Banner::Weapon {
                format!(
                    "EPITOMIZED PATH: {}  •  {}/1 FATE  •  [P] change path",
                    app.save.weapon_pity.path.name(),
                    app.save.weapon_pity.fate_points
                )
            } else if app.banner == Banner::Standard {
                format!(
                    "CHOSEN DESTINY: {}  •  {}/1 FATE  •  [P] change target",
                    app.save.standard_pity.path.name(),
                    app.save.standard_pity.fate_points
                )
            } else {
                "← / →  CHANGE BANNER  •  [B] VIEW LIMITED ARCHIVE".into()
            })
            .fg(hero_color),
        ]))
        .alignment(Alignment::Center),
        hero_inner,
    );

    let pity_width = area.width.min(78);
    let [pity_area] = Layout::horizontal([Constraint::Length(pity_width)])
        .flex(Flex::Center)
        .areas(pity);
    let rows = Layout::vertical([Constraint::Length(2), Constraint::Length(2)]).split(pity_area);
    let (five_pity, five_cap, four_pity, guaranteed) = if app.banner == Banner::Weapon {
        (
            app.save.weapon_pity.five_star,
            80,
            app.save.weapon_pity.four_star,
            app.save.weapon_pity.guaranteed_featured || app.save.weapon_pity.fate_points > 0,
        )
    } else if app.banner == Banner::Standard {
        (
            app.save.standard_pity.five_star,
            90,
            app.save.standard_pity.four_star,
            app.save.standard_pity.fate_points > 0,
        )
    } else {
        (
            app.save.pity.five_star,
            90,
            app.save.pity.four_star,
            app.save.pity.guaranteed_five,
        )
    };
    frame.render_widget(
        Gauge::default()
            .block(
                Block::new()
                    .title(if app.banner == Banner::Weapon {
                        " 5★ Weapon Pity "
                    } else if app.banner == Banner::Standard {
                        " 5★ Standard Fate "
                    } else {
                        " 5★ Shared Event Pity "
                    })
                    .title_style(Style::new().fg(GOLD)),
            )
            .gauge_style(Style::new().fg(GOLD).bg(Color::Rgb(30, 28, 48)))
            .ratio(f64::from(five_pity) / f64::from(five_cap))
            .label(format!(
                "{} / {}{}",
                five_pity,
                five_cap,
                if guaranteed { "  GUARANTEED" } else { "" }
            )),
        rows[0],
    );
    frame.render_widget(
        Gauge::default()
            .block(
                Block::new()
                    .title(" 4★ Pity ")
                    .title_style(Style::new().fg(PURPLE)),
            )
            .gauge_style(Style::new().fg(PURPLE).bg(Color::Rgb(28, 25, 48)))
            .ratio(f64::from(four_pity) / 10.0)
            .label(format!("{} / 10", four_pity)),
        rows[1],
    );

    let wish_buttons = Line::from(vec![
        Span::styled(
            "  [1]  WISH ×1  ",
            Style::new()
                .fg(Color::Rgb(10, 15, 30))
                .bg(Color::Rgb(210, 225, 245))
                .bold(),
        ),
        Span::raw("   "),
        Span::styled(
            "  [0]  WISH ×10  ",
            Style::new().fg(Color::Rgb(35, 24, 5)).bg(GOLD).bold(),
        ),
        Span::raw("    "),
        Span::styled(
            " [H] HISTORY ",
            Style::new().fg(Color::White).bg(Color::Rgb(45, 52, 76)),
        ),
    ]);
    let archive_buttons = Line::from(vec![
        Span::raw("   "),
        Span::styled(
            " [I] INVENTORY ",
            Style::new().fg(Color::White).bg(Color::Rgb(40, 75, 72)),
        ),
        Span::raw("   "),
        Span::styled(
            " [C] ARCHIVE ",
            Style::new().fg(Color::White).bg(Color::Rgb(52, 48, 82)),
        ),
    ]);
    frame.render_widget(
        Paragraph::new(Text::from(vec![wish_buttons, archive_buttons]))
            .alignment(Alignment::Center)
            .block(Block::new().padding(Padding::vertical(1))),
        actions,
    );

    let mode = if app.graphics {
        "PROTOCOL GRAPHICS ✦"
    } else {
        "PORTABLE ANSI"
    };
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            format!(" {} ", mode).fg(BLUE),
            format!(
                "  {} wishes  •  {} unique items",
                app.save.total_wishes,
                app.save.inventory.len()
            )
            .fg(DIM),
            "    Q quit".fg(DIM),
        ]))
        .alignment(Alignment::Center),
        footer,
    );
}

fn banner_select(frame: &mut Frame, app: &App, cursor: usize) {
    let area = frame.area();
    frame.render_widget(
        Starfield {
            time: 0.0,
            intensity: 0.7,
        },
        area,
    );
    let panel = centered(area, area.width.min(104), area.height.min(32));
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(GOLD))
            .title(" FIVE-STAR WISH ARCHIVE ")
            .title_alignment(Alignment::Center)
            .title_style(Style::new().fg(GOLD).bold()),
        panel,
    );
    let inner = panel.inner(ratatui::layout::Margin {
        horizontal: 2,
        vertical: 1,
    });
    let [heading, grid, help] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(20),
        Constraint::Length(2),
    ])
    .areas(inner);
    frame.render_widget(
        Paragraph::new("Choose a limited event record or the separately tracked Standard Archive.")
            .centered()
            .fg(DIM),
        heading,
    );
    let rows = Layout::vertical([
        Constraint::Percentage(34),
        Constraint::Percentage(33),
        Constraint::Percentage(33),
    ])
    .spacing(1)
    .split(grid);
    for (index, banner) in Banner::SELECTOR.iter().enumerate() {
        let columns = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .spacing(1)
        .split(rows[index / 3]);
        let card = columns[index % 3];
        let selected = index == cursor;
        let current = *banner == app.banner;
        let item = if *banner == Banner::Standard {
            standard_character(app.save.standard_pity.path)
        } else {
            crate::simulation::featured_character(*banner)
        };
        let profile = item_profile(&WishResult {
            item,
            rarity: Rarity::Five,
            featured: true,
            wish_number: 0,
        });
        let style = if selected {
            Style::new().fg(Color::Rgb(8, 12, 25)).bg(GOLD).bold()
        } else {
            Style::new().fg(profile.color).bg(Color::Rgb(8, 12, 28))
        };
        frame.render_widget(
            Paragraph::new(Text::from(vec![
                Line::from(item.name).bold(),
                Line::from(profile.title),
                Line::from(format!("{}  •  {}", profile.element, profile.weapon)),
                Line::from(if current {
                    "✦ CURRENT RECORD"
                } else if *banner == Banner::Standard {
                    "STANDARD  •  1 FATE"
                } else {
                    "★★★★★"
                }),
            ]))
            .alignment(Alignment::Center)
            .style(style)
            .block(Block::new().borders(Borders::ALL).border_style(style)),
            card,
        );
    }
    frame.render_widget(
        Paragraph::new("← ↑ ↓ → select  •  ENTER open banner  •  ESC / B return")
            .centered()
            .fg(DIM),
        help,
    );
}

fn weapon_select(frame: &mut Frame, app: &App, cursor: usize, preview: bool) {
    let area = frame.area();
    frame.render_widget(
        Starfield {
            time: 0.0,
            intensity: 0.7,
        },
        area,
    );
    let panel = centered(area, area.width.min(94), area.height.min(31));
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(GOLD))
            .title(" INCARNATE PATH ")
            .title_alignment(Alignment::Center)
            .title_style(Style::new().fg(GOLD).bold()),
        panel,
    );
    let inner = panel.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });
    let [body, help] = Layout::vertical([Constraint::Min(20), Constraint::Length(2)]).areas(inner);
    let [list, art] = Layout::horizontal([Constraint::Percentage(46), Constraint::Percentage(54)])
        .spacing(2)
        .areas(body);
    let selected = weapon_for_path(crate::model::WeaponPath::ALL[cursor]);
    let rows = crate::model::WeaponPath::ALL
        .iter()
        .enumerate()
        .map(|(index, path)| {
            let marker = if *path == app.save.weapon_pity.path {
                "✦"
            } else {
                " "
            };
            let style = if index == cursor {
                Style::new().fg(Color::Rgb(8, 12, 25)).bg(GOLD).bold()
            } else {
                Style::new().fg(rarity_color(Rarity::Five))
            };
            Line::from(format!(" {marker} {:<25}", path.name())).style(style)
        })
        .collect::<Vec<_>>();
    frame.render_widget(
        Paragraph::new(rows).block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(PURPLE))
                .title(" SELECTED DESTINY "),
        ),
        list,
    );
    let result = WishResult {
        item: selected,
        rarity: Rarity::Five,
        featured: true,
        wish_number: 0,
    };
    let profile = item_profile(&result);
    let art_inner = art.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_style(Style::new().fg(profile.color))
            .title(format!(" {} ", selected.name)),
        art,
    );
    if !preview {
        if !app.graphics
            && let Some(portrait) = app.gallery.get(selected.name)
        {
            frame.render_widget(TerminalPortrait::new(&portrait.detail), art_inner);
        }
    } else {
        let stats = selected.stats();
        frame.render_widget(
            Paragraph::new(Text::from(vec![
                Line::from("WEAPON RECORD").fg(GOLD).bold(),
                Line::from(selected.name).fg(Color::White).bold(),
                Line::from(format!("{}  •  {}", selected.rarity.stars(), selected.kind))
                    .fg(rarity_color(selected.rarity)),
                Line::from(""),
                stat_line(
                    &[("ATK", stats.atk), ("ELEMENTAL ATK", stats.elemental_atk)],
                    profile.color,
                ),
                stat_line(
                    &[("CRIT RATE", stats.crit_rate), ("CRIT DMG", stats.crit_dmg)],
                    profile.color,
                ),
                Line::from(""),
                Line::from(profile.title).fg(profile.color),
                Line::from(profile.lore).fg(DIM),
            ]))
            .wrap(Wrap { trim: true })
            .block(Block::new().padding(Padding::uniform(1))),
            art_inner,
        );
    }
    frame.render_widget(
        Paragraph::new(
            "↑ / ↓ browse  •  V art / details  •  ENTER choose (resets Fate)  •  ESC return",
        )
        .centered()
        .fg(DIM),
        help,
    );
}

fn character_archive(frame: &mut Frame, app: &App, cursor: usize) {
    let area = frame.area();
    frame.render_widget(
        Starfield {
            time: 0.0,
            intensity: 0.65,
        },
        area,
    );
    let panel = centered(area, area.width.min(104), area.height.min(32));
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(GOLD))
            .title(" CHARACTER ARCHIVE ")
            .title_alignment(Alignment::Center)
            .title_style(Style::new().fg(GOLD).bold()),
        panel,
    );
    let inner = panel.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });
    let [heading, grid, help] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(22),
        Constraint::Length(2),
    ])
    .areas(inner);
    let characters = all_characters();
    let owned = characters
        .iter()
        .filter(|item| app.save.inventory.contains_key(item.name))
        .count();
    frame.render_widget(
        Paragraph::new(format!(
            "RESONANCES RECORDED  {owned} / {}",
            characters.len()
        ))
        .centered()
        .fg(DIM),
        heading,
    );
    let page_start = (cursor / 6) * 6;
    let rows = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
        .spacing(1)
        .split(grid);
    for (slot, item) in characters.iter().skip(page_start).take(6).enumerate() {
        let columns = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .spacing(1)
        .split(rows[slot / 3]);
        let card = columns[slot % 3];
        let index = page_start + slot;
        let selected = index == cursor;
        let unlocked = app.save.inventory.contains_key(item.name);
        let border = if selected {
            GOLD
        } else if unlocked {
            rarity_color(item.rarity)
        } else {
            Color::Rgb(55, 62, 78)
        };
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(border)),
            card,
        );
        let card_inner = card.inner(Margin {
            horizontal: 1,
            vertical: 0,
        });
        let [portrait_area, name_area] =
            Layout::vertical([Constraint::Min(7), Constraint::Length(2)]).areas(card_inner);
        if unlocked {
            if let Some(portrait) = app.gallery.get(item.name) {
                frame.render_widget(TerminalPortrait::new(&portrait.archive), portrait_area);
            }
        } else {
            frame.render_widget(
                Paragraph::new("\n   ?")
                    .centered()
                    .style(Style::new().fg(Color::Rgb(70, 75, 88)).bold()),
                portrait_area,
            );
        }
        let label = if unlocked {
            item.rarity.stars()
        } else {
            "LOCKED"
        };
        frame.render_widget(
            Paragraph::new(Text::from(vec![
                Line::from(item.name)
                    .fg(if unlocked { Color::White } else { DIM })
                    .bold(),
                Line::from(label).fg(border),
            ]))
            .centered(),
            name_area,
        );
    }
    frame.render_widget(
        Paragraph::new(format!(
            "← ↑ ↓ → browse  •  page {} / {}  •  ESC / C return",
            cursor / 6 + 1,
            characters.len().div_ceil(6)
        ))
        .centered()
        .fg(DIM),
        help,
    );
}

fn flight(frame: &mut Frame, elapsed: f32, results: &[WishResult]) {
    let area = frame.area();
    let progress = (elapsed / 1.65).clamp(0.0, 1.0);
    let rarity = results
        .iter()
        .map(|r| r.rarity)
        .max_by_key(|r| match r {
            Rarity::Three => 3,
            Rarity::Four => 4,
            Rarity::Five => 5,
        })
        .unwrap_or(Rarity::Three);
    let mystery_color = Color::Rgb(195, 225, 255);
    let rarity_color = rarity_color(rarity);
    // A brief purple false glint appears before the real color settles. Even a
    // blue pull can look promising for a moment, while the actual rarity starts
    // bleeding through early enough to invite second-guessing.
    let decoy = (1.0 - ((progress - 0.33) / 0.16).abs()).clamp(0.0, 1.0) * 0.24;
    let teased_color = mix_f32(mystery_color, PURPLE, decoy);
    let reveal_blend = smoothstep(0.42, 0.86, progress);
    let color = mix_f32(teased_color, rarity_color, reveal_blend);
    frame.render_widget(
        Starfield {
            time: elapsed,
            intensity: 1.4,
        },
        area,
    );

    let eased = 1.0 - (1.0 - progress).powi(3);
    let star_x = (area.width as f32 * (0.12 + eased * 0.74)) as u16;
    let star_y = (area.height as f32 * (0.72 - eased * 0.48)) as u16;
    let buffer = frame.buffer_mut();
    for tail in 0..22u16 {
        let x = star_x.saturating_sub(tail * 2);
        let y = (star_y + tail / 3).min(area.bottom() - 1);
        if x > area.left() && x < area.right() {
            let fade = 255u8.saturating_sub((tail * 9) as u8);
            let tail_color = mix(color, Color::Rgb(30, 55, 100), fade);
            buffer[(x, y)]
                .set_symbol(if tail < 4 {
                    "━"
                } else if tail < 12 {
                    "─"
                } else {
                    "·"
                })
                .set_fg(tail_color);
        }
    }
    if star_x < area.right() && star_y < area.bottom() {
        buffer[(star_x, star_y)]
            .set_symbol("✹")
            .set_fg(color)
            .set_style(Style::new().add_modifier(Modifier::BOLD));
        for (x, y, symbol) in [
            (star_x.saturating_sub(1), star_y, "━"),
            (star_x.saturating_add(1), star_y, "━"),
            (star_x, star_y.saturating_sub(1), "✦"),
            (star_x, star_y.saturating_add(1), "✦"),
        ] {
            if x >= area.left() && x < area.right() && y >= area.top() && y < area.bottom() {
                buffer[(x, y)].set_symbol(symbol).set_fg(color);
            }
        }
    }

    if progress > 0.86 {
        let flash = ((progress - 0.86) / 0.14).clamp(0.0, 1.0);
        let white = (flash * 210.0) as u8;
        frame.render_widget(Block::new().bg(Color::Rgb(white, white, white)), area);
    }

    let caption = if progress < 0.70 {
        "A LIGHT CROSSES THE FIRMAMENT..."
    } else if rarity == Rarity::Five {
        "THE SKY ANSWERS..."
    } else {
        "ITS TRUE COLOR EMERGES..."
    };
    let caption_area = Rect::new(area.x, area.bottom() - 4, area.width, 2);
    frame.render_widget(
        Paragraph::new(caption)
            .centered()
            .style(Style::new().fg(color).bold()),
        caption_area,
    );
    if results.len() > 1 {
        frame.render_widget(
            Paragraph::new("[S] SKIP ALL").right_aligned().fg(DIM),
            Rect::new(area.x, area.bottom() - 2, area.width - 2, 1),
        );
    }
}

fn reveal(
    frame: &mut Frame,
    elapsed: f32,
    result: &WishResult,
    index: usize,
    total: usize,
    gallery: &CharacterGallery,
    portable_art: bool,
) {
    let area = frame.area();
    let t = (elapsed / 1.05).clamp(0.0, 1.0);
    let color = rarity_color(result.rarity);
    frame.render_widget(
        Starfield {
            time: elapsed * 0.25,
            intensity: if result.rarity == Rarity::Five {
                2.3
            } else {
                1.2
            },
        },
        area,
    );

    let width = 72.min(area.width - 8);
    let height = 36.min(area.height - 4);
    let card = centered(area, width, height);
    frame.render_widget(Clear, card);
    let border = if t < 0.12 { Color::White } else { color };
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(border))
            .bg(Color::Rgb(10, 12, 28))
            .title(format!(" REVELATION  {}/{} ", index + 1, total))
            .title_alignment(Alignment::Center),
        card,
    );
    let inner = card.inner(ratatui::layout::Margin {
        horizontal: 2,
        vertical: 1,
    });
    let [sprite_area, label_area] =
        Layout::vertical([Constraint::Min(8), Constraint::Length(6)]).areas(inner);
    if portable_art && let Some(portrait) = gallery.get(result.item.name) {
        frame.render_widget(TerminalPortrait::new(&portrait.reveal), sprite_area);
    } else if result.item.kind != "Character" && gallery.get(result.item.name).is_none() {
        frame.render_widget(
            Paragraph::new(
                item_sprite(result)
                    .iter()
                    .map(|line| Line::from(*line).style(Style::new().fg(color).bold()))
                    .collect::<Vec<_>>(),
            )
            .centered(),
            sprite_area,
        );
    }

    let star_interval = 0.24;
    let shown_stars = ((elapsed - 0.18) / star_interval)
        .ceil()
        .clamp(0.0, result.rarity.value() as f32) as usize;
    let color_start = 0.18 + result.rarity.value() as f32 * star_interval;
    let star_color = mix_f32(
        Color::White,
        color,
        smoothstep(color_start, color_start + 0.5, elapsed),
    );
    let star_top = "╲│╱  ".repeat(shown_stars);
    let star_mid = "─★─  ".repeat(shown_stars);
    let star_bottom = "╱│╲  ".repeat(shown_stars);
    let mut lines = vec![
        Line::from(result.item.name).style(Style::new().fg(Color::White).bold()),
        Line::from(star_top).style(Style::new().fg(star_color).bold()),
        Line::from(star_mid).style(Style::new().fg(star_color).bold()),
        Line::from(star_bottom).style(Style::new().fg(star_color).bold()),
    ];
    if result.featured {
        lines.push(Line::from("✦  FEATURED  ✦").style(Style::new().fg(GOLD).bold()));
    }
    frame.render_widget(
        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        label_area,
    );
    let reveal_help = if total > 1 {
        "SPACE next  •  S skip all  •  Q quit"
    } else {
        "SPACE skip  •  Q quit"
    };
    frame.render_widget(
        Paragraph::new(reveal_help).centered().fg(DIM),
        Rect::new(area.x, area.bottom() - 2, area.width, 1),
    );
}

fn five_star_intro(frame: &mut Frame, elapsed: f32, total: usize, index: usize) {
    let area = frame.area();
    let progress = (elapsed / 3.4).clamp(0.0, 1.0);
    frame.render_widget(Block::new().bg(Color::Rgb(1, 1, 7)), area);
    frame.render_widget(
        Starfield {
            time: elapsed * 0.12,
            intensity: 0.18,
        },
        area,
    );

    let star_area = centered(area, area.width.min(64), 7);
    let mut star_top = Vec::new();
    let mut star_middle = Vec::new();
    let mut star_bottom = Vec::new();
    for number in 0..5 {
        let birth = 0.14 + number as f32 * 0.12;
        let glow = smoothstep(birth, birth + 0.13, progress);
        let color = mix_f32(Color::Rgb(25, 25, 45), GOLD, glow);
        let style = Style::new().fg(color).bold();
        star_top.push(Span::styled(" ╲│╱ ", style));
        star_middle.push(Span::styled(
            if glow > 0.82 {
                " ─★─ "
            } else {
                " ─✦─ "
            },
            style,
        ));
        star_bottom.push(Span::styled(" ╱│╲ ", style));
        if number < 4 {
            star_top.push(Span::raw("   "));
            star_middle.push(Span::raw("   "));
            star_bottom.push(Span::raw("   "));
        }
    }
    let omen = if progress < 0.68 {
        "SOMETHING STIRS BEYOND THE VEIL..."
    } else if progress < 0.86 {
        "A LEGEND STEPS FROM THE LIGHT..."
    } else {
        "THE HEAVENS OPEN"
    };
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(if progress < 0.12 {
                ""
            } else {
                "THE CONSTELLATION HOLDS ITS BREATH"
            })
            .fg(Color::Rgb(115, 105, 145)),
            Line::from(""),
            Line::from(star_top),
            Line::from(star_middle),
            Line::from(star_bottom),
            Line::from(omen).style(Style::new().fg(GOLD).bold()),
        ])
        .centered(),
        star_area,
    );

    if progress > 0.88 {
        let flash = smoothstep(0.88, 1.0, progress);
        let value = (flash * 245.0) as u8;
        frame.render_widget(Block::new().bg(Color::Rgb(value, value, value)), area);
    }
    let help = if total > 1 {
        format!(
            "Five-star revelation {}/{}  •  SPACE advance  •  S skip all",
            index + 1,
            total
        )
    } else {
        "SPACE advance".into()
    };
    frame.render_widget(
        Paragraph::new(help).centered().fg(DIM),
        Rect::new(area.x, area.bottom() - 2, area.width, 1),
    );
}

#[allow(dead_code)]
fn render_banner_art(frame: &mut Frame, area: Rect, banner: Banner) {
    let character = match banner {
        Banner::Astraea => Some("Astraea, Starbound"),
        Banner::Kaelis => Some("Kaelis, Ashen Vanguard"),
        Banner::Seraphine => Some("Seraphine, Verdant Oracle"),
        Banner::Vaughn => Some("Vaughn, Violet Oath"),
        Banner::Steven => Some("Steven, Azure Shade"),
        Banner::Sergei => Some("Sergei, Winterfang"),
        Banner::Saif => Some("Saif, Dune Sovereign"),
        Banner::Standard => None,
        Banner::Weapon => None,
    };
    if let Some(sprite) = character.and_then(character_sprite) {
        render_halfblock_sprite(frame, area, &sprite);
        return;
    }

    let sword = [
        "    ◇    ",
        "   ╱│╲   ",
        "    │    ",
        "  ──┼──  ",
        "    †    ",
    ];
    let book = [
        "   ·✦·   ",
        " ╭─────╮ ",
        " │  ◈  │ ",
        " ╰─────╯ ",
        "   ·✧·   ",
    ];
    let lines = sword
        .iter()
        .zip(book)
        .map(|(left, right)| {
            Line::from(vec![
                Span::styled(*left, Style::new().fg(GOLD).bold()),
                Span::raw("   "),
                Span::styled(right, Style::new().fg(PURPLE).bold()),
            ])
        })
        .collect::<Vec<_>>();
    frame.render_widget(Paragraph::new(lines).centered(), area);
}

#[allow(dead_code)]
fn render_halfblock_sprite(frame: &mut Frame, area: Rect, sprite: &PixelSprite) {
    let color_for = |pixel| match pixel {
        'K' => Some(sprite.outline),
        'H' => Some(sprite.hair),
        'S' => Some(sprite.skin),
        'O' => Some(sprite.outfit),
        'E' => Some(sprite.element),
        _ => None,
    };
    let mut lines = Vec::new();
    for rows in sprite.pixels.chunks(2) {
        let top = rows[0].chars().collect::<Vec<_>>();
        let bottom = rows
            .get(1)
            .map_or_else(|| vec!['.'; top.len()], |row| row.chars().collect());
        let spans = top
            .iter()
            .zip(bottom)
            .map(|(&top, bottom)| match (color_for(top), color_for(bottom)) {
                (Some(fg), Some(bg)) => Span::styled("▀▀", Style::new().fg(fg).bg(bg)),
                (Some(fg), None) => Span::styled("▀▀", Style::new().fg(fg)),
                (None, Some(fg)) => Span::styled("▄▄", Style::new().fg(fg)),
                (None, None) => Span::raw("  "),
            })
            .collect::<Vec<_>>();
        lines.push(Line::from(spans));
    }
    frame.render_widget(Paragraph::new(lines).centered(), area);
}

fn summary(frame: &mut Frame, results: &[WishResult], selected: usize) {
    let area = frame.area();
    frame.render_widget(
        Starfield {
            time: 2.0,
            intensity: 0.7,
        },
        area,
    );
    let [title, cards, detail, help] = Layout::vertical([
        Constraint::Length(4),
        Constraint::Min(8),
        Constraint::Length(6),
        Constraint::Length(2),
    ])
    .areas(area);
    frame.render_widget(
        Paragraph::new("W I S H   R E S U L T S")
            .centered()
            .style(Style::new().fg(GOLD).bold())
            .block(Block::new().padding(Padding::top(1))),
        title,
    );

    let count = results.len() as u16;
    let card_width = if count == 1 {
        18
    } else {
        ((area.width - 4) / count).clamp(7, 12)
    };
    let total_width = card_width * count;
    let row = centered(cards, total_width.min(cards.width), 7);
    let constraints = vec![Constraint::Ratio(1, count as u32); count as usize];
    let columns = Layout::horizontal(constraints).split(row);
    for (i, result) in results.iter().enumerate() {
        let color = rarity_color(result.rarity);
        let style = if i == selected {
            Style::new().fg(color).add_modifier(Modifier::BOLD)
        } else {
            Style::new().fg(Color::Rgb(125, 135, 165))
        };
        let label = if result.item.kind == "Character" {
            "◇"
        } else {
            "†"
        };
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(label),
                Line::from(""),
                Line::from(result.rarity.stars()),
                Line::from(short_name(result.item.name, card_width as usize - 2)),
            ])
            .centered()
            .style(style)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(if i == selected {
                        BorderType::Double
                    } else {
                        BorderType::Plain
                    })
                    .border_style(style),
            ),
            columns[i],
        );
    }
    let chosen = &results[selected];
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(chosen.item.name).style(Style::new().fg(Color::White).bold()),
            Line::from(format!(
                "{}  •  Wish #{}{}",
                chosen.item.kind,
                chosen.wish_number,
                if chosen.featured {
                    "  •  FEATURED"
                } else {
                    ""
                }
            ))
            .fg(rarity_color(chosen.rarity)),
        ])
        .centered()
        .block(
            Block::new()
                .borders(Borders::TOP)
                .border_style(Style::new().fg(Color::Rgb(38, 50, 80)))
                .padding(Padding::top(1)),
        ),
        detail,
    );
    frame.render_widget(
        Paragraph::new("←/→ select  •  ENTER inspect  •  SPACE return  •  Q quit")
            .centered()
            .fg(DIM),
        help,
    );
}

fn detail(
    frame: &mut Frame,
    result: &WishResult,
    inventory_count: Option<u32>,
    gallery: &CharacterGallery,
    portable_art: bool,
) {
    let area = frame.area();
    let profile = item_profile(result);
    frame.render_widget(
        Starfield {
            time: result.wish_number as f32 * 0.07,
            intensity: 0.9,
        },
        area,
    );

    let panel = centered(area, area.width.min(112), area.height.min(38));
    frame.render_widget(Clear, panel);
    let border_color = if result.item.kind == "Character" {
        profile.color
    } else {
        rarity_color(result.rarity)
    };
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(border_color))
            .bg(Color::Rgb(8, 11, 27))
            .title(format!(
                " ✦  {}  {} ",
                result.item.name,
                result.rarity.stars()
            ))
            .title_style(Style::new().fg(Color::White).bold())
            .title_alignment(Alignment::Center),
        panel,
    );
    let inner = panel.inner(ratatui::layout::Margin {
        horizontal: 2,
        vertical: 1,
    });
    let [art_area, info_area] =
        Layout::horizontal([Constraint::Percentage(42), Constraint::Percentage(58)])
            .spacing(2)
            .areas(inner);

    let art_inner = art_area.inner(ratatui::layout::Margin {
        horizontal: 1,
        vertical: 2,
    });
    frame.render_widget(
        Block::new()
            .borders(Borders::RIGHT)
            .border_style(Style::new().fg(Color::Rgb(38, 50, 80))),
        art_area,
    );
    if portable_art && let Some(portrait) = gallery.get(result.item.name) {
        frame.render_widget(TerminalPortrait::new(&portrait.detail), art_inner);
    } else if result.item.kind != "Character" && gallery.get(result.item.name).is_none() {
        let art_lines: Vec<Line> = item_sprite(result)
            .iter()
            .enumerate()
            .map(|(index, line)| {
                let color = match index % 3 {
                    0 => profile.color,
                    1 => Color::White,
                    _ => profile.accent,
                };
                Line::from(*line).style(Style::new().fg(color).bold())
            })
            .collect();
        frame.render_widget(
            Paragraph::new(art_lines).alignment(Alignment::Center),
            art_inner,
        );
    }

    let featured = if result.featured {
        "  ✦ FEATURED"
    } else {
        ""
    };
    let info = Text::from(vec![
        Line::from(result.item.name).style(Style::new().fg(Color::White).bold()),
        Line::from(format!("{}{}", profile.title, featured)).fg(border_color),
        Line::from(""),
        Line::from(vec![
            "ELEMENT  ".fg(DIM),
            result.item.element().fg(profile.color).bold(),
        ]),
        Line::from(vec![
            "WEAPON   ".fg(DIM),
            profile.weapon.fg(Color::Rgb(225, 230, 245)),
        ]),
        Line::from(vec![
            "RARITY   ".fg(DIM),
            result.rarity.stars().fg(rarity_color(result.rarity)).bold(),
        ]),
        Line::from(""),
        Line::from("COMBAT PROFILE").style(Style::new().fg(GOLD).bold()),
        stat_line(
            &[
                ("ATK", result.item.stats().atk),
                ("DEF", result.item.stats().def),
            ],
            border_color,
        ),
        stat_line(
            &[
                ("HP", result.item.stats().hp),
                ("SPD", result.item.stats().spd),
            ],
            border_color,
        ),
        stat_line(
            &[
                ("CRIT RATE", result.item.stats().crit_rate),
                ("CRIT DMG", result.item.stats().crit_dmg),
            ],
            border_color,
        ),
        stat_line(
            &[
                ("ELEMENTAL ATK", result.item.stats().elemental_atk),
                ("POISE", result.item.stats().poise),
            ],
            border_color,
        ),
        Line::from(""),
        Line::from("ARCHIVE LORE").style(Style::new().fg(GOLD).bold()),
        Line::from(profile.lore).fg(Color::Rgb(185, 195, 220)),
        Line::from(""),
        Line::from(format!("“{}”", profile.quote)).style(Style::new().fg(profile.accent).italic()),
        Line::from(""),
        Line::from(inventory_count.map_or_else(
            || format!("Obtained on wish #{}", result.wish_number),
            |count| format!("Owned: {count}"),
        ))
        .fg(DIM),
    ]);
    frame.render_widget(
        Paragraph::new(info)
            .wrap(Wrap { trim: true })
            .block(Block::new().padding(Padding::uniform(1))),
        info_area,
    );
    let return_label = if inventory_count.is_some() {
        "return to inventory"
    } else {
        "return to results"
    };
    frame.render_widget(
        Paragraph::new(format!("ESC / ENTER  {return_label}  •  Q quit"))
            .centered()
            .fg(DIM),
        Rect::new(area.x, area.bottom() - 2, area.width, 1),
    );
}

fn stat_line(stats: &[(&str, u16)], color: Color) -> Line<'static> {
    let mut spans = Vec::new();
    for (index, (label, value)) in stats.iter().enumerate() {
        if index > 0 {
            spans.push(Span::raw("   "));
        }
        spans.push(Span::styled(format!("{label} "), Style::new().fg(DIM)));
        spans.push(Span::styled(
            value.to_string(),
            Style::new().fg(color).bold(),
        ));
    }
    Line::from(spans)
}

fn inventory(frame: &mut Frame, app: &App, cursor: usize, selected: &BTreeSet<String>) {
    let area = frame.area();
    frame.render_widget(
        Starfield {
            time: 1.0,
            intensity: 0.55,
        },
        area,
    );
    let panel = centered(area, area.width.min(94), area.height.min(34));
    let [header, list, footer] = Layout::vertical([
        Constraint::Length(4),
        Constraint::Min(8),
        Constraint::Length(4),
    ])
    .areas(panel.inner(ratatui::layout::Margin {
        horizontal: 2,
        vertical: 1,
    }));
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::new().fg(Color::Rgb(80, 205, 185)))
            .bg(Color::Rgb(7, 12, 25))
            .title(" INVENTORY ")
            .title_alignment(Alignment::Center),
        panel,
    );
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(format!(
                "{} shown  •  {} unique items  •  {} selected",
                app.inventory_names().len(),
                app.save.inventory.len(),
                selected.len()
            ))
            .style(Style::new().fg(Color::White).bold()),
            Line::from("Keep what matters. The Archive remembers everything else.")
                .italic()
                .fg(DIM),
        ])
        .centered(),
        header,
    );

    let inventory_names = app.inventory_names();
    if inventory_names.is_empty() {
        frame.render_widget(
            Paragraph::new("Inventory is empty.\n\nPress ESC to return.")
                .centered()
                .fg(DIM),
            list,
        );
    } else {
        let visible = list.height as usize;
        let start = cursor.saturating_sub(visible.saturating_sub(1));
        let lines = inventory_names
            .iter()
            .enumerate()
            .skip(start)
            .take(visible)
            .map(|(index, name)| {
                let count = app.save.inventory.get(name).copied().unwrap_or(0);
                let item = catalog_item(name);
                let focused = index == cursor;
                let checked = if selected.contains(name) {
                    "[×]"
                } else {
                    "[ ]"
                };
                let rarity = item.map_or("???", |item| item.rarity.stars());
                let kind = item.map_or("Unknown", |item| item.kind);
                let marker = if focused { "›" } else { " " };
                let style = if focused {
                    Style::new().fg(Color::Rgb(15, 24, 35)).bg(GOLD).bold()
                } else if selected.contains(name) {
                    Style::new().fg(Color::Rgb(105, 235, 205))
                } else {
                    Style::new().fg(Color::Rgb(205, 215, 235))
                };
                Line::from(format!(
                    "{marker} {checked}  {name:<27} {kind:<10} {rarity:<5} ×{count}"
                ))
                .style(style)
            })
            .collect::<Vec<_>>();
        frame.render_widget(Paragraph::new(lines), list);
    }
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::styled(" [S] SORT ", Style::new().fg(Color::Rgb(10, 20, 32)).bg(GOLD).bold()),
                Span::styled(format!(" {} ", app.inventory_sort.label()), Style::new().fg(GOLD)),
                Span::raw("  "),
                Span::styled(" [F] TYPE ", Style::new().fg(Color::Rgb(8, 25, 26)).bg(Color::Rgb(80, 205, 185)).bold()),
                Span::styled(format!(" {} ", app.inventory_kind.label()), Style::new().fg(Color::Rgb(105, 235, 205))),
                Span::raw("  "),
                Span::styled(" [E] ELEMENT ", Style::new().fg(Color::Rgb(15, 15, 32)).bg(PURPLE).bold()),
                Span::styled(format!(" {} ", ELEMENT_FILTERS[app.inventory_element]), Style::new().fg(PURPLE)),
            ]),
            Line::from("↑/↓ move  •  SPACE select  •  A select all  •  ENTER inspect  •  D delete  •  ESC return").fg(DIM),
        ])
        .centered()
        .block(
            Block::new()
                .borders(Borders::TOP)
                .border_style(Style::new().fg(Color::Rgb(38, 60, 78))),
        ),
        footer,
    );
}

fn confirm_inventory_delete(frame: &mut Frame, targets: &[String]) {
    let area = frame.area();
    let dialog = centered(area, area.width.min(62), 10);
    frame.render_widget(Clear, dialog);
    let subject = if targets.len() == 1 {
        targets[0].clone()
    } else {
        format!("{} selected inventory entries", targets.len())
    };
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("DELETE FROM INVENTORY?")
                .style(Style::new().fg(Color::Rgb(255, 105, 95)).bold()),
            Line::from(""),
            Line::from(subject).style(Style::new().fg(Color::White).bold()),
            Line::from("Pity and wish history are preserved.").fg(DIM),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    " [Y] DELETE ",
                    Style::new()
                        .fg(Color::White)
                        .bg(Color::Rgb(150, 40, 45))
                        .bold(),
                ),
                Span::raw("     "),
                Span::styled(
                    " [N] CANCEL ",
                    Style::new().fg(Color::White).bg(Color::Rgb(45, 65, 80)),
                ),
            ]),
        ])
        .centered()
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::new().fg(Color::Rgb(255, 95, 85)))
                .bg(Color::Rgb(18, 8, 15))
                .padding(Padding::uniform(1)),
        ),
        dialog,
    );
}

fn confirm_quit(frame: &mut Frame) {
    let area = frame.area();
    let dialog = centered(area, area.width.min(54), 9);
    frame.render_widget(Clear, dialog);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from("EXIT WISHSIM?").style(Style::new().fg(GOLD).bold()),
            Line::from(""),
            Line::from("Your progress is already saved.").fg(DIM),
            Line::from(""),
            Line::from("Press Y or ENTER to exit").style(Style::new().fg(Color::White).bold()),
            Line::from("Press N or ESC to return to the game").fg(Color::Rgb(180, 195, 220)),
        ])
        .centered()
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::new().fg(GOLD))
                .bg(Color::Rgb(12, 10, 22))
                .padding(Padding::uniform(1)),
        ),
        dialog,
    );
}

fn history(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let panel = centered(area, area.width.min(84), area.height.min(30));
    let mut lines = vec![
        Line::from("Most recent wishes").style(Style::new().fg(GOLD).bold()),
        Line::from(""),
    ];
    if app.save.history.is_empty() {
        lines.push(Line::from("No stars have fallen yet.").fg(DIM));
    }
    for wish in app
        .save
        .history
        .iter()
        .rev()
        .take(panel.height.saturating_sub(6) as usize)
    {
        lines.push(Line::from(vec![
            format!("#{:<5} ", wish.wish_number).fg(DIM),
            wish.rarity.stars().fg(rarity_color(wish.rarity)),
            format!("  {}", wish.name).fg(Color::White),
            if wish.featured {
                "  ✦".fg(GOLD)
            } else {
                Span::raw("")
            },
        ]));
    }
    frame.render_widget(
        Paragraph::new(lines).block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::new().fg(BLUE))
                .padding(Padding::uniform(1)),
        ),
        panel,
    );
    frame.render_widget(
        Paragraph::new("ESC return  •  Q quit").centered().fg(DIM),
        Rect::new(area.x, area.bottom() - 2, area.width, 1),
    );
}

struct TerminalPortrait<'a> {
    raster: &'a TerminalRaster,
}

impl<'a> TerminalPortrait<'a> {
    const fn new(raster: &'a TerminalRaster) -> Self {
        Self { raster }
    }
}

impl Widget for TerminalPortrait<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let image_height = self.raster.height.div_ceil(2);
        let visible_width = self.raster.width.min(area.width);
        let visible_height = image_height.min(area.height);
        let origin_x = area.x + area.width.saturating_sub(visible_width) / 2;
        let origin_y = area.y + area.height.saturating_sub(visible_height) / 2;

        for cell_y in 0..visible_height {
            for x in 0..visible_width {
                let top = self.raster.pixel(x, cell_y * 2);
                let bottom_y = cell_y * 2 + 1;
                let bottom = if bottom_y < self.raster.height {
                    self.raster.pixel(x, bottom_y)
                } else {
                    [0, 0, 0, 0]
                };
                let top_visible = top[3] > 40;
                let bottom_visible = bottom[3] > 40;
                let cell = &mut buf[(origin_x + x, origin_y + cell_y)];
                match (top_visible, bottom_visible) {
                    (true, true) => {
                        cell.set_symbol("▀")
                            .set_fg(Color::Rgb(top[0], top[1], top[2]))
                            .set_bg(Color::Rgb(bottom[0], bottom[1], bottom[2]));
                    }
                    (true, false) => {
                        cell.set_symbol("▀")
                            .set_fg(Color::Rgb(top[0], top[1], top[2]));
                    }
                    (false, true) => {
                        cell.set_symbol("▄")
                            .set_fg(Color::Rgb(bottom[0], bottom[1], bottom[2]));
                    }
                    (false, false) => {}
                }
            }
        }
    }
}

struct Starfield {
    time: f32,
    intensity: f32,
}

impl Widget for Starfield {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let threshold = (14.0 * self.intensity) as u32;
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let hash =
                    u32::from(x).wrapping_mul(73_856_093) ^ u32::from(y).wrapping_mul(19_349_663);
                if hash % 1000 < threshold {
                    let pulse = ((self.time * 2.0 + (hash % 17) as f32).sin() + 1.0) * 0.5;
                    let value = (75.0 + pulse * 145.0) as u8;
                    let symbol = if hash % 19 == 0 {
                        "✦"
                    } else if hash % 7 == 0 {
                        "·"
                    } else {
                        "⠂"
                    };
                    buf[(x, y)].set_symbol(symbol).set_fg(Color::Rgb(
                        value / 2,
                        value / 2 + 25,
                        value,
                    ));
                }
            }
        }
    }
}

fn rarity_color(rarity: Rarity) -> Color {
    match rarity {
        Rarity::Three => BLUE,
        Rarity::Four => PURPLE,
        Rarity::Five => GOLD,
    }
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let [vertical] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    let [result] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(vertical);
    result
}

fn short_name(name: &str, max: usize) -> String {
    if name.chars().count() <= max {
        name.into()
    } else {
        format!(
            "{}…",
            name.chars().take(max.saturating_sub(1)).collect::<String>()
        )
    }
}

fn mix(a: Color, b: Color, alpha: u8) -> Color {
    let (Color::Rgb(ar, ag, ab), Color::Rgb(br, bg, bb)) = (a, b) else {
        return a;
    };
    let blend = |x: u8, y: u8| {
        ((u16::from(x) * u16::from(alpha) + u16::from(y) * u16::from(255 - alpha)) / 255) as u8
    };
    Color::Rgb(blend(ar, br), blend(ag, bg), blend(ab, bb))
}

fn mix_f32(a: Color, b: Color, amount: f32) -> Color {
    mix(b, a, (amount.clamp(0.0, 1.0) * 255.0) as u8)
}

fn smoothstep(start: f32, end: f32, value: f32) -> f32 {
    let t = ((value - start) / (end - start)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Tiny portraits designed for a monospace terminal cell grid. Each character
/// gets a distinct silhouette; weapons retain a compact symbolic crest.
fn item_sprite(result: &WishResult) -> &'static [&'static str] {
    match result.item.name {
        "Astraea, Starbound" => &[
            "   ✦  .  ✦   ",
            "  ╭╲___╱╮   ",
            "  │ ◕ ◕ │   ",
            "  ╰─╲◇╱─╯   ",
            "   ╱╱✧╲╲    ",
        ],
        "Veyra, Stormseeker" => &[
            "   ϟ╲  ╱ϟ   ",
            "  ╭─╲╱─╮    ",
            "  │ ◉ ◉│    ",
            "  ╰─⌁──╯    ",
            "   ╱⚡╲     ",
        ],
        "Orin, Keeper of Embers" => &[
            "    (  )     ",
            "   ╭🔥╮     ",
            "   │• •│     ",
            "   ╰─⌣╯     ",
            "   ╱╬╬╲     ",
        ],
        "Mira" => &[
            "    .✧.      ",
            "   ╭~~~╮     ",
            "   │ᵔ ᵔ│     ",
            "   ╰─◡─╯     ",
            "   ╱│◇│╲    ",
        ],
        "Thorne" => &[
            "   ╱♠╲      ",
            "  ╭╱─╲╮     ",
            "  │•  •│     ",
            "  ╰─︿─╯     ",
            "   ╱╳╲      ",
        ],
        "Lumen" => &[
            "    \u{2600}       ",
            "   ╭───╮     ",
            "   │◠ ◠│     ",
            "   ╰─▽─╯     ",
            "  ─╯│✦│╰─   ",
        ],
        "Polaris Edge" => &[
            "      ◇      ",
            "     ╱│╲     ",
            "      │      ",
            "    ──┼──    ",
            "      †      ",
        ],
        "Nova Grimoire" => &[
            "    · ✦ ·    ",
            "   ╭─────╮   ",
            "   │  ◈  │   ",
            "   ╰─────╯   ",
            "    · ✧ ·    ",
        ],
        _ => match result.item.kind {
            "Character" => &[
                "    ✦       ",
                "   ╭───╮    ",
                "   │• •│    ",
                "   ╰─⌣─╯    ",
                "   ╱│◇│╲   ",
            ],
            "Bow" => &[
                "    ╭ │ ╮   ",
                "   ╭  │  ╮  ",
                "   │  ◆  │  ",
                "   ╰  │  ╯  ",
                "    ╰ │ ╯   ",
            ],
            "Sword" => &[
                "     ◇      ",
                "     │      ",
                "     │      ",
                "   ──┼──    ",
                "     †      ",
            ],
            "Polearm" => &[
                "     ◆      ",
                "     │      ",
                "     │      ",
                "     │      ",
                "     ╵      ",
            ],
            "Catalyst" => &[
                "    ·✧·     ",
                "   ╭───╮    ",
                "   │ ◈ │    ",
                "   ╰───╯    ",
                "    ·✧·     ",
            ],
            "Claymore" => &[
                "     ◆      ",
                "    ║║      ",
                "    ║║      ",
                "  ──╬╬──    ",
                "     ╨      ",
            ],
            "Gauntlet" => &[
                "   ╔════╗   ",
                " ╔═╬████╬═╗ ",
                " ║ ██████ ║ ",
                " ╚═██████═╝ ",
                "   ╚╦══╦╝   ",
            ],
            "Scythe" => &[
                "   ╭────◆   ",
                " ╭─╯   ╱    ",
                "◆╯    ╱     ",
                "     ╱      ",
                "    ╵       ",
            ],
            "Dual Blades" => &[
                "  ◆     ◆   ",
                "  ║     ║   ",
                "──╬── ──╬── ",
                "  ║     ║   ",
                "  †     †   ",
            ],
            _ => &[
                "            ",
                "     ✧      ",
                "    ✦✦✦     ",
                "     ✧      ",
                "            ",
            ],
        },
    }
}

#[allow(dead_code)]
struct PixelSprite {
    pixels: &'static [&'static str],
    outline: Color,
    hair: Color,
    skin: Color,
    outfit: Color,
    element: Color,
}

#[allow(dead_code)]
fn render_hd_sprite(frame: &mut Frame, area: Rect, sprite: &PixelSprite) {
    // Each authored pixel becomes a shaded 2x2 cluster. Highlights, midtones,
    // shadows, and elemental bloom give the inspection portrait a denser
    // HD-2D RPG texture while preserving hard pixel-art silhouettes.
    let mut lines = Vec::with_capacity(sprite.pixels.len() * 2);
    for (y, row) in sprite.pixels.iter().enumerate() {
        for lower_half in [false, true] {
            let spans = row
                .chars()
                .enumerate()
                .map(|(x, pixel)| {
                    let base = match pixel {
                        'K' => Some(sprite.outline),
                        'H' => Some(sprite.hair),
                        'S' => Some(sprite.skin),
                        'O' => Some(sprite.outfit),
                        'E' => Some(sprite.element),
                        _ => None,
                    };
                    base.map_or_else(
                        || Span::raw("  "),
                        |base| {
                            let amount = if pixel == 'E' && (x + y) % 2 == 0 {
                                0.34
                            } else if !lower_half && (x + y) % 3 == 0 {
                                0.18
                            } else if lower_half {
                                -0.20
                            } else {
                                0.0
                            };
                            Span::styled("██", Style::new().fg(shade(base, amount)))
                        },
                    )
                })
                .collect::<Vec<_>>();
            lines.push(Line::from(spans));
        }
    }
    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), area);
}

#[allow(dead_code)]
fn shade(color: Color, amount: f32) -> Color {
    let Color::Rgb(r, g, b) = color else {
        return color;
    };
    let channel = |value: u8| {
        {
            if amount >= 0.0 {
                f32::from(value) + (255.0 - f32::from(value)) * amount
            } else {
                f32::from(value) * (1.0 + amount)
            }
        }
        .clamp(0.0, 255.0) as u8
    };
    Color::Rgb(channel(r), channel(g), channel(b))
}

#[allow(dead_code)]
fn character_sprite(name: &str) -> Option<PixelSprite> {
    let black = Color::Rgb(25, 25, 40);
    let skin = Color::Rgb(255, 205, 175);
    let sprite = match name {
        "Astraea, Starbound" => PixelSprite {
            pixels: &[
                "E...HHHHHH....",
                "..EHHHHHHHH...",
                ".EHHHHKSSSKH..",
                "..HHHKSSSSSK..",
                "...HHKSKSSSKK.",
                "....HKSSKSK...",
                ".....KKSSK....",
                "...OOOKKOOE...",
                "..OOOOEEOOOEE..",
                "...OOOEEOOOO...",
                "....OOO..OO....",
                "...KK.....KK...",
            ],
            outline: black,
            hair: Color::Rgb(215, 225, 255),
            skin,
            outfit: Color::Rgb(70, 85, 150),
            element: Color::Rgb(105, 225, 255),
        },
        "Kaelis, Ashen Vanguard" => PixelSprite {
            pixels: &[
                "E..HHHHHH.....",
                ".EHHHHHHHH....",
                "..HHHHKSSSKH..",
                "..HHHKSSSSSK...",
                "...HHKSKSSSKK..",
                "....HKSSKSK....",
                ".....KKSSK...E.",
                "...OOOKKOO...EE",
                "..OOOOEEOOO...E",
                "...OOOEEOOOO..E",
                "....OOO..OO...E",
                "...KK.....KK..E",
            ],
            outline: black,
            hair: Color::Rgb(95, 35, 30),
            skin,
            outfit: Color::Rgb(120, 35, 38),
            element: Color::Rgb(255, 85, 35),
        },
        "Seraphine, Verdant Oracle" => PixelSprite {
            pixels: &[
                "EE..HHHHHH....",
                ".E.HHHHHHHH...",
                "E.HHHHKSSSKH..",
                "..HHHKSSSSSK...",
                "E..HHKSKSSSKK..",
                "...EHKSSKSK....",
                "....EKKSSK.....",
                "..EOOOKKOOE....",
                ".EOOOOEEOOOEE..",
                "..EOOOEEOOOO...",
                "...EOOO..OO....",
                "..EKK.....KK...",
            ],
            outline: black,
            hair: Color::Rgb(85, 190, 125),
            skin,
            outfit: Color::Rgb(230, 235, 175),
            element: Color::Rgb(100, 220, 70),
        },
        "Veyra, Stormseeker" => PixelSprite {
            pixels: &[
                "E..HHHHHH..E.",
                "..EHHHHHHHH...",
                ".EHHHHKSSSKH..",
                "..HHHKSSSSSK..E",
                "...HHKSKSSSKK..",
                "....HKSSKSK...E",
                ".....KKSSK....E",
                "...OOOKKOOE...E",
                "..OOOOEEOOOEE..",
                "...OOOEEOOOO...",
                "....OOO..OO....",
                "...KK.....KK...",
            ],
            outline: black,
            hair: Color::Rgb(70, 55, 120),
            skin,
            outfit: Color::Rgb(105, 55, 175),
            element: Color::Rgb(195, 100, 255),
        },
        "Orin, Keeper of Embers" => PixelSprite {
            pixels: &[
                ".E.HHHHHHH....",
                "E.HHHHHHHHH...",
                "..HHHHKSSSKH..",
                "..HHHKSSSSSK...",
                "...HHKSKSSSKK..",
                "....HKSSKSK....",
                ".....KKSSK.....",
                "...OOOKKOO..E..",
                "..OOOOEEOOO.EEE",
                "..OOOOEEOOOO.E.",
                "...OOOO..OO....",
                "..KK......KK...",
            ],
            outline: black,
            hair: Color::Rgb(80, 45, 35),
            skin: Color::Rgb(185, 120, 85),
            outfit: Color::Rgb(75, 65, 65),
            element: Color::Rgb(255, 110, 35),
        },
        "Mira" => PixelSprite {
            pixels: &[
                "E...HHHHHH....",
                ".E.HHHHHHHH...",
                "..HHHHKSSSKH..",
                "..HHHKSSSSSK...",
                "...HHKSKSSSKK..",
                "....HKSSKSK....",
                ".....KKSSK...E.",
                "...OOOKKOOE.EE.",
                "..OOOOEEOOOEE..",
                "...OOOEEOOOO...",
                "....OOO..OO....",
                "...KK.....KK...",
            ],
            outline: black,
            hair: Color::Rgb(55, 150, 205),
            skin,
            outfit: Color::Rgb(50, 95, 175),
            element: Color::Rgb(60, 195, 255),
        },
        "Thorne" => PixelSprite {
            pixels: &[
                "E..HHHHHH.....",
                "EEHHHHHHHH....",
                ".EHHHHKSSSKH..",
                "..HHHKSSSSSK...",
                "E..HHKSKSSSKK..",
                "EE..HKSSKSK....",
                ".E...KKSSK.....",
                "EE.OOOKKOO.....",
                "E.OOOOEEOOO....",
                "..OOOOEEOOOO...",
                "...OOOO..OO....",
                "..KK......KK...",
            ],
            outline: black,
            hair: Color::Rgb(55, 95, 45),
            skin: Color::Rgb(205, 155, 110),
            outfit: Color::Rgb(75, 110, 55),
            element: Color::Rgb(125, 220, 65),
        },
        "Lumen" => PixelSprite {
            pixels: &[
                "..E.HHHHHH....",
                ".E.HHHHHHHH...",
                "..HHHHKSSSKH..",
                "..HHHKSSSSSK...",
                "...HHKSKSSSKK..",
                "....HKSSKSK....",
                ".....KKSSK.....",
                "...OOOKKOOE....",
                "..OOOOEEOOOEE..",
                "...OOOEEOOOO.E.",
                "....OOO..OO....",
                "...KK.....KK...",
            ],
            outline: black,
            hair: Color::Rgb(245, 210, 105),
            skin,
            outfit: Color::Rgb(235, 225, 190),
            element: Color::Rgb(255, 190, 50),
        },
        _ => return None,
    };
    Some(sprite)
}

#[allow(dead_code)]
struct ItemProfile {
    title: &'static str,
    element: &'static str,
    weapon: &'static str,
    lore: &'static str,
    quote: &'static str,
    color: Color,
    accent: Color,
    art: &'static [&'static str],
}

fn item_profile(result: &WishResult) -> ItemProfile {
    match result.item.name {
        "Astraea, Starbound" => ItemProfile {
            title: "Celestial Navigator",
            element: "Cryo",
            weapon: "Catalyst",
            lore: "Astraea charts roads through the night that no ordinary compass can find. The frost orbiting her astrolabe is said to preserve the memory of every star that has fallen.",
            quote: "Even a lost star leaves a path in the dark.",
            color: Color::Rgb(120, 225, 255),
            accent: Color::Rgb(220, 180, 255),
            art: &[
                "       ·  ✦  ·       ",
                "    ✧ ╲  │  ╱ ✧     ",
                "      ╭╲___╱╮       ",
                "     ╱│ ◕ ◕ │╲      ",
                "    ✦ ╰─╲◇╱─╯ ✦     ",
                "       ╱╱│╲╲        ",
                "    ╭─╯ ✧│✧ ╰─╮     ",
                "   ╱   ╱ ◇ ╲   ╲    ",
                "  ✧___╱__│__╲___✧   ",
                "       ╱   ╲        ",
            ],
        },
        "Kaelis, Ashen Vanguard" => ItemProfile {
            title: "The Ashen Vanguard",
            element: "Pyro",
            weapon: "Sword",
            lore: "Once the youngest captain of the imperial guard, Kaelis laid down his crest when ordered to silence a hungry province. Now his ember-red blade guards the roads used by rebels, pilgrims, and anyone else denied a home.",
            quote: "An oath that fears the truth deserves to become ash.",
            color: Color::Rgb(255, 85, 40),
            accent: Color::Rgb(255, 190, 65),
            art: &[""],
        },
        "Seraphine, Verdant Oracle" => ItemProfile {
            title: "Oracle of the Dreamwood",
            element: "Dendro",
            weapon: "Bow",
            lore: "Seraphine reads possibilities in the rings of ancient trees. She speaks softly because the Dreamwood repeats every promise, and plants silverleaf wherever a forgotten story deserves another ending.",
            quote: "Do not ask the future what it wants. Ask what it needs to grow.",
            color: Color::Rgb(100, 220, 80),
            accent: Color::Rgb(225, 245, 130),
            art: &[""],
        },
        "Veyra, Stormseeker" => ItemProfile {
            title: "Eye of the Tempest",
            element: "Electro",
            weapon: "Bow",
            lore: "Veyra hunts the thunderheads that gather beyond the mapped horizon. Her arrows do not follow the wind; they provoke it, splitting the clouds into brilliant violet scars.",
            quote: "If the storm will not come to us, I will wake it.",
            color: Color::Rgb(190, 105, 255),
            accent: Color::Rgb(110, 205, 255),
            art: &[
                "   ϟ      ╱╲      ϟ  ",
                "     ϟ ╭╱──╲╮ ϟ     ",
                "       │ ◉ ◉│       ",
                "   ────╰─⌁──╯────   ",
                "      ╱╲╱ϟ╲╱╲      ",
                "  ︶──╯  ╲│╱  ╰──︶  ",
                "          │     ➴   ",
                "       ╱──┼──╲      ",
                "      ╱  ╱ ╲  ╲     ",
                "     ϟ  ╱   ╲  ϟ    ",
            ],
        },
        "Orin, Keeper of Embers" => ItemProfile {
            title: "Keeper of the Last Hearth",
            element: "Pyro",
            weapon: "Claymore",
            lore: "Orin carries a coal from the first communal hearth in a lantern over his heart. When winter closes a mountain pass, travelers follow the red arc of his blade home.",
            quote: "A flame is only small when no one gathers around it.",
            color: Color::Rgb(255, 95, 55),
            accent: Color::Rgb(255, 195, 70),
            art: &[
                "        (  (        ",
                "      (  /\\  )      ",
                "       ╭────╮       ",
                "    ╭──│ •  •│──╮   ",
                "    │  ╰─⌣──╯  │    ",
                "    ╰╮ ╱╬🔥╬╲ ╭╯    ",
                "     │╱ ║  ║ ╲│     ",
                "     ╱══╬══╬══╲     ",
                "    ╱   ║  ║   ╲    ",
                "       ╱    ╲       ",
            ],
        },
        "Mira" => ItemProfile {
            title: "Tideglass Minstrel",
            element: "Hydro",
            weapon: "Catalyst",
            lore: "Mira collects voices inside beads of enchanted seawater. Her concerts sound different to every listener, always recalling the shore they most wish to see again.",
            quote: "The sea remembers your song. Shall we ask it to sing back?",
            color: Color::Rgb(65, 165, 255),
            accent: Color::Rgb(80, 245, 225),
            art: &[
                "     ~  .✧.  ~     ",
                "   ≋   ╭~~~╮   ≋   ",
                "       │ᵔ ᵔ│       ",
                "    ~  ╰─◡─╯  ~    ",
                "      ╱│ ◇ │╲      ",
                "   ≋ ╱ │~~~│ ╲ ≋   ",
                "    ╱  ╰─┬─╯  ╲    ",
                "   ◇    ╱ ╲    ◇   ",
                "       ╱   ╲       ",
                "    ~·         ·~  ",
            ],
        },
        "Thorne" => ItemProfile {
            title: "Warden of Briarwatch",
            element: "Dendro",
            weapon: "Polearm",
            lore: "Thorne guards a forest whose paths rearrange at dusk. Though his manner is severe, every bird in Briarwatch knows it can safely build a nest on his shoulder.",
            quote: "The wilds need no ruler—only someone willing to listen.",
            color: Color::Rgb(110, 205, 75),
            accent: Color::Rgb(225, 210, 70),
            art: &[
                "    ❧  ╱♠╲  ❧     ",
                "      ╭╱─╲╮       ",
                "   ───│•  •│───   ",
                "  ❧   ╰─︿─╯   ❧  ",
                "     ╱╲╱│╲╱╲     ",
                "    ╱  ╲♣╱  ╲    ",
                "   ╱────┼────╲   │",
                "        │        │",
                "       ╱ ╲       ◆",
                "    ❧ ╱   ╲ ❧     ",
            ],
        },
        "Lumen" => ItemProfile {
            title: "Dawn's Errant Knight",
            element: "Geo",
            weapon: "Sword",
            lore: "Lumen left a ceremonial order to bring light to places omitted from royal maps. The little sun pinned to his cloak was forged from stone found at daybreak, and he carries its warmth to every forgotten road.",
            quote: "Dawn belongs to everyone. That is why I keep walking.",
            color: Color::Rgb(255, 190, 55),
            accent: Color::Rgb(255, 245, 165),
            art: &[
                "        ☀          ",
                "     ╲  │  ╱       ",
                "      ╭───╮        ",
                "   ───│◠ ◠│───     ",
                "      ╰─▽─╯        ",
                "    ─╯╲│✦│╱╰─     ",
                "      ╱╪╪╲       †",
                "     ╱ ╪╪ ╲    ──┼",
                "    ◇  ╱╲  ◇      │",
                "      ╱  ╲        ◇",
            ],
        },
        "Vaughn, Violet Oath" => ItemProfile {
            title: "Knight of the Sealed Storm",
            element: "Electro",
            weapon: "Claymore",
            lore: "No living witness has seen the face beneath Vaughn's great helm. He walks where thunder has scarred the earth, carrying an oath too heavy for any ordinary blade.",
            quote: "The helm is not my prison. It is my promise.",
            color: Color::Rgb(155, 75, 255),
            accent: Color::Rgb(220, 180, 255),
            art: &[
                "       ⚡       ",
                "     ╭███╮     ",
                "     │ ▬ │     ",
                "   ╭─╯███╰─╮   ",
                "   │███████│ ║ ",
                "   ╰╮█████╭╯ ║ ",
                "    │█████│ ═╬═",
                "    ╱█████╲  ║ ",
                "   ╱ ╱   ╲ ╲ ◆ ",
            ],
        },
        "Steven, Azure Shade" => ItemProfile {
            title: "Keeper of the Quiet Flame",
            element: "Pyro",
            weapon: "Catalyst",
            lore: "Steven served unseen along the Archive's border roads until Wick chose his shoulder as a throne. Their veilfire burns blue and leaves no ash; one moves like a whisper, while the other strikes with the force of a falling star.",
            quote: "Stealth is not silence. It is choosing what the enemy hears.",
            color: Color::Rgb(65, 155, 255),
            accent: Color::Rgb(145, 225, 255),
            art: &[
                "     ≋🔥     🔥≋    ",
                "       ╭───╮  ᴥ    ",
                "       │◉ ◉│ ╱■╲   ",
                "       ╰─▰─╯ ╰─╯   ",
                "    ≋╲╱│◆│╲╱≋     ",
                "   🔥  ╱███╲  🔥   ",
                "      ╱╱███╲╲      ",
                "       ╱   ╲       ",
                "      ╱     ╲      ",
            ],
        },
        "Cinder, Forgeheart" => ItemProfile {
            title: "The Unbroken Furnace",
            element: "Pyro",
            weapon: "Gauntlet",
            lore: "Cinder learned smithing by breaking every tool her masters gave her. Now she tempers armor with her bare hands and meets every impossible fight with a delighted grin.",
            quote: "If it won't bend, hit it until it remembers how.",
            color: Color::Rgb(255, 85, 35),
            accent: GOLD,
            art: &[
                "    🔥   🔥     ",
                "     ╭───╮     ",
                "   ╔═│•  •│═╗   ",
                "   ║ ╰─⌣─╯ ║   ",
                "   █╲╱███╲╱█   ",
                "   █  ███  █   ",
                "     ╱███╲     ",
                "    ╱ ╱ ╲ ╲    ",
            ],
        },
        "Sergei, Winterfang" => ItemProfile {
            title: "Heir of the White Hunt",
            element: "Cryo",
            weapon: "Catalyst",
            lore: "Sergei carries a splinter of the old winter inside a crystal catalyst beneath his furs. When danger draws near, the guardian called Volkodav rises at his back and shapes that cold into armor, claws, and twin shields.",
            quote: "The wolf does not chase the storm. It waits where the storm must pass.",
            color: Color::Rgb(105, 185, 255),
            accent: Color::Rgb(205, 240, 255),
            art: &[
                "      ╱╲___╱╲      ",
                "   ≋ ╱  ◈ ◈  ╲ ≋   ",
                "     ╲__︿____╱     ",
                "       ╭▰▰╮        ",
                "    ≋══│▰▰│══≋     ",
                "   ◈╲ ╱╱◇╲╲ ╱◈    ",
                "   ╰◆╯│❄│╰◆╯     ",
                "      ╱▰▰╲        ",
                "     ╱ ╱ ╲ ╲       ",
                "    ❄       ❄      ",
            ],
        },
        "Saif, Dune Sovereign" => ItemProfile {
            title: "Sovereign of Shifting Sands",
            element: "Geo",
            weapon: "Polearm",
            lore: "Saif reads the desert as others read a courtly ledger: every dune records a bargain, every buried stone remembers a boundary. With a turn of his polearm he calls those old agreements into a spiraling wall of sand.",
            quote: "Stone boasts of permanence. Sand knows better.",
            color: Color::Rgb(239, 234, 187),
            accent: Color::Rgb(85, 132, 103),
            art: &[""],
        },
        "Pyrite, Gilded Step" => ItemProfile {
            title: "The Gilded Step",
            element: "Geo",
            weapon: "Sword",
            lore: "Pyrite trained among mountain couriers who measure skill by how little dust remains after a passage. Her gold eye catches the fault lines in stone, letting her cross a battlefield in one brilliant, blade-first dash.",
            quote: "If you saw me move, I was being polite.",
            color: Color::Rgb(235, 180, 55),
            accent: Color::Rgb(255, 235, 155),
            art: &[""],
        },
        "Jeanette, Tidemender" => ItemProfile {
            title: "The Laughing Tidemender",
            element: "Hydro",
            weapon: "Bow",
            lore: "Jeanette turns every expedition into a reunion waiting to happen. Her silver longbow draws restorative water from the air; even her arrows burst into bright ribbons that close wounds and lift weary hearts.",
            quote: "Hold still, smile, and let the tide put you back together!",
            color: Color::Rgb(65, 155, 255),
            accent: GOLD,
            art: &[""],
        },
        "Farah" => ItemProfile {
            title: "Keeper of the Dune Ledger",
            element: "Geo",
            weapon: "Catalyst",
            lore: "Farah preserves treaties on tablets of singing sandstone. She travels beside Saif when old borders stir, shaping the tablets into shelter while he turns the open desert against their foes.",
            quote: "A promise survives when someone carries its weight.",
            color: Color::Rgb(210, 184, 118),
            accent: Color::Rgb(85, 132, 103),
            art: &[""],
        },
        "Anya" => ItemProfile {
            title: "Warden of the Warm Trail",
            element: "Cryo",
            weapon: "Sword",
            lore: "Anya follows the marks Sergei leaves across the white frontier, tending travelers caught in the wake of the Winterfang. Her frost wards preserve warmth instead of stealing it.",
            quote: "Cold is only cruel when no one knows the road home.",
            color: Color::Rgb(150, 205, 245),
            accent: Color::Rgb(225, 245, 255),
            art: &[""],
        },
        "Rook" => ItemProfile {
            title: "Breaker of Brass Giants",
            element: "Electro",
            weapon: "Gauntlet",
            lore: "Rook dismantles abandoned war machines before scavengers can wake them. Each rescued gear joins his gauntlets, where captured current answers every impossible mechanism with a louder idea.",
            quote: "Everything comes apart. The trick is making it useful afterward.",
            color: Color::Rgb(190, 105, 255),
            accent: Color::Rgb(215, 155, 75),
            art: &[""],
        },
        "Kestrel" => ItemProfile {
            title: "Wayfinder of the Open Gale",
            element: "Anemo",
            weapon: "Bow",
            lore: "Kestrel charts roads by firing ribboned arrows into uncertain winds. Caravans trust the returning feathers: green means passage, white means shelter, and none means she has gone ahead to clear the danger herself.",
            quote: "A road only ends when the wind stops asking.",
            color: Color::Rgb(100, 220, 185),
            accent: Color::Rgb(235, 230, 180),
            art: &[""],
        },
        "Mako" => ItemProfile {
            title: "Laughing Reefrunner",
            element: "Hydro",
            weapon: "Dual Blades",
            lore: "Mako guides fishing crews through reefs that rearrange with the moon. His hooked blades turn the same currents that once threatened his village into swift steps and brighter stories.",
            quote: "If the tide cheats, cheat faster.",
            color: Color::Rgb(75, 165, 245),
            accent: Color::Rgb(175, 235, 255),
            art: &[""],
        },
        "Ysra" => ItemProfile {
            title: "Marshal of Quiet Embers",
            element: "Pyro",
            weapon: "Polearm",
            lore: "Ysra builds compact flame wards that conceal movement and sharpen elemental focus. Steven respects her disciplined silence; Wick respects that she always carries smoked biscuits.",
            quote: "A controlled flame reveals only what I permit.",
            color: Color::Rgb(255, 105, 45),
            accent: Color::Rgb(85, 180, 255),
            art: &[""],
        },
        "Dolma" => ItemProfile {
            title: "Stone of the High Pass",
            element: "Geo",
            weapon: "Claymore",
            lore: "Dolma repairs mountain roads with the same slab-edged blade she carries into battle. She has never hurried a crossing and has never lost a traveler entrusted to her care.",
            quote: "The mountain is patient. So am I.",
            color: Color::Rgb(220, 175, 80),
            accent: Color::Rgb(165, 135, 105),
            art: &[""],
        },
        "Corvin" => ItemProfile {
            title: "Frost-Ring Interceptor",
            element: "Cryo",
            weapon: "Gauntlet",
            lore: "Corvin learned to stop charging beasts without meeting force with force. Each precise frozen strike steals momentum until even the wildest opponent stands motionless before him.",
            quote: "Speed is only useful while you still possess it.",
            color: Color::Rgb(125, 195, 245),
            accent: Color::Rgb(225, 245, 255),
            art: &[""],
        },
        "Zephra" => ItemProfile {
            title: "Gale-Road Courier",
            element: "Anemo",
            weapon: "Gauntlet",
            lore: "Zephra delivers sealed messages across borders no map admits exist. A fair wind follows her, though it has an unfortunate habit of stealing hats.",
            quote: "Keep up—the shortcut only lasts one gust!",
            color: Color::Rgb(75, 220, 195),
            accent: Color::White,
            art: &[
                "   ~  ✦  ~    ",
                "     ╭───╮     ",
                "  ≋══│◠ ◠│══≋  ",
                "     ╰─▽─╯     ",
                "   ◉╲╱│◇│╲╱◉   ",
                "     ╱   ╲     ",
            ],
        },
        "Neris" => ItemProfile {
            title: "Waltzer at Winter's End",
            element: "Cryo",
            weapon: "Scythe",
            lore: "Neris performs the last dance at abandoned winter courts. Each sweep of his scythe preserves one fading memory in flawless ice.",
            quote: "Even an ending deserves perfect form.",
            color: Color::Rgb(125, 190, 255),
            accent: Color::White,
            art: &[
                "       ❄  ╭─╮ ",
                "     ╭───╮│ │ ",
                "     │◐ ◐│╰╮│ ",
                "     ╰─︿─╯ ││ ",
                "      ╱◇╲  ││ ",
                "     ╱   ╲ ◆╯ ",
            ],
        },
        "Brikka" => ItemProfile {
            title: "Emberworks Prodigy",
            element: "Pyro",
            weapon: "Dual Blades",
            lore: "Brikka's workshop has exploded seven times and improved after every incident. Her paired furnace blades are equal parts weapon, tool, and very loud argument.",
            quote: "Safety third. Inspiration first. Snacks second.",
            color: Color::Rgb(255, 105, 35),
            accent: GOLD,
            art: &[
                "    ⚙  🔥  ⚙    ",
                "     ╭───╮     ",
                "     │◠ ◠│     ",
                "     ╰─▽─╯     ",
                "  †═╲╱│◇│╲╱═†  ",
                "     ╱   ╲     ",
            ],
        },
        _ => weapon_profile(result),
    }
}

fn weapon_profile(result: &WishResult) -> ItemProfile {
    let (element, color, accent, art): (_, _, _, &'static [&'static str]) = match result.item.kind {
        "Bow" => (
            "Unaligned",
            BLUE,
            Color::Rgb(220, 235, 255),
            &[
                "       ╭  │  ╮      ",
                "      ╭   │   ╮     ",
                "     ╭    ◆    ╮    ",
                "     │    │    │    ",
                "     ╰    │    ╯    ",
                "      ╰   │   ╯     ",
                "       ╰  │  ╯      ",
            ],
        ),
        "Catalyst" => (
            "Unaligned",
            PURPLE,
            Color::Rgb(230, 190, 255),
            &[
                "       ·  ✧  ·      ",
                "    ✦  ╭────╮  ✦   ",
                "       │ ╱╲ │       ",
                "    ·  │ ◈  │  ·    ",
                "       │ ╲╱ │       ",
                "    ✦  ╰────╯  ✦   ",
                "       ·  ✧  ·      ",
            ],
        ),
        "Polearm" => (
            "Unaligned",
            BLUE,
            Color::White,
            &[
                "         ◆         ",
                "        ╱│╲        ",
                "         │         ",
                "         │         ",
                "         │         ",
                "       ──┼──       ",
                "         ╵         ",
            ],
        ),
        "Claymore" => (
            "Unaligned",
            PURPLE,
            GOLD,
            &[
                "        ╱◆╲        ",
                "       ║██║        ",
                "       ║██║        ",
                "       ║██║        ",
                "    ───╬██╬───     ",
                "       ║  ║        ",
                "        ╨          ",
            ],
        ),
        "Gauntlet" => (
            "Unaligned",
            Color::Rgb(75, 220, 195),
            GOLD,
            &[
                "      ╔════╗      ",
                "    ╔═╬████╬═╗    ",
                "    ║ ██████ ║    ",
                "    ╚═██████═╝    ",
                "      ╚╦══╦╝      ",
                "       ║  ║       ",
                "       ╚══╝       ",
            ],
        ),
        "Scythe" => (
            "Unaligned",
            Color::Rgb(125, 190, 255),
            Color::White,
            &[
                "       ╭────◆     ",
                "    ╭──╯   ╱      ",
                "  ◆─╯     ╱       ",
                "         ╱        ",
                "        ╱         ",
                "       ╱          ",
                "      ╵           ",
            ],
        ),
        "Dual Blades" => (
            "Unaligned",
            Color::Rgb(255, 105, 35),
            GOLD,
            &[
                "    ◆       ◆     ",
                "    ║       ║     ",
                "    ║       ║     ",
                "  ──╬──   ──╬──   ",
                "    ║       ║     ",
                "    †       †     ",
            ],
        ),
        _ => (
            "Unaligned",
            BLUE,
            Color::White,
            &[
                "         ◇         ",
                "        ╱│╲        ",
                "         │         ",
                "         │         ",
                "      ───┼───      ",
                "         │         ",
                "         †         ",
            ],
        ),
    };
    ItemProfile {
        title: "Archive Armament",
        element,
        weapon: result.item.kind,
        lore: "An armament recovered from the Starfall Archive. Fine lines along its surface brighten whenever its bearer looks toward a place they have never visited.",
        quote: "A road imagined is already half discovered.",
        color,
        accent,
        art,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rarity_color_teases_early_but_settles_late() {
        assert_eq!(smoothstep(0.42, 0.86, 0.30), 0.0);
        assert_eq!(smoothstep(0.42, 0.86, 0.95), 1.0);
        assert!(smoothstep(0.42, 0.86, 0.55) > 0.1);
        assert!(smoothstep(0.42, 0.86, 0.75) < 0.9);
    }

    #[test]
    fn terminal_portrait_writes_colored_cells() {
        let raster = TerminalRaster {
            width: 1,
            height: 2,
            pixels: vec![[255, 0, 0, 255], [0, 0, 255, 255]],
        };
        let area = Rect::new(0, 0, 1, 1);
        let mut buffer = Buffer::empty(area);
        TerminalPortrait::new(&raster).render(area, &mut buffer);
        assert_eq!(buffer[(0, 0)].symbol(), "▀");
        assert_eq!(buffer[(0, 0)].fg, Color::Rgb(255, 0, 0));
        assert_eq!(buffer[(0, 0)].bg, Color::Rgb(0, 0, 255));
    }

    #[test]
    fn ascension_display_uses_requested_copy_endpoints() {
        assert_eq!(ascension_level(1), 0);
        assert_eq!(ascension_level(10), 10);
        assert_eq!(ascension_level(99), 10);
    }
}
