/*!
 * Logging module for the Reputator project.
 * 
 * This module provides a simple interface for logging using Juno's native logging system.
 * All logs are persisted on the blockchain and visible in the Juno Console.
 * 
 * Note: Juno's native logging does not support log levels, so all logs appear as "Error" in the Juno Console.
 * This is a limitation of the native logging system.
 */

use ic_cdk;

/// Log a message with a prefix.
/// 
/// # Arguments
/// * `prefix` - The prefix to use for the log message (e.g., "ERROR", "WARN", "INFO")
/// * `message` - The message to log
/// 
/// # Example
/// ```
/// log_with_prefix("ERROR", "Failed to process request");
/// ```
pub fn log_with_prefix(prefix: &str, message: &str) {
    ic_cdk::print(format!("[{}] {}", prefix, message));
}

/// Log an error message and trap.
/// 
/// # Arguments
/// * `message` - The error message to log
/// 
/// # Example
/// ```
/// log_error("Invalid input data");
/// ```
pub fn log_error(message: &str) {
    ic_cdk::trap(message);
}

/// Log a warning message.
/// 
/// # Arguments
/// * `message` - The warning message to log
/// 
/// # Example
/// ```
/// log_warn("Resource usage is high");
/// ```
pub fn log_warn(message: &str) {
    log_with_prefix("WARN", message);
}

/// Log an info message.
/// 
/// # Arguments
/// * `message` - The info message to log
/// 
/// # Example
/// ```
/// log_info("Operation completed successfully");
/// ```
pub fn log_info(message: &str) {
    log_with_prefix("INFO", message);
}

/// Log a debug message.
/// 
/// # Arguments
/// * `message` - The debug message to log
/// 
/// # Example
/// ```
/// log_debug("Processing step 1 of 3");
/// ```
pub fn log_debug(message: &str) {
    log_with_prefix("DEBUG", message);
}

/// Macro for logging user-related messages.
/// 
/// # Arguments
/// * `message` - The message to log
/// 
/// # Example
/// ```
/// log_user!("User profile updated");
/// ```
#[macro_export]
macro_rules! log_user {
    ($message:expr) => {
        log_with_prefix("USER", $message);
    };
}

