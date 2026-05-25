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
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(area);

    draw_championship_table(f, cols[0], app);

    let right_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(cols[1]);

    draw_session_card(f, right_rows[0], app);
    draw_weather_card(f, right_rows[1], app);
}

// ─── Championship Standings Table ─────────────────────────────────────────────

pub fn draw_championship_table(f: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["POS", "  DRIVER", "POINTS", "WINS"]
        .iter()
        .map(|h| Cell::from(*h).style(style_bold(ACCENT_GOLD)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(BG_PANEL))
        .height(1);

    let rows: Vec<Row> = app
        .state
        .championship
        .iter()
        .map(|s| make_championship_row(s))
        .collect();

    let widths = [
        Constraint::Length(5),  // POS
        Constraint::Length(25), // DRIVER
        Constraint::Length(8),  // POINTS
        Constraint::Length(5),  // WINS
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(Span::styled(" 🏆  DRIVER CHAMPIONSHIP ", style_bold(ACCENT_GOLD)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BORDER))
                .style(Style::default().bg(BG)),
        )
        .highlight_style(Style::default().bg(BG_SELECTED));

    f.render_widget(table, area);
}

fn make_championship_row<'a>(s: &'a crate::state::ChampionshipStanding) -> Row<'a> {
    let driver_color = hex_color(&s.team_color);

    let pos_cell = Cell::from(Span::styled(
        format!("{:2}", s.position),
        style_bold(TEXT_PRIMARY),
    ));

    let drv_cell = Cell::from(Span::styled(
        s.driver_name.clone(),
        style_bold(driver_color),
    ));

    let pts_cell = Cell::from(Span::styled(
        s.points.clone(),
        style_normal(TEXT_PRIMARY),
    ));

    let wins_cell = Cell::from(Span::styled(
        s.wins.clone(),
        style_normal(TEXT_SECONDARY),
    ));

    Row::new(vec![pos_cell, drv_cell, pts_cell, wins_cell])
        .height(1)
        .style(Style::default().bg(BG))
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

    let p = Paragraph::new(content).block(
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

    let air = w
        .air_temperature
        .map(|t| format!("{:.1}°C", t))
        .unwrap_or_else(|| "—".to_string());
    let trk = w
        .track_temperature
        .map(|t| format!("{:.1}°C", t))
        .unwrap_or_else(|| "—".to_string());
    let hum = w
        .humidity
        .map(|h| format!("{:.0}%", h))
        .unwrap_or_else(|| "—".to_string());
    let wind = w
        .wind_speed
        .map(|s| format!("{:.1} m/s", s))
        .unwrap_or_else(|| "—".to_string());
    let pres = w
        .pressure
        .map(|p| format!("{:.0} hPa", p))
        .unwrap_or_else(|| "—".to_string());

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

    let p = Paragraph::new(content).block(
        Block::default()
            .title(Span::styled(" ⛅ WEATHER ", style_bold(ACCENT_CYAN)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BORDER))
            .style(Style::default().bg(BG)),
    );

    f.render_widget(p, area);
}
