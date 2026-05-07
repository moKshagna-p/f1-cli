/**
 * State Manager
 * Manages application state with efficient diffing
 */

const Logger = require('../utils/logger');
const { POSITION_ANIMATION_DURATION } = require('../utils/constants');

const logger = new Logger('StateManager');

const state = {
  session: null,
  drivers: new Map(),
  positions: new Map(),
  laps: new Map(),
  pits: new Map(),
  weather: [],

  // Animation tracking
  positionChanges: new Map(), // driver -> { from, to, timestamp }
  fastestLapHolder: null,
  previousFastestLapHolder: null,

  // Metadata
  isLive: false,
  lastUpdate: null,
  pollingError: null,
};

/**
 * Deep equality check
 */
function deepEqual(obj1, obj2) {
  if (obj1 === obj2) return true;
  if (obj1 == null || obj2 == null) return false;
  if (typeof obj1 !== 'object' || typeof obj2 !== 'object') return false;

  const keys1 = Object.keys(obj1);
  const keys2 = Object.keys(obj2);

  if (keys1.length !== keys2.length) return false;

  for (const key of keys1) {
    if (!keys2.includes(key)) return false;
    if (!deepEqual(obj1[key], obj2[key])) return false;
  }

  return true;
}

/**
 * Update state with new telemetry and track changes
 */
function updateState(newTelemetry) {
  const changes = {
    driverChanges: [],
    positionChanges: [],
    lapChanges: [],
    weatherChange: false,
  };

  // Update session
  if (newTelemetry.session && !deepEqual(state.session, newTelemetry.session)) {
    state.session = newTelemetry.session;
    state.lastUpdate = Date.now();
    logger.debug('Session updated');
  }

  // Update drivers
  if (newTelemetry.drivers && Array.isArray(newTelemetry.drivers)) {
    const newDriverMap = new Map(newTelemetry.drivers.map((d) => [d.driver_number, d]));

    newDriverMap.forEach((driver, number) => {
      const oldDriver = state.drivers.get(number);
      if (!deepEqual(driver, oldDriver)) {
        state.drivers.set(number, driver);
        changes.driverChanges.push(number);
      }
    });
  }

  // Update positions and detect changes
  if (newTelemetry.positions && Array.isArray(newTelemetry.positions)) {
    const byDriver = new Map(newTelemetry.positions.map((p) => [p.driver_number, p]));

    byDriver.forEach((pos, driverNum) => {
      const oldPos = state.positions.get(driverNum);
      if (oldPos && oldPos.position !== pos.position) {
        state.positionChanges.set(driverNum, {
          from: oldPos.position,
          to: pos.position,
          timestamp: Date.now(),
        });
        changes.positionChanges.push(driverNum);
        logger.debug(`Position change: Driver ${driverNum} ${oldPos.position} → ${pos.position}`);
      }
      state.positions.set(driverNum, pos);
    });
  }

  // Update laps and find fastest lap holder
  if (newTelemetry.laps && Array.isArray(newTelemetry.laps)) {
    const lapsByDriver = new Map();

    newTelemetry.laps.forEach((lap) => {
      if (!lapsByDriver.has(lap.driver_number)) {
        lapsByDriver.set(lap.driver_number, lap);
      } else {
        const existing = lapsByDriver.get(lap.driver_number);
        if (lap.lap_duration && (!existing.lap_duration || lap.lap_duration < existing.lap_duration)) {
          lapsByDriver.set(lap.driver_number, lap);
        }
      }
    });

    lapsByDriver.forEach((lap, driverNum) => {
      const oldLap = state.laps.get(driverNum);
      if (!deepEqual(lap, oldLap)) {
        state.laps.set(driverNum, lap);
        changes.lapChanges.push(driverNum);
      }
    });

    // Find fastest lap holder
    let fastestLap = null;
    let fastestDriver = null;

    state.laps.forEach((lap, driverNum) => {
      if (lap.lap_duration && (!fastestLap || lap.lap_duration < fastestLap)) {
        fastestLap = lap.lap_duration;
        fastestDriver = driverNum;
      }
    });

    state.previousFastestLapHolder = state.fastestLapHolder;
    state.fastestLapHolder = fastestDriver;
  }

  // Update pits
  if (newTelemetry.pits && Array.isArray(newTelemetry.pits)) {
    const pitsByDriver = new Map();
    newTelemetry.pits.forEach((pit) => {
      if (!pitsByDriver.has(pit.driver_number)) {
        pitsByDriver.set(pit.driver_number, 0);
      }
      pitsByDriver.set(pit.driver_number, pitsByDriver.get(pit.driver_number) + 1);
    });

    pitsByDriver.forEach((count, driverNum) => {
      const oldCount = (state.pits.get(driverNum) || {}).count || 0;
      if (count !== oldCount) {
        state.pits.set(driverNum, { count, timestamp: Date.now() });
      }
    });
  }

  // Update weather
  if (newTelemetry.weather && Array.isArray(newTelemetry.weather)) {
    const latest = newTelemetry.weather[newTelemetry.weather.length - 1];
    if (!deepEqual(latest, state.weather[state.weather.length - 1])) {
      state.weather = newTelemetry.weather;
      changes.weatherChange = true;
    }
  }

  // Cleanup old position changes (animations)
  const now = Date.now();
  state.positionChanges.forEach((change, driver) => {
    if (now - change.timestamp > POSITION_ANIMATION_DURATION) {
      state.positionChanges.delete(driver);
    }
  });

  state.isLive = !!state.session;
  state.lastUpdate = Date.now();

  return changes;
}

/**
 * Get state snapshot
 */
function getState() {
  return {
    session: state.session,
    drivers: new Map(state.drivers),
    positions: new Map(state.positions),
    laps: new Map(state.laps),
    pits: new Map(state.pits),
    weather: [...state.weather],
    positionChanges: new Map(state.positionChanges),
    fastestLapHolder: state.fastestLapHolder,
    isLive: state.isLive,
    lastUpdate: state.lastUpdate,
    pollingError: state.pollingError,
  };
}

/**
 * Reset state (for new session)
 */
function resetState() {
  state.session = null;
  state.drivers.clear();
  state.positions.clear();
  state.laps.clear();
  state.pits.clear();
  state.weather = [];
  state.positionChanges.clear();
  state.fastestLapHolder = null;
  state.previousFastestLapHolder = null;
  state.lastUpdate = null;
  logger.info('State reset');
}

/**
 * Set polling error
 */
function setPollingError(error) {
  state.pollingError = error;
}

/**
 * Clear polling error
 */
function clearPollingError() {
  state.pollingError = null;
}

module.exports = {
  updateState,
  getState,
  resetState,
  setPollingError,
  clearPollingError,
};
