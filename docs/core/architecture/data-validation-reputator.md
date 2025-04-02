# Data Validation Guide

## Data Structures

### Vote Data

```typescript
interface VoteData {
    key: string;           // Unique identifier
    author_key: string;    // User who created the vote
    target_key: string;    // User being voted on
    tag_key: string;       // Tag for the vote
    is_positive: boolean;  // true = upvote, false = downvote
    created_at: bigint;    // Timestamp
    updated_at: bigint;    // Last update timestamp
    owner: Principal;      // Owner of the vote
    description: string;   // Searchable description
}
```

### Reputation Data

```typescript
interface ReputationData {
    key: string;           // Unique identifier
    user_key: string;      // User whose reputation this is
    tag_key: string;       // Tag for the reputation
    reputation_score: number; // Current reputation score
    total_votes: number;   // Total number of votes
    weighted_votes: number; // Total weighted votes
    total_voting_rewards_reputation: number;
    last_known_effective_reputation: number;
    last_calculation: number;  // timestamp in nanoseconds
    vote_weight: number;
    has_voting_power: boolean;
    calculation_month: string; // "YYYY-MM" format
    created_at: bigint;    // Creation timestamp
    updated_at: bigint;    // Last update timestamp
    owner: Principal;      // Owner of the reputation
    description: string;   // Searchable description
}
```

## Validation Functions

### Vote Validation

```typescript
function isVoteData(data: unknown): data is VoteData {
    if (!data || typeof data !== 'object') return false;
    
    const vote = data as VoteData;
    
    return (
        typeof vote.key === 'string' &&
        typeof vote.author_key === 'string' &&
        typeof vote.target_key === 'string' &&
        typeof vote.tag_key === 'string' &&
        typeof vote.is_positive === 'boolean' &&
        typeof vote.created_at === 'bigint' &&
        typeof vote.updated_at === 'bigint' &&
        vote.owner instanceof Principal &&
        typeof vote.description === 'string' &&
        vote.description.length <= 1024 &&
        vote.author_key !== vote.target_key // Prevent self-voting
    );
}

const validateVote = (vote: any): boolean => {
    return (
        typeof vote.key === 'string' &&
        typeof vote.description === 'string' &&
        typeof vote.owner === 'string' &&
        typeof vote.created_at === 'bigint' &&
        typeof vote.updated_at === 'bigint' &&
        typeof vote.version === 'bigint' &&
        typeof vote.data === 'object' &&
        typeof vote.data.author_key === 'string' &&
        typeof vote.data.target_key === 'string' &&
        typeof vote.data.tag_key === 'string' &&
        typeof vote.data.value === 'number' &&
        typeof vote.data.weight === 'number'
    );
};
```

### Reputation Validation

```typescript
function isReputationData(data: unknown): data is ReputationData {
    if (!data || typeof data !== 'object') return false;
    
    const reputation = data as ReputationData;
    
    return (
        typeof reputation.key === 'string' &&
        typeof reputation.user_key === 'string' &&
        typeof reputation.tag_key === 'string' &&
        typeof reputation.reputation_score === 'number' &&
        typeof reputation.total_votes === 'number' &&
        typeof reputation.weighted_votes === 'number' &&
        typeof reputation.total_voting_rewards_reputation === 'number' &&
        typeof reputation.last_known_effective_reputation === 'number' &&
        typeof reputation.last_calculation === 'number' &&
        reputation.last_calculation > 0 &&
        typeof reputation.vote_weight === 'number' &&
        typeof reputation.has_voting_power === 'boolean' &&
        typeof reputation.calculation_month === 'string' &&
        /^\d{4}-\d{2}$/.test(reputation.calculation_month) &&
        typeof reputation.created_at === 'bigint' &&
        typeof reputation.updated_at === 'bigint' &&
        reputation.owner instanceof Principal &&
        typeof reputation.description === 'string' &&
        reputation.description.length <= 1024 &&
        reputation.total_votes >= 0 &&
        reputation.weighted_votes >= 0
    );
}
```

## Juno Integration

### Collection Validation

```typescript
import { initJuno } from "@junobuild/core";

initJuno({
    satelliteId: "your-satellite-id",
    collections: {
        votes: {
            pre_set: (data) => {
                if (!isVoteData(data)) {
                    throw new Error("Invalid vote data");
                }
                
                // Additional validation rules
                if (data.description.length > 1024) {
                    throw new Error("Description too long");
                }
                
                // Validate vote weight
                if (data.weight && data.weight > 1000) {
                    throw new Error("Vote weight exceeds maximum");
                }
            }
        },
        reputations: {
            pre_set: (data) => {
                if (!isReputationData(data)) {
                    throw new Error("Invalid reputation data");
                }
                
                // Additional validation rules
                if (data.description.length > 1024) {
                    throw new Error("Description too long");
                }
                
                // Validate reputation score
                if (data.reputation_score < 0) {
                    throw new Error("Reputation score cannot be negative");
                }
                
                // Validate vote counts
                if (data.total_votes < 0 || data.weighted_votes < 0) {
                    throw new Error("Vote counts cannot be negative");
                }
            }
        }
    }
});
```

## Validation Rules

### Vote Rules

1. **Basic Validation**

   - All required fields must be present
   - Fields must have correct types
   - Description must be ≤ 1024 characters

2. **Business Rules**

   - Cannot vote on yourself
   - One vote per user per target per tag
   - Vote weight must be ≤ 1000
   - Created timestamp must be valid

3. **Security Rules**

   - Owner must match author
   - Cannot modify vote after creation
   - Cannot delete votes (only mark as deleted)

### Reputation Rules

1. **Basic Validation**

   - All required fields must be present
   - Fields must have correct types
   - Description must be ≤ 1024 characters

2. **Business Rules**

   - Score must be ≥ 0
   - Total votes must be ≥ 0
   - Weighted votes must be ≥ 0
   - Calculation month must be valid

3. **Security Rules**

   - Owner must match user
   - Cannot modify historical data
   - Can only update current month

### Time Period Validation Rules

1. **Structure Validation**

   ```typescript
   interface TimePeriod {
       months: number;    // Duration in months
       multiplier: number; // Weight multiplier
   }
   ```

2. **Months Validation**

   - Must be a positive integer
   - First period must be 1 month
   - Second period must be 2 months
   - Third period must be 3 months
   - Fourth period must be 6 months
   - Following periods must be 12 months
   - Last period can be 999 (treated as infinity)
   - Total of first four periods must equal 12 months

3. **Multiplier Validation**

   - Must be a number between 0.25 and 1.5
   - Must use 0.05 step increments
   - First period must be 1.5
   - Second period must be 1.2
   - Third period must be 1.1
   - Fourth period must be 1.0
   - Following periods must decrease gradually
   - Last period must be 0.25

4. **Period Count Validation**

   - Must have exactly 8 periods
   - Cannot add or remove periods
   - Periods must be in chronological order

Example Valid Time Periods:

```typescript
const validTimePeriods = [
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

## Error Handling

### Validation Errors

```typescript
class ValidationError extends Error {
    constructor(
        message: string,
        public field?: string,
        public value?: unknown
    ) {
        super(message);
        this.name = 'ValidationError';
    }
}

function validateVote(vote: unknown): VoteData {
    if (!isVoteData(vote)) {
        throw new ValidationError(
            'Invalid vote data structure',
            undefined,
            vote
        );
    }
    
    if (vote.author_key === vote.target_key) {
        throw new ValidationError(
            'Cannot vote on yourself',
            'target_key',
            vote.target_key
        );
    }
    
    return vote;
}
```

### Error Recovery

```typescript
async function createVote(data: unknown): Promise<VoteData> {
    try {
        const vote = validateVote(data);
        return await setDoc({
            collection: "votes",
            doc: vote
        });
    } catch (error) {
        if (error instanceof ValidationError) {
            // Handle validation errors
            console.error('Validation failed:', error.message);
        } else {
            // Handle other errors
            console.error('Unexpected error:', error);
        }
        throw error;
    }
}
```

## Testing

### Validation Tests

```typescript
describe('Vote Validation', () => {
    test('validates correct vote data', () => {
        const validVote: VoteData = {
            key: 'vote1',
            author_key: 'user1',
            target_key: 'user2',
            tag_key: 'gaming',
            is_positive: true,
            created_at: BigInt(Date.now()),
            updated_at: BigInt(Date.now()),
            owner: new Principal(),
            description: 'Great player'
        };
        
        expect(isVoteData(validVote)).toBe(true);
    });
    
    test('rejects invalid vote data', () => {
        const invalidVote = {
            key: 'vote1',
            author_key: 'user1',
            target_key: 'user1', // Self-vote
            tag_key: 'gaming',
            is_positive: true,
            created_at: BigInt(Date.now()),
            updated_at: BigInt(Date.now()),
            owner: new Principal(),
            description: 'Great player'
        };
        
        expect(isVoteData(invalidVote)).toBe(false);
    });
});
```

### Integration Tests

```typescript
describe('Vote Creation', () => {
    test('creates valid vote', async () => {
        const vote = await createVote({
            key: 'vote1',
            author_key: 'user1',
            target_key: 'user2',
            tag_key: 'gaming',
            is_positive: true,
            created_at: BigInt(Date.now()),
            updated_at: BigInt(Date.now()),
            owner: new Principal(),
            description: 'Great player'
        });
        
        expect(vote).toBeDefined();
        expect(vote.key).toBe('vote1');
    });
    
    test('rejects invalid vote', async () => {
        await expect(createVote({
            key: 'vote1',
            author_key: 'user1',
            target_key: 'user1', // Self-vote
            tag_key: 'gaming',
            is_positive: true,
            created_at: BigInt(Date.now()),
            updated_at: BigInt(Date.now()),
            owner: new Principal(),
            description: 'Great player'
        })).rejects.toThrow('Cannot vote on yourself');
    });
});
```
