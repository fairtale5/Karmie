# Technical Specification: Reputator System

## System Architecture

### 1. Backend Components (Rust/Juno)

#### Collections (Database Schema)
1. **Votes Collection**
   ```rust
   struct Vote {
       key: String,              // Unique identifier: "vote_{timestamp}"
       author: Principal,        // Voter's principal ID
       target_user: Principal,   // Target user's principal ID
       vote_type: VoteType,     // Enum: Positive | Negative
       reputation_tag: String,   // e.g., "#teamplay"
       timestamp: u64,          // Vote creation time
       decay_value: f64         // Current decay value (0.0-1.0)
   }
   ```

2. **UserAdmin Collection**
   ```rust
   struct UserAdmin {
       key: String,              // Format: "{user_id}_{reputation_tag}"
       user_id: Principal,       // User's principal ID
       reputation_tag: String,   // e.g., "#teamplay"
       total_voting_power: f64,  // Current voting power
       last_decay_timestamp: u64 // Last decay calculation time
   }
   ```

#### Core Services
1. **VotingService**
   - `cast_vote(target: Principal, tag: String, positive: bool) -> Result<(), Error>`
   - `calculate_voting_power(user: Principal, tag: String) -> f64`
   - `update_decay_values() -> Result<(), Error>`

2. **ReputationService**
   - `get_user_reputation(user: Principal, tag: String) -> ReputationInfo`
   - `check_trusted_status(user: Principal, tag: String) -> bool`
   - `get_active_users_count(tag: String) -> u64`

3. **TagManagementService**
   - `create_tag(tag: String) -> Result<(), Error>`
   - `get_tag_stats(tag: String) -> TagStats`

### 2. Frontend Components (SvelteKit)

#### Core Components
1. **VotingComponents**
   - `VoteForm.svelte`: Main voting interface
   - `ReputationDisplay.svelte`: Show user reputation
   - `TagSelector.svelte`: Tag selection/creation

2. **AuthComponents**
   - `Login.svelte`: Internet Identity integration
   - `UserProfile.svelte`: User profile management

3. **CommonComponents**
   - `TagBadge.svelte`: Display reputation tags
   - `VotingPowerIndicator.svelte`: Show voting power
   - `DecayTimer.svelte`: Visual decay indicator

#### Services
1. **JunoService**
   ```typescript
   interface JunoService {
       initializeConnection(): Promise<void>;
       authenticateUser(): Promise<Principal>;
       callSatellite(method: string, args: any[]): Promise<any>;
   }
   ```

2. **ReputationService**
   ```typescript
   interface ReputationService {
       getUserReputation(userId: string, tag: string): Promise<ReputationInfo>;
       castVote(targetId: string, tag: string, isPositive: boolean): Promise<void>;
       getTagStats(tag: string): Promise<TagStats>;
   }
   ```

## Implementation Phases

### Phase 1: Core Infrastructure
1. Set up Juno project structure
2. Implement basic collections
3. Create authentication flow
4. Set up basic frontend structure

### Phase 2: Voting System
1. Implement vote casting
2. Create voting power calculation
3. Build basic UI components
4. Add tag management

### Phase 3: Reputation Mechanics
1. Implement decay system
2. Add trusted user mechanics
3. Create threshold phase transitions
4. Build reputation displays

### Phase 4: Advanced Features
1. Add decay visualization
2. Implement tag statistics
3. Create admin interfaces
4. Add API documentation

## System Constants

```rust
const MINIMUM_ACTIVE_USERS: u64 = 100;           // Threshold for phase transition
const TRUSTED_POWER_THRESHOLD: f64 = 1.0;        // Minimum power for trusted status
const INITIAL_BONUS_PER_VOTE: f64 = 0.05;        // Initial phase voting bonus
const DAILY_DECAY_RATE: f64 = 0.01;              // 1% daily decay
const MINIMUM_VOTING_POWER: f64 = 0.0;           // Minimum voting power
```

## Security Considerations

1. **Authentication**
   - Use Internet Identity for user authentication
   - Implement principal-based access control
   - Validate all user inputs

2. **Vote Validation**
   - Prevent self-voting
   - Implement rate limiting
   - Validate tag formats

3. **Data Protection**
   - Implement proper access controls in collections
   - Validate all state changes
   - Protect against overflow in calculations

## Performance Optimizations

1. **Caching**
   - Cache frequently accessed reputation scores
   - Batch process decay calculations
   - Store pre-calculated voting power

2. **Query Optimization**
   - Index collections properly
   - Implement pagination for large datasets
   - Optimize frequent queries

## Monitoring and Maintenance

1. **System Health**
   - Track active users per tag
   - Monitor voting patterns
   - Track decay calculations

2. **Performance Metrics**
   - Response times for vote operations
   - Calculation overhead
   - Storage usage

## API Documentation

### Backend Endpoints

1. **Voting API**
   ```rust
   #[update]
   async fn cast_vote(target: Principal, tag: String, positive: bool) -> Result<(), String>;

   #[query]
   fn get_reputation(user: Principal, tag: String) -> ReputationInfo;
   ```

2. **Tag Management API**
   ```rust
   #[update]
   async fn create_tag(tag: String) -> Result<(), String>;

   #[query]
   fn get_tag_stats(tag: String) -> TagStats;
   ```

### Frontend Integration

```typescript
// Example usage
const reputator = {
    async castVote(targetId: string, tag: string, isPositive: boolean) {
        return await satellite.call('cast_vote', {
            target: targetId,
            tag,
            positive: isPositive
        });
    }
};
```

## Testing Strategy

1. **Unit Tests**
   - Test voting power calculations
   - Test decay mechanisms
   - Test phase transitions

2. **Integration Tests**
   - Test full voting flow
   - Test authentication integration
   - Test frontend-backend communication

3. **Load Tests**
   - Test system with many concurrent votes
   - Test large-scale decay calculations
   - Test threshold transitions 