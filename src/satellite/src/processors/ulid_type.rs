//! ULID Type Implementation
//! 
//! This module provides a strongly-typed ULID (Universally Unique Lexicographically 
//! Sortable Identifier) implementation with validation.
//!
//! ULIDs have several advantages over standard UUIDs:
//! - Lexicographically sortable (newer ULIDs sort after older ones)
//! - Case-insensitive (stored as uppercase)
//! - URL safe (no special characters)
//! - Compact (26 characters)
//! - Contains timestamp information
//!
//! Format: 01ARZ3NDEKTSV4RRFFQ69G5FAV (26 characters, Crockford Base32)
//! - First 10 chars: Timestamp (milliseconds since epoch)
//! - Last 16 chars: Random data
//!
//! This implementation ensures:
//! - Proper formatting (26 characters, valid Crockford Base32)
//! - Uppercase storage
//! - Timestamp validation
//! - Type-safe usage with serde support

use serde::{Deserialize, Serialize, Deserializer, Serializer};
use std::fmt;
use crate::validation::ulid_timestamp_validate::{validate_ulid_timestamp, CheckULIDisNew};
use crate::processors::ulid_timestamp_extract::extract_timestamp_ms;

/// A strongly-typed ULID implementation with validation
///
/// This type wraps a String but ensures it's a valid ULID through validation
/// during construction and deserialization.
///
/// # Examples
///
/// ```
/// use crate::processors::ulid_type::ULID;
///
/// // Create a ULID from a string (validates during creation)
/// let ulid = ULID::new("01ARZ3NDEKTSV4RRFFQ69G5FAV".to_string()).unwrap();
///
/// // Get the underlying string value
/// let ulid_string = ulid.value();
/// assert_eq!(ulid_string, "01ARZ3NDEKTSV4RRFFQ69G5FAV");
///
/// // Extract timestamp information
/// let timestamp_ms = ulid.timestamp_ms().unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ULID(String);

impl ULID {
    /// Creates a new ULID with validation
    ///
    /// Validates the provided string is a proper ULID format:
    /// - Exactly 26 characters long
    /// - All characters are valid Crockford Base32
    /// - All characters are uppercase
    /// - Timestamp component (first 10 chars) represents a valid timestamp
    ///
    /// # Arguments
    /// * `value` - The ULID string to validate and wrap
    ///
    /// # Returns
    /// * `Result<ULID, String>` - A validated ULID or error message
    pub fn new(value: String) -> Result<Self, String> {
        // Validate ULID format
        if value.len() != 26 {
            return Err(format!("ULID must be exactly 26 characters, got {}", value.len()));
        }
        
        // Check case
        if value != value.to_uppercase() {
            return Err("ULID must be uppercase".to_string());
        }
        
        // Check character set (Crockford Base32)
        if !value.chars().all(|c| "0123456789ABCDEFGHJKMNPQRSTVWXYZ".contains(c)) {
            return Err("ULID contains invalid characters".to_string());
        }
        
        // Validate timestamp component (first 10 characters)
        if let Err(e) = validate_ulid_timestamp(&value, CheckULIDisNew::no()) {
            return Err(e);
        }
        
        Ok(ULID(value))
    }
    
    /// Gets the underlying ULID string value
    ///
    /// # Returns
    /// * `&str` - The ULID as a string slice
    pub fn value(&self) -> &str {
        &self.0
    }
    
    /// Extract the timestamp value from this ULID
    ///
    /// # Returns
    /// * `Result<u64, String>` - The timestamp in milliseconds or error
    pub fn timestamp_ms(&self) -> Result<u64, String> {
        extract_timestamp_ms(&self.0)
    }
}

// Implement Display for easy string conversion
impl fmt::Display for ULID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Custom serialization - converts to a simple string
impl Serialize for ULID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

// Custom deserialization - validates the string before creating a ULID
impl<'de> Deserialize<'de> for ULID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ULID::new(s).map_err(serde::de::Error::custom)
    }
}

// Example structs showing ULID usage in data structures

/// Example of data with a simple string ID (no validation)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalData {
    pub id: String,
    pub name: String,
}

/// Example of data with a validated ULID field
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomData {
    pub id: ULID,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use junobuild_utils::{encode_doc_data, decode_doc_data};

    #[test]
    fn test_normal_serialization() {
        // Create normal data
        let data = NormalData {
            id: "01ARZ3NDEKTSV4RRFFQ69G5FAV".to_string(),
            name: "test".to_string(),
        };

        // Encode and decode
        let encoded = encode_doc_data(&data).unwrap();
        let decoded: NormalData = decode_doc_data(&encoded).unwrap();

        // Verify it matches
        assert_eq!(data, decoded);
        
        // Print what the encoded data looks like
        println!("Normal encoded: {:?}", encoded);
    }

    #[test]
    fn test_custom_serialization() {
        // Create data with custom ULID type - this will validate during creation
        let data = CustomData {
            id: ULID::new("01ARZ3NDEKTSV4RRFFQ69G5FAV".to_string()).unwrap(),
            name: "test".to_string(),
        };

        // Encode and decode - validation happens again during decode
        let encoded = encode_doc_data(&data).unwrap();
        let decoded: CustomData = decode_doc_data(&encoded).unwrap();

        // Verify it matches
        assert_eq!(data, decoded);
        
        // Print what the encoded data looks like
        println!("Custom encoded: {:?}", encoded);
    }
    
    #[test]
    fn test_ulid_validation() {
        // Valid ULID
        assert!(ULID::new("01ARZ3NDEKTSV4RRFFQ69G5FAV".to_string()).is_ok());
        
        // Invalid cases
        assert!(ULID::new("01ARZ3NDEKTSV4RRFFQ69G5FA".to_string()).is_err()); // Too short
        assert!(ULID::new("01ARZ3NDEKTSV4RRFFQ69G5FAVX".to_string()).is_err()); // Too long
        assert!(ULID::new("01ARZ3NDEKTSV4RRFFQ69G5FAv".to_string()).is_err()); // Lowercase
        assert!(ULID::new("01ARZ3NDEKTSV4RRFFQ69G5FAO".to_string()).is_err()); // Invalid char 'O'
        assert!(ULID::new("01ARZ3NDEKTSV4RRFFQ69G5FAI".to_string()).is_err()); // Invalid char 'I'
        assert!(ULID::new("01ARZ3NDEKTSV4RRFFQ69G5FAL".to_string()).is_err()); // Invalid char 'L'
    }
    
    #[test]
    fn test_invalid_timestamp() {
        // Valid format but invalid timestamp (future date far beyond reason)
        assert!(ULID::new("ZZZZZZZZZZZZZZZZZZZZZZZZZZ".to_string()).is_err()); // Far future timestamp
        
        // Invalid timestamp component
        let result = ULID::new("ZZZZZZZZZZZZZZZZZZZZZZZZZZ".to_string());
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.contains("timestamp"), "Error should mention invalid timestamp: {}", err);
        }
    }
    
    #[test]
    fn test_extract_timestamp() {
        // Known ULID with timestamp July 2016 (example from ULID spec)
        let ulid = "01ARZ3NDEKTSV4RRFFQ69G5FAV";
        let timestamp = extract_timestamp_ms(ulid).unwrap();
        
        // Should be around July 2016 (1467839849013), but we don't know exact value
        assert!(timestamp > 1400000000000, "Timestamp should be after 2014"); 
        assert!(timestamp < 1500000000000, "Timestamp should be before 2017");
        
        // Test extracting timestamp from a full ULID
        let ulid = ULID::new("01ARZ3NDEKTSV4RRFFQ69G5FAV".to_string()).unwrap();
        let timestamp = ulid.timestamp_ms().unwrap();
        assert!(timestamp > 1400000000000, "Timestamp should be after 2014");
        assert!(timestamp < 1500000000000, "Timestamp should be before 2017");
    }
    
    #[test]
    fn test_invalid_deserialization() {
        // Create JSON with invalid ULID
        let json = r#"{"id":"0123456789","name":"test"}"#;
        
        // This should fail because the ULID is invalid
        let result: Result<CustomData, _> = serde_json::from_str(json);
        assert!(result.is_err());
        
        // This is where validation happens during deserialization
        println!("Deserialization error: {:?}", result.err());
    }
} 