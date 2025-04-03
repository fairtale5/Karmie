# Optimized Reputation Calculation System

## Goals & Motivation

1. **Resource Efficiency**: Create a system that minimizes computational resources by avoiding unnecessary full recalculations
2. **Tiered Calculation Strategy**: Implement different calculation depths based on time periods and frequency needs
3. **Scalability**: Ensure the system can handle growth in users and votes without linear performance degradation
4. **Responsiveness**: Provide immediate feedback for new votes while deferring expensive calculations

## Architecture

### 1. Unified Entry Point with Multiple Calculation Modes

```rust
/// Updates a user's reputation in a specific tag with various calculation depths
///
/// This function provides multiple calculation modes to optimize resource usage:
/// - Instant: Only applies a single vote's impact (lowest resource usage)
/// - Recent: Recalculates using only votes from recent time period (medium usage)
/// - Full: Complete recalculation of all reputation components (highest usage)
pub async fn update_user_reputation(
    user_key: &str,
    tag_key: &str,
    mode: CalculationMode,
) -> Result<ReputationData, String> {
    // 1. Get or create reputation document
    let (rep_key, mut reputation_data, version) = get_or_create_reputation(user_key, tag_key).await?;
    
    // 2. Update reputation based on selected calculation depth
    match mode {
        CalculationMode::Instant { vote_value, vote_weight } => {
            // Simple increment - lowest computational cost
            instant_update_reputation(&mut reputation_data, vote_value, vote_weight)?
        },
        CalculationMode::Recent { timeframe } => {
            // Partial calculation of recent votes only - medium cost
            recent_votes_calculation(&mut reputation_data, user_key, tag_key, timeframe).await?
        },
        CalculationMode::Full => {
            // Complete recalculation - highest computational cost
            full_recalculate_reputation(user_key, tag_key, &mut reputation_data).await?
        }
    }
    
    // 3. Store updated document
    store_reputation_document(rep_key, &reputation_data, version).await
}
```

### 2. Calculation Modes with Time-Based Strategies

```rust
/// The calculation depth modes for reputation updates
pub enum CalculationMode {
    /// Instant update from a single vote (lowest computational cost)
    /// Used for immediate feedback when a vote is cast
    Instant {
        vote_value: f64,
        vote_weight: f64,
    },
    
    /// Calculate using only votes from a specific recent timeframe
    /// Balances accuracy with performance
    Recent {
        timeframe: TimeFrame,
    },
    
    /// Complete recalculation of all components (highest computational cost)
    /// Used for periodic maintenance or when accuracy is critical
    Full,
}

/// Timeframes for partial recalculations
pub enum TimeFrame {
    /// Only votes from current day
    Today,
    /// Votes from past week
    PastWeek,
    /// Votes from past month
    PastMonth,
    /// Votes from past quarter
    PastQuarter,
    /// Custom timeframe
    Custom {
        /// Start timestamp in nanoseconds
        from_ns: u64,
    }
}
```

### 3. Implementation of Helper Functions

```rust
// Get/create reputation document (shared functionality)
async fn get_or_create_reputation(user_key: &str, tag_key: &str) 
    -> Result<(String, ReputationData, Option<u64>), String> { ... }

// Instant reputation update (direct increment only)
fn instant_update_reputation(
    reputation: &mut ReputationData, 
    vote_value: f64, 
    vote_weight: f64
) -> Result<(), String> { ... }

// Recent votes calculation (timeframe-limited)
async fn recent_votes_calculation(
    reputation: &mut ReputationData,
    user_key: &str,
    tag_key: &str,
    timeframe: TimeFrame,
) -> Result<(), String> { ... }

// Full reputation recalculation
async fn full_recalculate_reputation(
    user_key: &str,
    tag_key: &str,
    reputation: &mut ReputationData
) -> Result<(), String> { ... }

// Store updated reputation document
async fn store_reputation_document(
    key: String,
    reputation: &ReputationData,
    version: Option<u64>
) -> Result<ReputationData, String> { ... }
```

## Open Discussion Points

### 1. Incremental Calculation Strategies

**Options to consider:**
- **Time-partitioned calculations**: 
  - Daily: Recalculate votes from past 24 hours
  - Weekly: Recalculate votes from past 7 days
  - Monthly: Recalculate votes from past 30 days
  - Quarterly: Recalculate votes from past 90 days

- **Hybrid approach**:
  - Maintain separate accumulators for different time periods
  - `current_month_reputation`, `past_year_reputation`, `historical_reputation`
  - Only update relevant accumulator based on vote age

- **Recalculation triggers**:
  - Time-based: Schedule recalculations at specified intervals
  - Event-based: Trigger after X new votes
  - Threshold-based: Recalculate when estimated error exceeds threshold

### 2. Caching Strategies

**Questions to resolve:**
- Should we cache intermediate calculation results?
- How to efficiently store time-bucketed vote aggregates?
- Can we precompute vote impact to avoid recalculating multipliers?
- How to handle cache invalidation when tag parameters change?

### 3. Staleness vs. Accuracy Tradeoffs

**Parameters to consider:**
- Maximum acceptable staleness for reputation scores
- Performance impact thresholds for different system loads
- User expectations for update latency
- Resource constraints for different deployment environments

### 4. Implementation Complexity vs. Performance Gain

**Factors to evaluate:**
- Development time required for complex time-partitioned calculations
- Testing effort for verifying calculation consistency
- Maintenance overhead for tiered calculation system
- Actual performance gains in different usage scenarios

## Implementation Plan

1. Fix current compilation errors
2. Extract shared functionality from existing functions
3. Implement helper functions
4. Create unified entry point
5. Update `on_set_doc` to use the new system
6. Implement scheduled recalculation jobs
7. Add configuration options for recalculation frequencies
8. Create admin controls for manual recalculations

## Benefits

1. **Resource Efficiency**: Significant reduction in computation for frequent operations
2. **Scalability**: System can handle more users and votes without proportional performance cost
3. **Responsiveness**: Immediate feedback for users while maintaining eventual consistency
4. **Flexibility**: Different calculation strategies for different contexts
5. **Maintainability**: Clear separation of concerns and reusable components
6. **Configurability**: Admin-adjustable parameters for balancing performance vs. accuracy

## Future Extensions

1. **Analytics Dashboard**: 
   - Monitor calculation performance metrics
   - Track computation resource usage
   - Visualize reputation staleness

2. **Adaptive Optimization**:
   - Automatically adjust calculation strategies based on system load
   - Use machine learning to predict optimal recalculation timing
   - Identify users/tags requiring more frequent recalculations

3. **Extended Reputation Model**:
   - Implement confidence scores for reputation values
   - Track estimation error bounds
   - Provide transparency indicators for reputation freshness
