use crate::app::App;
use crate::state::{format_lap_time, DriverStanding};
use crate::ui::theme::*;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

// ─── Full Timing Tower ────────────────────────────────────────────────────────

pub fn draw_timing(f: &mut Frame, area: Rect, app: &App) {
    let fastest_s1 = app.state.fastest_s1();
    let fastest_s2 = app.state.fastest_s2();
    let fastest_s3 = app.state.fastest_s3();

    let header_cells = [
        "POS", "  DRIVER", "TEAM", "GAP", "INTVL", "LAP", "LAP TIME",
        "  S1   ", "  S2   ", "  S3   ", "PITS",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(style_bold(ACCENT_GOLD)));

    let header = Row::new(header_cells)
        .style(Style::default().bg(BG_PANEL))
        .height(1);

    let rows: Vec<Row> = app
        .state
        .standings
        .iter()
        .map(|s| make_timing_row(s, fastest_s1, fastest_s2, fastest_s3))
        .collect();

    let widths = [
        Constraint::Length(5),   // POS
        Constraint::Length(10),  // DRIVER
        Constraint::Length(14),  // TEAM
        Constraint::Length(10),  // GAP
        Constraint::Length(9),   // INTERVAL
        Constraint::Length(4),   // LAP #
        Constraint::Length(9),   // LAP TIME
        Constraint::Length(8),   // S1
        Constraint::Length(8),   // S2
        Constraint::Length(8),   // S3
        Constraint::Length(4),   // PITS
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
                    " ⏱  FULL TIMING TOWER — j/k to scroll ",
                    style_bold(ACCENT_RED),
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BORDER_FOCUS))
                .style(Style::default().bg(BG)),
        )
        .highlight_style(Style::default().bg(BG_SELECTED).add_modifier(Modifier::BOLD));

    f.render_stateful_widget(table, area, &mut table_state);
}

fn make_timing_row<'a>(
    s: &'a DriverStanding,
    fastest_s1: Option<i32>,
    fastest_s2: Option<i32>,
    fastest_s3: Option<i32>,
) -> Row<'a> {
    let driver_color = hex_color(&s.team_color);

    // POS with delta
    let (arrow, arrow_color) = match s.position_delta {
        1  => ("▲", ACCENT_GREEN),
        -1 => ("▼", LIVE_RED),
        _  => (" ", BG),
    };
    let pos_cell = Cell::from(Line::from(vec![
        Span::styled(arrow, style_bold(arrow_color)),
        Span::styled(format!("{:2}", s.position), style_bold(TEXT_PRIMARY)),
    ]));

    // Driver
    let indicator = if s.is_fastest_lap { "● " } else { "  " };
    let ind_color  = if s.is_fastest_lap { ACCENT_PURPLE } else { BG };
    let drv_cell = Cell::from(Line::from(vec![
        Span::styled(indicator, style_bold(ind_color)),
        Span::styled(s.acronym.clone(), style_bold(driver_color)),
    ]));

    // Team (truncated)
    let team_short: String = s.team.chars().take(13).collect();
    let team_cell = Cell::from(Span::styled(team_short, style_normal(driver_color)));

    // Gap
    let gap_cell = Cell::from(Span::styled(
        s.gap_display(),
        if s.gap_to_leader.is_none() {
            style_bold(ACCENT_GOLD)
        } else {
            style_normal(TEXT_PRIMARY)
        },
    ));

    // Interval
    let int_cell = Cell::from(Span::styled(
        s.interval_display(),
        style_normal(TEXT_SECONDARY),
    ));

    // Lap number
    let lap_num_cell = Cell::from(Span::styled(
        if s.lap_number > 0 { format!("L{}", s.lap_number) } else { "—".to_string() },
        style_dim(TEXT_SECONDARY),
    ));

    // Lap time
    let lap_time_str = s.lap_time
        .map(format_lap_time)
        .unwrap_or_else(|| "  —:—".to_string());
    let lap_time_cell = Cell::from(Span::styled(
        lap_time_str,
        if s.is_fastest_lap {
            style_bold(ACCENT_PURPLE)
        } else {
            style_normal(TEXT_PRIMARY)
        },
    ));

    // Sector cells
    let s1_cell = sector_cell(s.sector1, fastest_s1 == Some(s.number));
    let s2_cell = sector_cell(s.sector2, fastest_s2 == Some(s.number));
    let s3_cell = sector_cell(s.sector3, fastest_s3 == Some(s.number));

    // Pit stops
    let pit_cell = Cell::from(Span::styled(
        if s.pit_stops > 0 { s.pit_stops.to_string() } else { "—".to_string() },
        style_normal(TEXT_DIM),
    ));

    let bg = if s.position <= 3 {
        Style::default().bg(BG_PANEL)
    } else {
        Style::default().bg(BG)
    };

    Row::new(vec![
        pos_cell, drv_cell, team_cell, gap_cell, int_cell,
        lap_num_cell, lap_time_cell, s1_cell, s2_cell, s3_cell, pit_cell,
    ])
    .height(1)
    .style(bg)
}

fn sector_cell(time: Option<f64>, is_fastest: bool) -> Cell<'static> {
    match time {
        None => Cell::from(Span::styled("  —    ".to_string(), style_dim(SECTOR_NO_DATA))),
        Some(t) => {
            let color = if is_fastest { SECTOR_FASTEST } else { SECTOR_NORMAL };
            let prefix = if is_fastest { "▌" } else { " " };
            Cell::from(Span::styled(
                format!("{}{:06.3}", prefix, t),
                style_bold(color),
            ))
        }
    }
}
