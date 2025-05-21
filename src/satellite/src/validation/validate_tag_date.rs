/*!
 * Time period validation utilities
 * 
 * This module provides functions for validating time periods used in tag data.
 * It ensures that time periods are properly formatted and logically valid.
 */

use crate::utils::structs::TimePeriod;
use crate::logger;

/// Validates a vector of TimePeriod structs
/// Each TimePeriod defines a duration in calendar months and a multiplier for votes that fall within that period
///
/// # Arguments
/// * `time_periods` - A reference to a vector of TimePeriod structs
///
/// # Returns
/// * `Result<(), String>` - Ok if valid, Err with error message if invalid
///
/// # Example
/// ```
/// time_periods: [
///   { months: 1, multiplier: 1.5 },  // Current month: 150% weight (month 1)
///   { months: 2, multiplier: 1.2 },  // Next 2 months: 120% weight (month 2 and 3)
///   { months: 3, multiplier: 1.1 },  // Next 3 months: 110% weight (month 4, 5, and 6)
///   { months: 6, multiplier: 1.0 },  // Next 6 months: 100% weight (month 7 through 12)
///   { months: 12, multiplier: 0.75 } // Next 12 months: 75% weight (month 13 through 24)
/// ]
/// ```
pub fn validate_tag_date_struct(time_periods: &Vec<TimePeriod>) -> Result<(), String> {
    if time_periods.is_empty() {
        return Ok(());
    }

    // Validate each period's values
    for (i, period) in time_periods.iter().enumerate() {
        // Validate months (must be positive integer)
        if period.months < 1 {
            logger!("error", "[validate_tag_date_struct] Invalid months value in period {}: {} (must be ≥ 1)", 
                i + 1, period.months);
            return Err(format!("Duration must be at least 1 month for period {}", i + 1));
        }

        // Validate multiplier range (0.05-10)
        if period.multiplier < 0.05 || period.multiplier > 10.0 {
            logger!("error", "[validate_tag_date_struct] Invalid multiplier in period {}: {} (must be between 0.05 and 10)", 
                i + 1, period.multiplier);
            return Err(format!("Multiplier must be between 0.05 and 10 for period {}", i + 1));
        }

        // Validate multiplier is a multiple of 0.05
        let steps = (period.multiplier / 0.05).round() as i32;
        let normalized = (steps as f64) * 0.05;
        if (period.multiplier - normalized).abs() > f64::EPSILON {
            logger!("error", "[validate_tag_date_struct] Invalid multiplier in period {}: {} (must be a multiple of 0.05)", 
                i + 1, period.multiplier);
            return Err(format!("Multiplier must be a multiple of 0.05 for period {} (e.g., 0.05, 0.10, 0.15, etc.)", i + 1));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_periods() {
        let periods: Vec<TimePeriod> = vec![];
        assert!(validate_tag_date_struct(&periods).is_ok());
    }

    #[test]
    fn test_valid_periods() {
        let periods = vec![
            TimePeriod { months: 1, multiplier: 1.5 },   // Current month
            TimePeriod { months: 2, multiplier: 1.2 },   // Next 2 months
            TimePeriod { months: 3, multiplier: 1.1 },   // Next 3 months
            TimePeriod { months: 6, multiplier: 1.0 },   // Next 6 months
            TimePeriod { months: 12, multiplier: 0.75 }  // Next 12 months
        ];
        assert!(validate_tag_date_struct(&periods).is_ok());
    }

    #[test]
    fn test_valid_multiplier_steps() {
        let periods = vec![
            TimePeriod { months: 1, multiplier: 0.05 },  // Minimum value
            TimePeriod { months: 1, multiplier: 0.10 },  // Double minimum
            TimePeriod { months: 1, multiplier: 0.15 },  // Triple minimum
            TimePeriod { months: 1, multiplier: 1.00 },  // Whole number
            TimePeriod { months: 1, multiplier: 10.00 }  // Maximum value
        ];
        assert!(validate_tag_date_struct(&periods).is_ok());
    }

    #[test]
    fn test_invalid_months() {
        let periods = vec![
            TimePeriod { months: 0, multiplier: 1.0 }  // Invalid: 0 months
        ];
        let result = validate_tag_date_struct(&periods);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Duration must be at least 1 month"));
    }

    #[test]
    fn test_invalid_multiplier_too_high() {
        let periods = vec![
            TimePeriod { months: 1, multiplier: 11.0 }  // Invalid: multiplier > 10
        ];
        let result = validate_tag_date_struct(&periods);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Multiplier must be between 0.05 and 10"));
    }

    #[test]
    fn test_invalid_multiplier_too_low() {
        let periods = vec![
            TimePeriod { months: 1, multiplier: 0.01 }  // Invalid: multiplier < 0.05
        ];
        let result = validate_tag_date_struct(&periods);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Multiplier must be between 0.05 and 10"));
    }

    #[test]
    fn test_invalid_multiplier_not_multiple() {
        let periods = vec![
            TimePeriod { months: 1, multiplier: 0.07 }  // Invalid: not a multiple of 0.05
        ];
        let result = validate_tag_date_struct(&periods);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be a multiple of 0.05"));
    }
} 