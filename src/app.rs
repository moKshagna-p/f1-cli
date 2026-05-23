use crate::event::AppEvent;
use crate::state::AppState;

/// Which tab is currently shown
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActiveTab {
    Dashboard = 0,
    Timing    = 1,
    Standings = 2,
}

impl ActiveTab {
    pub const ALL: [ActiveTab; 3] = [ActiveTab::Dashboard, ActiveTab::Timing, ActiveTab::Standings];

    pub fn index(self) -> usize { self as usize }

    pub fn next(self) -> Self {
        match self {
            ActiveTab::Dashboard => ActiveTab::Timing,
            ActiveTab::Timing    => ActiveTab::Standings,
            ActiveTab::Standings => ActiveTab::Dashboard,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            ActiveTab::Dashboard => ActiveTab::Standings,
            ActiveTab::Timing    => ActiveTab::Dashboard,
            ActiveTab::Standings => ActiveTab::Timing,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ActiveTab::Dashboard => " 󰕮  Dashboard ",
            ActiveTab::Timing    => "  Timing    ",
            ActiveTab::Standings => " 󰊫  Standings ",
        }
    }
}

// ─── App Controller ───────────────────────────────────────────────────────────

pub struct App {
    pub state: AppState,
    pub active_tab: ActiveTab,
    pub scroll_offset: usize,
    pub loading: bool,
    pub spinner_idx: usize,
    pub should_quit: bool,
    pub force_refresh: bool,
}

impl App {
    pub fn new(state: AppState) -> Self {
        Self {
            state,
            active_tab: ActiveTab::Dashboard,
            scroll_offset: 0,
            loading: false,
            spinner_idx: 0,
            should_quit: false,
            force_refresh: false,
        }
    }

    pub fn on_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Quit => self.should_quit = true,

            AppEvent::Tab(i) => {
                self.active_tab = match i {
                    0 => ActiveTab::Dashboard,
                    1 => ActiveTab::Timing,
                    2 => ActiveTab::Standings,
                    _ => self.active_tab,
                };
                self.scroll_offset = 0;
            }

            AppEvent::NextTab => {
                self.active_tab = self.active_tab.next();
                self.scroll_offset = 0;
            }

            AppEvent::PrevTab => {
                self.active_tab = self.active_tab.prev();
                self.scroll_offset = 0;
            }

            AppEvent::ScrollDown => {
                let max = self.state.standings.len().saturating_sub(1);
                if self.scroll_offset < max {
                    self.scroll_offset += 1;
                }
            }

            AppEvent::ScrollUp => {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                }
            }

            AppEvent::Refresh => {
                self.force_refresh = true;
                self.loading = true;
            }

            AppEvent::Tick => {
                self.spinner_idx = (self.spinner_idx + 1) % SPINNER_FRAMES.len();
                self.state.tick();
            }
        }
    }

    pub fn spinner(&self) -> &'static str {
        SPINNER_FRAMES[self.spinner_idx]
    }
}

pub const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
