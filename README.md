# F1 Live Telemetry Dashboard

Real-time F1 race data in your terminal. Live driver positions, lap times, weather, pit stops.

## Quick Start

```bash
npm install
npm start
```

Press `Ctrl+C` to exit.

## What You Get

- **Live standings** with position tracking
- **Animated position changes** (▲▼ icons)
- **Team colors** with ANSI 256-color rendering
- **Sector times**, pit stops, DRS status, tyre info
- **Weather data** (temp, rain)
- **Smart diffing** — only renders changes
- **Zero config** — just run it

## Requirements

- Node.js 14+
- Terminal: 100×40 chars min, 256-color support
- Internet (OpenF1 API)

## Project Structure

```
├── index.js                 # Entry point, polling loop
├── src/
│   ├── api/openf1Client.js  # OpenF1 API client
│   ├── state/stateManager.js # State & diffing
│   ├── ui/                   # Rendering (renderer, formatters, colors)
│   └── utils/                # Helpers (constants, logger, errors)
├── docs/ARCHITECTURE.md      # Design & patterns
└── docs/SETUP.md             # Troubleshooting
```

## Display Legend

```
Pos │ Drv │   Lap    │   Gap    │     S1     │     S2     │     S3     │ Tyres │ DRS │ Pit
 ▲ 1 │ ● VER │ 1:32.456 │ LEADER   │ 0:31.234   │ 0:32.123   │ 0:29.099   │ M2    │ —   │  1
```

| Symbol | Meaning |
|--------|---------|
| ● | Fastest lap holder (purple) |
| ▲/▼ | Position change (3s animation) |
| DRS | DRS active |
| M/S/H/I/W | Tyre compound |

## Data Source

**OpenF1 API** (no auth) — `https://api.openf1.org/v1`

Polling interval: 2 seconds | Network: ~10KB/poll

## Troubleshooting

| Issue | Fix |
|-------|-----|
| `404 error` | Check internet, verify `https://api.openf1.org/v1/sessions` works |
| `No data` | Wait 5s, check for active F1 session |
| `Garbled output` | Widen terminal to 100+ chars, enable 256-color mode |
| `Rate limited` | Use default 2s poll interval, don't run multiple instances |

## Performance

- CPU: Minimal (efficient diffing)
- Memory: 50-100MB
- Latency: 2-3s behind live F1 timing

## Development

Debug: Add `console.log` in `src/api/` or `src/state/`

Test API directly:
```bash
curl https://api.openf1.org/v1/sessions | jq '.[0]'
curl https://api.openf1.org/v1/drivers | jq '.[:3]'
curl https://api.openf1.org/v1/position | jq '.[:3]'
```

## Learn More

- `docs/ARCHITECTURE.md` — Deep dive: modules, data flow, extending
- `docs/SETUP.md` — Config, terminal compatibility, advanced troubleshooting
- `PROJECT_STRUCTURE.txt` — Quick reference

## License

MIT — Use freely.

---

**Built for F1 fans who live in the terminal** ❤️
