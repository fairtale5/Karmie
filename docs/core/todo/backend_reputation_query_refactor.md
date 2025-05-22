# Reputation Query Refactoring

## Overview
Refactor reputation data retrieval to use the standardized `query_doc_by_key` helper instead of direct `get_doc` calls. This change will improve memory efficiency and maintain consistency with our document query patterns.

## Files to Change
1. `src/satellite/src/core/reputation_calculations.rs`
   - Refactor `calculate_user_reputation` to use `query_doc_by_key`
   - Update error handling to match `ListResults` pattern
   - Ensure proper handling of multiple results (should be only one)

## Current vs Proposed Structure

### Current
```rust
// Direct document access
let existing_doc = junobuild_satellite::get_doc(
    String::from("reputations"),
    reputation_key.clone()
);
```

### Proposed
```rust
// Use standardized query helper
let results = query_doc_by_key("reputations", &reputation_key)?;
let doc = results.items.first()
    .ok_or_else(|| "No reputation document found".to_string())?;
```

## Key Changes Required
1. Update return type handling from `Option<Doc>` to `Result<ListResults<Doc>, String>`
2. Handle the `ListResults` structure and extract first matching document
3. Update error handling to use `Result` pattern consistently
4. Ensure proper logging of query results

## Benefits
1. Consistent with our document query patterns
2. More memory efficient (uses Juno's key-based queries)
3. Standardized error handling and logging
4. Better maintainability

## Potential Risks
1. Need to handle multiple results (should be only one)
2. Error handling pattern changes
3. May need to update tests if they exist

## Testing Required
1. Verify reputation calculations still work correctly
2. Check error handling in all cases
3. Verify logging is consistent
4. Test with non-existent reputation documents

## Related Documentation
- See `docs/core/architecture/database.md` for query patterns
- See `src/processors/document_queries.rs` for query helper implementation 