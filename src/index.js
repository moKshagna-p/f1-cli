/**
 * F1 Telemetry Dashboard - Main Application
 * Orchestrates API polling, state management, and UI rendering
 */

const api = require('./api/openf1Client');
const stateManager = require('./state/stateManager');
const ui = require('./ui/renderer');
const Logger = require('./utils/logger');
const { POLLING_INTERVAL, MAX_RETRIES } = require('./utils/constants');

const logger = new Logger('Dashboard');

class F1Dashboard {
  constructor() {
    this.pollCount = 0;
    this.retryCount = 0;
    this.currentSessionKey = null;
    this.pollTimeout = null;
    this.isRunning = false;
  }

  /**
   * Initialize the dashboard
   */
  async initialize() {
    logger.info('🏁 F1 Telemetry Dashboard - Initializing...');

    try {
      // Fetch initial session
      const session = await api.fetchSessions();
      if (!session) {
        logger.error('❌ Could not fetch session data');
        throw new Error('No session found');
      }

      logger.info(`✓ Found session: ${session.session_type} at ${session.location}`);
      this.currentSessionKey = session.session_key;

      // Load driver data
      await api.fetchDrivers();
      logger.info('✓ Loaded driver data');

      logger.info('📡 Starting live polling...\n');
      this.isRunning = true;
      await this.poll();
    } catch (error) {
      logger.error('Initialization failed', error);
      throw error;
    }
  }

  /**
   * Main polling loop
   */
  async poll() {
    try {
      // Fetch telemetry
      const telemetry = await api.fetchAllTelemetry(this.currentSessionKey);

      // Update state
      const changes = stateManager.updateState({
        session: telemetry.session,
        drivers: telemetry.drivers,
        positions: telemetry.positions,
        laps: telemetry.laps,
        pits: telemetry.pits,
        weather: telemetry.weather,
      });

      // Get state and render
      const currentState = stateManager.getState();
      ui.render(currentState);

      // Reset retry on success
      if (this.retryCount > 0) {
        this.retryCount = 0;
        stateManager.clearPollingError();
      }

      this.pollCount++;
      logger.debug(`Poll #${this.pollCount} completed`);
    } catch (error) {
      this.retryCount++;
      const errorMsg = `Polling error (${this.retryCount}/${MAX_RETRIES}): ${error.message}`;
      stateManager.setPollingError(errorMsg);
      logger.warn(errorMsg);

      if (this.retryCount >= MAX_RETRIES) {
        logger.error(`❌ ${errorMsg}`);
        this.shutdown();
        process.exit(1);
      }
    }

    // Schedule next poll
    if (this.isRunning) {
      this.pollTimeout = setTimeout(() => this.poll(), POLLING_INTERVAL);
    }
  }

  /**
   * Shutdown gracefully
   */
  shutdown() {
    logger.info('\n👋 Shutting down dashboard...');
    this.isRunning = false;
    if (this.pollTimeout) clearTimeout(this.pollTimeout);
  }
}

module.exports = F1Dashboard;
