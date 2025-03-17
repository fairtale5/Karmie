# Testing Documentation

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

## Integration Tests

### Database Operations
```typescript
describe('Database Integration', () => {
    test('Vote creation and reputation update', async () => {
        // Create test user
        const user = await createTestUser();
        
        // Create vote
        const vote = await createVote({
            author: user.key,
            target: user.key,
            is_positive: true
        });
        
        // Check reputation update
        const reputation = await getUserReputation(user.key);
        expect(reputation).toBe(1000);
    });
});
```

### UI Component Tests
```typescript
describe('UI Components', () => {
    test('UserList displays correct reputation', () => {
        const user = {
            key: 'test',
            data: { handle: 'test', display_name: 'Test User' },
            reputation: 1000
        };
        
        render(UserList, { props: { users: [user] } });
        
        const reputationCell = screen.getByText('1000');
        expect(reputationCell).toBeInTheDocument();
    });
});
```

## Test Environment Setup

### Configuration
```typescript
// jest.config.js
module.exports = {
    testEnvironment: 'jsdom',
    setupFilesAfterEnv: ['<rootDir>/jest.setup.js'],
    moduleNameMapper: {
        '^@/(.*)$': '<rootDir>/src/$1'
    }
};
```

### Mock Data
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
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions/setup-node@v2
        with:
          node-version: '18'
      - run: npm ci
      - run: npm test
```

## Test Coverage Requirements

### Minimum Coverage
- Statements: 80%
- Branches: 75%
- Functions: 85%
- Lines: 80%

### Coverage Report
```bash
npm run test:coverage
```

## Future Test Improvements

### Planned Enhancements
1. **Performance Benchmarking**
   - Regular performance regression tests
   - Load testing under various conditions
   - Memory usage monitoring

2. **Visual Regression Testing**
   - Component screenshot tests
   - Layout stability checks
   - Responsive design verification

3. **Security Testing**
   - Input validation tests
   - Permission boundary tests
   - Data integrity checks

4. **Accessibility Testing**
   - ARIA compliance checks
   - Keyboard navigation tests
   - Screen reader compatibility 