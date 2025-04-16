/*!
 * Logger module for the Reputator project.
 * 
 * This module provides a unified logging interface that:
 * 1. Automatically captures file and line information
 * 2. Supports multiple log levels (Log, Debug, Info, Warn, Error)
 * 3. Handles both simple messages and structured data
 * 4. Integrates with Juno's stable storage logging system
 * 
 * The logger is implemented as a macro (logger!) rather than a function because:
 * 1. Macros can capture file and line information at compile time (file!(), line!())
 * 2. Macros can handle variable arguments like println! (fmt!() style)
 * 3. Macros are expanded at compile time, reducing runtime overhead
 * 4. Macros can create custom syntax that looks like built-in Rust features
 * 
 * Usage examples:
 * ```rust
 * // RECOMMENDED: Always include the function name in the message
 * fn process_user_data(user_id: &str) {
 *     // Simple message with function context
 *     logger!("info", "[process_user_data] Processing started");
 * 
 *     // With variables - keep the function context
 *     logger!("error", "[process_user_data] Failed to process user: {}", user_id);
 * 
 *     // With structured data - maintain consistent naming
 *     logger!("debug", "[process_user_data] User state", &user_data);
 * }
 * ```
 * 
 * Storage details:
 * - Primary storage: Logs are stored in Juno's stable memory
 * - Maximum of 100 entries retained in Juno's system
 * - Oldest entries are discarded when limit is reached
 * - Logs are only saved if the containing function succeeds
 * 
 * TODO: Implement custom log storage in Juno datastore
 * - Create 'logs' collection with proper schema (see @database.md)
 * - Use set_doc_store to persist logs beyond Juno's 100 entry limit
 * - Add timestamp-based cleanup for log rotation
 * - Add query interface for log analysis
 */

use junobuild_satellite::{
    log, log_with_data,
    debug, debug_with_data,
    info, info_with_data,
    warn, warn_with_data,
    error, error_with_data
};
use serde::Serialize;
use std::str::FromStr;

/// Defines the available log levels in order of increasing severity.
/// 
/// This enum serves multiple purposes:
/// 1. Categorization: Groups logs by severity/purpose
/// 2. Routing: Determines which Juno logging function to call
/// 3. Future Storage: Will be used to filter/query logs when we implement
///    custom log storage in the Juno datastore (see @database.md)
/// 
/// The levels are designed to match common logging patterns:
/// - Log: Basic logging, lowest priority
/// - Debug: Development-time information
/// - Info: Normal operation events
/// - Warn: Concerning but non-fatal issues
/// - Error: Critical issues requiring immediate attention (data loss, security)
/// 
/// TODO: When implementing custom log storage:
/// - Add this enum to the LogEntry struct in @database.md
/// - Use for filtering in log queries
/// - Add severity-based retention policies
#[derive(Debug, Clone, Copy)]
pub enum LogType {
    Log,    // General purpose logging, lowest priority
    Debug,  // Detailed information for debugging and development
    Info,   // Notable events in normal operation (state changes, completions)
    Warn,   // Concerning but non-fatal issues (deprecated calls, recoverable errors)
    Error,  // Critical issues requiring immediate attention (data loss, security)
}

/// Implements string conversion for LogType to enable using string literals
/// in the logger! macro. This allows for a more natural API where users can
/// write logger!("info", ...) instead of logger!(LogType::Info, ...).
impl FromStr for LogType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "log" => Ok(LogType::Log),
            "debug" => Ok(LogType::Debug),
            "info" => Ok(LogType::Info),
            "warn" => Ok(LogType::Warn),
            "error" => Ok(LogType::Error),
            _ => Err(format!("Invalid log type: {}", s))
        }
    }
}

/// The logger! macro provides a user-friendly interface for logging.
/// 
/// Why a macro instead of a function?
/// 1. Can capture file!() and line!() at the call site
/// 2. Supports printf-style formatting like println!
/// 3. Type-safe at compile time
/// 4. Zero runtime overhead for formatting
/// 
/// The macro has two forms:
/// 1. logger!("level", "message")
/// 2. logger!("level", "message {}", variable)
/// 
/// Suggestion: Always include the function name in your message:
/// - Good: logger!("info", "[process_user] Starting validation")
/// - Bad:  logger!("info", "Starting validation")
/// 
/// At compile time, this expands to code that:
/// 1. Converts the log level string to LogType
/// 2. Formats the message with any variables
/// 3. Captures the current file and line
/// 4. Calls log_internal with all the pieces
/// 
/// Note: If an invalid log type is provided, we fall back to Error level
/// instead of Info. This is because an invalid log type indicates a 
/// programming error that should be fixed, not a runtime condition.
#[macro_export]
macro_rules! logger {
    // Pattern 1: Handle printf-style formatting with variables
    ($type:expr, $($arg:tt)*) => {{
        let log_type = LogType::from_str($type)
            .unwrap_or(LogType::Error);  // Fallback to Error - invalid types should not happen
        let message = format!($($arg)*); // Format message with variables
        $crate::utils::logger::log_internal(log_type, &message, None::<&()>, file!(), line!())
    }};
    // Pattern 2: Handle messages with additional structured data
    ($type:expr, $message:expr, $data:expr) => {{
        let log_type = LogType::from_str($type)
            .unwrap_or(LogType::Error);  // Fallback to Error for consistency
        $crate::utils::logger::log_internal(log_type, $message, Some($data), file!(), line!())
    }};
}

/// Internal function that handles the actual logging mechanics.
/// This should not be called directly - use the logger! macro instead.
/// 
/// The function:
/// 1. Takes the processed inputs from the macro
/// 2. Formats the final message with file:line prefix
/// 3. Routes to the appropriate Juno logging function
/// 
/// Parameters:
/// - log_type: The severity/category of the log
/// - message: The pre-formatted message to log
/// - data: Optional structured data to include
/// - file: Source file name (from macro)
/// - line: Source line number (from macro)
#[doc(hidden)]
pub fn log_internal<T: Serialize>(
    log_type: LogType,
    message: &str,
    data: Option<&T>,
    file: &str,
    line: u32,
) {
    // Create the standardized message format: [file:line] message
    let formatted_message = format!("[{}:{}] {}", file, line, message);
    
    // Route to the appropriate Juno logging function based on:
    // 1. The log type (severity level)
    // 2. Whether we have additional structured data
    match (log_type, data) {
        (LogType::Log, None) => log(formatted_message),
        (LogType::Log, Some(d)) => log_with_data(formatted_message, d),
        (LogType::Debug, None) => debug(formatted_message),
        (LogType::Debug, Some(d)) => debug_with_data(formatted_message, d),
        (LogType::Info, None) => info(formatted_message),
        (LogType::Info, Some(d)) => info_with_data(formatted_message, d),
        (LogType::Warn, None) => warn(formatted_message),
        (LogType::Warn, Some(d)) => warn_with_data(formatted_message, d),
        (LogType::Error, None) => error(formatted_message),
        (LogType::Error, Some(d)) => error_with_data(formatted_message, d),
    }
}

