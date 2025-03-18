# Testing Documentation

## Test Strategy

### 1. Unit Tests
- Reputation calculation functions
- Time-based multiplier application
- Data validation
- Helper functions

### 2. Integration Tests
- Database operations
- Vote creation and management
- Reputation updates
- Cache invalidation

### 3. UI Tests
- Component rendering
- User interactions
- Data display
- Error handling

## Test Cases

### Reputation Calculation Tests

#### Basic Calculation
```typescript
describe('Reputation Calculation', () => {
    test('Single vote calculation', () => {
        const vote = {
            author_reputation: 1000,
            weight: 1,
            is_positive: true
        };
        const result = calculateVoteWeight(vote);
        expect(result).toBe(1000); // Total points should equal voter's reputation
    });

    test('Multiple votes in different timeframes', () => {
        const votes = [
            { author_reputation: 1000, weight: 2, is_positive: true, age: 'current_month' },
            { author_reputation: 1000, weight: 1.5, is_positive: true, age: 'last_3_months' },
            { author_reputation: 1000, weight: 1.3, is_positive: true, age: 'next_6_months' }
        ];
        const result = calculateTotalReputation(votes);
        expect(result).toBe(1000); // Total should equal voter's reputation
    });
});
```

#### Edge Cases
```typescript
describe('Edge Cases', () => {
    test('Zero reputation voter', () => {
        const vote = {
            author_reputation: 0,
            weight: 1,
            is_positive: true
        };
        const result = calculateVoteWeight(vote);
        expect(result).toBe(0);
    });

    test('Negative votes', () => {
        const vote = {
            author_reputation: 1000,
            weight: 1,
            is_positive: false
        };
        const result = calculateVoteWeight(vote);
        expect(result).toBe(-1000);
    });
});
```

### Database Integration Tests

#### Vote Operations
```typescript
describe('Database Operations', () => {
    test('creates vote correctly', async () => {
        const vote = await createVote({
            author_key: 'user1',
            target_key: 'user2',
            tag_key: 'gaming',
            is_positive: true
        });
        
        expect(vote).toMatchObject({
            author_key: 'user1',
            target_key: 'user2',
            tag_key: 'gaming',
            is_positive: true
        });
    });
    
    test('updates reputation on vote', async () => {
        const vote = await createVote({
            author_key: 'user1',
            target_key: 'user2',
            tag_key: 'gaming',
            is_positive: true
        });
        
        const reputation = await getReputation('user2', 'gaming');
        expect(reputation).toBeDefined();
        expect(reputation.reputation_score).toBeGreaterThan(0);
    });
});
```

#### Vote Management
```typescript
describe('Vote Management', () => {
    test('prevents duplicate votes', async () => {
        const vote1 = await createVote({
            author_key: 'user1',
            target_key: 'user2',
            tag_key: 'gaming',
            is_positive: true
        });
        
        const vote2 = await createVote({
            author_key: 'user1',
            target_key: 'user2',
            tag_key: 'gaming',
            is_positive: false
        });
        
        expect(vote2).toBeNull();
    });
    
    test('allows vote updates', async () => {
        const vote = await createVote({
            author_key: 'user1',
            target_key: 'user2',
            tag_key: 'gaming',
            is_positive: true
        });
        
        await updateVote(vote.key, {
            is_positive: false
        });
        
        const updated = await getVote(vote.key);
        expect(updated.is_positive).toBe(false);
    });
});
```

### UI Component Tests

#### UserCard Component
```typescript
describe('UserCard Component', () => {
    test('displays user information', () => {
        const user = {
            key: 'user1',
            handle: 'gamer123',
            display_name: 'Gamer Pro',
            reputation: 100
        };
        
        render(<UserCard user={user} />);
        
        expect(screen.getByText('Gamer Pro')).toBeInTheDocument();
        expect(screen.getByText('100')).toBeInTheDocument();
    });
    
    test('handles vote interactions', async () => {
        const user = {
            key: 'user1',
            handle: 'gamer123',
            display_name: 'Gamer Pro',
            reputation: 100
        };
        
        render(<UserCard user={user} />);
        
        const upvoteButton = screen.getByLabelText('Upvote');
        await userEvent.click(upvoteButton);
        
        expect(screen.getByText('101')).toBeInTheDocument();
    });
});
```

#### Error Handling
```typescript
describe('Error Handling', () => {
    test('displays error message on failed vote', async () => {
        const user = {
            key: 'user1',
            handle: 'gamer123',
            display_name: 'Gamer Pro',
            reputation: 100
        };
        
        // Mock failed vote
        mockCreateVote.mockRejectedValue(new Error('Network error'));
        
        render(<UserCard user={user} />);
        
        const upvoteButton = screen.getByLabelText('Upvote');
        await userEvent.click(upvoteButton);
        
        expect(screen.getByText('Failed to submit vote')).toBeInTheDocument();
    });
});
```

### Performance Tests

#### Load Testing
```typescript
describe('Performance', () => {
    test('Large vote set calculation', () => {
        const votes = generateTestVotes(1000);
        const start = performance.now();
        const result = calculateTotalReputation(votes);
        const end = performance.now();
        expect(end - start).toBeLessThan(100); // Should complete within 100ms
    });

    test('Concurrent calculations', async () => {
        const calculations = Array(10).fill().map(() => 
            calculateTotalReputation(generateTestVotes(100))
        );
        const results = await Promise.all(calculations);
        expect(results.every(r => r === 1000)).toBe(true);
    });
});
```

## Test Data Generation

### Helper Functions
```typescript
function generateTestVotes(count: number): Vote[] {
    return Array(count).fill().map((_, i) => ({
        author_reputation: 1000,
        weight: getWeightForAge(i),
        is_positive: Math.random() > 0.5,
        age: getAgeForIndex(i)
    }));
}

function getWeightForAge(index: number): number {
    const weights = [2, 1.5, 1.3, 1.1, 1];
    return weights[index % weights.length];
}

function getAgeForIndex(index: number): string {
    const ages = ['current_month', 'last_3_months', 'next_6_months', 'next_18_months', '27_plus_months'];
    return ages[index % ages.length];
}
```

### Mock Data
```typescript
// __mocks__/data.ts
export const mockUsers = [
    {
        key: 'user1',
        handle: 'gamer123',
        display_name: 'Gamer Pro',
        reputation: 100
    }
];

export const mockVotes = [
    {
        key: 'vote1',
        author_key: 'user1',
        target_key: 'user2',
        tag_key: 'gaming',
        is_positive: true,
        created_at: BigInt(Date.now())
    }
];
```

## Test Environment Setup

### Jest Configuration
```javascript
module.exports = {
    testEnvironment: 'jsdom',
    setupFilesAfterEnv: ['<rootDir>/jest.setup.js'],
    moduleNameMapper: {
        '^@/(.*)$': '<rootDir>/src/$1'
    },
    testMatch: ['**/__tests__/**/*.test.ts', '**/__tests__/**/*.test.tsx']
};
```

### Juno Mocks
```typescript
// __mocks__/juno.ts
export const listDocs = jest.fn().mockResolvedValue({
    items: [],
    items_length: 0,
    matches_length: 0
});

export const setDoc = jest.fn().mockResolvedValue({});
```

## Continuous Integration

### GitHub Actions Workflow
```yaml
name: Test

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v2
    
    - name: Setup Node.js
      uses: actions/setup-node@v2
      with:
        node-version: '18'
        
    - name: Install dependencies
      run: npm ci
      
    - name: Run tests
      run: npm test
      
    - name: Upload coverage
      uses: codecov/codecov-action@v3
```

## Test Coverage Requirements

### Minimum Coverage
- Statements: 80%
- Branches: 75%
- Functions: 85%
- Lines: 80%

### Coverage Report
```bash
npm test -- --coverage
```

## Future Test Improvements

### 1. Performance Testing
- Load testing with large vote sets
- Concurrent calculation testing
- Memory usage monitoring
- Response time benchmarks

### 2. Visual Regression Testing
- Component screenshot testing
- Layout stability checks
- Responsive design testing
- Theme consistency

### 3. Security Testing
- Input validation
- Permission checks
- Rate limiting
- Data integrity

### 4. Accessibility Testing
- Screen reader compatibility
- Keyboard navigation
- Color contrast
- ARIA attributes

### Time Period Tests

1. **Structure Validation Tests**
   ```typescript
   describe('Time Period Structure', () => {
       test('should have exactly 8 periods', () => {
           expect(tag.time_periods.length).toBe(8);
       });

       test('first four periods should sum to 12 months', () => {
           const firstFourMonths = tag.time_periods
               .slice(0, 4)
               .reduce((sum, period) => sum + period.months, 0);
           expect(firstFourMonths).toBe(12);
       });

       test('periods should be in chronological order', () => {
           const months = tag.time_periods.map(p => p.months);
           expect(months).toEqual([1, 2, 3, 6, 12, 12, 12, 999]);
       });
   });
   ```

2. **Multiplier Validation Tests**
   ```typescript
   describe('Time Period Multipliers', () => {
       test('multipliers should be within valid range', () => {
           tag.time_periods.forEach(period => {
               expect(period.multiplier).toBeGreaterThanOrEqual(0.25);
               expect(period.multiplier).toBeLessThanOrEqual(1.5);
           });
       });

       test('multipliers should use 0.05 step increments', () => {
           tag.time_periods.forEach(period => {
               expect(period.multiplier % 0.05).toBe(0);
           });
       });

       test('multipliers should follow correct sequence', () => {
           const multipliers = tag.time_periods.map(p => p.multiplier);
           expect(multipliers).toEqual([1.5, 1.2, 1.1, 1.0, 0.95, 0.75, 0.55, 0.25]);
       });
   });
   ```

3. **Integration Tests**
   ```typescript
   describe('Time Period Integration', () => {
       test('should calculate correct vote weights', () => {
           const vote = createTestVote();
           const weight = calculateVoteWeight(vote, tag.time_periods);
           expect(weight).toBe(expectedWeight);
       });

       test('should handle votes across different periods', () => {
           const votes = createTestVotesAcrossPeriods();
           const totalWeight = calculateTotalWeight(votes, tag.time_periods);
           expect(totalWeight).toBe(expectedTotalWeight);
       });
   });
   ``` 