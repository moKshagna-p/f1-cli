use crate::app::App;
use crate::state::DriverStanding;
use crate::ui::theme::*;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

// ─── Dashboard: 3-panel layout ────────────────────────────────────────────────

pub fn draw_dashboard(f: &mut Frame, area: Rect, app: &App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(62),
            Constraint::Percentage(38),
        ])
        .split(area);

    draw_timing_table(f, cols[0], app);

    let right_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(cols[1]);

    draw_session_card(f, right_rows[0], app);
    draw_weather_card(f, right_rows[1], app);
}

// ─── Mini Timing Table ────────────────────────────────────────────────────────

pub fn draw_timing_table(f: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["POS", "  DRIVER", "GAP", "INTERVAL", "LAP", "PITS"]
        .iter()
        .map(|h| Cell::from(*h).style(style_bold(ACCENT_GOLD)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(BG_PANEL))
        .height(1);

    let rows: Vec<Row> = app
        .state
        .standings
        .iter()
        .map(|s| make_dashboard_row(s, &app.state))
        .collect();

    let widths = [
        Constraint::Length(5),   // POS
        Constraint::Length(10),  // DRIVER
        Constraint::Min(10),     // GAP
        Constraint::Min(10),     // INTERVAL
        Constraint::Length(5),   // LAP
        Constraint::Length(4),   // PITS
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(Span::styled(" ⏱  LIVE TIMING ", style_bold(ACCENT_RED)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BORDER))
                .style(Style::default().bg(BG)),
        )
        .highlight_style(Style::default().bg(BG_SELECTED));

    f.render_widget(table, area);
}

fn make_dashboard_row<'a>(s: &'a DriverStanding, _state: &crate::state::AppState) -> Row<'a> {
    let driver_color = hex_color(&s.team_color);

    // Position with delta arrow
    let pos_cell = {
        let (arrow, arrow_color) = match s.position_delta {
            1  => ("▲", ACCENT_GREEN),
            -1 => ("▼", LIVE_RED),
            _  => (" ", TEXT_DIM),
        };
        Cell::from(Line::from(vec![
            Span::styled(arrow, style_bold(arrow_color)),
            Span::styled(format!("{:2}", s.position), style_bold(TEXT_PRIMARY)),
        ]))
    };

    // Driver with fastest lap indicator
    let drv_cell = {
        let indicator = if s.is_fastest_lap { "● " } else { "  " };
        let ind_color = if s.is_fastest_lap { ACCENT_PURPLE } else { TEXT_DIM };
        Cell::from(Line::from(vec![
            Span::styled(indicator, style_bold(ind_color)),
            Span::styled(s.acronym.clone(), style_bold(driver_color)),
        ]))
    };

    let gap_cell = Cell::from(Span::styled(
        s.gap_display(),
        if s.gap_to_leader.is_none() {
            style_bold(ACCENT_GOLD)
        } else {
            style_normal(TEXT_PRIMARY)
        },
    ));

    let interval_cell = Cell::from(Span::styled(
        s.interval_display(),
        style_normal(TEXT_SECONDARY),
    ));

    let lap_cell = Cell::from(Span::styled(
        if s.lap_number > 0 { format!("L{}", s.lap_number) } else { "—".to_string() },
        style_normal(TEXT_SECONDARY),
    ));

    let pit_cell = Cell::from(Span::styled(
        if s.pit_stops > 0 { s.pit_stops.to_string() } else { "—".to_string() },
        style_normal(TEXT_DIM),
    ));

    let height = 1u16;
    let style = if s.is_fastest_lap {
        Style::default().bg(BG_PANEL).add_modifier(Modifier::BOLD)
    } else {
        Style::default().bg(BG)
    };

    Row::new(vec![pos_cell, drv_cell, gap_cell, interval_cell, lap_cell, pit_cell])
        .height(height)
        .style(style)
}

// ─── Session Info Card ────────────────────────────────────────────────────────

fn draw_session_card(f: &mut Frame, area: Rect, app: &App) {
    let sess = &app.state.session;

    let content = vec![
        info_row("Circuit", &sess.circuit_short_name),
        info_row("Country", &sess.country_name),
        info_row("Meeting", &sess.meeting_name),
        info_row("Session", &sess.session_name),
        info_row("Type", &sess.session_type),
        info_row("Year", &sess.year.to_string()),
        info_row("Key", &sess.session_key.to_string()),
    ];

    let p = Paragraph::new(content)
        .block(
            Block::default()
                .title(Span::styled(" 🏎  SESSION ", style_bold(ACCENT_CYAN)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BORDER))
                .style(Style::default().bg(BG)),
        );

    f.render_widget(p, area);
}

fn info_row(label: &str, value: &str) -> Line<'static> {
    let v = if value.is_empty() { "—" } else { value };
    Line::from(vec![
        Span::styled(format!("  {:<10}", label), style_normal(TEXT_SECONDARY)),
        Span::styled(v.to_string(), style_bold(TEXT_PRIMARY)),
    ])
}

// ─── Weather Card ─────────────────────────────────────────────────────────────

fn draw_weather_card(f: &mut Frame, area: Rect, app: &App) {
    let w = &app.state.weather;

    let rain = w.rainfall.unwrap_or(false);
    let rain_str = if rain { "🌧  Yes" } else { "☀  No" };
    let rain_color = if rain { LIVE_RED } else { ACCENT_GREEN };

    let air  = w.air_temperature.map(|t| format!("{:.1}°C", t)).unwrap_or_else(|| "—".to_string());
    let trk  = w.track_temperature.map(|t| format!("{:.1}°C", t)).unwrap_or_else(|| "—".to_string());
    let hum  = w.humidity.map(|h| format!("{:.0}%", h)).unwrap_or_else(|| "—".to_string());
    let wind = w.wind_speed.map(|s| format!("{:.1} m/s", s)).unwrap_or_else(|| "—".to_string());
    let pres = w.pressure.map(|p| format!("{:.0} hPa", p)).unwrap_or_else(|| "—".to_string());

    let content = vec![
        Line::from(vec![
            Span::styled("  Air Temp  ", style_normal(TEXT_SECONDARY)),
            Span::styled(air, style_bold(ACCENT_ORANGE)),
        ]),
        Line::from(vec![
            Span::styled("  Track     ", style_normal(TEXT_SECONDARY)),
            Span::styled(trk, style_bold(ACCENT_ORANGE)),
        ]),
        Line::from(vec![
            Span::styled("  Humidity  ", style_normal(TEXT_SECONDARY)),
            Span::styled(hum, style_bold(TEXT_PRIMARY)),
        ]),
        Line::from(vec![
            Span::styled("  Wind      ", style_normal(TEXT_SECONDARY)),
            Span::styled(wind, style_bold(TEXT_PRIMARY)),
        ]),
        Line::from(vec![
            Span::styled("  Pressure  ", style_normal(TEXT_SECONDARY)),
            Span::styled(pres, style_bold(TEXT_PRIMARY)),
        ]),
        Line::from(vec![
            Span::styled("  Rainfall  ", style_normal(TEXT_SECONDARY)),
            Span::styled(rain_str, style_bold(rain_color)),
        ]),
    ];

    let p = Paragraph::new(content)
        .block(
            Block::default()
                .title(Span::styled(" ⛅ WEATHER ", style_bold(ACCENT_CYAN)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BORDER))
                .style(Style::default().bg(BG)),
        );

    f.render_widget(p, area);
}
