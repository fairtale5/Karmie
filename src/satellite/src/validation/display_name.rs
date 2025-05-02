use crate::logger;

/// Validates a display name string against expected format
/// 
/// Requirements:
/// - Length between 1 and 50 characters
/// - No leading/trailing whitespace
/// - No consecutive whitespace
pub fn validate_display_name(display_name: &str) -> Result<(), String> {
    if display_name.trim().is_empty() {
        logger!("error", "[validate_display_name] Display name is empty");
        return Err("Display name cannot be empty.".to_string());
    }
    if display_name.len() > 50 {
        logger!("error", "[validate_display_name] Display name too long: {}", display_name);
        return Err("Display name must be 50 characters or less.".to_string());
    }

    if display_name.trim() != display_name {
        return Err("Display name must not contain leading or trailing whitespace.".to_string());
    }

    let mut prev_char = ' ';
    for c in display_name.chars() {
        if c.is_whitespace() && prev_char.is_whitespace() {
            return Err("Display name must not contain consecutive whitespace.".to_string());
        }
        prev_char = c;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_display_name() {
        // Valid display names
        assert!(validate_display_name("John").is_ok());
        assert!(validate_display_name("John Doe").is_ok());
        assert!(validate_display_name("John Doe III").is_ok());

        // Invalid display names
        assert!(validate_display_name("").is_err()); // Empty
        assert!(validate_display_name(" John").is_err()); // Leading space
        assert!(validate_display_name("John ").is_err()); // Trailing space
        assert!(validate_display_name("John  Doe").is_err()); // Consecutive spaces
    }
} 