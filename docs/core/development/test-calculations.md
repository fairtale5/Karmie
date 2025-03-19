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

### Reputation Calculation Implementation

#### 1. Vote Processing
When a vote is cast, we:
1. Check if the author has reached the threshold
2. If yes, update the target's reputation immediately
3. If no, mark the vote as inactive

```rust
// In lib.rs (Satellite backend)

#[on_set_doc(collections = ["votes"])]
async fn on_set_vote(context: OnSetDocContext) -> Result<(), String> {
    let vote = context.doc.data;
    
    // Get author's current reputation
    let author_reputation = get_reputation(&vote.author_key, &vote.tag_key).await?;
    
    // Check if author has reached threshold
    let is_active = author_reputation >= get_tag_threshold(&vote.tag_key).await?;
    
    // Update vote with active status
    update_vote_status(&context.doc.key, is_active).await?;
    
    // If vote is active, update target's reputation
    if is_active {
        update_target_reputation(&vote.target_key, &vote.tag_key).await?;
    }
    
    Ok(())
}

async fn update_target_reputation(target_key: &str, tag_key: &str) -> Result<(), String> {
    // Get all active votes for this user in this tag
    let votes = get_active_votes_for_user(target_key, tag_key).await?;
    
    // Calculate base reputation from received votes
    let base_reputation = calculate_base_reputation(&votes);
    
    // If base reputation exceeds threshold, add voting rewards
    let final_reputation = if base_reputation >= get_tag_threshold(tag_key).await? {
        // Get votes cast by this user
        let cast_votes = get_votes_cast_by_user(target_key, tag_key).await?;
        
        // Calculate voting rewards with time decay
        let voting_rewards = calculate_voting_rewards(&cast_votes);
        
        base_reputation + voting_rewards
    } else {
        base_reputation
    };
    
    // Update reputation document atomically
    update_reputation_doc(target_key, tag_key, final_reputation).await?;
    
    Ok(())
}

fn calculate_base_reputation(votes: &[Vote]) -> f64 {
    // Group votes by time period
    let votes_by_period = group_votes_by_period(votes);
    
    // Calculate weighted votes for each period
    let weighted_votes = calculate_weighted_votes(&votes_by_period);
    
    // Calculate total weighted votes
    let total_weighted = weighted_votes.iter().sum::<f64>();
    
    // Calculate points per vote (1000 total points)
    let points_per_vote = 1000.0 / total_weighted;
    
    // Calculate final reputation score
    weighted_votes.iter()
        .map(|weight| weight * points_per_vote)
        .sum::<f64>()
}

fn calculate_voting_rewards(votes: &[Vote]) -> f64 {
    // Group votes by time period
    let votes_by_period = group_votes_by_period(votes);
    
    // Calculate weighted rewards (0.1 per vote with time decay)
    votes_by_period.iter().map(|(period, votes)| {
        let period_multiplier = get_period_multiplier(period);
        votes.len() as f64 * 0.1 * period_multiplier
    }).sum()
}
```

#### 2. Challenges and Considerations

1. **Performance Impact**
   - Each vote triggers reputation recalculation
   - Could be slow for users with many votes
   - Example: A user with 1000 votes might take 2-3 seconds to recalculate
   - Solution: Consider implementing background processing for large updates
   - Alternative: Split calculation into smaller chunks

2. **Data Consistency**
   - All votes must be processed in correct order
   - Need to handle failed updates
   - Example: If a vote update fails, we need to retry or mark for manual review
   - Solution: Use `set_many_docs` for atomic operations
   - Example of atomic update:
   ```rust
   await set_many_docs({
       docs: [
           {
               collection: "votes",
               doc: { /* vote update */ }
           },
           {
               collection: "reputations",
               doc: { /* reputation update */ }
           }
       ]
   });
   ```

3. **Memory Usage**
   - Each reputation calculation loads all votes
   - Could be problematic for users with many votes
   - Example: A user with 10,000 votes might use 100MB of memory
   - Solution: Process votes in batches
   - Alternative: Implement progressive loading

4. **Query Patterns**
   - Need efficient access to votes by time period
   - Must quickly find active vs inactive votes
   - Example: Finding all active votes from last month
   - Solution: Use compound indexes
   - Alternative: Pre-calculate period totals

5. **Edge Cases**
   - Users with no votes
   - Users with only inactive votes
   - Users who just reached threshold
   - Example: User with 9.9 reputation needs one more vote
   - Solution: Implement proper validation and error handling

6. **Future Optimization Opportunities**
   - Background job processing for large updates
   - Batch updates for multiple votes
   - Progressive updates for UI responsiveness
   - Partial recalculation for changed votes
   - Example: Only recalculate votes from last month

### Index Structure

#### Vote Indexes
```typescript
// Collection: vote_indexes
interface VoteIndex {
    key: string;              // Format: "author:{author_key},target:{target_key},tag:{tag_key}"
    data: {
        author_key: string;   // Indexed for quick author lookups
        author_name: string;  // Optional: for username searches
        target_key: string;   // Indexed for quick target lookups
        target_name: string;  // Optional: for username searches
        tag_key: string;      // Indexed for quick tag lookups
        is_active: boolean;   // Indexed for active vote filtering
        created_at: bigint;   // Indexed for time-based queries
    }
}
```

#### Username Indexing Considerations

1. **Performance Impact**
   - Adding usernames increases index size by ~20-30%
   - Query performance impact is minimal (O(log n) remains)
   - Memory usage increases linearly with number of votes

2. **Search Use Cases**
   - Enables username-based vote searches
   - Useful for user profile views
   - Helps with debugging and auditing

3. **Implementation Options**
   a. Include in main index:
      - Pros: Single query for all data
      - Cons: Larger index size
   
   b. Separate username index:
      - Pros: Smaller main index
      - Cons: Requires additional queries

4. **Recommendation**
   - Include usernames in main index for:
     - Simpler implementation
     - Better query performance
     - Easier debugging
   - The memory overhead is acceptable given the benefits

## Reputation Calculation Rules

### 1. Vote Processing
When a vote is cast:
1. Check if the author's reputation meets the tag's threshold
2. If threshold is met, process the vote normally
3. If threshold is not met:
   - Count how many users have reached the threshold in this tag
   - If count >= min_users_for_threshold:
     - Vote is processed but with 0 weight
     - No reputation reward is given
   - If count < min_users_for_threshold:
     - Vote is processed normally
     - Reputation reward is given (bootstrap phase)

### 2. Reputation Calculation
A user's reputation in a tag is calculated as:
```typescript
function calculateReputation(userKey: string, tagKey: string): number {
    // Get all active votes for this user in this tag
    const votes = await listDocs({
        collection: "votes",
        filter: {
            matcher: {
                description: `target:${userKey},tag:${tagKey}`
            }
        }
    });

    // Calculate base reputation from weighted votes
    let baseReputation = 0;
    for (const vote of votes.items) {
        if (vote.data.is_active) {
            baseReputation += vote.data.weight * (vote.data.is_positive ? 1 : -1);
        }
    }

    // Get number of users above threshold
    const usersAboveThreshold = await countUsersAboveThreshold(tagKey);

    // If we're in bootstrap phase, add voting rewards
    if (usersAboveThreshold < tag.min_users_for_threshold) {
        const userVotes = await listDocs({
            collection: "votes",
            filter: {
                matcher: {
                    description: `author:${userKey},tag:${tagKey}`
                }
            }
        });
        baseReputation += userVotes.items.length * tag.vote_reward;
    }

    return baseReputation;
}

async function countUsersAboveThreshold(tagKey: string): Promise<number> {
    const reputations = await listDocs({
        collection: "reputations",
        filter: {
            matcher: {
                description: `tag:${tagKey}`
            }
        }
    });

    return reputations.items.filter(
        rep => rep.data.reputation_score >= tag.reputation_threshold
    ).length;
}
```

### 3. Vote Weight Calculation
Vote weight is calculated based on:
1. Author's reputation at time of voting
2. Time period multipliers
3. Tag's threshold status

```typescript
function calculateVoteWeight(vote: VoteDocument, tag: TagDocument): number {
    // If author's reputation is below threshold and we're past bootstrap
    if (vote.data.author_reputation < tag.reputation_threshold) {
        const usersAboveThreshold = await countUsersAboveThreshold(tag.key);
        if (usersAboveThreshold >= tag.min_users_for_threshold) {
            return 0; // Vote has no weight
        }
    }

    // Calculate time-based weight
    const monthsOld = (Date.now() - vote.created_at) / (30 * 24 * 60 * 60 * 1000);
    const period = tag.time_periods.find(p => monthsOld <= p.months);
    const timeMultiplier = period ? period.multiplier : tag.time_periods[tag.time_periods.length - 1].multiplier;

    return timeMultiplier;
}
```

### 4. Example Scenarios

#### Scenario 1: Bootstrap Phase
- Tag has reputation_threshold = 10
- min_users_for_threshold = 5
- vote_reward = 0.1
- Only 3 users have reached threshold
- New user casts 20 votes
- Result: User gets 2.0 reputation (20 * 0.1)

#### Scenario 2: Post-Bootstrap Phase
- Same tag settings
- 6 users have reached threshold
- New user casts 20 votes
- Result: User gets 0 reputation (votes have 0 weight)

#### Scenario 3: Threshold User
- User has 10.5 reputation
- Casts 20 votes
- Result: Votes have normal weight, but no additional rewards

### 5. Performance Considerations
1. Cache user counts above threshold
2. Update cache when reputations change
3. Use indexes for efficient querying
4. Batch process reputation updates

### 6. Security Considerations
1. Validate all inputs
2. Prevent vote spam
3. Rate limit reputation updates
4. Monitor for abuse patterns

### 7. Testing Strategy
1. Unit tests for calculations
2. Integration tests for thresholds
3. Load tests for performance
4. Security tests for abuse prevention 