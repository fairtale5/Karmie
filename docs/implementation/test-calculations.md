# Reputator Test Phase Development Plan

## Phase 1: Basic Interface
- [ ] User creation interface
- [ ] Voting interface
- [ ] User list with deletion
- [ ] Vote list with deletion

## Phase 2: Reputation System
- [ ] Implement reputation calculation functions
  - [ ] Basic reputation score
  - [ ] Vote weight calculation
  - [ ] Total reputation impact
- [ ] Display reputation scores in UI
  - [ ] Show user reputation scores
  - [ ] Show vote weights
  - [ ] Show total reputation impact

## Data Structures

### User Document
```typescript
interface User {
    key: string;      // Unique identifier
    name: string;     // Display name
    created_at: bigint;
    updated_at: bigint;
    owner: Principal;
}
```

### Vote Document
```typescript
interface Vote {
    key: string;      // Unique identifier
    author: string;   // User key who created the vote
    target: string;   // User key being voted on
    positive: boolean; // true = upvote, false = downvote
    created_at: bigint;
    updated_at: bigint;
    owner: Principal;
}
```

## Test Cases
1. Create multiple users
2. Create votes between users
3. Verify vote recording
4. Test deletion functionality
5. Calculate and verify reputation scores

## Next Steps
1. Implement reputation calculation
2. Add reputation display to UI
3. Add vote weight visualization
4. Show total reputation impact 