#!/usr/bin/env node

/**
 * F1 Live Telemetry Dashboard
 * Entry point for the application
 */

const F1Dashboard = require('./src/index');

const dashboard = new F1Dashboard();

/**
 * Signal handlers for graceful shutdown
 */
process.on('SIGINT', () => {
  dashboard.shutdown();
  process.exit(0);
});

process.on('SIGTERM', () => {
  dashboard.shutdown();
  process.exit(0);
});

process.on('uncaughtException', (error) => {
  console.error('\n❌ Uncaught error:', error);
  dashboard.shutdown();
  process.exit(1);
});

process.on('unhandledRejection', (reason) => {
  console.error('\n❌ Unhandled rejection:', reason);
  dashboard.shutdown();
  process.exit(1);
});

/**
 * Start the dashboard
 */
(async () => {
  try {
    await dashboard.initialize();
  } catch (error) {
    console.error('Failed to start dashboard:', error.message);
    process.exit(1);
  }
})();
