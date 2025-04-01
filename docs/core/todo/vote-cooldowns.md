# Vote Cooldowns

## Overview
Each tag can define a cooldown period that users must wait between votes. This prevents rapid-fire voting while still allowing multiple votes between users over time.

## Implementation Details

### Tag Document Changes
Add to the tag's data structure:
```typescript
interface TagData {
    // ... existing fields ...
    vote_cooldown: number;  // Cooldown in minutes (0 = no cooldown)
}
```

### Validation Rules
1. **Cooldown Value**
   - Must be a whole number (no decimals)
   - Minimum: 0 (no cooldown)
   - Maximum: 44640 (31 days)
   - Default: 0

2. **Vote Validation**
   - Check last vote timestamp between author and target
   - Compare against current time
   - Enforce cooldown if set
   - Skip cooldown check if value is 0

### Storage Approach
- Store last vote timestamp in vote document
- Query votes collection with:
  - Same author and target
  - Same tag
  - Created within cooldown period

### Error Messages
- Clear message when cooldown is in effect:
  "Must wait X more minutes before voting on this user again in tag Y"

### Frontend Considerations
- Display remaining cooldown time
- Disable vote buttons during cooldown
- Show tooltip with remaining time

## Example Usage

```typescript
// Tag configuration
{
    name: "Technical Skills",
    vote_cooldown: 1440,  // 24 hours in minutes
    // ... other tag settings
}

// Vote validation
async function validateVote(author: string, target: string, tag: string): Promise<boolean> {
    const lastVote = await getLastVote(author, target, tag);
    if (!lastVote) return true;  // No previous vote
    
    const tag = await getTag(tag);
    if (tag.vote_cooldown === 0) return true;  // No cooldown set
    
    const minutesSinceLastVote = (Date.now() - lastVote.timestamp) / (1000 * 60);
    return minutesSinceLastVote >= tag.vote_cooldown;
}
```

## Migration Plan
1. Add vote_cooldown field to tag schema
2. Default to 0 for existing tags
3. Update tag creation/edit UI
4. Add cooldown validation to vote creation
5. Update frontend to show cooldown status

## Security Considerations
1. Validate cooldown server-side
2. Prevent timestamp manipulation
3. Handle timezone differences
4. Consider rate limiting in addition to cooldowns 