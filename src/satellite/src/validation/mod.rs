/*!
 * Basic data validation utilities
 * 
 * This module contains functions for validating various data types and formats:
 * - Username validation
 * - Display name validation
 * - Tag name validation
 * - Time period validation
 * - Description format validation
 * 
 * These validation functions are used throughout the codebase to ensure
 * data integrity and consistency.
 */

mod description;
mod display_name;
mod username;
mod tag_validate_time_periods;
pub mod ulid_timestamp_validate;

pub use description::validate_description;
pub use display_name::validate_display_name;
pub use tag_validate_time_periods::validate_time_periods;
pub use username::validate_username;
pub use username::validate_username_uniqueness;



// TODO: As validation functions are moved from utils to this module,
// add the following structure:

// mod validate_name;  // For username, display name, tag name validations
// mod validate_time;  // For time period validations
// mod validate_format; // For description format validations

// pub use validate_name::{validate_username, validate_display_name, validate_tag_name};
// pub use validate_time::validate_time_periods;
// pub use validate_format::validate_description; 