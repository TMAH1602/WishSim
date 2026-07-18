use std::{collections::BTreeSet, time::Instant};

use crate::{
    app::{App, ELEMENT_FILTERS, Phase},
    art::{CharacterGallery, TerminalRaster},
    model::{Banner, Rarity, WishResult},
    simulation::catalog_item,
};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Gauge, Padding, Paragraph, Widget, Wrap},
};

const GOLD: Color = Color::Rgb(255, 205, 90);
const PURPLE: Color = Color::Rgb(198, 120, 255);
const BLUE: Color = Color::Rgb(90, 180, 255);
const DIM: Color = Color::Rgb(100, 115, 145);

pub fn kitty_portrait(app: &App, area: Rect) -> Option<(&str, Rect)> {
    if app.confirm_quit || area.width < 80 || area.height < 34 {
        return None;
    }
    match &app.phase {
        Phase::Reveal { results, index, .. } if results[*index].item.kind == "Character" => {
            let card = centered(area, 72.min(area.width - 8), 36.min(area.height - 4));
            let inner = card.inner(ratatui::layout::Margin {
                horizontal: 2,
                vertical: 1,
            });
            let [sprite_area, _] =
                Layout::vertical([Constraint::Min(8), Constraint::Length(6)]).areas(inner);
            Some((results[*index].item.name, portrait_fit(sprite_area)))
        }
        Phase::Detail { results, selected } if results[*selected].item.kind == "Character" => {
            Some((results[*selected].item.name, detail_portrait_area(area)))
        }
        Phase::InventoryDetail { name, .. }
            if catalog_item(name).is_some_and(|item| item.kind == "Character") =>
        {
            Some((name, detail_portrait_area(area)))
        }
        _ => None,
    }
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
        Phase::Home => home(frame, app),
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
        } => reveal(
            frame,
            now.duration_since(*started).as_secs_f32(),
            &results[*index],
            *index,
            results.len(),
            &app.gallery,
        ),
        Phase::FiveStarIntro {
            started,
            results,
            index,
        } => five_star_intro(
            frame,
            now.duration_since(*started).as_secs_f32(),
            results.len(),
            *index,
        ),
        Phase::Summary { results, selected } => summary(frame, results, *selected),
        Phase::Detail { results, selected } => {
            detail(frame, &results[*selected], None, &app.gallery)
        }
    }
    if app.confirm_quit {
        confirm_quit(frame);
    }
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
        Banner::Weapon => (
            "I N C A R N A T E   A R M A M E N T S",
            "POLARIS EDGE  ✦  NOVA GRIMOIRE",
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
            } else {
                "← / →  CHANGE BANNER".into()
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

    let buttons = Line::from(vec![
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
        Span::raw("   "),
        Span::styled(
            " [H] HISTORY ",
            Style::new().fg(Color::White).bg(Color::Rgb(45, 52, 76)),
        ),
        Span::raw("   "),
        Span::styled(
            " [I] INVENTORY ",
            Style::new().fg(Color::White).bg(Color::Rgb(40, 75, 72)),
        ),
    ]);
    frame.render_widget(
        Paragraph::new(buttons)
            .alignment(Alignment::Center)
            .block(Block::new().padding(Padding::vertical(1))),
        actions,
    );

    let mode = if app.kitty {
        "KITTY ENHANCED ✦"
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
    if result.item.kind == "Character" {
        if let Some(portrait) = gallery.get(result.item.name) {
            frame.render_widget(TerminalPortrait::new(&portrait.reveal), sprite_area);
        }
    } else {
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
    if result.item.kind == "Character" {
        if let Some(portrait) = gallery.get(result.item.name) {
            frame.render_widget(TerminalPortrait::new(&portrait.detail), art_inner);
        }
    } else {
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
            lore: "Lumen left a ceremonial order to bring light to places omitted from royal maps. The little sun pinned to their cloak was forged from stone found at daybreak.",
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
}
