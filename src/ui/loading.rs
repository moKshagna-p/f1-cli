use crate::app::App;
use crate::ui::theme::*;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Full-screen loading splash shown while initializing
pub fn draw_loading(f: &mut Frame, area: Rect, app: &App) {
    let bg = Block::default().style(Style::default().bg(BG));
    f.render_widget(bg, area);

    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Min(16),
            Constraint::Percentage(25),
        ])
        .split(area);

    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(vert[1]);

    let splash_area = horiz[1];

    let logo = vec![
        Line::from(Span::styled(r"  ______ __  ", style_bold(ACCENT_RED))),
        Line::from(Span::styled(r" |  ____|/_ | ", style_bold(ACCENT_RED))),
        Line::from(Span::styled(r" | |__   | | ", style_bold(ACCENT_RED))),
        Line::from(Span::styled(r" |  __|  | | ", style_bold(ACCENT_RED))),
        Line::from(Span::styled(r" | |     | | ", style_bold(ACCENT_RED))),
        Line::from(Span::styled(r" |_|     |_| ", style_bold(ACCENT_RED))),
        Line::from(Span::raw("")),
        Line::from(Span::styled(" LIVE TIMING TUI", style_bold(TEXT_PRIMARY))),
        Line::from(Span::raw("")),
        Line::from(vec![
            Span::styled(format!("  {} ", app.spinner()), style_bold(ACCENT_GOLD)),
            Span::styled("Connecting to OpenF1...", style_normal(TEXT_SECONDARY)),
        ]),
        Line::from(Span::raw("")),
        Line::from(Span::styled(
            "  Fetching latest session & drivers",
            style_dim(TEXT_DIM),
        )),
    ];

    let splash = Paragraph::new(logo).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BORDER_FOCUS))
            .style(Style::default().bg(BG)),
    );

    f.render_widget(splash, splash_area);
}

/// Error screen shown on fatal failure
pub fn draw_error(f: &mut Frame, area: Rect, msg: &str) {
    let bg = Block::default().style(Style::default().bg(BG));
    f.render_widget(bg, area);

    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Min(10),
            Constraint::Percentage(35),
        ])
        .split(area);

    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(vert[1]);

    let content = vec![
        Line::from(Span::styled("  ✗  Connection Error", style_bold(LIVE_RED))),
        Line::from(Span::raw("")),
        Line::from(Span::styled(
            format!("  {}", msg),
            style_normal(TEXT_SECONDARY),
        )),
        Line::from(Span::raw("")),
        Line::from(Span::styled(
            "  Check internet connection or try: curl https://api.openf1.org/v1/sessions",
            style_dim(TEXT_DIM),
        )),
        Line::from(Span::raw("")),
        Line::from(Span::styled("  Press q to quit", style_bold(ACCENT_CYAN))),
    ];

    let panel = Paragraph::new(content).alignment(Alignment::Left).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(LIVE_RED))
            .style(Style::default().bg(BG)),
    );

    f.render_widget(panel, horiz[1]);
}
