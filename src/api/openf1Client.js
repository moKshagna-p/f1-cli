/**
 * OpenF1 API Client
 * Handles all communication with the OpenF1 REST API
 */

const axios = require('axios');
const Logger = require('../utils/logger');
const ErrorHandler = require('../utils/errorHandler');
const { API_BASE_URL, ENDPOINTS } = require('../utils/constants');

const logger = new Logger('OpenF1-API');

// Cache for expensive operations
const cache = {
  drivers: null,
  driverMap: null,
  lastFetchTime: {},
};

/**
 * Fetch sessions (preferring live/active sessions)
 */
async function fetchSessions() {
  try {
    logger.debug('Fetching sessions...');
    const url = `${API_BASE_URL}${ENDPOINTS.SESSIONS}?limit=50`;
    const response = await axios.get(url, { timeout: 5000 });
    const sessions = response.data || [];

    // Sort by date descending
    const sorted = sessions.sort((a, b) => {
      const dateA = new Date(a.date_start || 0).getTime();
      const dateB = new Date(b.date_start || 0).getTime();
      return dateB - dateA;
    });

    cache.lastFetchTime.sessions = Date.now();
    logger.debug(`Found ${sorted.length} sessions`);
    return sorted[0] || null;
  } catch (error) {
    logger.error('Failed to fetch sessions', error);
    throw error;
  }
}

/**
 * Fetch all drivers with metadata
 */
async function fetchDrivers() {
  try {
    logger.debug('Fetching drivers...');
    const url = `${API_BASE_URL}${ENDPOINTS.DRIVERS}`;
    const response = await axios.get(url, { timeout: 5000 });
    const drivers = response.data || [];

    // Build map for quick lookup
    const driverMap = {};
    drivers.forEach((driver) => {
      driverMap[driver.driver_number] = driver;
    });

    cache.drivers = drivers;
    cache.driverMap = driverMap;
    cache.lastFetchTime.drivers = Date.now();
    logger.debug(`Cached ${drivers.length} drivers`);
    return drivers;
  } catch (error) {
    logger.error('Failed to fetch drivers', error);
    // Return cached drivers if available
    return cache.drivers || [];
  }
}

/**
 * Fetch current driver positions
 */
async function fetchPositions(sessionKey) {
  try {
    let url = `${API_BASE_URL}${ENDPOINTS.POSITIONS}`;
    if (sessionKey) url += `?session_key=${sessionKey}`;

    const response = await axios.get(url, { timeout: 5000 });
    return response.data || [];
  } catch (error) {
    logger.error('Failed to fetch positions', error);
    return [];
  }
}

/**
 * Fetch latest lap times per driver
 */
async function fetchLaps(sessionKey) {
  try {
    let url = `${API_BASE_URL}${ENDPOINTS.LAPS}?limit=1000`;
    if (sessionKey) url += `&session_key=${sessionKey}`;

    const response = await axios.get(url, { timeout: 5000 });
    return response.data || [];
  } catch (error) {
    logger.error('Failed to fetch laps', error);
    return [];
  }
}

/**
 * Fetch pit stop data
 */
async function fetchPitStops(sessionKey) {
  try {
    let url = `${API_BASE_URL}${ENDPOINTS.PITS}`;
    if (sessionKey) url += `?session_key=${sessionKey}`;

    const response = await axios.get(url, { timeout: 5000 });
    return response.data || [];
  } catch (error) {
    logger.error('Failed to fetch pit stops', error);
    return [];
  }
}

/**
 * Fetch weather data
 */
async function fetchWeather(sessionKey) {
  try {
    let url = `${API_BASE_URL}${ENDPOINTS.WEATHER}`;
    if (sessionKey) url += `?session_key=${sessionKey}`;

    const response = await axios.get(url, { timeout: 5000 });
    return response.data || [];
  } catch (error) {
    logger.error('Failed to fetch weather', error);
    return [];
  }
}

/**
 * Aggregate all telemetry data in parallel
 */
async function fetchAllTelemetry(sessionKey) {
  try {
    const [drivers, positions, laps, pits, weather] = await Promise.all([
      fetchDrivers(),
      fetchPositions(sessionKey),
      fetchLaps(sessionKey),
      fetchPitStops(sessionKey),
      fetchWeather(sessionKey),
    ]);

    return {
      drivers,
      positions,
      laps,
      pits,
      weather,
      timestamp: Date.now(),
    };
  } catch (error) {
    logger.error('Failed to fetch telemetry', error);
    return {
      drivers: [],
      positions: [],
      laps: [],
      pits: [],
      weather: [],
      timestamp: Date.now(),
    };
  }
}

/**
 * Get driver by number
 */
function getDriver(driverNumber) {
  if (!cache.driverMap) return null;
  return cache.driverMap[driverNumber];
}

module.exports = {
  fetchSessions,
  fetchDrivers,
  fetchPositions,
  fetchLaps,
  fetchPitStops,
  fetchWeather,
  fetchAllTelemetry,
  getDriver,
};
