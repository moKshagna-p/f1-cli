use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::Duration;

/// All events the app can respond to
#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    Quit,
    Tick,
    Tab(usize),
    NextTab,
    PrevTab,
    ScrollUp,
    ScrollDown,
    Refresh,
}

pub fn poll_event(timeout: Duration) -> Option<AppEvent> {
    if event::poll(timeout).ok()? {
        if let Event::Key(key) = event::read().ok()? {
            return match (key.code, key.modifiers) {
                // Quit
                (KeyCode::Char('q'), KeyModifiers::NONE)
                | (KeyCode::Char('Q'), _)
                | (KeyCode::Esc, _) => Some(AppEvent::Quit),

                // Tab switching by number
                (KeyCode::Char('1'), KeyModifiers::NONE) => Some(AppEvent::Tab(0)),
                (KeyCode::Char('2'), KeyModifiers::NONE) => Some(AppEvent::Tab(1)),
                (KeyCode::Char('3'), KeyModifiers::NONE) => Some(AppEvent::Tab(2)),

                // Tab cycling
                (KeyCode::Tab, KeyModifiers::NONE) => Some(AppEvent::NextTab),
                (KeyCode::BackTab, _) => Some(AppEvent::PrevTab),
                (KeyCode::Right, _) => Some(AppEvent::NextTab),
                (KeyCode::Left, _) => Some(AppEvent::PrevTab),

                // Scrolling
                (KeyCode::Char('j'), KeyModifiers::NONE)
                | (KeyCode::Down, _) => Some(AppEvent::ScrollDown),
                (KeyCode::Char('k'), KeyModifiers::NONE)
                | (KeyCode::Up, _) => Some(AppEvent::ScrollUp),

                // Manual refresh
                (KeyCode::Char('r'), KeyModifiers::NONE) => Some(AppEvent::Refresh),

                _ => None,
            };
        }
    }
    None
}
