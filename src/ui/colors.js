/**
 * Color Utilities
 * ANSI color codes and color conversion utilities
 */

const ANSI_COLORS = {
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
 * Approximates RGB values into a 6x6x6 color cube (216 colors)
 */
function hexToAnsi256(hex) {
  if (!hex || hex === '#FFFFFF') return 15; // white

  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);

  // Convert to 6x6x6 cube (0-5 range)
  const ri = Math.round(r / 255 * 5);
  const gi = Math.round(g / 255 * 5);
  const bi = Math.round(b / 255 * 5);

  // Formula: 16 + 36*r + 6*g + b
  return 16 + 36 * ri + 6 * gi + bi;
}

/**
 * Apply ANSI 256-color to text
 */
function colorize256(text, colorCode) {
  return `\x1b[38;5;${colorCode}m${text}${ANSI_COLORS.reset}`;
}

/**
 * Apply standard ANSI color to text
 */
function colorizeText(text, colorName) {
  const color = ANSI_COLORS[colorName];
  if (!color) return text;
  return `${color}${text}${ANSI_COLORS.reset}`;
}

/**
 * Get team color swatch
 */
function getTeamColorSwatch(teamColor) {
  const colorCode = hexToAnsi256(teamColor || '#FFFFFF');
  return colorize256('●', colorCode);
}

/**
 * Get tyre compound color
 */
function getTyreColor(compound) {
  switch (compound?.toUpperCase()) {
    case 'SOFT':
      return ANSI_COLORS.brightRed;
    case 'MEDIUM':
      return ANSI_COLORS.yellow;
    case 'HARD':
      return ANSI_COLORS.white;
    case 'INTERMEDIATE':
      return ANSI_COLORS.green;
    case 'WET':
      return ANSI_COLORS.cyan;
    default:
      return ANSI_COLORS.white;
  }
}

module.exports = {
  ANSI_COLORS,
  hexToAnsi256,
  colorize256,
  colorizeText,
  getTeamColorSwatch,
  getTyreColor,
};
