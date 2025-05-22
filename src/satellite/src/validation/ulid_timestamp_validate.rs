//! ULID Timestamp Validation
//! 
//! This module provides utilities for validating ULIDs (Universally Unique Lexicographically 
//! Sortable Identifiers).
//! 
//! The timestamp component is the first 10 characters of a ULID and represents
//! a millisecond timestamp in Crockford Base32 encoding.
//!
//! # Validation Rules
//! 
//! ## General Validation (Always Applied)
//! - Timestamp must be after January 1, 2025 (arbitrary cutoff for reasonable ULIDs)
//! - Timestamp must not be more than 2 minutes in the future (prevents far-future timestamps)
//! - Valid base32 format
//! 
//! ## New Document Check (Optional Additional Check)
//! - Timestamp must not be more than 2 minutes in the past
//! - Used for validating new documents

use crate::processors::ulid_timestamp_extract::extract_timestamp_ms;

/// Configuration for checking if a ULID timestamp is new
/// This is a boolean that makes the code easier to read
pub enum CheckULIDisNew {
    /// Only perform general validation
    No,
    /// Also check if timestamp is not too old
    Yes,
}

impl CheckULIDisNew {
    /// Create a configuration that only checks general validity
    pub fn no() -> Self {
        Self::No
    }
    
    /// Create a configuration that also checks if timestamp is not too old
    pub fn yes() -> Self {
        Self::Yes
    }
}

/// Validates a ULID timestamp based on the specified check mode
/// 
/// # Arguments
/// * `ulid_str` - The ULID string to validate
/// * `check_new` - Whether to also check if the timestamp is not too old
/// 
/// # Returns
/// * `Result<(), String>` - Ok if valid, Err with descriptive message if invalid
/// 
/// # Example
/// ```
/// use crate::validation::ulid_timestamp_validate::{validate_ulid_timestamp, CheckULIDisNew};
/// 
/// // Basic validation only
/// validate_ulid_timestamp(ulid_str, CheckULIDisNew::no())?;
/// 
/// // Basic validation + freshness check
/// validate_ulid_timestamp(ulid_str, CheckULIDisNew::yes())?;
/// ```
pub fn validate_ulid_timestamp(ulid_str: &str, check_new: CheckULIDisNew) -> Result<(), String> {

    // Extract timestamp from the ULID
    let timestamp_ms = extract_timestamp_ms(ulid_str)?;
    
    // Get current time
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
        
    // General validation (always done)
    let year_2025_ms: u64 = 1735689600000; // Jan 1, 2025
    if timestamp_ms < year_2025_ms {
        return Err(format!("ULID timestamp is too old (before 2025): {} ms since epoch", timestamp_ms));
    }
    
    if timestamp_ms > now_ms + 120_000 { // 2 minutes
        return Err(format!("ULID timestamp is too far in the future (>2min): {} ms (now: {} ms)", 
            timestamp_ms, now_ms));
    }
    
    // Optional new document check
    if matches!(check_new, CheckULIDisNew::Yes) {
        if timestamp_ms + 120_000 < now_ms { // 2 minutes
            return Err(format!("ULID timestamp is too old (>2min): {} ms (now: {} ms)", 
                timestamp_ms, now_ms));
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_timestamp_basic() {
        let valid_ulid = "01ARZ3NDEKTSV4RRFFQ69G5FAV"; // From July 2016
        assert!(validate_ulid_timestamp(valid_ulid, CheckULIDisNew::no()).is_ok());
    }
    
    #[test]
    fn test_timestamp_too_old() {
        let too_old_ulid = "0000000000TSRQPNMKJH89876543"; 
        let result = validate_ulid_timestamp(too_old_ulid, CheckULIDisNew::no());
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.contains("too old"), "Error should mention timestamp being too old: {}", err);
        }
    }
    
    #[test]
    fn test_timestamp_future() {
        let future_ulid = "ZZZZZZZZZZZZZZZZZZZZZZZZZZ"; 
        let result = validate_ulid_timestamp(future_ulid, CheckULIDisNew::no());
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.contains("future"), "Error should mention timestamp being in the future: {}", err);
        }
    }
    
    #[test]
    fn test_timestamp_new_check() {
        let old_ulid = "01ARZ3NDEKTSV4RRFFQ69G5FAV"; // From July 2016
        let result = validate_ulid_timestamp(old_ulid, CheckULIDisNew::yes());
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.contains("too old"), "Error should mention timestamp being too old: {}", err);
        }
    }
} 