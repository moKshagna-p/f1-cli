pub mod dashboard;
pub mod footer;
pub mod header;
pub mod loading;
pub mod standings;
pub mod theme;
pub mod timing;

use crate::app::{ActiveTab, App};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::Block,
    Frame,
};
use theme::*;

/// Main draw dispatcher — called every frame
pub fn draw(f: &mut Frame, app: &App) {
    // Full-screen background
    let full = f.size();
    f.render_widget(Block::default().style(Style::default().bg(BG)), full);

    // Outer vertical layout: header | body | footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(1),    // body
            Constraint::Length(1), // footer
        ])
        .split(full);

    header::draw_header(f, chunks[0], app);
    footer::draw_footer(f, chunks[2], app);

    match app.active_tab {
        ActiveTab::Dashboard => dashboard::draw_dashboard(f, chunks[1], app),
        ActiveTab::Timing => timing::draw_timing(f, chunks[1], app),
        ActiveTab::Standings => standings::draw_standings(f, chunks[1], app),
    }
}

/// Draw the loading splash (before app state is ready)
pub fn draw_loading_screen(f: &mut Frame, app: &App) {
    let full = f.size();
    loading::draw_loading(f, full, app);
}

/// Draw the error screen
pub fn draw_error_screen(f: &mut Frame, msg: &str) {
    let full = f.size();
    loading::draw_error(f, full, msg);
}
