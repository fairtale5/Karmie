# Testing Documentation

## Test Strategy

### 1. Unit Tests
- Reputation calculation functions
- Vote weight normalization
- Time-based multiplier application
- Data validation

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

### Integration Tests

#### Database Operations
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

### UI Tests

#### Component Tests
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

### Mock Data
```typescript
// __mocks__/data.ts
export const mockUsers = [
    {
        key: 'user1',
        handle: 'gamer123',
        display_name: 'Gamer Pro',
        reputation: 100
    },
    // ... more mock users
];

export const mockVotes = [
    {
        key: 'vote1',
        author_key: 'user1',
        target_key: 'user2',
        tag_key: 'gaming',
        is_positive: true,
        created_at: BigInt(Date.now())
    },
    // ... more mock votes
];
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
        node-version: '16'
        
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