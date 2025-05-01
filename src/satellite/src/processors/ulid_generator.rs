//! ULID Generator for Document Keys
//! 
//! This module provides utilities for generating ULID (Universally Unique Lexicographically 
//! Sortable Identifier) for use as document keys in Juno collections.
//!
//! ULIDs have several advantages over standard UUIDs:
//! - Lexicographically sortable (newer ULIDs sort after older ones)
//! - Case-insensitive
//! - URL safe (no special characters)
//! - Compact (26 characters)
//! - Contains timestamp information
//!
//! Format: 01ARZ3NDEKTSV4RRFFQ69G5FAV (26 characters, Crockford Base32)
//! - First 10 chars: Timestamp (milliseconds since epoch)
//! - Last 16 chars: Random data

use ic_cdk;
use ulid::Ulid;

/// Generates a ULID (Universally Unique Lexicographically Sortable Identifier)
/// 
/// Uses IC's time for the timestamp and raw_rand function for randomness
/// 
/// # Returns
/// * `String` - A ULID string in uppercase format
pub async fn generate_ulid() -> String {
    // Get current time from IC in milliseconds 
    // (IC time is in nanoseconds, divide by 1_000_000 to get milliseconds)
    let timestamp_ms = ic_cdk::api::time() / 1_000_000;
    
    // Get random bytes from IC for the random component
    let random_bytes = ic_cdk::api::management_canister::main::raw_rand()
        .await
        .map(|result| result.0)
        .unwrap_or_default();
    
    // Generate ULID with timestamp and random bytes
    // Note: We'll only use the first 10 bytes of random_bytes (80 bits)
    let mut random_data = [0u8; 10];
    for i in 0..10.min(random_bytes.len()) {
        random_data[i] = random_bytes[i];
    }
    
    // Create ULID with timestamp and random data
    let ulid = Ulid::from_parts(timestamp_ms, u128::from_be_bytes([
        0, 0, 0, 0, 0, 0, 
        random_data[0], random_data[1], random_data[2], random_data[3], random_data[4],
        random_data[5], random_data[6], random_data[7], random_data[8], random_data[9],
    ]));
    
    // Return uppercase string representation
    ulid.to_string().to_uppercase()
}

/// Validates a ULID string
/// 
/// Checks that the given string is a valid ULID:
/// - Exactly 26 characters
/// - All characters are valid Crockford Base32
/// - Is uppercase
/// 
/// # Arguments
/// * `ulid_str` - The ULID string to validate
/// 
/// # Returns
/// * `Result<(), String>` - Ok if valid, Err with message if invalid
pub fn validate_ulid(ulid_str: &str) -> Result<(), String> {
    // Check length
    if ulid_str.len() != 26 {
        return Err(format!("[validate_ulid] ULID must be exactly 26 characters, got {}", ulid_str.len()));
    }
    
    // Check case
    if ulid_str != ulid_str.to_uppercase() {
        return Err("[validate_ulid] ULID must be uppercase".to_string());
    }
    
    // Check character set (Crockford Base32)
    if !ulid_str.chars().all(|c| "0123456789ABCDEFGHJKMNPQRSTVWXYZ".contains(c)) {
        return Err("[validate_ulid] ULID contains invalid characters".to_string());
    }
    
    // Attempt to parse (validates format)
    match Ulid::from_string(ulid_str) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("[validate_ulid] Invalid ULID: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_generate_ulid() {
        // Generate two ULIDs
        let ulid1 = generate_ulid().await;
        let ulid2 = generate_ulid().await;
        
        // They should be different
        assert_ne!(ulid1, ulid2);
        
        // They should be valid
        assert!(validate_ulid(&ulid1).is_ok());
        assert!(validate_ulid(&ulid2).is_ok());
        
        // They should be 26 characters
        assert_eq!(ulid1.len(), 26);
        
        // Generate a ULID after a short delay
        // It should be lexicographically greater (sorts after) the first one
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        let ulid3 = generate_ulid().await;
        
        // Test lexicographic sorting
        assert!(ulid3 > ulid1);
    }
    
    #[test]
    fn test_validate_ulid() {
        // Valid ULID
        assert!(validate_ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV").is_ok());
        
        // Invalid cases
        assert!(validate_ulid("01ARZ3NDEKTSV4RRFFQ69G5FA").is_err()); // Too short
        assert!(validate_ulid("01ARZ3NDEKTSV4RRFFQ69G5FAVX").is_err()); // Too long
        assert!(validate_ulid("01ARZ3NDEKTSV4RRFFQ69G5FAv").is_err()); // Lowercase
        assert!(validate_ulid("01ARZ3NDEKTSV4RRFFQ69G5FAO").is_err()); // Invalid char 'O'
        assert!(validate_ulid("01ARZ3NDEKTSV4RRFFQ69G5FAI").is_err()); // Invalid char 'I'
        assert!(validate_ulid("01ARZ3NDEKTSV4RRFFQ69G5FAL").is_err()); // Invalid char 'L'
    }
} 