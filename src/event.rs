use crossterm::event::{self, Event, KeyCode, KeyModifiers};

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppEvent {
    Quit,
    Tick,
    Render,
}

#[allow(dead_code)]
pub fn handle_events() -> Option<AppEvent> {
    if event::poll(std::time::Duration::from_millis(100)).ok()? {
        if let Event::Key(key) = event::read().ok()? {
            return match (key.code, key.modifiers) {
                (KeyCode::Char('q'), KeyModifiers::NONE) => Some(AppEvent::Quit),
                (KeyCode::Esc, KeyModifiers::NONE) => Some(AppEvent::Quit),
                _ => None,
            };
        }
    }
    None
}
