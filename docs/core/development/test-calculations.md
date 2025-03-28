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

### Core Reputation Rules

#### 1. Reputation Threshold System
The reputation system uses a threshold-based approach to determine who can influence the system:

1. **Reputation Threshold**
   - Each tag has a minimum reputation requirement
   - Users below this threshold are considered "untrusted"
   - Users above this threshold are considered "trusted" and "active"
   - Only trusted users can give meaningful votes to others

2. **Minimum Users Threshold**
   - Each tag requires a minimum number of trusted users
   - Before this threshold is reached:
     - All users can give votes
     - All users receive voting rewards
     - System is in "bootstrap phase"
   - After threshold is reached:
     - Only trusted users can give meaningful votes
     - Only trusted users receive voting rewards
     - System is in "restricted phase"

3. **Voting Rewards**
   - Users receive reputation points for voting
   - Reward amount is configured per tag
   - Eligibility rules:
     - In bootstrap phase: Everyone gets rewards
     - In restricted phase: Only trusted users get rewards
   - Example: If reward is 0.1 points per vote:
     - Trusted user casting 20 votes = 2.0 points
     - Untrusted user casting 20 votes = 0 points (in restricted phase)

#### 2. Example Scenarios

**Bootstrap Phase:**
```
Tag Configuration:
- Reputation Threshold: 10 points
- Minimum Users: 100
- Vote Reward: 0.1 points
- Current Trusted Users: 50

Scenario:
1. New user joins
2. Casts 20 votes
3. Result: Gets 2.0 points (20 * 0.1)
   - Because we're in bootstrap phase (< 100 trusted users)
   - Everyone gets voting rewards
```

**Restricted Phase:**
```
Same Configuration but:
- Current Trusted Users: 150

Scenario:
1. New user joins
2. Casts 20 votes
3. Result: Gets 0 points
   - Because we're in restricted phase (≥ 100 trusted users)
   - Only trusted users get voting rewards
```

**Trusted User:**
```
Scenario:
1. User has 15 points (above 10-point threshold)
2. Casts 20 votes
3. Result: Gets 2.0 points
   - Because they're above threshold
   - They get voting rewards in any phase
```

---

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
```
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
We use that to calculate the individual vote weight in percentage:
individualVoteWeightForThisUserOnly = 100% / 73.5 ≈ %1.361


This means each vote has a participation of %1.361 of the user's total voting power.


So to calculate the effective reputation of a vote, we can now take:
the vote's multiplier from its date
the vote's weight from the user's `reputation` document for this `tag`.
and the vote author's total effective reputation also from the `reputation` document for this `tag`

So in this example, a vote from the current month (multiplier 1.5x) would be calculated like this:
Each vote worth %1.361 * 1.5 = %2,0415
His ffective reputation for example 1000
so 1000 * %2.0415 = 20,415 effective reputation for the vote

So this specific user's votes would look like this if he did 10 votes in each period:

period   | votes | multi | weight | reput = result
Period 1:   10   * 1.50  * %1.361 * 1000  = 204,15
Period 2:   10   * 1.20  * %1.361 * 1000  = 163.32
Period 3:   10   * 1.10  * %1.361 * 1000  = 149.71
Period 4:   10   * 1.00  * %1.361 * 1000  = 136.1
Period 5:   10   * 0.95  * %1.361 * 1000  = 129.295
Period 6:   10   * 0.75  * %1.361 * 1000  = 102.075
Period 7:   10   * 0.50  * %1.361 * 1000  = 68.05
Period 8:   10   * 0.25  * %1.361 * 1000  = 34.025

HOWEVER, when we calculate a user's reputation, the path we take is not this one.
1. after each time a user VOTES on another user, we only calculate his individual vote weight using the method described above (to calculate the weight %1.361 in the example above).
2. then we store that weight in the user's `reputation` collection for that `tag`
 -> steps 1 and 2 are NOT used to calculate the user's own reputation, this is only used by other users when they want to calculate their reputation
3. then we call the calculate_user_reputation function to calculate the reputation of the user who is voting.
4. then we call the calculate_user_reputation functiona gain, this time to calculate the reputation of the user he voted on, the target.

The calculate_user_reputation function works like this:
parameters are a target user principal, and a target tag.
    1. we query all votes where the user is the target
    2. Generates a new list based on that, an index of unique authors+their reputation+and the weight of their votes. This way we dont have to query the same author many times if he votes many times.
        2.1.  add a field for the reputation of the author
        2.2.  add a field for the author's weight
    3. Uses those two lists (or maps) together, to iterate through the first list, and getting author info from the index. Iterate through the first list, and for each row, get the author's reputation and weight and calculate:
        3.1.  Each row's +/-1 * multiplierBasedOnDateAndTagRules * authorWeight * authorReputation
        3.2 the sum of all rows is the total_basis_reputation
    4. Now we check if the user is trusted. we check if the total_basis_reputation is equal or over the minimum reputation threshold of that tag. if the user is above this, he is considered trusted.
    5 we calculate total_voting_rewards_reputation by conunting the total votes the user has CREATED targeting others, and multiplying that by the `tag`'s voting reward. we store that in the `reputation` as well
    6 now we check if either of these conditions are true: if the user is trusted OR if the community has less than the minimum threshold of trusted users
        6.1 if either is true, then last_known_effective_reputation = total_basis_reputation + total_voting_rewards_reputation
        6.1 but if neither is true: if the user isn't trusted AND the community is already over the threshhold, then last_known_effective_reputation = total_basis_reputation
    7 we store all values in the db under `reputations` collection
        total_basis_reputation
        total_voting_rewards_reputation
        last_known_effective_reputation
        last_calculation
        vote_weight
        has_voting_power

---

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
    key: string;           // Unique identifier
    description: string;
    owner: Principal;      // User key who created the vote document
    created_at: bigint;   // Document creation timestamp
    updated_at: bigint;
    version: bigint;
    data: {
        author_key: string;   // User key who 'owns' the vote
        target_key: string;   // User key being voted on
        tag_key: string;
        value: number;
        weight: number;
    }
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

## Reputation Calculation Process

### Detailed Explanation

When calculating a user's reputation in a tag, we need to focus on votes directed at the user. The calculation process is as follows:

1. **Vote Collection**
   - Query all votes where this user is the target in this tag
   - We use the description field to filter votes efficiently
   - Format: "author:{author_key},target:{target_key},tag:{tag_key}"

2. **Author Index Creation**
   - Create an index of unique authors to avoid duplicate queries
   - We only process authors from our vote list
   - For each author in our votes:
     - Get their current effective reputation
     - Get their vote weight in this tag
     - Get their trust status (if they are trusted or not)
     - Store all this information for use in basis reputation calculation

3. **Basis Reputation Calculation**
   - Calculate total basis reputation from all received votes
   - For each vote, calculate its contribution by multiplying:
     - Base value (+1 for positive, -1 for negative)
     - Author's effective reputation
     - Author's vote weight
     - Time-based multiplier from tag rules
   - Then sum all vote contributions to get total_basis_reputation

4. **Trust Status Check**
   - Compare total_basis_reputation against tag's minimum threshold
   - User is considered "trusted" if his total_basis_reputation is above threshold
   - If the user is trusted/untrusted, we will need to store the fact that he is trusted in the user's reputation document for this tag

5. **Voting Rewards Calculation**
   - Retrieve all votes where author is the user being calculated and uses the tag key
   - Get voting reward value from tag's configuration (tag.vote_reward)
   - For each vote made by user:
     - Calculate reward = tag.vote_reward * time multiplier
   - Sum all rewards to get total_voting_rewards_reputation

6. **Final Reputation Calculation**
   - If user is trusted OR community is in bootstrap phase:
     - effective_reputation = total_basis_reputation + total_voting_rewards_reputation
   - Otherwise:
     - effective_reputation = total_basis_reputation

7. **Trust Status Check and Storage**
   - Compare total_basis_reputation against tag's minimum_reputation_threshold
   - Store trust status in reputation document:
     - If total_basis_reputation >= minimum_reputation_threshold:
       - User is considered "trusted"
       - Their votes will be active
     - If total_basis_reputation < minimum_reputation_threshold:
       - User is considered "untrusted"
       - Their votes will be inactive
   - Store all calculated values in reputations collection:
     - total_basis_reputation
     - total_voting_rewards_reputation
     - last_known_effective_reputation
     - trust status
     - Associate with user and tag
   - This status is used in future calculations to determine if their votes count

### Time-Based Vote Weighting System

The reputation system uses a sophisticated time-based weighting system to ensure that recent votes have more impact than older ones. This is implemented through the `get_period_multiplier` function, which assigns different multipliers to votes based on their age.

#### Month Calculation Rules
The system ONLY counts the number of months between dates, ignoring days completely. For example:
- Jan 1st to Jan 31st = 0 months (same month)
- Jan 15th to Feb 1st = 1 month (different months)
- Jan 31st to Feb 1st = 1 month (different months)
- Jan 1st 2024 to Mar 15th 2025 = 14 months (11 months in 2024 + 3 months in 2025)

This is used for reputation calculations where we only care about how many months have passed, not the specific days.

#### Period Configuration
Each tag defines a set of time periods with corresponding multipliers:

```typescript
time_periods: [
    { months: 1, multiplier: 1.5 },    // First month: 150% weight
    { months: 2, multiplier: 1.2 },    // Months 2-3: 120% weight
    { months: 3, multiplier: 1.1 },    // Months 4-6: 110% weight
    { months: 6, multiplier: 1.0 },    // Months 7-12: 100% weight
    { months: 12, multiplier: 0.95 },  // Months 13-24: 95% weight
    { months: 12, multiplier: 0.75 },  // Months 25-36: 75% weight
    { months: 12, multiplier: 0.55 },  // Months 37-48: 55% weight
    { months: 999, multiplier: 0.25 }  // Months 49+: 25% weight
]
```

#### How It Works

1. **Age Calculation**
   - When a vote is processed, its age is calculated in months
   - The system looks at the vote's timestamp and compares it with the current time
   - The age is used to determine which time period the vote falls into
   - Days are completely ignored - only month boundaries matter

2. **Multiplier Assignment**
   - The system iterates through the time periods in order
   - For each period, it adds the months to an accumulated total
   - When the vote's age is less than or equal to the accumulated months, that period's multiplier is used
   - If the vote is older than all defined periods, the last period's multiplier is used

3. **Example Scenarios**

   **Recent Vote (1 week old):**
   ```
   Age: 0 months (same month)
   Result: Uses first period multiplier (1.5)
   Vote Impact: 150% of base value
   ```

   **6-Month-Old Vote:**
   ```
   Age: 6 months
   Result: Uses fourth period multiplier (1.0)
   Vote Impact: 100% of base value
   ```

   **2-Year-Old Vote:**
   ```
   Age: 24 months
   Result: Uses fifth period multiplier (0.95)
   Vote Impact: 95% of base value
   ```

   **Very Old Vote (5 years):**
   ```
   Age: 60 months
   Result: Uses last period multiplier (0.25)
   Vote Impact: 25% of base value
   ```

#### Benefits of This Approach

1. **Recency Bias**
   - Recent votes have more influence on reputation
   - Helps maintain an active and dynamic system
   - Encourages ongoing participation

2. **Gradual Decay**
   - Votes don't suddenly lose value
   - Smooth transition between periods
   - Maintains historical context while favoring recent activity

3. **Configurable Per Tag**
   - Each tag can define its own time periods
   - Allows for different decay rates in different contexts
   - Flexible for different use cases

4. **Memory Efficient**
   - No need to store intermediate calculations
   - Multipliers are calculated on-demand
   - Minimal storage overhead

### 2. Reputation Calculation
A user's reputation in a tag is calculated as:
```
{insert code here once we have it}
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

### Vote Rewards and Threshold Conditions

#### 1. Vote Reward Calculation Rules
The system calculates vote rewards based on two key conditions:

1. **User's Trust Status**
   - A user is considered "trusted" if their reputation is above the tag's threshold
   - Trusted users always receive vote rewards
   - Untrusted users only receive rewards in bootstrap phase

2. **Community Size Status**
   - Each tag has a minimum number of trusted users required
   - Below this threshold: All users receive vote rewards (bootstrap phase)
   - Above this threshold: Only trusted users receive rewards (restricted phase)

#### 2. Implementation Logic
```rust
// Calculate vote rewards only if:
// 1. User is trusted (reputation above threshold) OR
// 2. There aren't enough trusted users yet
let mut reputation_from_voting = 0.0;
if reputation_from_votes >= tag.reputation_threshold || total_trusted_users < tag.minimum_users {
    let user_votes = get_user_votes(user_key, tag_key).await?;
    for vote in user_votes {
        let period = get_period_for_timestamp(vote.created_at);
        let multiplier = get_period_multiplier(&period, &tag.time_periods);
        reputation_from_voting += vote.weight * multiplier * tag.voting_reward;
    }
}
```

#### 3. Example Scenarios

**Scenario 1: Trusted User in Restricted Phase**
```
Tag Configuration:
- Reputation Threshold: 10
- Minimum Users: 100
- Current Trusted Users: 150
- User's Reputation: 15

Result: User receives vote rewards
- Because they are trusted (15 > 10)
- Regardless of community size
```

**Scenario 2: Untrusted User in Bootstrap Phase**
```
Same Configuration but:
- Current Trusted Users: 50
- User's Reputation: 5

Result: User receives vote rewards
- Because community is in bootstrap phase (< 100 trusted users)
- Even though user is untrusted (5 < 10)
```

**Scenario 3: Untrusted User in Restricted Phase**
```
Same Configuration but:
- Current Trusted Users: 150
- User's Reputation: 5

Result: User receives NO vote rewards
- Because community is in restricted phase (≥ 100 trusted users)
- And user is untrusted (5 < 10)
```

#### 4. Key Points
1. Trusted users always get vote rewards
2. Untrusted users get rewards only in bootstrap phase
3. Community size determines phase (bootstrap vs restricted)
4. Vote rewards help bootstrap new communities
5. Once community is established, only trusted users get rewards 