/*!
 * Logging Configuration and Utilities
 * 
 * This module provides centralized logging configuration and utilities.
 * It allows enabling/disabling logs globally or by specific areas.
 */

use ic_cdk;

// Log Areas - Add new areas as needed
#[allow(dead_code)]  // Disable warnings for unused variants
#[derive(Clone, Copy)]
pub enum LogArea {
    UserManagement,
    Authentication,
    DataValidation,
    // Add more areas as needed
}

/// Determines if logging is enabled for a specific area
pub fn is_logging_enabled(_area: LogArea) -> bool {
    // Currently always enabled for UserManagement
    true
}

/// Log a message if logging is enabled for the specified area
pub fn log(area: LogArea, message: &str) {
    if is_logging_enabled(area) {
        ic_cdk::print(format!("[{}] {}", area_to_string(area), message));
    }
}

/// Convert LogArea to string for display
fn area_to_string(_area: LogArea) -> &'static str {
    "USER"
}

// Convenience macro for user management logging
#[macro_export]
macro_rules! log_user {
    ($($arg:tt)*) => {
        $crate::utils::logging::log(
            $crate::utils::logging::LogArea::UserManagement,
            &format!($($arg)*)
        )
    };
} 