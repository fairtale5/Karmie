//! ULID Timestamp Validation
//! 
//! This module provides utilities for validating the timestamp component of ULIDs
//! (Universally Unique Lexicographically Sortable Identifiers).
//!
//! The timestamp component is the first 10 characters of a ULID and represents
//! a millisecond timestamp in Crockford Base32 encoding.
//!
//! # Validation rules
//! - Timestamp must be after January 1, 2010 (arbitrary cutoff for reasonable ULIDs)
//! - Timestamp must not be more than 5 minutes in the future (prevents far-future timestamps)

use crate::processors::ulid_timestamp_extract::extract_timestamp_ms;

/// Validates the timestamp component of a ULID (first 10 characters)
/// 
/// This function ensures that the timestamp in a ULID makes sense:
/// - It must be after January 1, 2010 (a reasonable minimum date for ULIDs)
/// - It must not be more than 5 minutes in the future (prevents unreasonable future dates)
/// 
/// # Arguments
/// * `ulid_str` - The complete ULID string to validate
/// 
/// # Returns
/// * `Result<(), String>` - Ok if valid, Err with descriptive message if invalid
/// 
/// # Example
/// ```
/// use crate::validation::ulid_timestamp_validate::validate_timestamp_component;
/// 
/// // Check if a ULID has a valid timestamp
/// let result = validate_timestamp_component("01ARZ3NDEKTSV4RRFFQ69G5FAV");
/// assert!(result.is_ok());
/// ```
pub fn validate_timestamp_component(ulid_str: &str) -> Result<(), String> {
    // Extract timestamp component (first 10 characters)
    // Note: We don't need to use this directly as extract_timestamp_ms handles it
    let _timestamp_str = &ulid_str[..10];
    
    // Attempt to parse timestamp (should be a valid base32 number)
    match extract_timestamp_ms(ulid_str) {
        Ok(timestamp_ms) => {
            // Get current time in milliseconds
            let now_ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;
                
            // Minimum timestamp: January 1, 2010 (arbitrary cutoff for reasonable ULIDs)
            let year_2010_ms: u64 = 1262304000000; // Jan 1, 2010
            
            // Maximum timestamp: 5 minutes in the future
            // This allows for small clock differences between systems
            // 5 minutes = 300 seconds = 300,000 milliseconds
            let max_future_ms = now_ms + 300_000; // 5 minutes in the future
                
            // Check if timestamp is too old
            if timestamp_ms < year_2010_ms {
                return Err(format!("ULID timestamp is too old (before 2010): {} ms since epoch", timestamp_ms));
            }
            
            // Check if timestamp is too far in the future
            if timestamp_ms > max_future_ms {
                return Err(format!("ULID timestamp is too far in the future (>5min): {} ms (now: {} ms)", 
                    timestamp_ms, now_ms));
            }
            
            Ok(())
        },
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_timestamp() {
        // Get current time and create a valid ULID timestamp component (first 10 chars)
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
            
        // We don't have a function to convert a timestamp to a ULID timestamp component,
        // so we'll just use a known valid one for testing
        let valid_ulid = "01ARZ3NDEKTSV4RRFFQ69G5FAV"; // From July 2016
        assert!(validate_timestamp_component(valid_ulid).is_ok());
    }
    
    #[test]
    fn test_timestamp_too_old() {
        // Create a ULID with timestamp from 1970 (too old)
        // This is just a made-up ULID with very low timestamp component
        let too_old_ulid = "0000000000TSRQPNMKJH89876543"; 
        let result = validate_timestamp_component(too_old_ulid);
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.contains("too old"), "Error should mention timestamp being too old: {}", err);
        }
    }
    
    #[test]
    fn test_timestamp_future() {
        // Create a ULID with timestamp far in the future
        // Using all Z's for the timestamp component (maximum possible value in Crockford Base32)
        let future_ulid = "ZZZZZZZZZZZZZZZZZZZZZZZZZZ"; 
        let result = validate_timestamp_component(future_ulid);
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.contains("future"), "Error should mention timestamp being in the future: {}", err);
        }
    }
} 