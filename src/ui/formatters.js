/**
 * UI Formatters
 * Functions to format telemetry data for display
 */

const { ANSI_COLORS, colorizeText, getTyreColor } = require('./colors');

/**
 * Format time duration (milliseconds to MM:SS.sss)
 */
function formatTime(ms) {
  if (!ms || ms === 0) return '—';
  const totalSeconds = ms / 1000;
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes}:${seconds.toFixed(3).padStart(7, '0')}`;
}

/**
 * Format gap to leader (seconds)
 */
function formatGap(gap) {
  if (gap === null || gap === undefined || gap === 0) return 'LEADER';
  if (gap < 0.001) return '+0.001';
  return `+${gap.toFixed(3)}`;
}

/**
 * Format position with change indicator
 */
function formatPosition(position, positionChange) {
  const posInd = getPositionIndicator(positionChange);
  return `${posInd}${String(position).padStart(2, ' ')}`;
}

/**
 * Get position change indicator (▲ up, ▼ down)
 */
function getPositionIndicator(positionChange) {
  if (!positionChange) return '  ';
  const direction = positionChange.to < positionChange.from ? '▲' : '▼';
  return `${direction} `;
}

/**
 * Format tyre display (compound + laps on tyre)
 */
function formatTyre(lap) {
  if (!lap || !lap.tyre_compound) return '-';

  const compound = lap.tyre_compound.slice(0, 1).toUpperCase();
  const lapsOnTyre = lap.lap_number ? lap.lap_number : 1;
  const color = getTyreColor(lap.tyre_compound);

  return `${color}${compound}${ANSI_COLORS.reset}${lapsOnTyre}`;
}

/**
 * Format DRS status
 */
function formatDRS(drsEnabled) {
  if (!drsEnabled) return '—';
  return colorizeText('DRS', 'brightGreen');
}

/**
 * Format pit stop count
 */
function formatPitCount(pitData) {
  if (!pitData) return '0';
  return String(pitData.count).padStart(2);
}

/**
 * Format driver code with team color
 */
function formatDriverCode(driverCode, teamColor) {
  const { getTeamColorSwatch } = require('./colors');
  const swatch = getTeamColorSwatch(teamColor);
  return `${swatch} ${driverCode.padEnd(4)}`;
}

/**
 * Format lap time (purple if fastest lap)
 */
function formatLapTime(time, isFastestLap) {
  const formatted = formatTime(time);
  if (isFastestLap) {
    return colorizeText(formatted, 'magenta');
  }
  return formatted;
}

/**
 * Format sector time
 */
function formatSector(time) {
  return formatTime(time).padStart(12);
}

/**
 * Pad string for table alignment
 */
function padColumn(text, width, align = 'left') {
  const str = String(text);
  if (align === 'right') {
    return str.padStart(width);
  }
  return str.padEnd(width);
}

module.exports = {
  formatTime,
  formatGap,
  formatPosition,
  getPositionIndicator,
  formatTyre,
  formatDRS,
  formatPitCount,
  formatDriverCode,
  formatLapTime,
  formatSector,
  padColumn,
};
