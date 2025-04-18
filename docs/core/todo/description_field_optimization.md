# Document Key Optimization Plan

## Main Problem

Currently, we query documents using the description field, which requires loading the entire table into memory first. This is not scalable as the dataset grows.

The key field is the only field that can be queried without loading the table into memory, making it much more efficient for queries. We need to move our query patterns from description-based to key-based.

## Solution

### 1. Reputation Documents

**Current (Problem):**
- Uses description field for queries
- Allows duplicate reputation documents
- No standardized key format

**New Solution:**
- Key format: `USR={uuid}_TAG={uuid}`
- Example: `USR=67e5504410b1426f9247bb680e5fe0c8_TAG=b068e9ee1422edf7878440ab8b6`
- Benefits:
  - Can query without loading table into memory
  - Prevents duplicate reputation documents (same user+tag combination)
  - Standardized UUID format (Simple format - no hyphens)
  - Easy validation using UUID library

### 2. Vote Documents

**Current (Problem):**
- Uses description field for queries
- Complex query patterns
- No standardized key format

**New Solution:**
- Key format: `USR_{uuid}_TAG_{uuid}_TAR_{uuid}_KEY_{uuid}`
- Example: `USR_67e5504410b1426f9247bb680e5fe0c8_TAG_b068e9ee1422edf7878440ab8b6_TAR_c168e9ee1422edf7878440ab8c7_KEY_d268e9ee1422edf7878440ab8c8`
- Query patterns:
  - Find votes by user in tag: Search for `USR_{uuid}_TAG_{uuid}`
  - Find votes for target in tag: Search for `TAG_{uuid}_TAR_{uuid}`

### 3. Standard Format for All Documents

All document keys will follow these standards:
- User keys: `usr_{uuid}`
- Tag keys: `usr_{uuid}_tag_{uuid}`
- Reputation keys: `usr_{uuid}_tag_{uuid}`
- Vote keys: `usr_{uuid}_tag_{uuid}_tar_{uuid}_key_{uuid}`

Note: All UUIDs use Simple format (no hyphens) for cleaner keys and easier parsing.

## Required Updates

### Backend Changes

1. **UUID Generation**
   - Use UUID v4 for document keys:
     ```rust
     // UUID v4 is cryptographically random and suitable for distributed systems
     // - No central coordination needed (unlike v1's MAC+timestamp)
     // - No privacy concerns (v1 exposes MAC address)
     // - Collision probability negligible (2^122 unique values)
     let uuid = Uuid::new_v4();
     
     // Use Simple format (no hyphens) for cleaner keys and better performance
     // - 32 chars vs 36 chars (11% shorter)
     // - No hyphen parsing/validation needed
     // - Easier to work with in URLs and file systems
     let simple_uuid = uuid.simple().encode_lower(&mut Uuid::encode_buffer());
     // Result: "67e5504410b1426f9247bb680e5fe0c8"
     ```

   - Validation ensures UUIDs are correctly formatted:
     ```rust
     fn validate_uuid(uuid_str: &str) -> Result<(), String> {
         // Validation checks:
         // 1. Length must be exactly 32 chars (Simple format)
         // 2. All characters must be valid hex (0-9, a-f)
         // 3. Must conform to UUID v4 format:
         //    - Version bits (13th hex char) must be '4'
         //    - Variant bits (17th hex char) must be '8', '9', 'a', or 'b'
         match Uuid::parse_str(uuid_str) {
             Ok(uuid) => {
                 if uuid.get_version_num() == 4 {
                     Ok(())
                 } else {
                     Err(format!("UUID must be version 4: {}", uuid_str))
                 }
             },
             Err(_) => Err(format!("Invalid UUID format: {}", uuid_str))
         }
     }

     // Example key validation function:
     fn validate_document_key(key: &str) -> Result<(), String> {
         // Split key into parts: USR_uuid_TAG_uuid_...
         let parts: Vec<&str> = key.split('_').collect();
         
         // Validate structure (odd indices should be UUIDs)
         for i in (1..parts.len()).step_by(2) {
             validate_uuid(parts[i])?;
         }
         Ok(())
     }
     ```

   - Key format benefits:
     ```rust
     // Example key: USR_67e5504410b1426f9247bb680e5fe0c8_TAG_b068e9ee1422edf7878440ab8b6
     // Benefits:
     // 1. Prefix (USR_, TAG_) makes key type immediately identifiable
     // 2. Underscore separator is URL-safe and easy to split/parse
     // 3. Simple UUID format (32 chars) is compact yet maintains uniqueness
     // 4. Consistent format makes validation and parsing straightforward
     ```

   - Replace current key generation with UUID-based system

2. **Document Creation/Updates**
   - Update all document creation to use new key formats
   - Implement retry logic for reputation document creation (if it can't write to the database, it will 
   retry querying for the document until it finds it, get the version number, and then write to the database 
   with the correct version number)
   - Implement retry logic for reputation document creation with exponential backoff:
     - Max attempts: 3
     - Initial delay: 100ms
     - Backoff: 100ms -> 200ms -> 400ms
   - Add validation for new key formats
   - Rename `data.owner` to `data.user` in documents

3. **Query System**
   - Update all queries to use key-based search instead of description
   - Implement new query patterns for votes and reputations
   - Add validation for query parameters

### Frontend Changes

1. **UUID Implementation**
   - Add UUID library
   - Replace nanoid with UUID for all document keys
   - Update document creation to use new key formats

2. **Document Creation**
   - Update all document creation flows to use new key format
   - Add validation for UUID fields
   - Update any UI components that display/handle keys

### Migration Plan

1. **New Documents**
   - All new documents will use new key format
   - Implement validation for new format

2. **Existing Documents**
   - Create migration script for existing documents
   - Update keys to new format
   - Validate data integrity after migration

## Next Steps

1. Choose UUID library for both frontend and backend
2. Create UUID generation/validation functions
3. Update document creation flows
4. Update query patterns
5. Create migration script
6. Test performance impact
7. Document new patterns for team

## Questions to Resolve

1. Retry strategy for reputation documents:
   - How many retries? (Currently set to 3)
   - Delay between retries? (Currently exponential: 100ms -> 200ms -> 400ms)
   - Error handling after max retries?

2. Migration strategy:
   - How to handle existing documents?
   - Migration timeline?
   - Rollback plan?
