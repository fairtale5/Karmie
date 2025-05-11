# User Document Key Format Migration

## Background

Currently, user documents in the `users` collection use a key format based on a ULID and username handle:

```
usr_{user.data.user_key}_hdl_{user.data.name}_
```
- `user.data.user_key`: Unique ULID for the user (referenced by other documents)
- `user.data.name`: Normalized username (for uniqueness queries)

**Example:**
```
usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_hdl_johndoe_
```

## Problem
- When a user logs in, Juno provides a principal (Internet Identity string), not a ULID.
- User documents are keyed by ULID, so you cannot efficiently look up a user by principal.
- Querying by the `owner` field (principal) is not efficient in Juno/NoSQL (requires loading the whole collection).

## New Approach: Key Includes Principal

**Proposed new key format:**
```
usr_{user.data.user_key}_prn_{user.owner}_hdl_{user.data.name}_
```
- `user.data.user_key`: Unique ULID for the user (for references)
- `user.owner`: Principal (Internet Identity string)
- `user.data.name`: Normalized username

**Example:**
```
usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_prn_2vxsx-fae_hdl_johndoe_
```

### Field Definitions
- `user.data.user_key`: ULID, used for references from other documents (e.g., tags, votes, reputations)
- `user.owner`: Principal, used for access control and now for key-based queries
- `user.data.name`: Normalized username, used for uniqueness checks and queries

## Query Patterns Enabled
- **By ULID:**
  - `usr_{ulid}_` → Find all docs for a given user ULID
- **By Principal:**
  - `prn_{principal}_` → Find all docs for a given principal
- **By Username:**
  - `hdl_{username}_` → Find all docs for a given username
- **Combined:**
  - Any combination of the above for more specific queries

## Tradeoffs & Considerations
- **Pros:**
  - Enables efficient key-based queries for ULID, principal, and username
  - No need for a secondary index collection
  - All lookups are fast and scalable
- **Cons:**
  - Key length increases (ULID + principal + username)
  - Principal strings can be long (but are required for access control)
  - Key contains more user-identifiable information (privacy consideration)
  - If a user ever changes principal (rare), a migration is needed

## Privacy & Key Length Notes
- The key will contain the principal and username in plaintext. If privacy is a concern, consider hashing or encoding these fields.
- Juno and most NoSQL/document stores handle long keys, but be aware of any backend/document size limits.

## Migration Steps
1. Update backend logic to generate user keys in the new format on creation/update.
2. Update all references and queries to use the new key format.
3. Migrate existing user documents to the new format (if needed).
4. Update frontend logic to use the new key format for lookups and queries.

## Summary Table
| Field                | In Key? | Purpose                        |
|----------------------|:-------:|--------------------------------|
| user.data.user_key   |   ✅    | Reference by other docs        |
| user.owner           |   ✅    | Principal, access control, query|
| user.data.name       |   ✅    | Username uniqueness/query      |

---

**This new key format enables efficient, flexible queries for user documents by ULID, principal, or username, at the cost of longer keys and more information in the key.** 