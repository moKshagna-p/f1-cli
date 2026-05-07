# Setup and Installation Guide

## ✅ Prerequisites

- **Node.js** 14.0.0 or higher
- **npm** or **yarn**
- **Terminal** with 256-color support
- **Internet connection** (for OpenF1 API)

## 📋 Installation Steps

### Step 1: Clone or Download Project

```bash
git clone https://github.com/yourusername/f1-telemetry-dashboard.git
cd f1-telemetry-dashboard
```

Or download the ZIP and extract:
```bash
unzip f1-telemetry-dashboard.zip
cd f1-telemetry-dashboard
```

### Step 2: Verify Project Structure

```bash
ls -la src/
```

Should show:
```
api/
state/
ui/
utils/
index.js
```

### Step 3: Install Dependencies

```bash
npm install
```

This installs:
- `axios` - HTTP client for API requests

Should take ~10-30 seconds.

### Step 4: Verify Installation

```bash
node -c index.js
node -c src/index.js
```

Both should show no errors.

## 🚀 Running the Dashboard

### Basic Startup

```bash
npm start
```

Or directly:

```bash
node index.js
```

### Expected Output

You should see:
```
🏁 F1 Telemetry Dashboard - Initializing...

✓ Found session: Race at Monaco
✓ Session Key: 1234
✓ Loaded driver data

📡 Starting live polling...
```

Then the live dashboard appears.

### Exiting

Press `Ctrl+C` to exit gracefully:
```
👋 Shutting down dashboard...
```

## 🔧 Configuration

### Change Polling Interval

Edit `src/utils/constants.js`:

```javascript
const POLLING_INTERVAL = 2000; // milliseconds (default 2 seconds)
```

Change `2000` to your desired interval (e.g., `3000` for 3 seconds).

### Show More/Fewer Drivers

Edit `src/utils/constants.js`:

```javascript
const DISPLAY_LIMITS = {
  MAX_DRIVERS: 30,  // Change this number
  // ...
};
```

### Enable Debug Logging

Run with environment variable:

```bash
DEBUG=true npm start
```

This will print debug information to help troubleshoot.

## 🐛 Troubleshooting

### "npm: command not found"

Install Node.js from: https://nodejs.org/

### "No data appearing after 15 seconds"

**Check 1: Internet Connection**
```bash
curl https://openf1.org/v1/sessions
```

**Check 2: F1 Session Status**
If there's no active F1 session, try again during race weekend.

**Check 3: Look for Error Messages**
Check the ticker at the bottom of dashboard for error indicators.

### "Display looks garbled or cut off"

**Solution 1: Resize Terminal**
- Minimum width: 100 characters
- Minimum height: 40 lines
- Run `tput cols` and `tput lines` to check

**Solution 2: Enable 256-color Support**
- macOS Terminal: Preferences → Advanced → Declare terminal as "xterm-256color"
- iTerm2: Preferences → Profiles → Terminal → Report Terminal Type "xterm-256color"
- Linux: Add to `.bashrc`: `export TERM=xterm-256color`

**Solution 3: Try Different Terminal**
- macOS: iTerm2 (better than built-in Terminal)
- Linux: GNOME Terminal or Konsole
- Windows: Windows Terminal or WSL2

### "Module not found" or "Cannot find module"

```bash
# Clear and reinstall
rm -rf node_modules package-lock.json
npm install

# Verify structure
ls -la src/
ls -la src/api/
ls -la src/state/
ls -la src/ui/
ls -la src/utils/

# Try again
npm start
```

### "API Error" messages

**429 (Rate Limited)**
- OpenF1 has rate limits
- Dashboard respects this automatically
- Wait a few seconds and it will recover

**Connection Refused**
- Check internet connection
- Verify OpenF1 API is accessible
- Try: `curl https://openf1.org/v1/sessions`

**Timeout**
- API might be slow or unreachable
- Dashboard will retry automatically
- Check internet stability

## 🔍 Verification Checklist

After installation, verify:

- [ ] `node --version` returns v14 or higher
- [ ] `npm --version` returns v6 or higher
- [ ] `src/` directory exists with all subdirectories
- [ ] `npm install` completed without errors
- [ ] `node -c index.js` passes
- [ ] Terminal is at least 100×40 characters
- [ ] Internet connection works
- [ ] Can access https://openf1.org/v1/sessions

## 📁 Project Structure After Setup

```
f1-telemetry-dashboard/
├── src/                      # ✅ All application code
│   ├── api/
│   │   └── openf1Client.js
│   ├── state/
│   │   └── stateManager.js
│   ├── ui/
│   │   ├── renderer.js
│   │   ├── formatters.js
│   │   └── colors.js
│   ├── utils/
│   │   ├── constants.js
│   │   ├── logger.js
│   │   └── errorHandler.js
│   └── index.js
├── docs/
│   ├── ARCHITECTURE.md       # ✅ Read this for code overview
│   ├── SETUP.md             # ✅ You are here
│   └── API.md
├── index.js                  # ✅ Entry point
├── package.json             # ✅ Dependencies
├── README.md                # ✅ User guide
├── QUICKSTART.md            # ✅ Quick start
├── node_modules/            # ✅ Installed packages
└── .gitignore              # ✅ Git configuration
```

## 🌐 Network Requirements

The dashboard needs internet access for:

1. **Session Data** - Latest F1 sessions
2. **Driver Info** - Team colors and names
3. **Live Positions** - Current standings
4. **Lap Times** - Sector and lap data
5. **Pit Stops** - Pit stop information
6. **Weather** - Track conditions

All data comes from: https://openf1.org/v1

## 💾 System Requirements

| Item | Minimum | Recommended |
|------|---------|------------|
| Node.js | 14.0.0 | 18.0.0+ |
| RAM | 100MB | 500MB+ |
| Disk | 50MB | 100MB+ |
| Internet | 10KB/s | 100KB/s+ |
| Terminal | 100×40 | 120×50+ |

## 🎓 Next Steps

1. **Read the Architecture**: `docs/ARCHITECTURE.md`
2. **Understand the Code**: Read code comments
3. **Modify and Experiment**: Change constants and test
4. **Deploy**: Run on a server using process manager

## 📚 Documentation

- `README.md` - User guide and features
- `QUICKSTART.md` - 2-minute quick start
- `docs/ARCHITECTURE.md` - Code structure and design
- `docs/API.md` - API documentation
- `src/api/openf1Client.js` - API client code comments
- `src/state/stateManager.js` - State management code comments
- `src/ui/renderer.js` - UI rendering code comments

## 🆘 Getting Help

### Check Documentation

1. Read relevant `docs/*.md` file
2. Check code comments in source files
3. Look at function JSDoc blocks

### Debug Information

Run with debug mode:
```bash
DEBUG=true npm start
```

This will show:
- When data is fetched
- State changes
- Position updates
- Rendering cycles

### Common Issues

| Issue | Solution |
|-------|----------|
| No data | Wait 15s, check internet, check if F1 session is live |
| Garbled display | Resize terminal, enable 256-color, try different terminal |
| High CPU | This is normal (updating every 2s), not a real issue |
| Freeze | Ctrl+C to exit, then restart |
| Missing module | Run `npm install` again |

## ✨ You're Ready!

Your dashboard is ready to display live F1 telemetry!

```bash
npm start
```

Enjoy! 🏁
