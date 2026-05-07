/**
 * UI layer using pure ANSI terminal rendering
 * Builds and manages the terminal dashboard with real-time updates
 */

// ANSI color codes for terminal
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  dim: '\x1b[2m',
  
  // Standard colors
  black: '\x1b[30m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m',
  white: '\x1b[37m',
  
  // Bright colors
  brightRed: '\x1b[91m',
  brightGreen: '\x1b[92m',
  brightYellow: '\x1b[93m',
  brightBlue: '\x1b[94m',
  brightMagenta: '\x1b[95m',
  brightCyan: '\x1b[96m',
};

/**
 * Convert hex color to ANSI 256-color code
 */
function hexToAnsi256(hex) {
  if (!hex || hex === '#FFFFFF') return 15;
  
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  
  const ri = Math.round(r / 255 * 5);
  const gi = Math.round(g / 255 * 5);
  const bi = Math.round(b / 255 * 5);
  
  return 16 + 36 * ri + 6 * gi + bi;
}

/**
 * Format time duration (milliseconds to MM:SS.sss)
 */
function formatTime(ms) {
  if (!ms) return '—';
  const totalSeconds = ms / 1000;
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes}:${seconds.toFixed(3).padStart(7, '0')}`;
}

/**
 * Format gap to leader
 */
function formatGap(gap) {
  if (gap === null || gap === undefined || gap === 0) return 'LEADER';
  if (gap < 0.001) return '+0.001';
  return `+${gap.toFixed(3)}`;
}

/**
 * Get position change indicator
 */
function getPositionIndicator(positionChange) {
  if (!positionChange) return '  ';
  const direction = positionChange.to < positionChange.from ? '▲' : '▼';
  return `${direction} `;
}

/**
 * Get tyre compound display
 */
function getTyreDisplay(lap) {
  if (!lap || !lap.tyre_compound) return '-';
  
  const compound = lap.tyre_compound.slice(0, 1).toUpperCase();
  const lapsOnTyre = lap.lap_number ? lap.lap_number : 1;
  
  switch (lap.tyre_compound) {
    case 'SOFT': return `${colors.brightRed}${compound}${colors.reset}${lapsOnTyre}`;
    case 'MEDIUM': return `${colors.yellow}${compound}${colors.reset}${lapsOnTyre}`;
    case 'HARD': return `${colors.white}${compound}${colors.reset}${lapsOnTyre}`;
    case 'INTERMEDIATE': return `${colors.green}${compound}${colors.reset}${lapsOnTyre}`;
    case 'WET': return `${colors.cyan}${compound}${colors.reset}${lapsOnTyre}`;
    default: return `${compound}${lapsOnTyre}`;
  }
}

/**
 * Build dashboard using ANSI colors and standard output
 * (OpenTUI rendering happens through the main app)
 */
function buildDashboard(state) {
  const session = state.session || {};
  const weather = state.weather && state.weather[state.weather.length - 1];
  
  const sessionType = session.session_type || 'Practice';
  const trackName = session.location || 'N/A';
  
  const tempStr = weather ? `${Math.round(weather.air_temperature || 0)}°C` : '—°C';
  const rainStr = weather && weather.rainfall ? '🌧' : '☀';
  
  // Header
  const header = `┌─────────────────────────────────────────────────────────────────────────────────────┐
│ ${colors.bright}${sessionType.toUpperCase()} | ${trackName}${colors.reset}${' '.repeat(Math.max(0, 50 - sessionType.length - trackName.length))} Temp: ${tempStr} ${rainStr}  │
└─────────────────────────────────────────────────────────────────────────────────────┘`;

  // Driver standings
  const drivers = state.drivers || new Map();
  const positions = state.positions || new Map();
  const laps = state.laps || new Map();
  const pits = state.pits || new Map();
  const positionChanges = state.positionChanges || new Map();
  const fastestLapDriver = state.fastestLapHolder;
  
  const sorted = Array.from(positions.values())
    .sort((a, b) => a.position - b.position)
    .slice(0, 30);

  const tableHeader = `┌────┬──────┬──────────┬──────────┬────────────┬────────────┬────────────┬───────┬─────┬─────┐
│Pos │ Drv  │   Lap    │   Gap    │     S1     │     S2     │     S3     │ Tyres │ DRS │ Pit │
├────┼──────┼──────────┼──────────┼────────────┼────────────┼────────────┼───────┼─────┼─────┤`;

  const rows = sorted.map(pos => {
    const driver = drivers.get(pos.driver_number);
    if (!driver) return null;
    
    const lap = laps.get(pos.driver_number);
    const pitData = pits.get(pos.driver_number);
    const posChange = positionChanges.get(pos.driver_number);
    
    const posInd = getPositionIndicator(posChange);
    const posStr = `${posInd}${String(pos.position).padStart(2, ' ')}`;
    
    const colorCode = hexToAnsi256(driver.team_colour || '#FFFFFF');
    const codeSwatch = `\x1b[38;5;${colorCode}m●${colors.reset}`;
    const driverCode = driver.driver_abbr || `P${pos.position}`;
    
    const lastLapTime = lap ? lap.lap_duration : null;
    const lapStr = fastestLapDriver === pos.driver_number 
      ? `${colors.magenta}${formatTime(lastLapTime)}${colors.reset}` 
      : formatTime(lastLapTime);
    
    const gapStr = pos.gap_to_leader === null || pos.gap_to_leader === 0 ? 'LEADER' : formatGap(pos.gap_to_leader);
    
    const s1 = lap && lap.sector_1 ? formatTime(lap.sector_1) : '—';
    const s2 = lap && lap.sector_2 ? formatTime(lap.sector_2) : '—';
    const s3 = lap && lap.sector_3 ? formatTime(lap.sector_3) : '—';
    
    const tyres = getTyreDisplay(lap);
    const drs = pos.drs_enabled ? `${colors.brightGreen}DRS${colors.reset}` : '—';
    const pitsStr = pitData ? String(pitData.count).padStart(2) : '0';
    
    return `│${posStr}│ ${codeSwatch} ${driverCode.padEnd(4)}│${lapStr.padStart(10)}│${gapStr.padStart(10)}│${s1.padStart(12)}│${s2.padStart(12)}│${s3.padStart(12)}│${tyres.padEnd(7)}│${drs.padEnd(5)}│${pitsStr.padStart(3)} │`;
  }).filter(Boolean);

  const tableFooter = `└────┴──────┴──────────┴──────────┴────────────┴────────────┴────────────┴───────┴─────┴─────┘`;

  const ticker = `┌─────────────────────────────────────────────────────────────────────────────────────┐
│ LIVE • F1 Telemetry Dashboard • Updated: ${new Date().toLocaleTimeString().padEnd(37)} │
└─────────────────────────────────────────────────────────────────────────────────────┘`;

  return [header, '', tableHeader, ...rows, tableFooter, '', ticker].join('\n');
}

/**
 * Render dashboard to console
 */
function renderDashboard(state) {
  try {
    console.clear();
    const dashboard = buildDashboard(state);
    console.log(dashboard);
  } catch (error) {
    console.error('Render error:', error);
  }
}

module.exports = {
  renderDashboard,
  buildDashboard,
  formatTime,
  formatGap,
  hexToAnsi256,
};
