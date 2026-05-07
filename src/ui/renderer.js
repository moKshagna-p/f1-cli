/**
 * UI Renderer
 * Terminal dashboard rendering with ANSI formatting
 */

const {
  formatTime,
  formatGap,
  formatPosition,
  formatTyre,
  formatDRS,
  formatPitCount,
  formatDriverCode,
  formatLapTime,
  formatSector,
  padColumn,
} = require('./formatters');

const { DISPLAY_LIMITS } = require('../utils/constants');

/**
 * Build header with session info and weather
 */
function buildHeader(state) {
  const session = state.session || {};
  const weather = state.weather && state.weather[state.weather.length - 1];

  const sessionType = session.session_type || 'Waiting';
  const trackName = session.location || 'N/A';

  const tempStr = weather ? `${Math.round(weather.air_temperature || 0)}°C` : '—°C';
  const rainStr = weather && weather.rainfall ? '🌧' : '☀';

  const header = `┌─────────────────────────────────────────────────────────────────────────────────────┐
│ ${sessionType.toUpperCase().padEnd(6)} | ${trackName.padEnd(50)} Temp: ${tempStr} ${rainStr}  │
└─────────────────────────────────────────────────────────────────────────────────────┘`;

  return header;
}

/**
 * Build driver standings table
 */
function buildStandings(state) {
  const drivers = state.drivers || new Map();
  const positions = state.positions || new Map();
  const laps = state.laps || new Map();
  const pits = state.pits || new Map();
  const positionChanges = state.positionChanges || new Map();
  const fastestLapDriver = state.fastestLapHolder;

  // Sort by position
  const sorted = Array.from(positions.values())
    .sort((a, b) => a.position - b.position)
    .slice(0, DISPLAY_LIMITS.MAX_DRIVERS);

  const tableHeader = `┌────┬──────┬──────────┬──────────┬────────────┬────────────┬────────────┬───────┬─────┬─────┐
│Pos │ Drv  │   Lap    │   Gap    │     S1     │     S2     │     S3     │ Tyres │ DRS │ Pit │
├────┼──────┼──────────┼──────────┼────────────┼────────────┼────────────┼───────┼─────┼─────┤`;

  const rows = sorted
    .map((pos) => {
      const driver = drivers.get(pos.driver_number);
      if (!driver) return null;

      const lap = laps.get(pos.driver_number);
      const pitData = pits.get(pos.driver_number);
      const posChange = positionChanges.get(pos.driver_number);

      const posStr = formatPosition(pos.position, posChange);
      const driverCode = driver.driver_abbr || `P${pos.position}`;
      const formattedCode = formatDriverCode(driverCode, driver.team_colour);

      const lapStr = formatLapTime(lap?.lap_duration, fastestLapDriver === pos.driver_number);
      const gapStr = formatGap(pos.gap_to_leader || 0);

      const s1 = lap?.sector_1 ? formatSector(lap.sector_1) : '           —';
      const s2 = lap?.sector_2 ? formatSector(lap.sector_2) : '           —';
      const s3 = lap?.sector_3 ? formatSector(lap.sector_3) : '           —';

      const tyres = formatTyre(lap);
      const drs = formatDRS(pos.drs_enabled);
      const pitsStr = formatPitCount(pitData);

      return `│${posStr}│${formattedCode}│${lapStr.padStart(10)}│${gapStr.padStart(10)}│${s1}│${s2}│${s3}│${tyres.padEnd(7)}│${drs.padEnd(5)}│${pitsStr.padStart(3)} │`;
    })
    .filter(Boolean);

  const tableFooter = `└────┴──────┴──────────┴──────────┴────────────┴────────────┴────────────┴───────┴─────┴─────┘`;

  return [tableHeader, ...rows, tableFooter].join('\n');
}

/**
 * Build status ticker at bottom
 */
function buildTicker(state) {
  const messages = [
    'LIVE • F1 Telemetry Dashboard',
    `Updated: ${new Date().toLocaleTimeString()}`,
    'Polling every 2s',
  ];

  if (state.pollingError) {
    messages.push(`⚠ ${state.pollingError}`);
  }

  const ticker = messages.join(' • ');

  return `┌─────────────────────────────────────────────────────────────────────────────────────┐
│ ${ticker.padEnd(85)} │
└─────────────────────────────────────────────────────────────────────────────────────┘`;
}

/**
 * Build complete dashboard
 */
function buildDashboard(state) {
  const header = buildHeader(state);
  const standings = buildStandings(state);
  const ticker = buildTicker(state);

  return [header, '', standings, '', ticker].join('\n');
}

/**
 * Render dashboard to terminal
 */
function render(state) {
  try {
    console.clear();
    const dashboard = buildDashboard(state);
    console.log(dashboard);
  } catch (error) {
    console.error('❌ Render error:', error.message);
  }
}

module.exports = {
  render,
  buildDashboard,
  buildHeader,
  buildStandings,
  buildTicker,
};
