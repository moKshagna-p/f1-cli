/**
 * Constants and Configuration
 * Centralized configuration for the entire application
 */

const POLLING_INTERVAL = 2000; // milliseconds
const MAX_RETRIES = 5;
const RETRY_DELAY = 1000; // milliseconds
const POSITION_ANIMATION_DURATION = 3000; // milliseconds

const API_BASE_URL = 'https://api.openf1.org/v1';

const ENDPOINTS = {
  SESSIONS: '/sessions',
  DRIVERS: '/drivers',
  POSITIONS: '/position',
  LAPS: '/laps',
  PITS: '/pit',
  WEATHER: '/weather',
  CAR_DATA: '/car_data',
};

const SESSION_TYPES = ['Practice', 'Qualifying', 'Race', 'Sprint', 'Sprint Shootout'];

const TYRE_COMPOUNDS = {
  SOFT: 'SOFT',
  MEDIUM: 'MEDIUM',
  HARD: 'HARD',
  INTERMEDIATE: 'INTERMEDIATE',
  WET: 'WET',
};

const DISPLAY_LIMITS = {
  MAX_DRIVERS: 30,
  MIN_TERMINAL_WIDTH: 100,
  MIN_TERMINAL_HEIGHT: 40,
};

module.exports = {
  POLLING_INTERVAL,
  MAX_RETRIES,
  RETRY_DELAY,
  POSITION_ANIMATION_DURATION,
  API_BASE_URL,
  ENDPOINTS,
  SESSION_TYPES,
  TYRE_COMPOUNDS,
  DISPLAY_LIMITS,
};
