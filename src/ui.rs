use crate::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Table, Row},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(15),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Header
    draw_header(f, chunks[0], app);

    // Main standings table
    draw_standings(f, chunks[1], app);

    // Footer
    draw_footer(f, chunks[2]);
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let header_text = format!(
        "🏁 F1 LIVE TIMING  •  {}  •  {}",
        app.state.session.circuit_short_name,
        app.state.session.session_type
    );

    let header = Paragraph::new(header_text)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM).style(
            Style::default().fg(Color::DarkGray),
        ));

    f.render_widget(header, area);
}

fn draw_standings(f: &mut Frame, area: Rect, app: &App) {
    let mut rows = vec![];

    for standing in &app.state.standings {
        let pos_str = match standing.position_changed {
            1 => format!("▲ {}", standing.position),
            -1 => format!("▼ {}", standing.position),
            _ => format!("  {}", standing.position),
        };

        let color = hex_to_color(&standing.team_color);
        let indicator = if standing.is_fastest_lap { "●" } else { "○" };

        let cells = vec![
            pos_str,
            format!("{} {}", indicator, standing.name),
            standing.lap_time.clone(),
            format!("{:.2}", standing.gap),
            standing.sector1.clone(),
            standing.sector2.clone(),
            standing.sector3.clone(),
            standing.tyres.clone(),
            if standing.drs { "DRS" } else { "—" }.to_string(),
            standing.pit_stops.to_string(),
        ];

        let style = if standing.is_fastest_lap {
            Style::default().fg(Color::Magenta)
        } else {
            Style::default().fg(color)
        };

        let row = Row::new(cells).style(style);
        rows.push(row);
    }

    let table = Table::new(rows, [
        Constraint::Length(4),
        Constraint::Length(6),
        Constraint::Length(9),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(6),
        Constraint::Length(4),
        Constraint::Length(3),
    ])
    .header(
        Row::new(vec![
            "POS", "DRV", "LAP", "GAP", "S1", "S2", "S3", "TYRES", "DRS", "PIT",
        ])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    )
    .block(
        Block::default()
            .title(" STANDINGS ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Green)),
    );

    f.render_widget(table, area);
}

fn draw_footer(f: &mut Frame, area: Rect) {
    let footer = Paragraph::new("Press 'q' or ESC to quit  •  Polling every 2s")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP));

    f.render_widget(footer, area);
}

fn hex_to_color(hex: &str) -> Color {
    // Convert hex color to RGB, then to nearest ANSI 256 color
    let hex = hex.trim_start_matches('#');
    
    if let Ok(num) = u32::from_str_radix(hex, 16) {
        let r = ((num >> 16) & 0xFF) as u8;
        let g = ((num >> 8) & 0xFF) as u8;
        let b = (num & 0xFF) as u8;
        
        // RGB color support in newer ratatui
        Color::Rgb(r, g, b)
    } else {
        Color::White
    }
}
