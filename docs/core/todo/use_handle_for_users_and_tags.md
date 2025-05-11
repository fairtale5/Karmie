# Migration Plan: Use `handle` for Users and Tags

## Purpose and Motivation

To improve consistency and clarity across the Reputator codebase, we propose renaming all user `username` fields and tag `name` fields to `handle`. This change will:
- Unify terminology for user and tag identifiers
- Simplify validation and key generation logic
- Reduce confusion in queries and UI
- Align with best practices for unique, user-facing identifiers

## Scope of Change

This migration affects all places where:
- Users have a `username` field (should become `handle`)
- Tags have a `name` field (should become `handle`)
- Any code, validation, or documentation references these fields

## Affected Files and Locations

### Frontend
- `src/lib/types.ts`  
  - `UserData.username` → `UserData.handle`
  - `TagData.name` → `TagData.handle`
- All usages of `username` and tag `name` in Svelte components, stores, and API calls
- All set/get doc logic referencing these fields

### Backend
- `src/satellite/src/utils/structs.rs`  
  - `UserData.username` → `UserData.handle`
  - `TagData.name` → `TagData.handle`
- `src/satellite/src/validation/validate_handle.rs`  
  - Ensure validation logic and error messages use `handle`
- `src/satellite/src/assert_set_doc/assert_doc_user.rs`  
  - All references to `username` → `handle`
- `src/satellite/src/assert_set_doc/assert_doc_tag.rs`  
  - All references to tag `name` → `handle`
- Key generation and query logic that uses these fields

### Documentation
- `docs/core/architecture/database.md`  
  - Update all schema definitions, examples, and validation rules to use `handle` for both users and tags
- Any other docs referencing `username` or tag `name`

## Analysis: Symbol Rename vs. Deeper Change

### Simple Symbol Rename
- Type/interface field names in TypeScript and Rust
- Variable names in code
- Most UI labels and form fields

### Deeper Changes Required
- **Validation Logic:** Ensure all validation (e.g., `validate_handle`) is generic and applies to both users and tags. Update error messages and docs accordingly.
- **Key Generation:** Update key generation functions to use `handle` instead of `username`/`name`.
- **Query Logic:** Update all queries that use `username` or tag `name` as a segment to use `handle`.
- **API Compatibility:** Ensure all frontend-backend communication uses the new field names.
- **UI/UX:** Update all forms, labels, and error messages to refer to `handle`.
- **Documentation:** Update all schema diagrams, field descriptions, and examples.
- **Testing:** Update and add tests for all affected logic.

### Migration/Compatibility Concerns
- **Existing Data:** If any data exists with the old field names, a migration script or manual update may be required.
- **API Consumers:** If any external clients use the old field names, coordinate updates.

## Migration Checklist

1. **Update Type Definitions**
   - [ ] Rename `username` to `handle` in `UserData` (TS/Rust)
   - [ ] Rename tag `name` to `handle` in `TagData` (TS/Rust)
2. **Update Validation Logic**
   - [ ] Ensure `validate_handle` is generic and used for both users and tags
   - [ ] Update error messages and docs
3. **Update Key Generation and Queries**
   - [ ] Update all key generation functions to use `handle`
   - [ ] Update all queries to use `handle` segment
4. **Update Backend Logic**
   - [ ] Update all backend logic, including assert/set functions, to use `handle`
5. **Update Frontend Logic**
   - [ ] Update all Svelte components, stores, and API calls
   - [ ] Update all forms and UI labels
6. **Update Documentation**
   - [ ] Update `docs/core/architecture/database.md` and all related docs
7. **Test Thoroughly**
   - [ ] Add/update tests for all affected logic
   - [ ] Test user and tag creation, editing, and querying
8. **Data Migration (if needed)**
   - [ ] Write migration script or manual steps to update existing data
   - [ ] Validate migrated data
9. **Coordinate with API Consumers**
   - [ ] Notify and support any external clients

## Risks and Edge Cases

- **Data Migration:** Existing documents with `username` or tag `name` fields will not be recognized after the change unless migrated.
- **Validation:** If validation logic is not fully updated, invalid handles may be accepted or valid ones rejected.
- **Key Generation:** Inconsistent key generation could break queries or document references.
- **API Compatibility:** Out-of-sync frontend/backend or external clients may break.
- **Testing:** Insufficient testing could miss regressions in user/tag creation or querying.

## References
- [Database Schema: Users](../architecture/database.md#users-collection)
- [Database Schema: Tags](../architecture/database.md#tags-collection)
- [Validation Logic](../architecture/database.md#validation-rules)
- [Key Generation](../architecture/database.md#key-generation)
- [Frontend Types](../../../src/lib/types.ts)
- [Backend Structs](../../../src/satellite/src/utils/structs.rs)
- [Handle Validation](../../../src/satellite/src/validation/validate_handle.rs)
- [User Assert Logic](../../../src/satellite/src/assert_set_doc/assert_doc_user.rs)
- [Tag Assert Logic](../../../src/satellite/src/assert_set_doc/assert_doc_tag.rs)

---

**Open Questions:**
- Is there any existing data that needs migration?
- Are there any external API consumers to coordinate with?
- Should the field be called `handle` everywhere, or are there exceptions?

**Next Steps:**
- Review this plan and confirm scope.
- Proceed with incremental implementation and testing. 