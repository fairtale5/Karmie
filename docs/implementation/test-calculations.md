# Reputator Test Phase Development Plan

## Phase 1: Basic Interface ✅
- [x] User creation interface
  - Create new users with handle and display name
  - Validate unique usernames
  - Handle validation and normalization
- [x] Voting interface
  - Cast votes on users with specific tags
  - Support for positive/negative votes
- [x] User list with deletion
  - Display all users in a table
  - Delete individual users
  - Show user details
- [x] Vote list with deletion
  - List all votes in the system
  - Show vote details (author, target, tag)
  - Delete individual votes

## Phase 2: Reputation System 🚧

### Reputation Calculation Approach

#### 1. On-Demand Calculation
Reputation is calculated per-tag when:
- User logs in (for all their tags)
- Someone views user's profile (for specific tag)
- Someone votes on the user (for specific tag)
- User casts a vote (for specific tag)

Key Features:
- Tag-specific recalculation
- Other tags remain untouched
- Only relevant votes are processed
- Efficient querying by tag

Example Implementation:
```typescript
async function getUserReputation(userKey: string, tagKey: string): Promise<number> {
    // 1. Check if we need to recalculate
    const reputation = await getReputationDoc(userKey, tagKey);
    const currentMonth = getCurrentYearMonth();
    
    if (!reputation || reputation.calculation_month !== currentMonth) {
        // 2. Get only votes for this specific tag
        const votes = await listDocs({
            collection: "votes",
            filter: {
                matcher: {
                    description: `target:${userKey},tag:${tagKey}`
                }
            }
        });
        
        // 3. Calculate reputation for this tag only
        return await recalculateReputation(userKey, tagKey, votes);
    }
    
    return reputation.reputation_score;
}
```

#### 2. Time-Based Decay
Conservative multipliers for vote weighting (customizable per tag):
```typescript
const CONSERVATIVE_MULTIPLIERS = {
    current_month: 2.0,    // Last 30 days
    last_3_months: 1.5,    // 31-90 days
    next_6_months: 1.3,    // 91-180 days
    next_18_months: 1.1,   // 181-540 days
    older: 1.0            // 540+ days
};
```

Example Calculation (10 votes per period):
```
Current Month (2x):
- 10 votes * 2 = 20 weighted votes

Last 3 Months (1.5x):
- 10 votes * 1.5 = 15 weighted votes

Next 6 Months (1.3x):
- 10 votes * 1.3 = 13 weighted votes

Next 18 Months (1.1x):
- 10 votes * 1.1 = 11 weighted votes

27+ Months (1x):
- 10 votes * 1 = 10 weighted votes

Total Weighted Votes = 69
Power per Vote = 1000 / 69 ≈ 14.49

Final Distribution:
Current Month: 20 * 14.49 = 289.8 points (29%)
Last 3 Months: 15 * 14.49 = 217.4 points (22%)
Next 6 Months: 13 * 14.49 = 188.4 points (19%)
Next 18 Months: 11 * 14.49 = 159.4 points (16%)
27+ Months: 10 * 14.49 = 144.9 points (14%)
```

Benefits of this approach:
- Balanced distribution of voting power
- Meaningful differences between periods
- Maintains "recent votes matter more" principle
- Prevents reputation inflation
- Easy to understand and explain
- Forgiving for new users while still valuing experience

#### 3. Vote Weight Distribution
For a user with reputation R and N votes:
- Base vote weight = R / N
- Applied decay based on vote age
- Normalized to ensure total influence = R

#### 4. Caching Strategy
```typescript
interface ReputationCache {
    score: number;           // Cached reputation score
    last_calculation: bigint; // When we last calculated
    calculation_month: string; // "YYYY-MM" format
}
```

### Implementation Steps

1. **Add Reputation Display**
- [ ] Add reputation column to users list
- [ ] Show per-tag reputation scores
- [ ] Add reputation history view

2. **Implement Calculation Functions**
- [ ] Basic reputation aggregation
- [ ] Time-based decay application
- [ ] Vote weight normalization
- [ ] Caching mechanism

3. **Add Weight Visualization**
- [ ] Show vote weights in UI
- [ ] Display decay factors
- [ ] Visualize reputation changes

### Data Structures

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

### New Test Cases

6. Reputation Calculation Tests
   - Create users with various voting patterns
   - Verify decay calculations
   - Test weight distribution
   - Validate caching behavior

7. Performance Tests
   - Test with 1000+ votes
   - Measure calculation time
   - Monitor memory usage
   - Verify cache effectiveness

## Next Steps
1. Implement reputation calculation
2. Add reputation display to UI
3. Add vote weight visualization
4. Show total reputation impact 

## Implementation Notes

### Calculation Optimization
- Cache reputation scores
- Calculate only when needed
- Store intermediate results
- Use efficient queries

### Scalability Considerations
1. **Database Impact**
   - Document size < 2MB limit
   - Description field < 1024 chars
   - Efficient indexing via description field

2. **Computation Efficiency**
   - O(n) for vote calculations
   - Cached results reduce load
   - On-demand updates limit overhead

3. **Memory Usage**
   - Cached scores in ReputationDocument
   - Grouped votes by period
   - Minimal duplicate data

### Future Improvements
1. **Custom Decay Rules**
   - Per-tag time brackets
   - Adjustable weights
   - Community-specific rules

2. **Performance Optimizations**
   - Background calculation jobs
   - Partial updates
   - Progressive loading

3. **UI Enhancements**
   - Reputation graphs
   - Weight visualizations
   - History tracking 