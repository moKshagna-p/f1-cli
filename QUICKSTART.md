# F1 Telemetry Dashboard - Quick Start Guide

## 🚀 Get Started in 2 Minutes

### 1. Install Dependencies
```bash
npm install
```

### 2. Run the Dashboard
```bash
npm start
```

That's it! The dashboard will start polling the OpenF1 API and display live F1 data.

---

## 📁 Project Structure

```
f1-telemetry-dashboard/
├── index.js          # Main entry point - initializes and runs polling loop
├── api.js            # OpenF1 API client - handles all data fetching
├── state.js          # State management - tracks changes efficiently
├── ui.js             # Terminal UI rendering - ANSI colors and formatting
├── package.json      # Dependencies and scripts
├── README.md         # Full documentation
├── QUICKSTART.md     # This file
├── .gitignore        # Git configuration
└── node_modules/     # Installed dependencies
```

---

## 🎯 Architecture Overview

### Data Flow
```
┌─────────────────┐
│  index.js (Main)│ ◄─── Orchestration
└────────┬────────┘
         │
         ▼
┌──────────────────────────────────────────┐
│ Poll Every 2 Seconds:                    │
│  1. api.js fetches from OpenF1           │
│  2. state.js diffs and updates state     │
│  3. ui.js renders to terminal            │
└──────────────────────────────────────────┘
         │
         ▼
   ┌─────────────┐
   │  Terminal   │ Display
   └─────────────┘
```

### Component Responsibilities

| File | Purpose | Key Functions |
|------|---------|---|
| **index.js** | Entry point & polling orchestration | initialize(), pollTelemetry() |
| **api.js** | OpenF1 API client | fetchSessions(), fetchPositions(), fetchLaps(), etc. |
| **state.js** | State management with diffing | updateState(), getState(), resetState() |
| **ui.js** | Terminal rendering | renderDashboard(), formatTime(), getTyreDisplay() |

---

## 🎨 Features Implemented

✅ **Live Driver Standings** — Real-time position tracking  
✅ **Position Animations** — ▲▼ indicators for position changes (3s fade)  
✅ **Team Colors** — Official F1 team hex colors as ANSI 256-color codes  
✅ **Fastest Lap Highlight** — Purple highlight for fastest lap holder  
✅ **Sector Times** — S1, S2, S3 with purple/green/white coding  
✅ **Tyre Tracking** — Compound type and laps on tyre with color coding  
✅ **DRS Status** — Real-time DRS activation indicator  
✅ **Pit Stop Count** — Number of pit stops per driver  
✅ **Weather Display** — Temperature and rainfall status  
✅ **Efficient Diffing** — Only updates changed data  
✅ **Error Handling** — Graceful retry logic with user feedback  

---

## 🔧 Configuration

### Polling Interval
Edit `index.js` line 6:
```javascript
const POLL_INTERVAL = 2000; // milliseconds
```

### Visible Drivers
Edit `ui.js` buildDashboard() function:
```javascript
.slice(0, 30) // Change 30 to show more/fewer drivers
```

### API Base URL
Edit `api.js` line 3:
```javascript
const BASE_URL = 'https://openf1.org/v1';
```

---

## 🌍 API Data Sources

All data comes from **OpenF1 REST API** (https://openf1.org):

| Endpoint | Data | Purpose |
|----------|------|---------|
| `/sessions` | Session metadata | Find live sessions |
| `/drivers` | Driver info & team colors | Display driver details |
| `/position` | Live positions & gaps | Main standings table |
| `/laps` | Lap times & sectors | Display lap performance |
| `/pit` | Pit stop events | Track pit stops |
| `/weather` | Track/air temp, rainfall | Weather display |
| `/car_data` | Speed, throttle, brake | Optional telemetry (not used) |

---

## 📊 Display Format

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│ RACE | Monaco                                                        Temp: 24°C ☀    │
└─────────────────────────────────────────────────────────────────────────────────────┘
┌────┬──────┬──────────┬──────────┬────────────┬────────────┬────────────┬───────┬─────┬─────┐
│Pos │ Drv  │   Lap    │   Gap    │     S1     │     S2     │     S3     │ Tyres │ DRS │ Pit │
├────┼──────┼──────────┼──────────┼────────────┼────────────┼────────────┼───────┼─────┼─────┤
│ 1  │ ● VER │ 1:32.456 │ LEADER   │ 0:31.234   │ 0:32.123   │ 0:29.099   │ M2    │ —   │  1 │
│▲ 2 │ ● LEC │ 1:32.678 │ +0.222   │ 0:31.456   │ 0:32.234   │ 0:29.032   │ S4    │ DRS │  2 │
└────┴──────┴──────────┴──────────┴────────────┴────────────┴────────────┴───────┴─────┴─────┘
┌─────────────────────────────────────────────────────────────────────────────────────┐
│ LIVE • F1 Telemetry Dashboard • Updated: 16:45:23                                  │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

**Legend:**
- `●` = Team color swatch
- `▲▼` = Position change (fades after 3 seconds)
- Purple time = Fastest lap holder
- Compact format fits in ~100 char × 40 line terminal

---

## 🛠️ Troubleshooting

### "Module not found" Error
```bash
npm install
```

### Garbled display
- Ensure terminal is at least 100 characters wide
- Enable 256-color support in terminal settings
- Try a different terminal (iTerm2, Konsole, etc.)

### No data appearing
- Wait 10-15 seconds for initial API calls
- Check internet connection
- Verify OpenF1 API is accessible: `curl https://openf1.org/v1/sessions`

### Exit the dashboard
Press `Ctrl+C` (will display "👋 Shutting down dashboard...")

---

## 📝 Code Quality Standards

This project follows high-quality practices:

✅ **Modular Design** — Each file has a single responsibility  
✅ **Error Handling** — Try-catch blocks, validation, retry logic  
✅ **Performance** — Efficient diffing, no unnecessary re-renders  
✅ **Documentation** — JSDoc comments on all functions  
✅ **Type Safety** — Proper error handling and edge cases  
✅ **User Feedback** — Clear status messages and indicators  

---

## 🎓 Learning This Code

### To understand data flow:
1. Read `index.js` to see the main loop
2. Trace through `api.js` to understand API calls
3. Study `state.js` to see state management
4. Review `ui.js` for rendering logic

### To add a new feature:
1. Add API endpoint call in `api.js`
2. Add state tracking in `state.js`
3. Add UI rendering in `ui.js`
4. Update main loop in `index.js` if needed

### To customize display:
1. All formatting in `ui.js` buildDashboard()
2. Color codes at top of `ui.js`
3. Layout strings use box-drawing characters (┌─┐│└┘)

---

## 📚 Further Reading

- Full documentation: See `README.md`
- OpenF1 API: https://openf1.org
- OpenTUI: https://opentui.com
- ANSI Colors: https://en.wikipedia.org/wiki/ANSI_escape_code

---

## 📄 License

MIT License - Feel free to use and modify this project

---

**Happy F1 watching from the terminal!** 🏁
