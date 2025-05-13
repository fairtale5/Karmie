/// Validates a handle (username or tag) string against expected format
/// 
/// Requirements:
/// - Length between 3 and 20 characters
/// - Only alphanumeric characters and dashes
/// - Must start with a letter
/// - No consecutive dashes
/// - No trailing dash
pub fn validate_handle(username: &str) -> Result<(), String> {
    if username.len() < 3 || username.len() > 20 {
        return Err("[validate_handle] Handle must be between 3 and 20 characters.".to_string());
    }

    let first_char = username.chars().next().unwrap();
    if !first_char.is_alphabetic() {
        return Err("[validate_handle] Handle must start with a letter.".to_string());
    }

    let mut prev_char = '-';
    for c in username.chars() {
        if !c.is_alphanumeric() && c != '-' {
            return Err("[validate_handle] Handle can only contain alphanumeric characters and dashes.".to_string());
        }
        if c == '-' && prev_char == '-' {
            return Err("[validate_handle] Handle cannot contain consecutive dashes.".to_string());
        }
        prev_char = c;
    }

    if prev_char == '-' {
        return Err("[validate_handle] Handle cannot end with a dash.".to_string());
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

        // Invalid usernames
        assert!(validate_handle("jo").is_err()); // Too short
        assert!(validate_handle("john doe").is_ok()); // Invalid with spaces
        assert!(validate_handle("johndoejohndoejohndoe1").is_err()); // Too long
        assert!(validate_handle("123john").is_err()); // Starts with number
        assert!(validate_handle("john--doe").is_err()); // Consecutive dashes
        assert!(validate_handle("john-").is_err()); // Ends with dash
        assert!(validate_handle("john@doe").is_err()); // Invalid character
    }
} 