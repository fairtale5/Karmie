# Store Function Updates Required

## Overview
Update all document operations to use proper store functions with correct Principal ID handling.

## Required Changes

### 1. Function Replacements
- Replace `list_docs` → `list_docs_store`
- Replace `get_doc` → `get_doc_store`
- Replace `set_doc` → `set_doc_store`
- Replace `delete_doc` → `delete_doc_store`

### 2. Principal ID Guidelines

#### Use `ic_cdk::id()` (Canister Principal) ONLY for:
- Reputation collection operations
- Other controller-only collections
- System-level operations where users shouldn't have direct access

Example:
```rust
// Correct: Reputation calculations are system-level
list_docs_store(
    ic_cdk::id(),
    String::from("reputations"),
    params
)
```

#### Use `ic_cdk::caller()` (User Principal) for:
- User-owned document operations (votes, profiles, etc)
- Operations that should maintain user ownership
- Any operation where user attribution matters

Example:
```rust
// Correct: Votes should be attributed to users
list_docs_store(
    ic_cdk::caller(),
    String::from("votes"),
    params
)
```

### 3. Files Needing Updates

- [ ] `src/satellite/src/utils/reputation_calculations.rs`
  - Update reputation document operations to use `ic_cdk::id()`
  - Update vote queries to use `ic_cdk::caller()`

- [ ] `src/satellite/src/utils/vote_calculations.rs`
  - Update all vote operations to use `ic_cdk::caller()`

- [ ] `src/satellite/src/utils/tag_calculations.rs`
  - Update tag queries based on access level

### 4. Testing Requirements

- [ ] Verify proper ownership attribution
- [ ] Test access control restrictions
- [ ] Validate version handling still works
- [ ] Check description field handling

## Implementation Notes

1. **Access Control**
   - Document operations should respect collection permissions
   - Use appropriate Principal ID based on operation context
   - Consider future permission changes

2. **Version Handling**
   - Maintain proper version increments
   - Handle version conflicts appropriately
   - Test concurrent modifications

3. **Error Handling**
   - Add proper error messages for permission issues
   - Log Principal ID conflicts
   - Handle store function failures

4. **Documentation**
   - Update function documentation to reflect store usage
   - Document Principal ID requirements
   - Add examples of proper store function usage

## Migration Strategy

1. Update one collection at a time
2. Start with reputation calculations (most critical)
3. Update vote operations next
4. Finally update tag and user operations
5. Test thoroughly between each phase

## Related Documentation
- See `docs/core/architecture/database.md` for Principal ID guidelines
- Check `docs/juno/docs/build/storage/development.md` for store function details 