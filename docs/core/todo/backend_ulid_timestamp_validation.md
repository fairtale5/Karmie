# ULID Timestamp Validation Refactoring

## Overview
Refactor the ULID timestamp validation to use a more readable and flexible approach with `CheckULIDisNew` enum.

## Files to Change
1. `src/satellite/src/validation/ulid_timestamp_validate.rs`
   - Add `CheckULIDisNew` enum
   - Update `validate_ulid_timestamp` function
   - Update tests

2. `src/satellite/src/assert_set_doc/assert_doc_vote.rs`
   - Update to use new validation function

## Implementation Details

### New Code Structure
```rust
pub enum CheckULIDisNew {
    No,  // Only basic validation
    Yes, // Also check freshness
}

impl CheckULIDisNew {
    pub fn no() -> Self {
        Self::No
    }
    
    pub fn yes() -> Self {
        Self::Yes
    }
}

pub fn validate_ulid_timestamp(ulid_str: &str, check_freshness: CheckULIDisNew) -> Result<(), String>
```

### Usage Examples
```rust
// Basic validation only
validate_ulid_timestamp(ulid_str, CheckULIDisNew::no())?;

// Basic validation + freshness check
validate_ulid_timestamp(ulid_str, CheckULIDisNew::yes())?;
```

## Benefits
1. More readable and intuitive API
2. Clear separation between basic and freshness validation
3. Easy to understand intent with `yes()` and `no()`
4. Maintains existing functionality while improving code structure

## Testing Required
1. Update existing tests to use new API
2. Add tests for both validation modes
3. Verify error messages are still clear and helpful

## Related Documentation
- See `docs/core/validation.md` for validation patterns
- See `docs/core/ulid.md` for ULID format details 