mod api;
mod app;
mod event;
mod state;
mod ui;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{self, IsTerminal},
    sync::Arc,
    time::{Duration, Instant},
};

use app::App;
use event::{poll_event, AppEvent};

// ─── Entry Point ──────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    // Require a real terminal
    if !io::stdout().is_terminal() {
        eprintln!("✗  f1-dashboard requires an interactive terminal.");
        eprintln!("   Run: ./target/release/f1-dashboard");
        std::process::exit(1);
    }

    // Enter TUI mode
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Run the app (handles its own errors gracefully)
    let result = run(&mut terminal).await;

    // Always restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("✗  {}", e);
        std::process::exit(1);
    }

    Ok(())
}

// ─── Main Run Loop ────────────────────────────────────────────────────────────

async fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let http = api::make_client();
    let spinner_app = make_spinner_app();

    // ── Loading screen loop ────────────────────────────────────────────────
    let (session, drivers) = tokio::select! {
        result = init_data(http.clone()) => result,
        _ = loading_loop(terminal, &spinner_app) => {
            // This branch is cancelled when init_data completes
            unreachable!()
        }
    };

    // Handle init failure
    let (session, drivers) = match (session, drivers) {
        (Ok(s), Ok(d)) => (s, d),
        (Err(e), _) | (_, Err(e)) => {
            // Show error screen, wait for q
            loop {
                terminal.draw(|f| {
                    ui::draw_error_screen(f, &e.to_string());
                })?;
                if let Some(ev) = poll_event(Duration::from_millis(100)) {
                    if ev == AppEvent::Quit { return Ok(()); }
                }
            }
        }
    };

    // ── Build app ─────────────────────────────────────────────────────────
    let state = state::AppState::new(session, drivers);
    let mut app = App::new(state);

    // Kick off initial telemetry fetch in background
    let mut last_poll = Instant::now().checked_sub(Duration::from_secs(10)).unwrap_or(Instant::now());
    let poll_interval = Duration::from_secs(5);

    // Tick interval: 200ms for smooth animations
    let tick_duration = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    // ── Main event loop ───────────────────────────────────────────────────
    loop {
        // Render
        terminal.draw(|f| ui::draw(f, &app))?;

        // Handle input (non-blocking, 50ms timeout)
        if let Some(ev) = poll_event(Duration::from_millis(50)) {
            app.on_event(ev.clone());
            if app.should_quit {
                break;
            }
        }

        // Tick
        if last_tick.elapsed() >= tick_duration {
            app.on_event(AppEvent::Tick);
            last_tick = Instant::now();
        }

        // API poll
        let should_poll = last_poll.elapsed() >= poll_interval || app.force_refresh;
        if should_poll {
            app.force_refresh = false;
            app.loading = true;
            let session_key = app.state.session.session_key;
            let client_ref = http.clone();

            // Non-blocking: spawn & poll next frame
            let telemetry = tokio::time::timeout(
                Duration::from_secs(12),
                api::fetch_all_telemetry(&client_ref, session_key),
            )
            .await;

            app.loading = false;
            match telemetry {
                Ok(Ok(t)) => {
                    app.state.update(t);
                    app.state.error = None;
                }
                Ok(Err(e)) => {
                    app.state.error = Some(format!("Fetch error: {}", e));
                }
                Err(_) => {
                    app.state.error = Some("API timeout (>12s)".to_string());
                }
            }

            last_poll = Instant::now();
        }
    }

    Ok(())
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Fetch session + drivers in parallel, return Results separately
async fn init_data(
    client: Arc<reqwest::Client>,
) -> (Result<api::Session>, Result<Vec<api::Driver>>) {
    let session = api::fetch_latest_session(&client).await;
    let drivers = match &session {
        Ok(s) => api::fetch_drivers_for_session(&client, s.session_key).await,
        Err(e) => Err(anyhow::anyhow!("{}", e)),
    };
    (session, drivers)
}

/// A lightweight spinner-only App for use during the loading screen
fn make_spinner_app() -> App {
    use api::Session;
    use state::AppState;
    let s = AppState::new(Session::default(), vec![]);
    App::new(s)
}

/// Drives the loading screen animation — this future is cancelled when init_data resolves
async fn loading_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    _app: &App,
) {
    let mut spinner_idx = 0usize;
    let frames = app::SPINNER_FRAMES;
    loop {
        // We can't mutate app here (borrowed), so build a fake app each frame
        let fake = make_spinner_app_static(spinner_idx);
        terminal.draw(|f| ui::draw_loading_screen(f, &fake)).ok();
        spinner_idx = (spinner_idx + 1) % frames.len();
        tokio::time::sleep(Duration::from_millis(80)).await;
    }
}

fn make_spinner_app_static(spinner_idx: usize) -> App {
    use api::Session;
    use state::AppState;
    let s = AppState::new(Session::default(), vec![]);
    let mut a = App::new(s);
    a.spinner_idx = spinner_idx;
    a
}
