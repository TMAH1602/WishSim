use std::time::Instant;

use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Gauge, Padding, Paragraph, Widget, Wrap},
};

use crate::{
    app::{App, Phase},
    model::{Banner, Rarity, WishResult},
};

const GOLD: Color = Color::Rgb(255, 205, 90);
const PURPLE: Color = Color::Rgb(198, 120, 255);
const BLUE: Color = Color::Rgb(90, 180, 255);
const DIM: Color = Color::Rgb(100, 115, 145);

pub fn render(frame: &mut Frame, app: &App, now: Instant) {
    let area = frame.area();
    frame.render_widget(Block::new().bg(Color::Rgb(4, 7, 19)), area);
    if area.width < 64 || area.height < 20 {
        frame.render_widget(
            Paragraph::new("✦  WishSim needs a terminal at least 64 × 20\n\nResize the window, or press q to quit.")
                .centered()
                .style(Style::new().fg(GOLD)),
            area,
        );
        return;
    }

    match &app.phase {
        Phase::Home => home(frame, app),
        Phase::History => history(frame, app),
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
        Phase::Detail { results, selected } => detail(frame, &results[*selected]),
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
    let [preview_area, copy_area] =
        Layout::horizontal([Constraint::Percentage(34), Constraint::Percentage(66)])
            .areas(hero_inner);
    render_banner_art(frame, preview_area, app.banner);
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
        copy_area,
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
        Span::raw("     "),
        Span::styled(
            "  [0]  WISH ×10  ",
            Style::new().fg(Color::Rgb(35, 24, 5)).bg(GOLD).bold(),
        ),
        Span::raw("     "),
        Span::styled(
            " [H] HISTORY ",
            Style::new().fg(Color::White).bg(Color::Rgb(45, 52, 76)),
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
    // Keep every flight visually identical at first. The true rarity only bleeds
    // into the trail during the final beat, just before the reveal flash.
    let mystery_color = Color::Rgb(195, 225, 255);
    let rarity_color = rarity_color(rarity);
    let reveal_blend = smoothstep(0.60, 0.88, progress);
    let color = mix_f32(mystery_color, rarity_color, reveal_blend);
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
            .set_symbol(if progress > 0.78 { "✹" } else { "✦" })
            .set_fg(color)
            .set_style(Style::new().add_modifier(Modifier::BOLD));
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

fn reveal(frame: &mut Frame, elapsed: f32, result: &WishResult, index: usize, total: usize) {
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

    let width = 56.min(area.width - 8);
    let height = 16.min(area.height - 4);
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
        Layout::vertical([Constraint::Length(7), Constraint::Min(4)]).areas(inner);
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
    let mut lines = vec![
        Line::from(result.item.name).style(Style::new().fg(Color::White).bold()),
        Line::from(result.item.kind).fg(DIM),
        Line::from(result.rarity.stars()).style(Style::new().fg(color).bold()),
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
    let mut stars = Vec::new();
    for number in 0..5 {
        let birth = 0.14 + number as f32 * 0.12;
        let glow = smoothstep(birth, birth + 0.13, progress);
        let color = mix_f32(Color::Rgb(25, 25, 45), GOLD, glow);
        stars.push(Span::styled(
            if glow > 0.82 { "★" } else { "✦" },
            Style::new().fg(color).bold(),
        ));
        if number < 4 {
            stars.push(Span::raw("     "));
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
            Line::from(stars),
            Line::from(""),
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

fn render_banner_art(frame: &mut Frame, area: Rect, banner: Banner) {
    let character = match banner {
        Banner::Astraea => Some("Astraea, Starbound"),
        Banner::Kaelis => Some("Kaelis, Ashen Vanguard"),
        Banner::Seraphine => Some("Seraphine, Verdant Oracle"),
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

fn detail(frame: &mut Frame, result: &WishResult) {
    let area = frame.area();
    let profile = item_profile(result);
    frame.render_widget(
        Starfield {
            time: result.wish_number as f32 * 0.07,
            intensity: 0.9,
        },
        area,
    );

    let panel = centered(area, area.width.min(104), area.height.min(32));
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
    if let Some(sprite) = character_sprite(result.item.name) {
        render_hd_sprite(frame, art_inner, &sprite);
    } else {
        let art_lines: Vec<Line> = profile
            .art
            .iter()
            .enumerate()
            .map(|(index, line)| {
                let color = match index % 4 {
                    0 => profile.color,
                    1 => Color::White,
                    2 => profile.accent,
                    _ => mix_f32(profile.color, Color::White, 0.38),
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
            profile.element.fg(profile.color).bold(),
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
        Line::from("ARCHIVE LORE").style(Style::new().fg(GOLD).bold()),
        Line::from(profile.lore).fg(Color::Rgb(185, 195, 220)),
        Line::from(""),
        Line::from(format!("“{}”", profile.quote)).style(Style::new().fg(profile.accent).italic()),
        Line::from(""),
        Line::from(format!("Obtained on wish #{}", result.wish_number)).fg(DIM),
    ]);
    frame.render_widget(
        Paragraph::new(info)
            .wrap(Wrap { trim: true })
            .block(Block::new().padding(Padding::uniform(1))),
        info_area,
    );
    frame.render_widget(
        Paragraph::new("ESC / ENTER  return to results  •  Q quit")
            .centered()
            .fg(DIM),
        Rect::new(area.x, area.bottom() - 2, area.width, 1),
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

struct PixelSprite {
    pixels: &'static [&'static str],
    outline: Color,
    hair: Color,
    skin: Color,
    outfit: Color,
    element: Color,
}

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
    fn rarity_color_stays_hidden_until_late_in_flight() {
        assert_eq!(smoothstep(0.60, 0.88, 0.50), 0.0);
        assert_eq!(smoothstep(0.60, 0.88, 0.95), 1.0);
        assert!(smoothstep(0.60, 0.88, 0.74) > 0.4);
    }
}
