use crate::app::{ActiveTab, App};
use crate::ui::theme::*;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

pub fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    // Split header into: [title | tabs | status]
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(22),
            Constraint::Min(1),
            Constraint::Length(24),
        ])
        .split(area);

    draw_title(f, cols[0], app);
    draw_tabs(f, cols[1], app);
    draw_status(f, cols[2], app);
}

fn draw_title(f: &mut Frame, area: Rect, _app: &App) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled("▌", style_bold(ACCENT_RED)),
        Span::styled("F1", Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)),
        Span::styled(" LIVE", style_bold(ACCENT_RED)),
        Span::styled(" TIMING", style_bold(TEXT_SECONDARY)),
        Span::styled("▐", style_bold(ACCENT_RED)),
    ]))
    .alignment(Alignment::Left)
    .block(Block::default().style(Style::default().bg(BG)));

    f.render_widget(title, area);
}

fn draw_tabs(f: &mut Frame, area: Rect, app: &App) {
    let mut all_spans: Vec<Span<'static>> = Vec::new();

    for (i, &tab) in ActiveTab::ALL.iter().enumerate() {
        let is_active = tab == app.active_tab;
        let label: &'static str = match tab {
            ActiveTab::Dashboard => "  Dashboard ",
            ActiveTab::Timing    => "  Timing   ",
            ActiveTab::Standings => "  Standings",
        };
        let hint = format!("[{}]", i + 1);

        if is_active {
            all_spans.push(Span::styled(hint, style_bold(ACCENT_RED)));
            all_spans.push(Span::styled(label, Style::default()
                .fg(Color::White)
                .bg(BG_SELECTED)
                .add_modifier(Modifier::BOLD)));
            all_spans.push(Span::raw(" "));
        } else {
            all_spans.push(Span::styled(hint, style_dim(TEXT_DIM)));
            all_spans.push(Span::styled(label, style_normal(TEXT_SECONDARY)));
            all_spans.push(Span::raw(" "));
        }
    }

    let tabs_widget = Paragraph::new(Line::from(all_spans))
        .alignment(Alignment::Left)
        .block(Block::default().style(Style::default().bg(BG)));

    f.render_widget(tabs_widget, area);
}

fn draw_status(f: &mut Frame, area: Rect, app: &App) {
    let secs = app.state.seconds_since_update();
    let (dot_color, status_text) = if app.loading {
        (ACCENT_GOLD, format!("{} fetching...", app.spinner()))
    } else if app.state.error.is_some() {
        (LIVE_RED, "  error".to_string())
    } else if secs < 5 {
        (LIVE_GREEN, "  live".to_string())
    } else {
        (LIVE_YELLOW, format!("  {}s ago", secs))
    };

    let circuit = &app.state.session.circuit_short_name;
    let year = app.state.session.year;
    let sname = &app.state.session.session_name;

    let status = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("●", style_bold(dot_color)),
            Span::styled(format!(" {}", status_text), style_normal(TEXT_SECONDARY)),
        ]),
        Line::from(vec![
            Span::styled(
                format!(" {} {} · {}", year, circuit, sname),
                style_dim(TEXT_DIM),
            ),
        ]),
    ])
    .alignment(Alignment::Right)
    .block(Block::default().style(Style::default().bg(BG)));

    f.render_widget(status, area);
}
