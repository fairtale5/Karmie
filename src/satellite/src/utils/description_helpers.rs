use candid::Principal;
use crate::utils::structs::{User, UserData, Tag, TagData, Vote, VoteData, Reputation, ReputationData};
use junobuild_satellite::get_doc;
use regex::Regex;
use lazy_static::lazy_static;

/// Helper struct for creating and parsing document descriptions
/// This follows the new format standard defined in docs/core/architecture/database.md
pub struct DocumentDescription {
    fields: Vec<(String, String)>,
}

impl DocumentDescription {
    /// Creates a new empty description
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
        }
    }

    /// Adds a field to the description
    pub fn add_field(&mut self, name: &str, value: &str) -> &mut Self {
        self.fields.push((name.to_string(), value.to_string()));
        self
    }

    /// Adds an owner field, handling both key and Principal formats
    pub fn add_owner(&mut self, value: &str) -> &mut Self {
        self.add_field("owner", value)
    }

    /// Sanitizes a key by removing special characters that could cause regex issues
    /// Only allows alphanumeric characters and underscores
    /// For example: "user-123" becomes "user123", "tag_456" remains "tag_456"
    /// 
    /// This function:
    /// 1. Removes all characters except a-z, A-Z, 0-9, and underscore
    /// 2. Returns the sanitized key or the original key if it's already sanitized
    /// 3. Ensures we always have a valid key, even if the input is empty or all special chars
    pub fn sanitize_key(key: &str) -> String {
        // If the key is empty, return a default value
        if key.is_empty() {
            return String::from("key");
        }
        
        // Filter out non-alphanumeric and non-underscore characters
        let sanitized: String = key.chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        
        // If sanitizing removed all characters, return a default value
        if sanitized.is_empty() {
            return String::from("key");
        }
        
        // Ensure the key doesn't start with a number (which could cause issues in some systems)
        // If it does, prefix it with 'k'
        if sanitized.chars().next().unwrap().is_numeric() {
            return format!("k{}", sanitized);
        }
        
        sanitized
    }

    /// Takes a string and makes it safe to use in a regex pattern by adding backslashes
    /// before special characters. For example: "user-123" becomes "user\-123"
    pub fn escape_regex_chars(value: &str) -> String {
        // Characters that have special meaning in regex patterns (like - means "range" in [a-z])
        let special_chars = ['.', '*', '+', '?', '^', '$', '[', ']', '(', ')', '{', '}', '|', '\\', '-'];
        
        // Pre-allocate string to avoid multiple memory allocations
        let mut escaped = String::with_capacity(value.len() * 2);
        
        // For each character, if it's special, add a backslash before it
        for c in value.chars() {
            if special_chars.contains(&c) {
                escaped.push('\\');  // Add backslash before special character
            }
            escaped.push(c);        // Add the character itself
        }
        escaped
    }

    /// Creates a regex pattern that can safely match our description format
    /// Example: "owner=user-123;" becomes "owner=user\-123;"
    /// NOTE: This is for backward compatibility only - new code should use the build() method
    pub fn build_matcher_pattern(&self) -> String {
        let mut result = String::new();
        
        // Build pattern for each field (like owner:value, tag:value)
        for (i, (name, value)) in self.fields.iter().enumerate() {
            // Escape the value so regex special characters are treated as normal text
            let escaped_value = Self::escape_regex_chars(value);
            
            // Add escaped brackets and field info
            result.push_str(&format!("\\[{}:{}\\]", name, escaped_value));
            
            // Add comma between fields (but not after the last one)
            if i < self.fields.len() - 1 {
                result.push(',');
            }
        }
        result
    }

    /// Builds the description string in the new format: field1=value1;field2=value2;
    /// This format avoids special regex characters like brackets and is easier to query
    pub fn build(&self) -> String {
        let mut result = String::new();
        for (name, value) in &self.fields {
            // Sanitize the value to remove any special characters
            let sanitized_value = Self::sanitize_key(value);
            result.push_str(&format!("{}={};", name, sanitized_value));
        }
        result
    }

    /// Builds the description in the old format: [field1:value1],[field2:value2]
    /// NOTE: This is for backward compatibility only - new code should use the build() method
    pub fn build_old_format(&self) -> String {
        let mut result = String::new();
        for (i, (name, value)) in self.fields.iter().enumerate() {
            result.push_str(&format!("[{}:{}]", name, value));
            if i < self.fields.len() - 1 {
                result.push(',');
            }
        }
        result
    }

    /// Parses a description string in the new format: field1=value1;field2=value2;
    pub fn parse(description: &str) -> Result<Self, String> {
        let mut fields = Vec::new();
        
        // Split by semicolons to get each field=value pair
        for pair in description.split(';') {
            if pair.is_empty() {
                continue;
            }
            
            // Split by = to get the field name and value
            if let Some(separator_pos) = pair.find('=') {
                let name = &pair[..separator_pos];
                let value = &pair[separator_pos + 1..];
                if !name.is_empty() {
                    fields.push((name.to_string(), value.to_string()));
                }
            }
        }

        Ok(Self { fields })
    }

    /// Gets a field value by name
    pub fn get_field(&self, name: &str) -> Option<&str> {
        self.fields.iter()
            .find(|(field_name, _)| field_name == name)
            .map(|(_, value)| value.as_str())
    }

    /// Gets the owner value, which could be either a key or Principal
    pub fn get_owner(&self) -> Option<&str> {
        self.get_field("owner")
    }
}

/// Helper functions for user document descriptions
/// Creates a document description for a user document
/// 
/// # Document Structure
/// ```
/// key: String,         // Document's unique identifier
/// data: {
///     username: String,  // User's username
///     display_name: String
/// }
/// ```
/// 
/// # Description Format
/// New format: owner=sanitized_key;username=sanitized_username;
pub fn create_user_description(user: &User, owner: &Principal, is_playground: bool) -> String {
    let mut desc = DocumentDescription::new();
    let owner_str = if is_playground {
        // Sanitize the key to remove special characters
        DocumentDescription::sanitize_key(&user.key)
    } else {
        // Sanitize the Principal string to remove special characters
        DocumentDescription::sanitize_key(&owner.to_string())
    };
    
    // Sanitize the username
    let sanitized_username = DocumentDescription::sanitize_key(&user.data.username);
    
    desc.add_owner(&owner_str)
        .add_field("username", &sanitized_username);
    desc.build()
}

/// Helper functions for tag document descriptions
/// Creates a document description for a tag document
/// 
/// # Document Structure
/// ```
/// key: String,           // Document's unique identifier
/// data: {
///     name: String,      // Tag's display name
///     author_key: String // Reference to Users collection
///     description: String,
///     time_periods: Vec<TimePeriod>,
///     reputation_threshold: f64,
///     vote_reward: f64,
///     min_users_for_threshold: u32
/// }
/// ```
/// 
/// # Description Format
/// New format: owner=sanitized_key;name=sanitized_name;
pub fn create_tag_description(tag: &Tag, owner: &Principal, is_playground: bool) -> String {
    let mut desc = DocumentDescription::new();
    let owner_str = if is_playground {
        // Sanitize the key to remove special characters
        DocumentDescription::sanitize_key(&tag.key)
    } else {
        // Sanitize the Principal string to remove special characters
        DocumentDescription::sanitize_key(&owner.to_string())
    };
    
    // Sanitize the tag name
    let sanitized_name = DocumentDescription::sanitize_key(&tag.data.name);
    
    desc.add_owner(&owner_str)
        .add_field("name", &sanitized_name);
    desc.build()
}

/// Helper functions for vote document descriptions
/// Creates a document description for a vote document
/// 
/// # Document Structure
/// ```
/// key: String,           // Document's unique identifier
/// data: {
///     author_key: String // Reference to Users collection
///     target_key: String // Reference to Users collection
///     tag_key: String    // Reference to Tags collection
///     value: f64,        // Vote value (+1 or -1)
///     weight: f64,       // Vote weight (0.0 to 1.0)
/// }
/// ```
/// 
/// # Description Format
/// New format: owner=sanitized_key;target=sanitized_target_key;tag=sanitized_tag_key;
pub fn create_vote_description(vote: &Vote, owner: &Principal, is_playground: bool) -> String {
    let mut desc = DocumentDescription::new();
    let owner_str = if is_playground { 
        // Sanitize the key to remove special characters
        DocumentDescription::sanitize_key(&vote.key)
    } else { 
        // Sanitize the Principal string to remove special characters
        DocumentDescription::sanitize_key(&owner.to_string())
    };
    
    // Sanitize the target and tag keys
    let sanitized_target_key = DocumentDescription::sanitize_key(&vote.data.target_key);
    let sanitized_tag_key = DocumentDescription::sanitize_key(&vote.data.tag_key);
    
    desc.add_owner(&owner_str)
        .add_field("target", &sanitized_target_key)
        .add_field("tag", &sanitized_tag_key);
    desc.build()
}

/// Helper functions for reputation document descriptions
/// Creates a document description for a reputation document
/// 
/// # Document Structure
/// ```
/// key: String,           // Document's unique identifier
/// data: {
///     user_key: String   // Reference to Users collection
///     tag_key: String    // Reference to Tags collection
///     total_basis_reputation: f64,
///     total_voting_rewards_reputation: f64,
///     last_known_effective_reputation: f64,
///     last_calculation: u64,
///     vote_weight: VoteWeight,
///     has_voting_power: bool
/// }
/// ```
/// 
/// # Description Format
/// New format: owner=sanitized_key;tag=sanitized_tag_key;
pub fn create_reputation_description(reputation: &Reputation, owner: &Principal, is_playground: bool) -> String {
    let mut desc = DocumentDescription::new();
    let owner_str = if is_playground { 
        // Sanitize the key to remove special characters
        DocumentDescription::sanitize_key(&reputation.key)
    } else { 
        // Sanitize the Principal string to remove special characters
        DocumentDescription::sanitize_key(&owner.to_string())
    };
    
    // Sanitize the tag key
    let sanitized_tag_key = DocumentDescription::sanitize_key(&reputation.data.tag_key);
    
    desc.add_owner(&owner_str)
        .add_field("tag", &sanitized_tag_key);
    desc.build()
}

lazy_static! {
    // Updated regex patterns for the new format
    static ref USER_DESC_PATTERN: Regex = Regex::new(r"owner=([^;]+);username=([^;]+);").unwrap();
    static ref TAG_DESC_PATTERN: Regex = Regex::new(r"owner=([^;]+);name=([^;]+);").unwrap();
    static ref VOTE_DESC_PATTERN: Regex = Regex::new(r"owner=([^;]+);target=([^;]+);tag=([^;]+);").unwrap();
    static ref REP_DESC_PATTERN: Regex = Regex::new(r"owner=([^;]+);tag=([^;]+);").unwrap();
}

/// Validates a description string against expected format and referenced documents
pub async fn validate_description(collection: &str, description: &str, document_key: &str) -> Result<(), String> {
    // Step 1: Validate format using regex
    let (pattern, field_names) = match collection {
        "users" => (&*USER_DESC_PATTERN, vec!["owner", "username"]),
        "tags" => (&*TAG_DESC_PATTERN, vec!["owner", "name"]),
        "votes" => (&*VOTE_DESC_PATTERN, vec!["owner", "target", "tag"]),
        "reputations" => (&*REP_DESC_PATTERN, vec!["owner", "tag"]),
        _ => return Err(format!("Unknown collection: {}", collection))
    };

    let captures = pattern.captures(description)
        .ok_or_else(|| format!("Invalid description format for {}: {}", collection, description))?;

    // Step 2: Extract and validate referenced documents
    for (i, field_name) in field_names.iter().enumerate() {
        let field_value = captures.get(i + 1)
            .ok_or_else(|| format!("Missing {} field in description", field_name))?
            .as_str();

        // Skip validation for non-reference fields (like username and name)
        if *field_name == "username" || *field_name == "name" {
            continue;
        }

        // For owner field in user documents, it must match the document key
        if collection == "users" && *field_name == "owner" && field_value != document_key {
            return Err(format!(
                "Owner field must match document key. Expected: {}, Got: {}",
                document_key, field_value
            ));
        }

        // For other reference fields, verify the referenced document exists
        // This section validates that any referenced documents (users, tags) actually exist
        // in their respective collections before allowing the reference to be created.
        let referenced_collection = match *field_name {
            "owner" | "target" => "users",  // References to users (either as owner or target)
            "tag" => "tags",               // References to tag documents
            _ => continue                  // Skip validation for non-reference fields
        };

        // Verify the referenced document exists in its collection
        // Parameters for get_doc:
        // 1. collection_name: The collection where we expect to find the document
        // 2. document_key: The unique identifier of the document we're looking for
        // Example: For a tag reference, we look for the tag_id in the "tags" collection
        if let None = junobuild_satellite::get_doc(
            referenced_collection.to_string(),  // Collection name first
            field_value.to_string(),            // Document key second
        ) {
            return Err(format!(
                "Referenced {} document not found: {}",
                referenced_collection, field_value
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn test_description_builder() {
        let mut desc = DocumentDescription::new();
        desc.add_owner("user_123")
            .add_field("tag", "tag_456");
        
        assert_eq!(desc.build(), "owner=user_123;tag=tag_456;");
    }

    #[test]
    fn test_description_builder_playground() {
        let mut desc = DocumentDescription::new();
        desc.add_owner("user_123")
            .add_field("tag", "tag_456");
        
        assert_eq!(desc.build(), "owner=user_123;tag=tag_456;");
    }

    #[test]
    fn test_description_builder_production() {
        let mut desc = DocumentDescription::new();
        desc.add_owner("2vxsx-fae")
            .add_field("tag", "tag_456");
        
        assert_eq!(desc.build(), "owner=k2vxsxfae;tag=tag_456;");
    }

    #[test]
    fn test_description_parser() {
        // Test playground format
        let desc = DocumentDescription::parse("owner=user_123;tag=tag_456").unwrap();
        assert_eq!(desc.get_owner(), Some("user_123"));
        assert_eq!(desc.get_field("tag"), Some("tag_456"));
        
        // Test production format
        let desc = DocumentDescription::parse("owner=k2vxsxfae;tag=tag_456").unwrap();
        assert_eq!(desc.get_owner(), Some("k2vxsxfae"));
        assert_eq!(desc.get_field("tag"), Some("tag_456"));
    }

    #[test]
    fn test_user_description_creator() {
        // Create a test user
        let user = User {
            key: "user_123".to_string(),
            data: UserData {
                username: "john_doe".to_string(),
                display_name: "John Doe".to_string()
            },
            description: None,
            owner: None,
            created_at: None,
            updated_at: None,
            version: None
        };
        
        // Create a test owner
        let owner = Principal::from_text("2vxsx-fae").unwrap();
        
        // Test playground format - should use document key
        let playground_desc = create_user_description(&user, &owner, true);
        assert!(playground_desc.contains("owner=user_123"));  // Check for document key
        
        // Test production format - should use Principal ID
        let production_desc = create_user_description(&user, &owner, false);
        assert!(production_desc.contains("owner=k2vxsxfae")); // Check for Principal ID
    }

    #[test]
    fn test_tag_description_creator() {
        // Create a test tag
        let tag = Tag {
            key: "tag_123".to_string(),
            data: TagData {
                author_key: "user_456".to_string(),
                name: "test_tag".to_string(),
                description: "Test tag description".to_string(),
                time_periods: vec![],
                reputation_threshold: 10.0,
                vote_reward: 0.1,
                min_users_for_threshold: 5
            },
            description: None,
            owner: None,
            created_at: None,
            updated_at: None,
            version: None
        };
        
        // Create a test owner
        let owner = Principal::from_text("2vxsx-fae").unwrap();
        
        // Test playground format - should use document key
        let playground_desc = create_tag_description(&tag, &owner, true);
        assert!(playground_desc.contains("owner=tag_123"));  // Check for document key
        assert!(playground_desc.contains("name=test_tag")); // Check for tag name
        
        // Test production format - should use Principal ID
        let production_desc = create_tag_description(&tag, &owner, false);
        assert!(production_desc.contains("owner=k2vxsxfae")); // Check for Principal ID
        assert!(production_desc.contains("name=test_tag")); // Check for tag name
    }

    #[test]
    fn test_vote_description_creator() {
        // Create a test vote
        let vote = Vote {
            key: "vote_123".to_string(),
            data: VoteData {
                author_key: "user_456".to_string(),
                target_key: "target_789".to_string(),
                tag_key: "tag_123".to_string(),
                value: 1.0,
                weight: 1.0
            },
            description: None,
            owner: None,
            created_at: None,
            updated_at: None,
            version: None
        };
        
        // Create a test owner
        let owner = Principal::from_text("2vxsx-fae").unwrap();
        
        // Test playground format - should use document key
        let playground_desc = create_vote_description(&vote, &owner, true);
        assert!(playground_desc.contains("owner=vote_123"));   // Check for document key
        assert!(playground_desc.contains("target=target_789")); // Check for target key
        assert!(playground_desc.contains("tag=tag_123"));      // Check for tag key
        
        // Test production format - should use Principal ID
        let production_desc = create_vote_description(&vote, &owner, false);
        assert!(production_desc.contains("owner=k2vxsxfae"));  // Check for Principal ID
        assert!(production_desc.contains("target=target_789")); // Check for target key
        assert!(production_desc.contains("tag=tag_123"));      // Check for tag key
    }

    #[test]
    fn test_reputation_description_creator() {
        // Create a test reputation
        let reputation = Reputation {
            key: "rep_123".to_string(),
            data: ReputationData {
                user_key: "user_456".to_string(),
                tag_key: "tag_789".to_string(),
                total_basis_reputation: 10.0,
                total_voting_rewards_reputation: 5.0,
                last_known_effective_reputation: 15.0,
                last_calculation: 0,
                vote_weight: VoteWeight::new(0.5).unwrap(),
                has_voting_power: true
            },
            description: None,
            owner: None,
            created_at: None,
            updated_at: None,
            version: None
        };
        
        // Create a test owner
        let owner = Principal::from_text("2vxsx-fae").unwrap();
        
        // Test playground format - should use document key
        let playground_desc = create_reputation_description(&reputation, &owner, true);
        assert!(playground_desc.contains("owner=rep_123"));  // Check for document key
        assert!(playground_desc.contains("tag=tag_789"));    // Check for tag key
        
        // Test production format - should use Principal ID
        let production_desc = create_reputation_description(&reputation, &owner, false);
        assert!(production_desc.contains("owner=k2vxsxfae")); // Check for Principal ID
        assert!(production_desc.contains("tag=tag_789"));     // Check for tag key
    }

    #[test]
    fn test_regex_patterns() {
        // Test user description pattern
        assert!(USER_DESC_PATTERN.is_match("owner=user_123;username=john_doe;"));
        assert!(!USER_DESC_PATTERN.is_match("owner=user_123;tag=tag_456;")); // Wrong field name
        
        // Test tag description pattern
        assert!(TAG_DESC_PATTERN.is_match("owner=user_123;name=tech_skills;"));
        assert!(!TAG_DESC_PATTERN.is_match("owner=user_123;username=john_doe;")); // Wrong field name
        
        // Test vote description pattern
        assert!(VOTE_DESC_PATTERN.is_match("owner=user_123;target=user_456;tag=tag_789;"));
        assert!(!VOTE_DESC_PATTERN.is_match("owner=user_123;target=user_456;")); // Missing tag field
        
        // Test reputation description pattern
        assert!(REP_DESC_PATTERN.is_match("owner=user_123;tag=tag_789;"));
        assert!(!REP_DESC_PATTERN.is_match("owner=user_123;field=value;")); // Wrong field name
    }

    #[test]
    fn test_sanitize_key() {
        // Normal key with only alphanumeric chars
        assert_eq!(DocumentDescription::sanitize_key("user123"), "user123");
        
        // Key with dashes (common in nanoid)
        assert_eq!(DocumentDescription::sanitize_key("user-123-abc"), "user123abc");
        
        // Principal ID style key
        assert_eq!(DocumentDescription::sanitize_key("2vxsx-fae"), "2vxsxfae");
        
        // Key with various special characters
        assert_eq!(
            DocumentDescription::sanitize_key("user!@#$%^&*()_+{}:\"<>?-=[];',./"),
            "user_"
        );
        
        // Empty key
        assert_eq!(DocumentDescription::sanitize_key(""), "key");
        
        // Key with only special characters
        assert_eq!(DocumentDescription::sanitize_key("!@#$%^&*()"), "key");
        
        // Key starting with number
        assert_eq!(DocumentDescription::sanitize_key("123abc"), "k123abc");
        
        // Key with underscores (allowed)
        assert_eq!(DocumentDescription::sanitize_key("user_name_123"), "user_name_123");
    }
    
    #[test]
    fn test_description_format_with_sanitized_keys() {
        // Test with normal keys
        let mut desc = DocumentDescription::new();
        desc.add_owner("user123")
            .add_field("tag", "tag456");
        assert_eq!(desc.build(), "owner=user123;tag=tag456;");
        
        // Test with keys containing special chars
        let mut desc = DocumentDescription::new();
        desc.add_owner("user-123-abc")
            .add_field("tag", "tag-456-def");
        assert_eq!(desc.build(), "owner=user123abc;tag=tag456def;");
        
        // Test with Principal ID style key
        let mut desc = DocumentDescription::new();
        desc.add_owner("2vxsx-fae")
            .add_field("tag", "tag_456");
        assert_eq!(desc.build(), "owner=k2vxsxfae;tag=tag_456;");
    }
} 