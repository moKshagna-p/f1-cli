const axios = require('axios');

const BASE_URL = 'https://openf1.org/v1';
const POLL_INTERVAL = 2000; // 2 seconds

// Cache for expensive operations
const cache = {
  sessions: null,
  drivers: null,
  driverMap: null,
  lastFetchTime: {},
};

/**
 * Fetch a session (preferring live/active sessions)
 */
async function fetchSessions() {
  try {
    const response = await axios.get(`${BASE_URL}/sessions?limit=50`);
    const sessions = response.data || [];
    
    // Sort by date descending and find the latest active session
    const sorted = sessions.sort((a, b) => {
      const dateA = new Date(a.date_start || 0).getTime();
      const dateB = new Date(b.date_start || 0).getTime();
      return dateB - dateA;
    });

    const liveSession = sorted.find(s => 
      s.session_type && 
      ['Practice', 'Qualifying', 'Race', 'Sprint', 'Sprint Shootout'].includes(s.session_type)
    );

    cache.sessions = sorted;
    cache.lastFetchTime.sessions = Date.now();
    return liveSession || sorted[0];
  } catch (error) {
    console.error('Error fetching sessions:', error.message);
    return null;
  }
}

/**
 * Fetch all drivers with metadata
 */
async function fetchDrivers() {
  try {
    const response = await axios.get(`${BASE_URL}/drivers`);
    const drivers = response.data || [];
    
    // Build a map for quick lookup
    const driverMap = {};
    drivers.forEach(driver => {
      driverMap[driver.driver_number] = driver;
    });

    cache.drivers = drivers;
    cache.driverMap = driverMap;
    cache.lastFetchTime.drivers = Date.now();
    return drivers;
  } catch (error) {
    console.error('Error fetching drivers:', error.message);
    return cache.drivers || [];
  }
}

/**
 * Fetch current driver positions
 */
async function fetchPositions(sessionKey) {
  try {
    const url = sessionKey 
      ? `${BASE_URL}/position?session_key=${sessionKey}`
      : `${BASE_URL}/position`;
    
    const response = await axios.get(url);
    cache.lastFetchTime.positions = Date.now();
    return response.data || [];
  } catch (error) {
    console.error('Error fetching positions:', error.message);
    return [];
  }
}

/**
 * Fetch latest lap times per driver
 */
async function fetchLaps(sessionKey) {
  try {
    const url = sessionKey
      ? `${BASE_URL}/laps?session_key=${sessionKey}&limit=1000`
      : `${BASE_URL}/laps?limit=1000`;
    
    const response = await axios.get(url);
    cache.lastFetchTime.laps = Date.now();
    return response.data || [];
  } catch (error) {
    console.error('Error fetching laps:', error.message);
    return [];
  }
}

/**
 * Fetch pit stop data
 */
async function fetchPitStops(sessionKey) {
  try {
    const url = sessionKey
      ? `${BASE_URL}/pit?session_key=${sessionKey}`
      : `${BASE_URL}/pit`;
    
    const response = await axios.get(url);
    cache.lastFetchTime.pits = Date.now();
    return response.data || [];
  } catch (error) {
    console.error('Error fetching pit data:', error.message);
    return [];
  }
}

/**
 * Fetch weather data
 */
async function fetchWeather(sessionKey) {
  try {
    const url = sessionKey
      ? `${BASE_URL}/weather?session_key=${sessionKey}`
      : `${BASE_URL}/weather`;
    
    const response = await axios.get(url);
    cache.lastFetchTime.weather = Date.now();
    return response.data || [];
  } catch (error) {
    console.error('Error fetching weather:', error.message);
    return [];
  }
}

/**
 * Fetch car telemetry data (speed, throttle, brake)
 */
async function fetchCarData(sessionKey, driverNumber, limit = 100) {
  try {
    let url = `${BASE_URL}/car_data?limit=${limit}`;
    if (sessionKey) url += `&session_key=${sessionKey}`;
    if (driverNumber) url += `&driver_number=${driverNumber}`;
    
    const response = await axios.get(url);
    cache.lastFetchTime.carData = Date.now();
    return response.data || [];
  } catch (error) {
    console.error('Error fetching car data:', error.message);
    return [];
  }
}

/**
 * Aggregate all telemetry data for a session
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
    console.error('Error aggregating telemetry:', error.message);
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
 * Get driver info by number
 */
function getDriver(driverNumber) {
  if (!cache.driverMap) return null;
  return cache.driverMap[driverNumber];
}

/**
 * Get color for a team
 */
function getTeamColor(driverNumber) {
  const driver = getDriver(driverNumber);
  if (!driver) return '#FFFFFF';
  return driver.team_colour || '#FFFFFF';
}

module.exports = {
  fetchSessions,
  fetchDrivers,
  fetchPositions,
  fetchLaps,
  fetchPitStops,
  fetchWeather,
  fetchCarData,
  fetchAllTelemetry,
  getDriver,
  getTeamColor,
  POLL_INTERVAL,
};
