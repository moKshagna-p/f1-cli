# F1 Telemetry Dashboard - Architecture Guide

## Project Structure

```
f1-telemetry-dashboard/
├── src/                          # Main application source code
│   ├── api/
│   │   └── openf1Client.js       # OpenF1 REST API client
│   ├── state/
│   │   └── stateManager.js       # Application state management
│   ├── ui/
│   │   ├── renderer.js           # Terminal rendering engine
│   │   ├── formatters.js         # Data formatting utilities
│   │   └── colors.js             # ANSI color utilities
│   ├── utils/
│   │   ├── constants.js          # Configuration constants
│   │   ├── logger.js             # Logging utility
│   │   └── errorHandler.js       # Error handling utilities
│   └── index.js                  # Main application class
│
├── docs/                         # Documentation
│   ├── ARCHITECTURE.md           # This file
│   ├── API.md                    # API documentation
│   └── SETUP.md                  # Setup and deployment
│
├── config/                       # Configuration files
│   ├── .editorconfig            # Code editor config
│   └── .gitignore               # Git ignore rules
│
├── index.js                      # Application entry point
├── package.json                  # Dependencies and scripts
├── README.md                     # User guide
└── QUICKSTART.md                # Quick start guide
```

---

## Module Overview

### 🔌 API Layer (`src/api/`)

**File**: `openf1Client.js`

Handles all communication with the OpenF1 REST API.

**Key Functions**:
- `fetchSessions()` - Get current F1 session
- `fetchDrivers()` - Get driver list with team colors
- `fetchPositions()` - Get live driver positions
- `fetchLaps()` - Get lap times and sector data
- `fetchPitStops()` - Get pit stop information
- `fetchWeather()` - Get weather conditions
- `fetchAllTelemetry()` - Parallel fetch of all data

**Responsibilities**:
- HTTP requests to OpenF1 API
- Response caching for driver data
- Error handling and logging
- Timeout management

**Example**:
```javascript
const api = require('./api/openf1Client');

const telemetry = await api.fetchAllTelemetry(sessionKey);
const driver = api.getDriver(driverNumber);
```

---

### 📊 State Manager (`src/state/`)

**File**: `stateManager.js`

Manages application state with efficient change detection.

**Key Functions**:
- `updateState()` - Update state and detect changes
- `getState()` - Get current state snapshot
- `resetState()` - Clear all state (new session)
- `setPollingError()` / `clearPollingError()` - Error management

**Responsibilities**:
- Track driver data, positions, laps
- Detect position changes for animations
- Identify fastest lap holder
- Implement efficient diffing (deep equals)
- Manage animation timeouts

**Data Structures**:
```javascript
{
  session: Object,                      // Current F1 session
  drivers: Map<number, Object>,         // Driver info by driver number
  positions: Map<number, Object>,       // Positions by driver number
  laps: Map<number, Object>,            // Latest laps by driver number
  pits: Map<number, Object>,            // Pit stop counts
  weather: Array,                       // Weather history
  positionChanges: Map,                 // Track animations
  fastestLapHolder: number,             // Current fastest lap driver
  isLive: boolean,                      // Session active flag
  pollingError: string,                 // Error message if any
}
```

**Example**:
```javascript
const stateManager = require('./state/stateManager');

const changes = stateManager.updateState(newTelemetry);
const state = stateManager.getState();
```

---

### 🎨 UI Layer (`src/ui/`)

#### 1. **Renderer** (`renderer.js`)
Terminal rendering engine that builds and displays the dashboard.

**Key Functions**:
- `render()` - Clear screen and display dashboard
- `buildDashboard()` - Assemble complete dashboard
- `buildHeader()` - Create session/weather header
- `buildStandings()` - Create driver standings table
- `buildTicker()` - Create status ticker

#### 2. **Formatters** (`formatters.js`)
Data formatting for display.

**Key Functions**:
- `formatTime()` - Convert ms to MM:SS.sss
- `formatGap()` - Format gap to leader
- `formatPosition()` - Format position with change indicator
- `formatTyre()` - Format tyre compound and laps
- `formatDRS()` - Format DRS status
- `formatLapTime()` - Format with fastest lap highlight
- `formatSector()` - Format sector times
- `padColumn()` - Table column padding

#### 3. **Colors** (`colors.js`)
ANSI color utilities.

**Key Functions**:
- `hexToAnsi256()` - Convert hex to ANSI 256-color
- `colorize256()` - Apply 256-color to text
- `colorizeText()` - Apply standard ANSI color
- `getTeamColorSwatch()` - Get team color symbol
- `getTyreColor()` - Get tyre compound color

**Responsibilities**:
- Terminal-based rendering with ASCII art
- ANSI color handling
- Data formatting for display
- Table building and alignment

**Example**:
```javascript
const renderer = require('./ui/renderer');

renderer.render(state);  // Display the dashboard
```

---

### 🛠️ Utilities (`src/utils/`)

#### 1. **Constants** (`constants.js`)
Centralized configuration.

**Exports**:
- `POLLING_INTERVAL` - 2000ms
- `MAX_RETRIES` - 5
- `API_BASE_URL` - 'https://openf1.org/v1'
- `ENDPOINTS` - API endpoint mappings
- `DISPLAY_LIMITS` - Terminal constraints

#### 2. **Logger** (`logger.js`)
Structured logging with namespaces.

**Methods**:
- `debug()` - Debug messages
- `info()` - Info messages
- `warn()` - Warning messages
- `error()` - Error messages

**Example**:
```javascript
const Logger = require('./utils/logger');
const logger = new Logger('MyModule');

logger.info('Starting process...');
logger.error('Something went wrong', error);
```

#### 3. **Error Handler** (`errorHandler.js`)
Error handling and recovery logic.

**Methods**:
- `handleApiError()` - Process API errors
- `isRetryable()` - Check if error should be retried
- `getBackoffDelay()` - Calculate exponential backoff
- `formatErrorMessage()` - User-friendly error text

---

### 🚀 Main Application (`src/index.js`)

**Class**: `F1Dashboard`

Orchestrates the entire application.

**Methods**:
- `initialize()` - Setup and start polling
- `poll()` - Fetch telemetry, update state, render
- `shutdown()` - Graceful shutdown

**Workflow**:
```
1. Initialize
   ├─ Fetch current session
   ├─ Load driver data
   └─ Start polling loop

2. Poll Loop (every 2 seconds)
   ├─ Fetch telemetry (parallel)
   ├─ Update state
   ├─ Render dashboard
   └─ Schedule next poll

3. Shutdown
   ├─ Stop polling
   └─ Exit process
```

---

## Data Flow

### Complete Request Cycle

```
┌─────────────────────────────────────────────────────┐
│ index.js (Entry Point)                              │
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────┐
│ F1Dashboard.poll()                                  │
│ • Call api.fetchAllTelemetry()                      │
└──────────────────┬──────────────────────────────────┘
                   │
        ┌──────────┴──────────┐
        │                     │
        ▼                     ▼
┌──────────────────┐  ┌──────────────────┐
│ API Client       │  │ API Client       │
│ fetchPositions   │  │ fetchLaps        │
│ fetchDrivers     │  │ fetchPits        │
│ fetchWeather     │  │ fetchSessions    │
└────────┬─────────┘  └────────┬─────────┘
         │                     │
         └──────────┬──────────┘
                    │
                    ▼ (Wait for all Promise.all())
         ┌──────────────────────┐
         │ Telemetry Data       │
         └──────────┬───────────┘
                    │
                    ▼
         ┌──────────────────────┐
         │ stateManager         │
         │ .updateState()       │
         │ • Deep equal check   │
         │ • Detect changes     │
         │ • Track animations   │
         └──────────┬───────────┘
                    │
                    ▼
         ┌──────────────────────┐
         │ renderer.render()    │
         │ • Get state snapshot │
         │ • Build dashboard    │
         │ • Display to terminal│
         └──────────────────────┘
```

---

## Key Design Decisions

### 1. **Modular Architecture**
- Each module has single responsibility
- Clear interfaces between modules
- Easy to test and maintain

### 2. **Efficient Diffing**
- Only render when state changes
- Deep equality comparison
- Minimal terminal updates

### 3. **Parallel API Calls**
- Use `Promise.all()` for concurrent requests
- Faster telemetry fetching
- Better responsiveness

### 4. **ANSI Rendering**
- No heavy UI framework dependencies
- Maximum performance
- Terminal agnostic

### 5. **Error Recovery**
- Automatic retry with exponential backoff
- Graceful degradation
- User-friendly error messages

---

## Configuration

### Add New Constants

Edit `src/utils/constants.js`:
```javascript
const MY_NEW_CONSTANT = 'value';

module.exports = {
  // ... existing exports
  MY_NEW_CONSTANT,
};
```

### Change Polling Interval

Edit `src/utils/constants.js`:
```javascript
const POLLING_INTERVAL = 3000; // Change from 2000
```

### Add New API Endpoint

1. Add to `src/utils/constants.js`:
```javascript
const ENDPOINTS = {
  // ... existing
  MY_ENDPOINT: '/my-endpoint',
};
```

2. Add function in `src/api/openf1Client.js`:
```javascript
async function fetchMyData(sessionKey) {
  try {
    const url = `${API_BASE_URL}${ENDPOINTS.MY_ENDPOINT}`;
    const response = await axios.get(url);
    return response.data || [];
  } catch (error) {
    logger.error('Failed to fetch my data', error);
    return [];
  }
}
```

---

## Testing

### Check Syntax
```bash
node -c src/**/*.js
```

### Run with Debug Logging
```bash
DEBUG=true npm start
```

### Test Individual Modules
```javascript
// Test API
const api = require('./src/api/openf1Client');
api.fetchSessions().then(session => console.log(session));

// Test State Manager
const stateManager = require('./src/state/stateManager');
const state = stateManager.getState();
console.log(state);
```

---

## Future Improvements

### Short Term
- [ ] Add configuration file (config.json)
- [ ] Add data export (CSV/JSON)
- [ ] Add historical lap tracking
- [ ] Add custom themes

### Medium Term
- [ ] Add interactive driver selection
- [ ] Add telemetry graphs (speed, brake, throttle)
- [ ] Add lap comparison
- [ ] Add strategy calculator

### Long Term
- [ ] Web-based dashboard
- [ ] Real-time push notifications
- [ ] Database storage for historical data
- [ ] Multi-session comparison

---

## Debugging

### Enable Debug Logs
```bash
DEBUG=true npm start
```

### Check Module Exports
```javascript
console.log(require('./src/api/openf1Client'));
console.log(require('./src/state/stateManager'));
console.log(require('./src/ui/renderer'));
```

### Trace Data Flow
Add logging in `src/index.js`:
```javascript
logger.debug('Fetched telemetry', telemetry);
logger.debug('State changes', changes);
```

---

## Summary

This modular architecture provides:
- ✅ Clear separation of concerns
- ✅ Easy to understand and modify
- ✅ Good for scaling and adding features
- ✅ Maintainable and testable
- ✅ Production-ready code quality
