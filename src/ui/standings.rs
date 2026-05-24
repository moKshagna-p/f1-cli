use crate::app::App;
use crate::state::{format_lap_time, DriverStanding};
use crate::ui::theme::*;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};

// ─── Standings View ───────────────────────────────────────────────────────────

pub fn draw_standings(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    draw_standings_table(f, chunks[0], app);
    draw_legend_and_info(f, chunks[1], app);
}

fn draw_standings_table(f: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["POS", "  DRIVER", "TEAM", "NATIONALITY", "BEST LAP", "#"]
        .iter()
        .map(|h| Cell::from(*h).style(style_bold(ACCENT_GOLD)));

    let header = Row::new(header_cells)
        .style(Style::default().bg(BG_PANEL))
        .height(1);

    let rows: Vec<Row> = app
        .state
        .standings
        .iter()
        .map(|s| make_standings_row(s))
        .collect();

    let widths = [
        Constraint::Length(5),
        Constraint::Length(10),
        Constraint::Length(18),
        Constraint::Length(13),
        Constraint::Length(10),
        Constraint::Length(4),
    ];

    let mut table_state = TableState::default();
    if !app.state.standings.is_empty() {
        table_state.select(Some(app.scroll_offset.min(app.state.standings.len() - 1)));
    }

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(Span::styled(
                    " 🏆  DRIVER STANDINGS ",
                    style_bold(ACCENT_GOLD),
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BORDER))
                .style(Style::default().bg(BG)),
        )
        .highlight_style(
            Style::default()
                .bg(BG_SELECTED)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(table, area, &mut table_state);
}

fn make_standings_row(s: &DriverStanding) -> Row<'static> {
    let driver_color = hex_color(&s.team_color);

    // Position medal
    let pos_str = match s.position {
        1 => "🥇  1".to_string(),
        2 => "🥈  2".to_string(),
        3 => "🥉  3".to_string(),
        n => format!("   {}", n),
    };
    let pos_cell = Cell::from(Span::styled(pos_str, style_bold(TEXT_PRIMARY)));

    // Driver
    let indicator = if s.is_fastest_lap { "● " } else { "  " };
    let ind_color = if s.is_fastest_lap { ACCENT_PURPLE } else { BG };
    let drv_cell = Cell::from(Line::from(vec![
        Span::styled(indicator, style_bold(ind_color)),
        Span::styled(s.acronym.clone(), style_bold(driver_color)),
    ]));

    // Team badge
    let team_short: String = s.team.chars().take(17).collect();
    let team_cell = Cell::from(Span::styled(team_short, style_normal(driver_color)));

    // Country
    let nat_cell = Cell::from(Span::styled(s.country.clone(), style_dim(TEXT_SECONDARY)));

    // Best lap
    let lap_str = s
        .lap_time
        .map(format_lap_time)
        .unwrap_or_else(|| "  —:—".to_string());
    let lap_cell = Cell::from(Span::styled(
        lap_str,
        if s.is_fastest_lap {
            style_bold(ACCENT_PURPLE)
        } else {
            style_normal(TEXT_SECONDARY)
        },
    ));

    // Driver number
    let num_cell = Cell::from(Span::styled(s.number.to_string(), style_dim(TEXT_DIM)));

    let bg = match s.position {
        1 => Style::default().bg(BG_PANEL).add_modifier(Modifier::BOLD),
        2 | 3 => Style::default().bg(BG_PANEL),
        _ => Style::default().bg(BG),
    };

    Row::new(vec![
        pos_cell, drv_cell, team_cell, nat_cell, lap_cell, num_cell,
    ])
    .height(1)
    .style(bg)
}

fn draw_legend_and_info(f: &mut Frame, area: Rect, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(1)])
        .split(area);

    // Legend
    let legend_lines = vec![
        Line::from(vec![
            Span::styled("  ●", style_bold(ACCENT_PURPLE)),
            Span::styled("  Fastest Lap Driver", style_normal(TEXT_SECONDARY)),
        ]),
        Line::from(vec![
            Span::styled("  ▲", style_bold(ACCENT_GREEN)),
            Span::styled("  Gained Position", style_normal(TEXT_SECONDARY)),
        ]),
        Line::from(vec![
            Span::styled("  ▼", style_bold(LIVE_RED)),
            Span::styled("  Lost Position", style_normal(TEXT_SECONDARY)),
        ]),
        Line::from(vec![
            Span::styled("  S", style_bold(SECTOR_FASTEST)),
            Span::styled("  Fastest Sector", style_normal(TEXT_SECONDARY)),
        ]),
        Line::from(vec![
            Span::styled("  S", style_bold(SECTOR_NORMAL)),
            Span::styled("  Normal Sector", style_normal(TEXT_SECONDARY)),
        ]),
        Line::from(vec![
            Span::styled("  🥇", style_normal(TEXT_PRIMARY)),
            Span::styled("  P1–P3 Medals", style_normal(TEXT_SECONDARY)),
        ]),
        Line::from(Span::styled("", style_normal(TEXT_DIM))),
    ];

    let legend = Paragraph::new(legend_lines).block(
        Block::default()
            .title(Span::styled(" 📖  LEGEND ", style_bold(ACCENT_CYAN)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BORDER))
            .style(Style::default().bg(BG)),
    );

    f.render_widget(legend, chunks[0]);

    // Teams color swatches
    let teams = [
        ("Red Bull", "3041d8"),
        ("Ferrari", "dc0000"),
        ("Mercedes", "00d2be"),
        ("McLaren", "ff8700"),
        ("Aston Martin", "006e3c"),
        ("Alpine", "0093cc"),
        ("Williams", "005aff"),
        ("Haas", "b6babd"),
        ("RB", "1e41ff"),
        ("Kick Sauber", "00e701"),
    ];

    let team_lines: Vec<Line> = teams
        .iter()
        .map(|(name, hex)| {
            Line::from(vec![
                Span::styled("  █ ", style_bold(hex_color(hex))),
                Span::styled(name.to_string(), style_normal(TEXT_SECONDARY)),
            ])
        })
        .collect();

    let teams_widget = Paragraph::new(team_lines).block(
        Block::default()
            .title(Span::styled(" 🎨  TEAMS ", style_bold(ACCENT_CYAN)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BORDER))
            .style(Style::default().bg(BG)),
    );

    f.render_widget(teams_widget, chunks[1]);
}
