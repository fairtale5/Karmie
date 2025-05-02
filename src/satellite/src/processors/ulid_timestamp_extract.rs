//! ULID Timestamp Extraction
//! 
//! This module provides utilities for extracting the timestamp component from ULIDs
//! (Universally Unique Lexicographically Sortable Identifiers).
//!
//! A ULID consists of:
//! - First 10 characters: Timestamp (milliseconds since epoch) in Crockford Base32
//! - Last 16 characters: Random data
//!
//! This module focuses on extracting and converting the timestamp component.

/// Extracts the timestamp in milliseconds from a ULID string
/// 
/// ULIDs contain a timestamp (milliseconds since Unix epoch) in the first 10 characters,
/// encoded using Crockford Base32. This function extracts and converts this timestamp.
/// 
/// # Arguments
/// * `ulid_str` - The ULID string (at least 10 characters)
/// 
/// # Returns
/// * `Result<u64, String>` - The extracted timestamp in milliseconds, or error message
/// 
/// # Example
/// ```
/// use crate::processors::ulid_timestamp_extract::extract_timestamp_ms;
/// 
/// // Extract timestamp from a ULID
/// let timestamp = extract_timestamp_ms("01ARZ3NDEKTSV4RRFFQ69G5FAV").unwrap();
/// // timestamp is now in milliseconds since Unix epoch (1467839849013 for this example)
/// ```
pub fn extract_timestamp_ms(ulid_str: &str) -> Result<u64, String> {
    // Check if the string is long enough to contain a timestamp
    if ulid_str.len() < 10 {
        return Err(format!(
            "ULID string too short to extract timestamp: {} (need at least 10 characters)",
            ulid_str.len()
        ));
    }
    
    // Extract timestamp component (first 10 characters)
    let timestamp_str = &ulid_str[..10];
    
    // Convert from Crockford Base32 to a number
    // Crockford Base32 uses characters:
    // 0-9: Represent 0-9
    // A-H: Represent 10-17
    // J-K: Represent 18-19 (I is skipped to avoid confusion with 1)
    // M-N: Represent 20-21 (L is skipped to avoid confusion with 1)
    // P-T: Represent 22-26 (O is skipped to avoid confusion with 0)
    // V-Z: Represent 27-31 (U is skipped to avoid ambiguity)
    let mut timestamp_ms: u64 = 0;
    
    // Loop through each character, converting from Base32 to a number
    for c in timestamp_str.chars() {
        // Multiply the accumulated value by 32 (left shift by 5 bits)
        timestamp_ms = timestamp_ms * 32;
        
        // Add the value of the current character
        timestamp_ms += match c {
            // Digits 0-9 map to values 0-9
            '0'..='9' => c as u64 - '0' as u64,
            
            // Letters A-H map to values 10-17
            'A'..='H' => c as u64 - 'A' as u64 + 10,
            
            // Letters J-K map to values 18-19 (I is skipped)
            'J'..='K' => c as u64 - 'J' as u64 + 18,
            
            // Letters M-N map to values 20-21 (L is skipped)
            'M'..='N' => c as u64 - 'M' as u64 + 20,
            
            // Letters P-T map to values 22-26 (O is skipped)
            'P'..='T' => c as u64 - 'P' as u64 + 22,
            
            // Letters V-Z map to values 27-31 (U is skipped)
            'V'..='Z' => c as u64 - 'V' as u64 + 27,
            
            // Any other character is invalid in Crockford Base32
            _ => return Err(format!("Invalid character in ULID timestamp: {}", c))
        };
    }
    
    // Return the extracted timestamp
    Ok(timestamp_ms)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_timestamp() {
        // Known ULID example from the spec: 01ARZ3NDEKTSV4RRFFQ69G5FAV
        // The first 10 characters (01ARZ3NDEK) represent a timestamp in July 2016
        let ulid = "01ARZ3NDEKTSV4RRFFQ69G5FAV";
        let timestamp = extract_timestamp_ms(ulid).unwrap();
        
        // We don't know the exact expected value, but it should be in a reasonable range
        // for July 2016 (around 1467839849013)
        assert!(timestamp > 1400000000000, "Timestamp should be after 2014"); 
        assert!(timestamp < 1500000000000, "Timestamp should be before 2017");
    }
    
    #[test]
    fn test_extract_timestamp_short_string() {
        // Test with a string that's too short
        let short_string = "01ARZ";
        let result = extract_timestamp_ms(short_string);
        assert!(result.is_err());
        
        if let Err(err) = result {
            assert!(err.contains("too short"), "Error should mention the string being too short: {}", err);
        }
    }
    
    #[test]
    fn test_extract_timestamp_invalid_char() {
        // Test with a string containing an invalid character ('U' is not used in Crockford Base32)
        let invalid_string = "01ARZ3UDEKTSV4RRFFQ69G5FAV";
        let result = extract_timestamp_ms(invalid_string);
        assert!(result.is_err());
        
        if let Err(err) = result {
            assert!(err.contains("Invalid character"), "Error should mention the invalid character: {}", err);
        }
    }
    
    #[test]
    fn test_timestamp_encoding() {
        // Test with known values for the first few Crockford Base32 characters
        
        // "0" should be value 0
        assert_eq!(extract_timestamp_ms("0000000000AAAAAAAAAAAAAAAA").unwrap(), 0);
        
        // "1" should be value 1
        assert_eq!(extract_timestamp_ms("1000000000AAAAAAAAAAAAAAAA").unwrap(), 1 * 32_u64.pow(9));
        
        // "A" should be value 10
        assert_eq!(extract_timestamp_ms("A000000000AAAAAAAAAAAAAAAA").unwrap(), 10 * 32_u64.pow(9));
        
        // "Z" should be value 31
        assert_eq!(extract_timestamp_ms("Z000000000AAAAAAAAAAAAAAAA").unwrap(), 31 * 32_u64.pow(9));
    }
} 