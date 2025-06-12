# Global Error Handling System Implementation

## Overview
Implement a comprehensive global error handling system to gracefully handle canister maintenance periods and prevent duplicate toast messages while maintaining existing user experience.

## Problem Statement
Currently, when the canister is stopped during upgrades, users see technical error messages like:
```
Call failed: Canister: rigfr-siaaa-aaaal-ab4fa-cai Method: list_docs (query) "Status": "rejected" "Code": "CanisterError" "Message": "Canister rigfr-siaaa-aaaal-ab4fa-cai is stopped and therefore does not have a CallContextManager"
```

Additionally, implementing global error handling could create conflicts with existing local error handling, resulting in duplicate toast messages.

## Proposed Solution: Hybrid Approach

### 1. Selective Global Error Handler
- **Target specific error types only**:
  - Canister stopped errors (`CallContextManager`, `CanisterError`)
  - Network/connection errors
  - Unhandled promise rejections
- **Avoid handling**:
  - Form validation errors
  - Business logic errors
  - Already-handled errors

### 2. Special Canister Maintenance Message
When canister is stopped, show user-friendly message:
```
🔧 System Maintenance
The system is currently being upgraded. Please check @Fairtal3 on Twitter for updates and try again in a few minutes.
```

### 3. Prevent Double Toast Messages
Add mechanism to prevent duplicate error handling:
```typescript
// Add flag to prevent double handling
if (error._alreadyHandled) return;
error._alreadyHandled = true;
```

## Files to Implement

### Core Error Handling
1. `src/lib/utils/errorHandler.ts` - Main error detection and handling logic
2. `src/lib/utils/junoHelpers.ts` - Safe wrappers for Juno SDK calls
3. `src/routes/+layout.svelte` - Global error event listeners

### Error Detection Patterns
```typescript
// Canister stopped detection
if (
  errorMessage.includes('CallContextManager') ||
  errorMessage.includes('canister is stopped') ||
  errorCode === 'CanisterError'
) {
  // Show maintenance message with Twitter link
}

// Network errors
if (
  errorMessage.includes('network') ||
  errorMessage.includes('connection') ||
  errorMessage.includes('timeout')
) {
  // Show connection error message
}
```

## Implementation Strategy

### Phase 1: Core Infrastructure
- [ ] Create `errorHandler.ts` with selective error detection
- [ ] Create `junoHelpers.ts` with safe SDK wrappers
- [ ] Add global error listeners to layout
- [ ] Implement canister stopped message

### Phase 2: Conflict Prevention
- [ ] Audit existing error handling patterns
- [ ] Add `_alreadyHandled` flag system
- [ ] Test for duplicate toast scenarios
- [ ] Update existing components gradually

### Phase 3: Testing & Validation
- [ ] Test canister stopped scenario (manually stop canister)
- [ ] Test network error scenarios
- [ ] Verify no duplicate toasts appear
- [ ] Test with existing form validation errors

## Current Error Handling Patterns to Preserve

### New User Page (`src/routes/new/user/+page.svelte`)
- Uses `toaster.promise()` wrapper
- Has manual `try/catch` with `toaster.error()`
- **Keep as-is** - these are user-facing validation errors

### Profile Components
- Use manual `try/catch` with `toaster.error()`
- Handle business logic errors
- **Keep as-is** - these provide specific context

### Admin Components
- Mix of error handling patterns
- **Evaluate case-by-case** during implementation

## Benefits
1. **User-friendly maintenance messages** with clear guidance
2. **No duplicate error toasts** 
3. **Preserves existing UX** for form validation and business logic
4. **Graceful degradation** for unhandled technical errors
5. **Consistent error experience** across the app

## Testing Requirements

### Manual Testing Scenarios
1. **Canister Stopped**: 
   ```bash
   dfx canister stop your-satellite-id
   # Try to use app - should show Twitter message
   ```

2. **Double Toast Prevention**:
   ```typescript
   // Temporarily modify createUserDoc to always throw validation error
   throw new Error("Username 'test' is already taken");
   // Verify only one toast appears
   ```

3. **Network Errors**:
   ```typescript
   // Simulate network error in Juno call
   throw new Error("network connection failed");
   ```

### Automated Testing
- [ ] Unit tests for error detection logic
- [ ] Integration tests for global error handlers
- [ ] E2E tests for maintenance scenarios

## Documentation Updates
- [ ] Update `src/lib/utils/README.md` with usage examples
- [ ] Document error handling patterns in development guide
- [ ] Add troubleshooting section for maintenance periods

## Related Issues
- Canister maintenance user experience
- Error message consistency
- Toast notification management
- Developer experience for error handling

## Priority
**Medium** - Improves user experience during maintenance but not critical for core functionality.

## Estimated Effort
**2-3 days** - Includes implementation, testing, and documentation updates. 