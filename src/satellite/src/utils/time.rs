/*!
 * Time-related calculations and utilities
 * 
 * This module provides functions for calculating time-based metrics
 * using Juno's native time functions. It's designed to work efficiently
 * on the Internet Computer platform.
 * 
 * Key features:
 * - Month calculations between timestamps
 * - Efficient timestamp handling using Juno's native functions
 */

use junobuild_shared::day::calendar_date;

/// Calculates the number of months between two timestamps
/// 
/// This function ONLY counts the number of months between dates, ignoring days completely.
/// For example:
/// - Jan 1st to Jan 31st = 0 months (same month)
/// - Jan 15th to Feb 1st = 1 month (different months)
/// - Jan 31st to Feb 1st = 1 month (different months)
/// - Jan 1st 2024 to Mar 15th 2025 = 14 months (11 months in 2024 + 3 months in 2025)
/// 
/// The function is used for reputation calculations where we only care about
/// how many months have passed, not the specific days.
/// 
/// # Arguments
/// * `timestamp1` - First timestamp in nanoseconds
/// * `timestamp2` - Second timestamp in nanoseconds
/// 
/// # Returns
/// * `Result<u32, String>` - Number of months between the timestamps or an error message
/// 
/// # Examples
/// ```rust
/// // Same month = 0 months
/// let jan1 = 1704067200000000000;  // 2024-01-01
/// let jan31 = 1706745600000000000; // 2024-01-31
/// assert_eq!(calculate_months_between(jan1, jan31).map_err(|e| e.to_string()), Ok(0));
/// 
/// // Different months = 1 month
/// let feb1 = 1706832000000000000;  // 2024-02-01
/// assert_eq!(calculate_months_between(jan1, feb1).map_err(|e| e.to_string()), Ok(1));
/// 
/// // Different years = 12 months
/// let jan2025 = 1735689600000000000; // 2025-01-01
/// assert_eq!(calculate_months_between(jan1, jan2025).map_err(|e| e.to_string()), Ok(12));
/// 
/// // Error cases:
/// assert!(calculate_months_between(0, jan1).is_err()); // Invalid timestamp
/// assert!(calculate_months_between(jan1, future).is_err()); // Future date
/// ```
pub fn calculate_months_between(timestamp1: u64, timestamp2: u64) -> Result<u32, String> {
    // Convert nanoseconds to milliseconds for Juno's calendar_date function
    let t1 = timestamp1 / 1_000_000;
    let t2 = timestamp2 / 1_000_000;
    
    // Get calendar dates using Juno's native function
    let date1 = calendar_date(&t1);
    let date2 = calendar_date(&t2);
    
    // Check for future timestamps - return error instead of 0
    if t2 > ic_cdk::api::time() / 1_000_000 {
        return Err(format!("Cannot calculate months for future date: {}", timestamp2));
    }
    
    // Calculate months difference using year and month components
    let months = (date2.year - date1.year) * 12 + 
                 (date2.month as i32 - date1.month as i32);
    
    // Check for negative months
    if months < 0 {
        return Ok(0);
    }
    
    Ok(months as u32)
}

// NOTE: This function has been replaced by `get_period_multiplier` in reputation_calculations.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_months_between() {
        // Test case 1: Same month (January 2024)
        let jan1 = 1704067200000000000; // 2024-01-01 00:00:00 UTC
        let jan15 = 1705276800000000000; // 2024-01-15 00:00:00 UTC
        assert_eq!(calculate_months_between(jan1, jan15).map_err(|e| e.to_string()), Ok(0));

        // Test case 2: One month difference (January to February 2024)
        let feb1 = 1706745600000000000; // 2024-02-01 00:00:00 UTC
        assert_eq!(calculate_months_between(jan1, feb1).map_err(|e| e.to_string()), Ok(1));

        // Test case 3: Multiple months difference (January to April 2024)
        let apr1 = 1711929600000000000; // 2024-04-01 00:00:00 UTC
        assert_eq!(calculate_months_between(jan1, apr1).map_err(|e| e.to_string()), Ok(3));

        // Test case 4: Year boundary (December 2023 to January 2024)
        let dec1 = 1701388800000000000; // 2023-12-01 00:00:00 UTC
        assert_eq!(calculate_months_between(dec1, jan1).map_err(|e| e.to_string()), Ok(1));

        // Test case 5: Multiple years (January 2023 to January 2024)
        let jan2023 = 1672531200000000000; // 2023-01-01 00:00:00 UTC
        assert_eq!(calculate_months_between(jan2023, jan1).map_err(|e| e.to_string()), Ok(12));

        // Test case 6: Future timestamp (should return error)
        let future = ic_cdk::api::time() + 1_000_000_000; // 1 second in the future
        assert!(calculate_months_between(jan1, future).is_err());

        // Test case 7: Negative months (should return 0)
        assert_eq!(calculate_months_between(feb1, jan1).map_err(|e| e.to_string()), Ok(0));

        // Test case 8: Invalid timestamp
        assert!(calculate_months_between(0, jan1).is_err());
    }
} 