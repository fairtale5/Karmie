# UI Implementation Guide

## Component Structure

### UserCard Component
```typescript
interface UserCardProps {
    user: {
        key: string;
        handle: string;
        display_name: string;
        reputation: number;
    };
    onVote?: (isPositive: boolean) => Promise<void>;
    showVoteButtons?: boolean;
}

const UserCard: React.FC<UserCardProps> = ({
    user,
    onVote,
    showVoteButtons = true
}) => {
    return (
        <div className="user-card">
            <div className="user-info">
                <h3>{user.display_name}</h3>
                <span className="handle">@{user.handle}</span>
                <div className="reputation">
                    <span className="score">{user.reputation}</span>
                    <span className="label">Reputation</span>
                </div>
            </div>
            {showVoteButtons && (
                <div className="vote-buttons">
                    <button
                        onClick={() => onVote?.(true)}
                        aria-label="Upvote"
                    >
                        ↑
                    </button>
                    <button
                        onClick={() => onVote?.(false)}
                        aria-label="Downvote"
                    >
                        ↓
                    </button>
                </div>
            )}
        </div>
    );
};
```

### VoteHistory Component
```typescript
interface VoteHistoryProps {
    votes: Array<{
        key: string;
        author_key: string;
        target_key: string;
        tag_key: string;
        is_positive: boolean;
        created_at: bigint;
        weight: number;
    }>;
    onVoteUpdate?: (voteKey: string, isPositive: boolean) => Promise<void>;
}

const VoteHistory: React.FC<VoteHistoryProps> = ({
    votes,
    onVoteUpdate
}) => {
    return (
        <div className="vote-history">
            <h3>Vote History</h3>
            <div className="votes-list">
                {votes.map(vote => (
                    <div key={vote.key} className="vote-item">
                        <div className="vote-info">
                            <span className="vote-type">
                                {vote.is_positive ? '↑' : '↓'}
                            </span>
                            <span className="vote-weight">
                                Weight: {vote.weight.toFixed(2)}
                            </span>
                            <span className="vote-date">
                                {formatDate(vote.created_at)}
                            </span>
                        </div>
                        <div className="vote-actions">
                            <button
                                onClick={() => onVoteUpdate?.(vote.key, !vote.is_positive)}
                            >
                                Change Vote
                            </button>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};
```

### ReputationGraph Component
```typescript
interface ReputationGraphProps {
    history: Array<{
        date: string;
        score: number;
    }>;
    tag?: string;
}

const ReputationGraph: React.FC<ReputationGraphProps> = ({
    history,
    tag
}) => {
    return (
        <div className="reputation-graph">
            <h3>Reputation History {tag && `- ${tag}`}</h3>
            <div className="graph-container">
                {/* Implementation using charting library */}
                <LineChart
                    data={history}
                    xField="date"
                    yField="score"
                    tooltip={{
                        formatter: (datum) => ({
                            name: 'Reputation',
                            value: datum.score
                        })
                    }}
                />
            </div>
        </div>
    );
};
```

## Styling

### CSS Variables
```css
:root {
    /* Colors */
    --primary-color: #4a90e2;
    --secondary-color: #f5f5f5;
    --text-color: #333;
    --border-color: #ddd;
    
    /* Spacing */
    --spacing-xs: 4px;
    --spacing-sm: 8px;
    --spacing-md: 16px;
    --spacing-lg: 24px;
    
    /* Typography */
    --font-size-sm: 14px;
    --font-size-md: 16px;
    --font-size-lg: 18px;
    
    /* Border Radius */
    --border-radius-sm: 4px;
    --border-radius-md: 8px;
    --border-radius-lg: 16px;
}
```

### Component Styles
```css
.user-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--spacing-md);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius-md);
    background: var(--secondary-color);
}

.vote-buttons {
    display: flex;
    gap: var(--spacing-sm);
}

.vote-buttons button {
    padding: var(--spacing-xs) var(--spacing-sm);
    border: none;
    border-radius: var(--border-radius-sm);
    background: var(--primary-color);
    color: white;
    cursor: pointer;
}

.vote-history {
    margin-top: var(--spacing-lg);
}

.vote-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--spacing-sm);
    border-bottom: 1px solid var(--border-color);
}
```

## State Management

### Vote Context
```typescript
interface VoteContextType {
    votes: Vote[];
    addVote: (vote: Vote) => Promise<void>;
    updateVote: (key: string, isPositive: boolean) => Promise<void>;
    deleteVote: (key: string) => Promise<void>;
}

const VoteContext = createContext<VoteContextType | null>(null);

export const VoteProvider: React.FC<{ children: React.ReactNode }> = ({
    children
}) => {
    const [votes, setVotes] = useState<Vote[]>([]);
    
    const addVote = async (vote: Vote) => {
        // Implementation
    };
    
    const updateVote = async (key: string, isPositive: boolean) => {
        // Implementation
    };
    
    const deleteVote = async (key: string) => {
        // Implementation
    };
    
    return (
        <VoteContext.Provider
            value={{
                votes,
                addVote,
                updateVote,
                deleteVote
            }}
        >
            {children}
        </VoteContext.Provider>
    );
};
```

### Reputation Context
```typescript
interface ReputationContextType {
    reputation: number;
    updateReputation: (score: number) => void;
    history: ReputationHistory[];
}

const ReputationContext = createContext<ReputationContextType | null>(null);

export const ReputationProvider: React.FC<{ children: React.ReactNode }> = ({
    children
}) => {
    const [reputation, setReputation] = useState(0);
    const [history, setHistory] = useState<ReputationHistory[]>([]);
    
    const updateReputation = (score: number) => {
        setReputation(score);
        setHistory(prev => [
            ...prev,
            {
                date: new Date().toISOString(),
                score
            }
        ]);
    };
    
    return (
        <ReputationContext.Provider
            value={{
                reputation,
                updateReputation,
                history
            }}
        >
            {children}
        </ReputationContext.Provider>
    );
};
```

## Error Handling

### Error Boundaries
```typescript
class VoteErrorBoundary extends React.Component<
    { children: React.ReactNode },
    { hasError: boolean }
> {
    constructor(props: { children: React.ReactNode }) {
        super(props);
        this.state = { hasError: false };
    }
    
    static getDerivedStateFromError() {
        return { hasError: true };
    }
    
    render() {
        if (this.state.hasError) {
            return (
                <div className="error-container">
                    <h3>Something went wrong</h3>
                    <p>Please try refreshing the page</p>
                </div>
            );
        }
        
        return this.props.children;
    }
}
```

### Error Messages
```typescript
const ErrorMessage: React.FC<{ error: Error }> = ({ error }) => {
    return (
        <div className="error-message">
            <h4>Error</h4>
            <p>{error.message}</p>
            <button onClick={() => window.location.reload()}>
                Retry
            </button>
        </div>
    );
};
```

## Loading States

### Loading Components
```typescript
const LoadingSpinner: React.FC = () => {
    return (
        <div className="loading-spinner">
            <div className="spinner"></div>
            <span>Loading...</span>
        </div>
    );
};

const SkeletonLoader: React.FC = () => {
    return (
        <div className="skeleton-loader">
            <div className="skeleton-header"></div>
            <div className="skeleton-content"></div>
            <div className="skeleton-footer"></div>
        </div>
    );
};
```

## Accessibility

### ARIA Labels
```typescript
const VoteButton: React.FC<{
    isPositive: boolean;
    onClick: () => void;
}> = ({ isPositive, onClick }) => {
    return (
        <button
            onClick={onClick}
            aria-label={isPositive ? 'Upvote' : 'Downvote'}
            className={`vote-button ${isPositive ? 'upvote' : 'downvote'}`}
        >
            {isPositive ? '↑' : '↓'}
        </button>
    );
};
```

### Keyboard Navigation
```typescript
const VoteableCard: React.FC<{
    onVote: (isPositive: boolean) => void;
}> = ({ onVote }) => {
    const handleKeyPress = (e: React.KeyboardEvent) => {
        if (e.key === 'ArrowUp') {
            onVote(true);
        } else if (e.key === 'ArrowDown') {
            onVote(false);
        }
    };
    
    return (
        <div
            tabIndex={0}
            onKeyDown={handleKeyPress}
            role="button"
            aria-label="Vote on user"
        >
            {/* Card content */}
        </div>
    );
};
```

## Performance Optimization

### Memoization
```typescript
const MemoizedUserCard = React.memo(UserCard, (prev, next) => {
    return prev.user.key === next.user.key &&
           prev.user.reputation === next.user.reputation;
});

const MemoizedVoteHistory = React.memo(VoteHistory, (prev, next) => {
    return prev.votes.length === next.votes.length &&
           prev.votes.every((vote, i) => vote.key === next.votes[i].key);
});
```

### Lazy Loading
```typescript
const LazyReputationGraph = React.lazy(() => import('./ReputationGraph'));

const UserProfile: React.FC = () => {
    return (
        <div>
            <UserCard user={user} />
            <React.Suspense fallback={<LoadingSpinner />}>
                <LazyReputationGraph history={history} />
            </React.Suspense>
        </div>
    );
};
```

## Testing

### Component Tests
```typescript
describe('UserCard', () => {
    test('renders user information', () => {
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
        const onVote = jest.fn();
        render(<UserCard user={user} onVote={onVote} />);
        
        const upvoteButton = screen.getByLabelText('Upvote');
        await userEvent.click(upvoteButton);
        
        expect(onVote).toHaveBeenCalledWith(true);
    });
});
```

### Integration Tests
```typescript
describe('Vote Flow', () => {
    test('updates reputation on vote', async () => {
        render(
            <VoteProvider>
                <ReputationProvider>
                    <UserCard user={user} />
                </ReputationProvider>
            </VoteProvider>
        );
        
        const upvoteButton = screen.getByLabelText('Upvote');
        await userEvent.click(upvoteButton);
        
        const reputation = await screen.findByText(/101/);
        expect(reputation).toBeInTheDocument();
    });
});
``` 