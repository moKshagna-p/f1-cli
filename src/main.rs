mod api;
mod event;
mod state;
mod ui;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::{Duration, Instant};
use tokio::time::interval;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Initialize app
    let mut app = App::new().await?;

    // Main loop
    let result = run_app(&mut terminal, &mut app).await;

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

struct App {
    state: state::AppState,
    should_quit: bool,
    tick_count: u64,
}

impl App {
    async fn new() -> Result<Self> {
        println!("🏁 F1 Telemetry Dashboard - Initializing...");
        
        // Fetch initial data
        let session = api::fetch_sessions().await?;
        let drivers = api::fetch_drivers().await?;
        
        println!("✓ Session: {:?}", session.circuit_short_name);
        println!("✓ Drivers loaded: {}", drivers.len());
        
        Ok(Self {
            state: state::AppState::new(session, drivers),
            should_quit: false,
            tick_count: 0,
        })
    }
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    let mut ticker = interval(Duration::from_millis(200)); // 200ms for smooth UI
    let mut api_ticker = interval(Duration::from_secs(2)); // 2s polling
    let mut last_api_poll = Instant::now();
    
    loop {
        // Render
        terminal.draw(|f| ui::draw(f, app))?;

        // Handle events
        if crossterm::event::poll(Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.code == crossterm::event::KeyCode::Char('q') ||
                   key.code == crossterm::event::KeyCode::Esc {
                    app.should_quit = true;
                }
            }
        }

        if app.should_quit {
            break;
        }

        // API polling
        if last_api_poll.elapsed() >= Duration::from_secs(2) {
            if let Ok(telemetry) = api::fetch_all_telemetry(&app.state.session.session_key.to_string()).await {
                app.state.update(telemetry);
            }
            last_api_poll = Instant::now();
        }

        // Tick for animations
        ticker.tick().await;
        app.tick_count = app.tick_count.wrapping_add(1);
    }

    Ok(())
}
