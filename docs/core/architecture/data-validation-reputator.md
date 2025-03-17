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