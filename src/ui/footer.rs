use crate::app::{ActiveTab, App};
use crate::ui::theme::*;
use ratatui::{
    layout::Alignment,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

pub fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let keys = match app.active_tab {
        ActiveTab::Dashboard => vec![
            key_hint("1/2/3", "switch tab"),
            key_hint("r", "refresh"),
            key_hint("q", "quit"),
        ],
        ActiveTab::Timing => vec![
            key_hint("j/k ↑↓", "scroll"),
            key_hint("1/2/3", "switch tab"),
            key_hint("r", "refresh"),
            key_hint("q", "quit"),
        ],
        ActiveTab::Standings => vec![
            key_hint("j/k ↑↓", "scroll"),
            key_hint("1/2/3", "switch tab"),
            key_hint("q", "quit"),
        ],
    };

    let mut spans: Vec<Span> = vec![Span::raw("  ")];
    for (i, (k, v)) in keys.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ·  ", style_dim(TEXT_DIM)));
        }
        spans.push(Span::styled(k.as_str(), style_bold(ACCENT_CYAN)));
        spans.push(Span::styled(" ", style_normal(TEXT_DIM)));
        spans.push(Span::styled(v.as_str(), style_normal(TEXT_SECONDARY)));
    }

    let footer = Paragraph::new(Line::from(spans))
        .alignment(Alignment::Left)
        .block(Block::default().style(ratatui::style::Style::default().bg(BG).fg(TEXT_DIM)));

    f.render_widget(footer, area);
}

fn key_hint(key: &str, desc: &str) -> (String, String) {
    (key.to_string(), desc.to_string())
}
