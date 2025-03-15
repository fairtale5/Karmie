/*! 
 * Utility modules for the Reputator satellite
 * 
 * This module contains various utility functions used throughout the satellite,
 * organized into submodules for better maintainability and clarity.
 */

// Export utility modules
pub mod normalize;
pub mod validation;
pub mod logging;

// Re-export only the logging macro we're using
#[allow(unused_imports)]
pub use crate::log_user; 