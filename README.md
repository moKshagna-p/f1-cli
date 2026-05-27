# F1 Live Telemetry Dashboard

A real-time Formula 1 telemetry dashboard for the terminal. Built with Rust and Ratatui.

## Features

- **Performance**: Compiled Rust binary utilizing an in-memory diffing algorithm for efficient terminal rendering.
- **Terminal UI**: Full RGB color support (256-color and true color) representing actual F1 team colors.
- **Live Telemetry**: Real-time driver positions, intervals, sector times (S1/S2/S3), pit stops, and DRS status indicators.
- **API Polling**: Configurable polling interval (default 2s) with delta-based UI updates to minimize rendering overhead.
- **Championship Data**: Integrated driver championship standings on the main dashboard.

## Quick Start

### Prerequisites
- [Rust 1.70+](https://rustup.rs/) installed.
- A terminal emulator with true-color (256-color) support.

### Installation

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/moKshagna-p/f1-cli.git
cd f1-cli

cargo run --release
```

**Controls:**
- `q` or `ESC`: Exit the application.
- `Tab` or `j`/`k`: Navigate between tabs.

## Architecture

The project is structured into distinct modules handling networking, state management, and UI rendering.

```text
src/
├── main.rs          # Application entry point and TUI event loop
├── app.rs           # Core state machine and tab navigation logic
├── state.rs         # Data transformation and differential state updates
├── api.rs           # Asynchronous OpenF1 API client (reqwest)
├── event.rs         # Terminal input handling (crossterm)
└── ui/              # Modular rendering components (Ratatui)
    ├── dashboard.rs # Championship standings grid
    ├── header.rs    # Session metadata header
    └── theme.rs     # Styling constants
```

### Data Flow

```text
Event Loop (200ms ticks)
  ├─ Render UI (Ratatui)
  ├─ Poll keyboard events (crossterm)
  └─ API Polling (every 2s)
      ├─ Fetch drivers, positions, laps, pits, weather asynchronously
      ├─ Calculate diffs in state manager
      └─ Update live standings and trigger animations
```

## Performance

| Metric | Measurement | Notes |
|--------|-------------|-------|
| Binary Size | ~10 MB | Compiled `--release` binary |
| Memory Footprint | ~40-60 MB | Depending on session size |
| Render Time | <1 ms | Handled entirely in-memory |
| CPU Usage | <2% | During idle polling periods |
| Network I/O | ~15 KB | Compressed JSON payloads per 2s poll |
| API Latency | 2-3s | Typical delay behind live track timing |

## Dependencies

- `ratatui` (TUI rendering framework)
- `crossterm` (Terminal manipulation & events)
- `tokio` (Asynchronous runtime)
- `reqwest` (HTTP client)
- `serde` (JSON serialization/deserialization)

## Configuration

Polling behavior can be adjusted by modifying the interval constant in `src/main.rs`:

```rust
// Adjust the primary polling interval (default: 2 seconds)
let mut api_ticker = interval(Duration::from_secs(2));
```

## Troubleshooting

| Issue | Potential Fix |
|-------|---------------|
| Compilation Error | Ensure the Rust toolchain is up to date: `rustup update` |
| UI Alignment Issues | Ensure the terminal width is at least 100 columns. |
| Missing Colors | Verify the terminal emulator supports true-color formatting. |
| No Data Available | Ensure there is an active internet connection and that an F1 session is live or recently concluded. |

## License

MIT License
