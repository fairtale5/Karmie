# Reputator Test Phase Development Plan

## Phase 1: Basic Interface ✅
- [x] User creation interface
  - Create new users with handle and display name
  - Validate unique usernames
  - Handle validation
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

#### 2. Conservative Multiplier System
Time-based multipliers for vote weighting:
```typescript
const TIME_PERIODS = [
    { months: 1, multiplier: 1.5 },    // Period 1: First month
    { months: 2, multiplier: 1.2 },    // Period 2: Months 2-3
    { months: 3, multiplier: 1.1 },    // Period 3: Months 4-6
    { months: 6, multiplier: 1.0 },    // Period 4: Months 7-12
    { months: 12, multiplier: 0.95 },  // Period 5: Months 13-24
    { months: 12, multiplier: 0.75 },  // Period 6: Months 25-36
    { months: 12, multiplier: 0.55 },  // Period 7: Months 37-48
    { months: 999, multiplier: 0.25 }  // Period 8: Months 49+ (treated as infinity)
];
```

Example Calculation (10 votes per period):
```
Period 1 (1 month, 1.5x):
- 10 votes * 1.5 = 15 weighted votes

Period 2 (2 months, 1.2x):
- 10 votes * 1.2 = 12 weighted votes

Period 3 (3 months, 1.1x):
- 10 votes * 1.1 = 11 weighted votes

Period 4 (6 months, 1.0x):
- 10 votes * 1.0 = 10 weighted votes

Period 5 (12 months, 0.95x):
- 10 votes * 0.95 = 9.5 weighted votes

Period 6 (12 months, 0.75x):
- 10 votes * 0.75 = 7.5 weighted votes

Period 7 (12 months, 0.55x):
- 10 votes * 0.55 = 5.5 weighted votes

Period 8 (999 months, 0.25x):
- 10 votes * 0.25 = 2.5 weighted votes

Total Weighted Votes = 73.5
Power per Vote = 1000 / 73.5 ≈ 13.61

Final Distribution:
Period 1: 15 * 13.61 = 204.15 points (20.4%)
Period 2: 12 * 13.61 = 163.32 points (16.3%)
Period 3: 11 * 13.61 = 149.71 points (15.0%)
Period 4: 10 * 13.61 = 136.10 points (13.6%)
Period 5: 9.5 * 13.61 = 129.30 points (12.9%)
Period 6: 7.5 * 13.61 = 102.08 points (10.2%)
Period 7: 5.5 * 13.61 = 74.86 points (7.5%)
Period 8: 2.5 * 13.61 = 34.03 points (3.4%)
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
- Applied multiplier based on vote age
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
- [ ] Time-based multiplier application
- [ ] Caching mechanism

3. **Add Weight Visualization**
- [ ] Show vote weights in UI
- [ ] Display multipliers
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
   - Verify multiplier calculations
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
1. **Custom Multiplier Rules**
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