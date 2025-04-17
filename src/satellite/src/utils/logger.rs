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
 * 1. Macros can capture file!() and line!() at compile time (file!(), line!())
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
/// 3. logger!("level", "message", data)  // For structured data
/// 
/// Levels: "debug", "info", "warn", "error"
/// 
/// Suggestion: Always include the function name in your message:
/// - Good: logger!("info", "[process_user] Starting validation")
/// - Bad:  logger!("info", "Starting validation")
/// 
/// At compile time, this expands to code that:
/// 1. Formats any message variables using format!()
/// 2. Adds the [file:line] prefix using file!() and line!()
/// 3. Directly matches the log level string to the appropriate Juno function
/// 4. Falls back to error() if an invalid level is provided
/// 
/// Note: If an invalid log type is provided, we fall back to Error level
/// because an invalid log type indicates a programming error that should
/// be fixed, not a runtime condition.
///
/// Suggestion: Always include the function name in your message:
/// - Good: logger!("info", "[process_user] Starting validation")
/// - Bad:  logger!("info", "Starting validation")
#[macro_export]
macro_rules! logger {
    // Pattern 1: Handle printf-style formatting with variables
    ($type:expr, $($arg:tt)*) => {{
        let message = format!("[{}:{}] {}", file!(), line!(), format!($($arg)*));
        match $type {
            "log" => ::junobuild_satellite::log(message).unwrap_or_default(),
            "debug" => ::junobuild_satellite::debug(message).unwrap_or_default(),
            "info" => ::junobuild_satellite::info(message).unwrap_or_default(),
            "warn" => ::junobuild_satellite::warn(message).unwrap_or_default(),
            "error" => ::junobuild_satellite::error(message).unwrap_or_default(),
            _ => ::junobuild_satellite::error(message).unwrap_or_default()
        }
    }};

    // Pattern 2: Handle messages with additional structured data
    ($type:expr, $message:expr, $data:expr) => {{
        let message = format!("[{}:{}] {}", file!(), line!(), $message);
        match $type {
            "log" => ::junobuild_satellite::log_with_data(message, $data).unwrap_or_default(),
            "debug" => ::junobuild_satellite::debug_with_data(message, $data).unwrap_or_default(),
            "info" => ::junobuild_satellite::info_with_data(message, $data).unwrap_or_default(),
            "warn" => ::junobuild_satellite::warn_with_data(message, $data).unwrap_or_default(),
            "error" => ::junobuild_satellite::error_with_data(message, $data).unwrap_or_default(),
            _ => ::junobuild_satellite::error_with_data(message, $data).unwrap_or_default()
        }
    }};
}

