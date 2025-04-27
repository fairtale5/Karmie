/*!
 * Time period validation utilities
 * 
 * This module provides functions for validating time periods used in tag data.
 * It ensures that time periods are properly formatted and logically valid.
 */

use ic_cdk::api::time;

/// Validates a list of time periods to ensure they are properly formatted
/// and logically valid.
///
/// # Arguments
/// * `time_periods` - A vector of tuples, each containing start and end timestamps
///
/// # Returns
/// * `Result<(), String>` - Ok if valid, Err with error message if invalid
///
/// # Example
/// ```
/// let periods = vec![(1625097600000, 1625184000000)]; // Valid period
/// assert!(validate_time_periods(periods).is_ok());
/// ```
pub fn validate_time_periods(time_periods: Vec<(u64, u64)>) -> Result<(), String> {
    if time_periods.is_empty() {
        return Ok(());
    }

    let current_time = time();
    let current_time_ms = current_time / 1_000_000; // Convert nanoseconds to milliseconds

    for (i, (start, end)) in time_periods.iter().enumerate() {
        // Check if end time is after start time
        if end <= start {
            return Err(format!("End time must be after start time for period {}", i + 1));
        }

        // Check if times are in the past
        if *end < current_time_ms {
            return Err(format!("Time period {} is entirely in the past", i + 1));
        }

        // Check for overlapping periods
        for (j, (other_start, other_end)) in time_periods.iter().enumerate() {
            if i != j {
                if (*start <= *other_end && *end >= *other_start) {
                    return Err(format!("Time periods {} and {} overlap", i + 1, j + 1));
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_periods() {
        let periods: Vec<(u64, u64)> = vec![];
        assert!(validate_time_periods(periods).is_ok());
    }

    #[test]
    fn test_valid_periods() {
        // Get current time in milliseconds
        let current_time_ms = time() / 1_000_000;
        
        // Create periods in the future
        let future_start = current_time_ms + 10000;
        let future_end = future_start + 10000;
        
        let periods = vec![
            (future_start, future_end),
            (future_end + 1000, future_end + 20000)
        ];
        
        assert!(validate_time_periods(periods).is_ok());
    }

    #[test]
    fn test_end_before_start() {
        let current_time_ms = time() / 1_000_000;
        let future_time = current_time_ms + 10000;
        
        let periods = vec![(future_time + 10000, future_time)];
        
        let result = validate_time_periods(periods);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("End time must be after start time"));
    }

    #[test]
    fn test_past_periods() {
        let current_time_ms = time() / 1_000_000;
        
        let periods = vec![(current_time_ms - 20000, current_time_ms - 10000)];
        
        let result = validate_time_periods(periods);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("entirely in the past"));
    }

    #[test]
    fn test_overlapping_periods() {
        let current_time_ms = time() / 1_000_000;
        let future_time = current_time_ms + 10000;
        
        let periods = vec![
            (future_time, future_time + 10000),
            (future_time + 5000, future_time + 15000)
        ];
        
        let result = validate_time_periods(periods);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("overlap"));
    }
} 