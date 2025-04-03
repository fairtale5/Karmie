# Tag Hierarchies and Reputation Federation: Implementation Plan

This document outlines the implementation plan for adding tag hierarchies and reputation federation to the Reputator system.

## Overview

The goal is to create a system where:

1. **Tag Hierarchies** - Tags can be organized in parent-child relationships (e.g., "gamers" containing "friendly", "helpful", "skillful")
2. **Reputation Federation** - Communities can share reputation across boundaries (e.g., eBay, Amazon, and Alibaba sharing an "online-shopping" reputation)

## Phase 1: Tag Hierarchies (Local Aggregation)

### Database Changes

1. **Update Tags Collection Schema**:

```typescript
// Add to TagData interface
interface TagData {
    // ... existing fields ...
    
    // Hierarchy fields
    is_composite: boolean;        // Whether this tag aggregates other tags
    parent_tags: string[];        // Tags this tag belongs to (for reverse lookup)
    child_tags: string[];         // Tags that belong to this tag
    
    // Composite settings
    composite_weights: {          // How child tags contribute to this tag's score
        [tag_key: string]: number;  // Key is child tag key, value is weight (0.0-1.0)
    };
    composite_calculation: "weighted_average" | "sum" | "highest" | "custom";
    composite_display_mode: "individual" | "aggregate" | "both";
}
```

2. **Create Tag Relationships Index**:

```typescript
interface TagRelationshipIndex {
    key: string;              // Format: "parent:{parent_tag_key},child:{child_tag_key}"
    description: string;      // Metadata for searching
    owner: Principal;         // Document owner
    created_at: bigint;       // Creation timestamp
    updated_at: bigint;       // Update timestamp
    version: bigint;          // Version control
    
    data: {
        parent_tag_key: string;  // The parent tag
        child_tag_key: string;   // The child tag
        weight: number;          // How much this child contributes to parent (0.0-1.0)
    }
}
```

### Backend Implementation

1. **Add Tag Relationship Management**:
   - Create function to add/remove child tags
   - Create function to update weights
   - Add validation for circular references

```rust
pub async fn add_child_tag(
    parent_tag_key: &str,
    child_tag_key: &str,
    weight: f64
) -> Result<(), String> {
    // 1. Validate both tags exist
    // 2. Check for circular references
    // 3. Create relationship document
    // 4. Update parent tag document
    // 5. Update child tag document
}

pub async fn remove_child_tag(
    parent_tag_key: &str,
    child_tag_key: &str
) -> Result<(), String> {
    // 1. Find relationship document
    // 2. Delete relationship document
    // 3. Update parent tag document
    // 4. Update child tag document
}
```

2. **Update Reputation Calculation**:

```rust
pub async fn calculate_composite_reputation(
    user_key: &str,
    composite_tag_key: &str
) -> Result<f64, String> {
    // 1. Get the composite tag info
    // 2. Get child tags and their weights
    // 3. Calculate reputation in each child tag
    // 4. Apply weights and aggregate
    // 5. Store calculated composite reputation
}
```

### Frontend Implementation

1. **Tag Creation/Edit Interface**:
   - Add UI for making a tag composite
   - Add UI for adding/removing child tags
   - Add UI for adjusting weights
   
2. **Tag Hierarchy Visualization**:
   - Display tag hierarchy as a tree or nested structure
   - Show weights next to child tags
   
3. **User Profile Enhancement**:
   - Show both individual and composite tags
   - Add toggle for detailed view

## Phase 2: Reputation Federation (Cross-Community)

### Database Changes

1. **Create Federation Registry Collection**:

```typescript
interface FederationRegistry {
    key: string;                  // Unique identifier
    description: string;          // Searchable metadata
    owner: Principal;             // Document owner
    created_at: bigint;           // Creation timestamp
    updated_at: bigint;           // Update timestamp
    version: bigint;              // Version control
    
    data: {
        federation_tag_key: string;  // The federation tag
        name: string;                // Human-readable name
        description: string;         // Purpose description
        
        // Governance
        admin_users: string[];       // Federation admins
        membership_threshold: number; // Min reputation to join
        voting_threshold: number;     // Min reputation to vote on members
        
        // Member communities
        member_communities: {
            [tag_key: string]: {
                status: "pending" | "approved" | "rejected";
                weight: number;      // Influence (0.0-1.0)
                application_date: bigint;
                approved_by: string[];  // Admin keys who approved
                approved_date: bigint;
                rejection_reason: string;
            }
        }
    }
}
```

2. **Update Tag Schema for Federation**:

```typescript
interface TagData {
    // ... existing fields ...
    
    // Federation fields
    federation_memberships: string[];  // Federation registry keys this tag belongs to
    federation_settings: {
        [federation_key: string]: {
            local_weight: number;      // Weight of local reputation (0.0-1.0)
            federation_weight: number;  // Weight of federated reputation (0.0-1.0)
            enabled: boolean;           // Whether federation is active
        }
    }
}
```

### Backend Implementation

1. **Federation Management**:

```rust
pub async fn create_federation(
    federation_name: &str,
    description: &str,
    admin_users: Vec<String>,
    membership_threshold: f64,
    voting_threshold: f64
) -> Result<String, String> {
    // 1. Create federation tag
    // 2. Create federation registry document
    // 3. Set up initial governance settings
}

pub async fn apply_for_federation_membership(
    community_tag_key: &str,
    federation_key: &str
) -> Result<(), String> {
    // 1. Validate community meets threshold
    // 2. Add pending application to federation
    // 3. Update community tag document
}

pub async fn vote_on_federation_application(
    federation_key: &str,
    community_tag_key: &str,
    admin_key: &str,
    approve: bool,
    weight: Option<f64>,
    rejection_reason: Option<String>
) -> Result<(), String> {
    // 1. Validate admin has voting rights
    // 2. Record admin's vote
    // 3. Check if approval threshold is met
    // 4. Update status if needed
}
```

2. **Federation Reputation Calculation**:

```rust
pub async fn calculate_federated_reputation(
    user_key: &str,
    community_tag_key: &str,
    federation_key: &str
) -> Result<f64, String> {
    // 1. Get community federation settings
    // 2. Get local reputation
    // 3. Get federation reputation components
    // 4. Apply weights from settings
    // 5. Return combined reputation
}
```

### Frontend Implementation

1. **Federation Management UI**:
   - Create federation setup wizard
   - Add federation admin dashboard
   - Create application/voting interface
   
2. **Community Federation Settings**:
   - Add UI for joining federations
   - Add UI for configuring local/federation balance
   
3. **User Profile Enhancement**:
   - Show federated reputation alongside local
   - Add details popup explaining federation sources

## Phase 3: Governance and Security

### Federation Governance Features

1. **Probation System**:
   - New communities start with reduced weight
   - Gradually increase based on time and activity
   
2. **Anomaly Detection**:
   - Monitor abnormal reputation distributions
   - Flag suspicious activity for admin review
   
3. **Federation Health Metrics**:
   - Track diversity of votes
   - Monitor consistency across communities
   - Create health score for federations

### Security Measures

1. **Reputation Rate Limiting**:
   - Prevent sudden reputation inflation
   - Add cooldown periods for major changes
   
2. **Audit Trail**:
   - Log all federation actions
   - Track approval history
   
3. **Transparency Reports**:
   - Generate federation activity metrics
   - Create public reports on federation health

## Implementation Timeline

### Month 1: Tag Hierarchies
- Week 1-2: Database schema updates
- Week 3-4: Backend implementation
- Week 5-6: Frontend implementation
- Week 7-8: Testing and refinement

### Month 2: Basic Federation
- Week 1-2: Federation registry implementation
- Week 3-4: Application and approval process
- Week 5-6: Federated reputation calculation
- Week 7-8: Federation management UI

### Month 3: Governance and Security
- Week 1-2: Probation system
- Week 3-4: Anomaly detection
- Week 5-6: Transparency features
- Week 7-8: Final testing and documentation

## Future Considerations

1. **Multi-Level Federations**:
   - Allow federations of federations
   - Create reputation webs with multiple paths
   
2. **Federation Discovery**:
   - Create marketplace for finding relevant federations
   - Add recommendation engine
   
3. **Reputation Transfer Mechanisms**:
   - Allow users to "port" reputation under specific rules
   - Create standardized reputation attestations

4. **Federation Specialization**:
   - Allow federations to focus on specific aspects
   - Create specialized trust networks (e.g., "technical skill" vs. "reliability") 