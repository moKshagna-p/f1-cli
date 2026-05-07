/**
 * Logger Utility
 * Centralized logging with timestamp and log levels
 */

const LogLevel = {
  DEBUG: 'DEBUG',
  INFO: 'INFO',
  WARN: 'WARN',
  ERROR: 'ERROR',
};

class Logger {
  constructor(namespace = 'F1-Dashboard') {
    this.namespace = namespace;
    this.enableDebug = process.env.DEBUG === 'true';
  }

  /**
   * Format log message with timestamp and namespace
   */
  _format(level, message) {
    const timestamp = new Date().toISOString();
    return `[${timestamp}] [${this.namespace}] [${level}] ${message}`;
  }

  /**
   * Debug log
   */
  debug(message, data = null) {
    if (!this.enableDebug) return;
    const formatted = this._format(LogLevel.DEBUG, message);
    if (data) console.debug(formatted, data);
    else console.debug(formatted);
  }

  /**
   * Info log
   */
  info(message, data = null) {
    const formatted = this._format(LogLevel.INFO, message);
    if (data) console.log(formatted, data);
    else console.log(formatted);
  }

  /**
   * Warning log
   */
  warn(message, data = null) {
    const formatted = this._format(LogLevel.WARN, message);
    if (data) console.warn(formatted, data);
    else console.warn(formatted);
  }

  /**
   * Error log
   */
  error(message, error = null) {
    const formatted = this._format(LogLevel.ERROR, message);
    if (error) console.error(formatted, error.message || error);
    else console.error(formatted);
  }
}

module.exports = Logger;
