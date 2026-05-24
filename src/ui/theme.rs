use ratatui::style::{Color, Modifier, Style};

// ─── Color Palette ────────────────────────────────────────────────────────────

pub const BG: Color           = Color::Rgb(8, 8, 18);
pub const BG_PANEL: Color     = Color::Rgb(14, 14, 28);
pub const BG_SELECTED: Color  = Color::Rgb(25, 25, 50);
pub const BORDER: Color       = Color::Rgb(45, 45, 80);
pub const BORDER_FOCUS: Color = Color::Rgb(200, 30, 30);

pub const TEXT_PRIMARY:   Color = Color::Rgb(230, 230, 245);
pub const TEXT_SECONDARY: Color = Color::Rgb(140, 140, 170);
pub const TEXT_DIM:       Color = Color::Rgb(70, 70, 100);

pub const ACCENT_RED:    Color = Color::Rgb(220, 30,  30);
pub const ACCENT_GOLD:   Color = Color::Rgb(220, 180, 30);
pub const ACCENT_GREEN:  Color = Color::Rgb(40,  200, 80);
pub const ACCENT_PURPLE: Color = Color::Rgb(180, 60,  220);
pub const ACCENT_CYAN:   Color = Color::Rgb(40,  200, 220);
pub const ACCENT_ORANGE: Color = Color::Rgb(220, 120, 30);

pub const LIVE_GREEN:   Color = Color::Rgb(50,  220, 100);
pub const LIVE_YELLOW:  Color = Color::Rgb(240, 200, 0);
pub const LIVE_RED:     Color = Color::Rgb(240, 50,  50);

// Sector colour codes
pub const SECTOR_FASTEST:    Color = Color::Rgb(180, 50,  220); // purple = overall fastest
pub const SECTOR_NORMAL:     Color = Color::Rgb(220, 220, 60);  // yellow = normal
pub const SECTOR_NO_DATA:    Color = Color::Rgb(80,  80,  110); // grey = no data

// ─── F1 Team Colors ───────────────────────────────────────────────────────────

/// Parse a hex string like "FF8C00" or "#FF8C00" to Color::Rgb
pub fn hex_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if let Ok(num) = u32::from_str_radix(hex, 16) {
        let r = ((num >> 16) & 0xFF) as u8;
        let g = ((num >> 8)  & 0xFF) as u8;
        let b = (num         & 0xFF) as u8;
        Color::Rgb(r, g, b)
    } else {
        TEXT_PRIMARY
    }
}

// ─── Style Helpers ────────────────────────────────────────────────────────────

pub fn style_bold(fg: Color) -> Style {
    Style::default().fg(fg).add_modifier(Modifier::BOLD)
}

pub fn style_dim(fg: Color) -> Style {
    Style::default().fg(fg).add_modifier(Modifier::DIM)
}

pub fn style_normal(fg: Color) -> Style {
    Style::default().fg(fg)
}
