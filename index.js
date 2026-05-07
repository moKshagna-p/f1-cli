#!/usr/bin/env node

/**
 * F1 Live Telemetry Dashboard
 * Terminal-based real-time F1 data visualization
 */

const api = require('./api');
const state = require('./state');
const ui = require('./ui');

const POLL_INTERVAL = 2000; // 2 seconds
const MAX_RETRIES = 5;

let pollCount = 0;
let retryCount = 0;
let currentSessionKey = null;
let pollTimeout = null;

/**
 * Initialize the dashboard
 */
async function initialize() {
  console.log('🏁 F1 Telemetry Dashboard - Initializing...\n');
  
  try {
    // Fetch initial session data
    const session = await api.fetchSessions();
    if (!session) {
      console.error('❌ Could not fetch session data. Please check your internet connection.');
      process.exit(1);
    }

    console.log(`✓ Found session: ${session.session_type} at ${session.location}`);
    console.log(`✓ Session Key: ${session.session_key}`);
    
    currentSessionKey = session.session_key;
    
    // Fetch driver data for team colors
    await api.fetchDrivers();
    console.log('✓ Loaded driver data');
    
    // Initial render
    console.log('\n📡 Starting live polling...\n');
    await pollTelemetry();
    
  } catch (error) {
    console.error('Initialization error:', error.message);
    process.exit(1);
  }
}

/**
 * Main polling loop
 */
async function pollTelemetry() {
  try {
    // Fetch all telemetry data
    const telemetry = await api.fetchAllTelemetry(currentSessionKey);
    
    // Update state and get changes
    const changes = state.updateState({
      session: telemetry.session,
      drivers: telemetry.drivers,
      positions: telemetry.positions,
      laps: telemetry.laps,
      pits: telemetry.pits,
      weather: telemetry.weather,
    });

    // Get current state for rendering
    const currentState = state.getState();
    
    // Render UI
    ui.renderDashboard(currentState);
    
    // Reset retry count on success
    if (retryCount > 0) {
      retryCount = 0;
      state.clearPollingError();
    }
    
    pollCount++;

  } catch (error) {
    retryCount++;
    const errorMsg = `Polling error (${retryCount}/${MAX_RETRIES}): ${error.message}`;
    state.setPollingError(errorMsg);
    
    if (retryCount >= MAX_RETRIES) {
      console.error(`\n❌ ${errorMsg}`);
      process.exit(1);
    }
  }
  
  // Schedule next poll
  pollTimeout = setTimeout(pollTelemetry, POLL_INTERVAL);
}

/**
 * Handle graceful shutdown
 */
function handleShutdown() {
  console.log('\n\n👋 Shutting down dashboard...');
  if (pollTimeout) clearTimeout(pollTimeout);
  process.exit(0);
}

/**
 * Handle uncaught errors
 */
function handleUncaughtError(error) {
  console.error('\n❌ Uncaught error:', error);
  handleShutdown();
}

// Setup signal handlers
process.on('SIGINT', handleShutdown);
process.on('SIGTERM', handleShutdown);
process.on('uncaughtException', handleUncaughtError);
process.on('unhandledRejection', handleUncaughtError);

// Start the dashboard
initialize().catch(error => {
  console.error('Failed to initialize:', error);
  process.exit(1);
});
