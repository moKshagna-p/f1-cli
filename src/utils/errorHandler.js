/**
 * Error Handler Utility
 * Centralized error handling and recovery
 */

class ErrorHandler {
  /**
   * Handle API errors with retry logic
   */
  static handleApiError(error, endpoint, attempt, maxRetries) {
    const message = error.response?.statusText || error.message || 'Unknown error';
    const status = error.response?.status || 'N/A';

    return {
      success: false,
      status,
      message,
      endpoint,
      attempt,
      maxRetries,
      shouldRetry: attempt < maxRetries && status !== 401 && status !== 403 && status !== 404,
      timestamp: Date.now(),
    };
  }

  /**
   * Determine if error is retryable
   */
  static isRetryable(error) {
    if (!error.response) return true; // Network errors are retryable

    const status = error.response.status;
    // Don't retry 4xx errors except 408 (timeout) and 429 (rate limit)
    if (status >= 400 && status < 500) {
      return status === 408 || status === 429;
    }

    // Retry 5xx errors
    return status >= 500;
  }

  /**
   * Calculate exponential backoff delay
   */
  static getBackoffDelay(attempt, baseDelay = 1000) {
    return baseDelay * Math.pow(2, attempt - 1);
  }

  /**
   * Format error for user display
   */
  static formatErrorMessage(error) {
    if (error.response?.status === 429) {
      return 'Rate limited by API. Waiting...';
    }

    if (error.code === 'ECONNREFUSED') {
      return 'Cannot connect to API. Check internet connection.';
    }

    if (error.code === 'ETIMEDOUT') {
      return 'API request timed out.';
    }

    return `Error: ${error.message}`;
  }
}

module.exports = ErrorHandler;
