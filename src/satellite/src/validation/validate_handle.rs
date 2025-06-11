/// Validates a handle (username or tag) string against expected format
/// 
/// Requirements:
/// - Length between 3 and 20 characters
/// - Only alphanumeric characters and dashes
/// - Cannot start or end with a dash
/// - No consecutive dashes
pub fn validate_handle(username: &str) -> Result<(), String> {
    if username.len() < 3 || username.len() > 20 {
        return Err("[validate_handle] Handle must be between 3 and 20 characters.".to_string());
    }

    // Check if starts or ends with dash
    if username.starts_with('-') {
        return Err("[validate_handle] Handle cannot start with a dash.".to_string());
    }
    
    if username.ends_with('-') {
        return Err("[validate_handle] Handle cannot end with a dash.".to_string());
    }

    // Check for valid characters and consecutive dashes
    let mut prev_char = ' '; // Use space as initial value since it's not a dash
    for c in username.chars() {
        if !c.is_alphanumeric() && c != '-' {
            return Err("[validate_handle] Handle can only contain alphanumeric characters and dashes.".to_string());
        }
        if c == '-' && prev_char == '-' {
            return Err("[validate_handle] Handle cannot contain consecutive dashes.".to_string());
        }
        prev_char = c;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_handle() {
        // Valid usernames
        assert!(validate_handle("john").is_ok());
        assert!(validate_handle("john-doe").is_ok());
        assert!(validate_handle("john123").is_ok());
        assert!(validate_handle("123john").is_ok()); // Starts with number
        assert!(validate_handle("2024tag").is_ok()); // Starts with number
        assert!(validate_handle("abc").is_ok()); // Minimum length
        assert!(validate_handle("user1-test2").is_ok()); // Mixed alphanumeric with dash

        // Invalid usernames
        assert!(validate_handle("jo").is_err()); // Too short
        assert!(validate_handle("john doe").is_err()); // Invalid with spaces
        assert!(validate_handle("johndoejohndoejohndoe1").is_err()); // Too long
        assert!(validate_handle("-john").is_err()); // Starts with dash
        assert!(validate_handle("john-").is_err()); // Ends with dash
        assert!(validate_handle("john--doe").is_err()); // Consecutive dashes
        assert!(validate_handle("john@doe").is_err()); // Invalid character
        assert!(validate_handle("@handle").is_err()); // Starts with special character
        assert!(validate_handle("_user").is_err()); // Starts with underscore
        assert!(validate_handle("user_name").is_err()); // Contains underscore
    }
} 