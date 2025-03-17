# Development Guide

## Setup

### Prerequisites
- Node.js 16+
- WSL 2 (Windows Subsystem for Linux)
- Git
- VS Code with recommended extensions

### Environment Setup
```bash
# Clone repository
git clone https://github.com/yourusername/reputator.git
cd reputator

# Install dependencies
npm install

# Set up environment variables
cp .env.example .env
```

### VS Code Extensions
- ESLint
- Prettier
- TypeScript and JavaScript Language Features
- GitLens
- Error Lens
- Jest Runner

## Development Workflow

### Branch Strategy
1. Main branch: `main`
2. Development branch: `develop`
3. Feature branches: `feature/*`
4. Bug fix branches: `fix/*`
5. Release branches: `release/*`

### Commit Messages
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types:
- feat: New feature
- fix: Bug fix
- docs: Documentation
- style: Formatting
- refactor: Code restructuring
- test: Adding tests
- chore: Maintenance

### Code Style
- Use TypeScript for type safety
- Follow ESLint rules
- Format with Prettier
- Use meaningful variable names
- Add JSDoc comments for public APIs

## Testing

### Unit Tests
```bash
# Run all tests
npm test

# Run specific test file
npm test -- path/to/test.ts

# Run tests with coverage
npm test -- --coverage

# Run tests in watch mode
npm test -- --watch
```

### Integration Tests
```bash
# Run integration tests
npm run test:integration

# Run integration tests with coverage
npm run test:integration -- --coverage
```

### E2E Tests
```bash
# Run E2E tests
npm run test:e2e

# Run E2E tests in headless mode
npm run test:e2e -- --headless
```

## Building

### Development Build
```bash
# Start development server
npm run dev

# Build for development
npm run build:dev
```

### Production Build
```bash
# Build for production
npm run build

# Preview production build
npm run preview
```

## Deployment

### Local Deployment
```bash
# Deploy to local Juno instance
npm run deploy:local
```

### Production Deployment
```bash
# Deploy to production
npm run deploy
```

## Debugging

### VS Code Debug Configuration
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "node",
            "request": "launch",
            "name": "Debug Tests",
            "program": "${workspaceFolder}/node_modules/jest/bin/jest",
            "args": ["--runInBand"],
            "console": "integratedTerminal",
            "windows": {
                "program": "${workspaceFolder}/node_modules/jest/bin/jest.cmd"
            }
        }
    ]
}
```

### Logging
```typescript
import { logger } from './utils/logger';

// Debug logging
logger.debug('Debug message');

// Info logging
logger.info('Info message');

// Warning logging
logger.warn('Warning message');

// Error logging
logger.error('Error message', error);
```

## Performance Optimization

### Code Splitting
```typescript
// Lazy load components
const ReputationGraph = React.lazy(() => import('./ReputationGraph'));

// Lazy load routes
const UserProfile = React.lazy(() => import('./pages/UserProfile'));
```

### Memoization
```typescript
// Memoize expensive calculations
const memoizedCalculation = useMemo(() => {
    return calculateReputation(votes);
}, [votes]);

// Memoize callbacks
const handleVote = useCallback((isPositive: boolean) => {
    submitVote(isPositive);
}, [submitVote]);
```

### Caching
```typescript
// Cache API responses
const { data } = useQuery(['user', userId], () => 
    fetchUser(userId),
    {
        staleTime: 5 * 60 * 1000, // 5 minutes
        cacheTime: 30 * 60 * 1000 // 30 minutes
    }
);
```

## Security

### Input Validation
```typescript
// Validate user input
function validateUserInput(input: unknown): UserInput {
    if (!isUserInput(input)) {
        throw new ValidationError('Invalid user input');
    }
    return input;
}

// Sanitize HTML
function sanitizeHtml(html: string): string {
    return DOMPurify.sanitize(html);
}
```

### Authentication
```typescript
// Check authentication
function requireAuth() {
    const { isAuthenticated } = useAuth();
    if (!isAuthenticated) {
        throw new AuthError('Authentication required');
    }
}

// Handle authentication errors
try {
    requireAuth();
} catch (error) {
    if (error instanceof AuthError) {
        // Handle auth error
    }
}
```

## Monitoring

### Error Tracking
```typescript
// Track errors
function trackError(error: Error, context?: Record<string, unknown>) {
    Sentry.captureException(error, {
        extra: context
    });
}

// Track user actions
function trackAction(action: string, data?: Record<string, unknown>) {
    analytics.track(action, data);
}
```

### Performance Monitoring
```typescript
// Track page load
function trackPageLoad() {
    performance.mark('pageLoadStart');
    window.addEventListener('load', () => {
        performance.mark('pageLoadEnd');
        performance.measure('pageLoad', 'pageLoadStart', 'pageLoadEnd');
    });
}

// Track component render
function trackComponentRender(componentName: string) {
    performance.mark(`${componentName}RenderStart`);
    return () => {
        performance.mark(`${componentName}RenderEnd`);
        performance.measure(
            `${componentName}Render`,
            `${componentName}RenderStart`,
            `${componentName}RenderEnd`
        );
    };
}
```

## Documentation

### Code Documentation
```typescript
/**
 * Calculates user reputation based on votes
 * @param votes - Array of votes for the user
 * @param tag - Tag to calculate reputation for
 * @returns Calculated reputation score
 * @throws {ValidationError} If votes are invalid
 */
function calculateReputation(votes: Vote[], tag: string): number {
    // Implementation
}
```

### API Documentation
```typescript
/**
 * @api {post} /api/votes Create a new vote
 * @apiName CreateVote
 * @apiGroup Votes
 * @apiVersion 1.0.0
 *
 * @apiParam {String} author_key User who created the vote
 * @apiParam {String} target_key User being voted on
 * @apiParam {String} tag_key Tag for the vote
 * @apiParam {Boolean} is_positive Whether the vote is positive
 *
 * @apiSuccess {String} key Vote identifier
 * @apiSuccess {Number} created_at Creation timestamp
 *
 * @apiError {Object} 400 Invalid input
 * @apiError {Object} 401 Unauthorized
 */
```

## Troubleshooting

### Common Issues
1. **Build Failures**
   - Check Node.js version
   - Clear npm cache
   - Delete node_modules and reinstall

2. **Test Failures**
   - Check test environment
   - Verify mock data
   - Check for race conditions

3. **Runtime Errors**
   - Check browser console
   - Verify environment variables
   - Check network requests

### Debug Tools
- Chrome DevTools
- React DevTools
- Redux DevTools
- Network tab
- Performance tab

## Maintenance

### Dependency Updates
```bash
# Check for updates
npm outdated

# Update dependencies
npm update

# Update to latest versions
npm update --latest
```

### Code Cleanup
```bash
# Remove unused imports
npm run lint:fix

# Format code
npm run format

# Check for dead code
npm run deadcode
```

### Performance Checks
```bash
# Run lighthouse
npm run lighthouse

# Check bundle size
npm run analyze

# Run performance tests
npm run test:perf
``` 