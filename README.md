<div align="center">
  <h1>🏎️ F1 Live Telemetry Dashboard</h1>
  <p><strong>Blazingly fast, beautiful real-time F1 race data in your terminal.</strong></p>
  <p>Built with <b>Rust</b> and <b>Ratatui</b> for maximum performance & gorgeous UI.</p>
</div>

<br />

## ✨ Features

- 🚀 **Ultra-Fast & Efficient**: Compiled Rust binary with a zero-overhead diffing algorithm and sub-millisecond frame times. Uses only ~10MB of disk space and <2% CPU at idle.
- 🎨 **Beautiful UI**: Rich RGB color support (256-color + true color) with accurate F1 team colors.
- 📊 **Full Telemetry**: Live driver positions, gaps, sector times (S1/S2/S3), pit stop counting, DRS status indicators, and tyre compounds.
- 🎯 **Smart Polling**: Intelligent API polling (default 2s) that only renders changed data.
- 🎬 **Animations**: Smooth visual animations for position changes (▲▼) fading over time.

## 🛠️ Quick Start

### Prerequisites
- [Rust 1.70+](https://rustup.rs/) installed on your machine.
- A modern terminal emulator with true-color (256-color) support.

### Installation & Execution

Clone the repository and run it directly using Cargo:

```bash
# Clone the repo
git clone https://github.com/moKshagna-p/f1-cli.git
cd f1-cli

# Run directly (Cargo will build and run it for you)
cargo run --release
```

**Controls:**
- Press `q` or `ESC` to gracefully exit the dashboard.

## 📐 Architecture

The project follows a clean, decoupled architecture:

```text
src/
├── main.rs          # Application entry point and TUI event loop
├── app.rs           # Core state machine and tab navigation logic
├── state.rs         # Data transformation and differential state updates
├── api.rs           # Asynchronous OpenF1 API client (reqwest)
├── event.rs         # Terminal input handling (crossterm)
└── ui/              # Modular rendering components (Ratatui)
    ├── dashboard.rs # Main telemetry grid
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

## ⚡ Performance Profiling

| Metric | Measurement | Notes |
|--------|-------------|-------|
| **Binary Size** | ~10 MB | Compiled `--release` binary |
| **Memory Footprint**| ~40-60 MB | Depending on session size |
| **Render Time** | <1 ms | Handled entirely in-memory |
| **CPU Usage** | <2% | During idle polling periods |
| **Network I/O** | ~15 KB | Compressed JSON payloads per 2s poll |
| **API Latency** | 2-3s | Typical delay behind live track timing |

## 🧩 Dependencies

Built using industry-standard Rust ecosystem libraries:
- `ratatui` (TUI rendering framework)
- `crossterm` (Terminal manipulation & events)
- `tokio` (Asynchronous runtime)
- `reqwest` (HTTP client)
- `serde` (JSON serialization/deserialization)

## 🔧 Configuration

You can tweak the polling behavior by modifying the constants in `src/main.rs`:

```rust
// Adjust the primary polling interval (default: 2 seconds)
let mut api_ticker = interval(Duration::from_secs(2));
```

## 🐛 Troubleshooting

| Issue | Potential Fix |
|-------|---------------|
| **Compilation Error** | Ensure your Rust toolchain is up to date: `rustup update` |
| **Garbled UI** | Make sure your terminal is wide enough (100+ columns). |
| **Missing Colors** | Verify that your terminal supports RGB / True-Color. |
| **No Data Showing** | Ensure you have an active internet connection and that an F1 session is currently live (or recently finished). |

## 📜 License

MIT License — Use and modify freely.

<div align="center">
  <br/>
  <i>Built for F1 fans who need speed ⚡</i>
</div>
