Title: Data Validation in Juno: Best Practices and Security Considerations

# Why Data Validation Matters

When building applications with Juno's Datastore, data validation is crucial because:
1. Client-side code can be manipulated by users
2. Direct API calls can bypass client validation
3. Invalid data structures can break application functionality
4. Malicious users could inject harmful data
5. Data consistency is essential for reliable decentralized applications

# Available Approaches

Juno offers three main approaches for data validation:

1. **Hooks** (Recommended)
2. **Custom Endpoints**
3. **Serverless Functions**

## Security Analysis

### Custom Endpoints (Vulnerable)
```typescript
// Custom endpoint approach
const createUser = async (userData: UserData) => {
  // Validation here
  const response = await fetch('/api/createUser', {
    method: 'POST',
    body: JSON.stringify(userData)
  });
}
```
**Problems:**
- Original `setDoc` endpoint remains accessible
- Users can bypass custom endpoint entirely
- Validation can be circumvented

#### Serverless Functions (Vulnerable)
```typescript
// Serverless function approach
export const validateUser = async (context: Context) => {
  const data = await context.request.json();
  // Validation here
}
```
**Problems:**
- Same issues as custom endpoints
- Direct Datastore operations bypass these functions
- Only validates when explicitly called

## Best Approach: Hooks

Hooks provide the most secure validation because:
- They run automatically for EVERY Datastore operation
- Cannot be bypassed or circumvented
- Integrate directly with Juno's core functionality

### Implementation Example

```typescript
import { setDoc, type Doc } from "@junobuild/core";

// Define data structures
interface UserData {
  name: string;
  email: string;
}

interface PostData {
  title: string;
  content: string;
}

// Validation functions
const isUserData = (data: unknown): data is UserData => {
  const d = data as UserData;
  return (
    typeof d.name === 'string' &&
    d.name.length >= 2 &&
    typeof d.email === 'string' &&
    /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(d.email)
  );
};

const isPostData = (data: unknown): data is PostData => {
  const d = data as PostData;
  return (
    typeof d.title === 'string' &&
    d.title.length >= 3 &&
    typeof d.content === 'string' &&
    d.content.length >= 10
  );
};

// Initialize Juno with validation hooks
await initJuno({
  satelliteId: "YOUR_SATELLITE_ID",
  hooks: {
    datastore: {
      // Runs BEFORE data is written - can prevent invalid writes
      preSet: async ({ collection, doc }) => {
        // Collection-specific validation
        switch (collection) {
          case 'users':
            if (!isUserData(doc.data)) {
              throw new Error('Invalid user data structure');
            }
            break;

          case 'posts':
            if (!isPostData(doc.data)) {
              throw new Error('Invalid post data structure');
            }
            break;

          default:
            throw new Error(`Unknown collection: ${collection}`);
        }

        return true;
      },

      // Runs AFTER successful write - good for logging
      postSet: async ({ collection, doc }) => {
        console.log(`Document ${doc.key} set in ${collection}`);
      }
    }
  }
});
```

### Usage Examples

```typescript
// Create new document
const createUser = async (userData: UserData) => {
  return await setDoc({
    collection: "users",
    doc: {
      key: nanoid(),
      data: userData
    }
  });
};

// Update existing document
const updateUser = async (existingDoc: Doc<UserData>, updates: Partial<UserData>) => {
  return await setDoc({
    collection: "users",
    doc: {
      key: existingDoc.key,
      data: {
        ...existingDoc.data,
        ...updates
      },
      version: existingDoc.version // For concurrency control
    }
  });
};
```

## Hook Execution Flow

1. User calls `setDoc`
2. `preSet` hook runs
   - If validation passes → continue
   - If validation fails → operation cancelled
3. Data is written to Datastore
4. `postSet` hook runs
5. Operation completes

## Additional Security Measures

### 1. Collection Configuration
```typescript
{
    key: "users",
    readPermission: "private",
    writePermission: "managed",
    maxChangesPerUser: 100,
    maxUpdatesPerMinute: 10,
    immutablePermissions: true
}
```

### 2. Type Safety
- Use TypeScript interfaces
- Implement type guards
- Validate data structures

### 3. Concurrency Control
- Use version checking
- Let Juno handle timestamps
- Prevent race conditions

## Best Practices Summary

1. **Always use hooks** for validation
2. Implement **type guards** for data structures
3. Set appropriate **collection permissions**
4. Use **version control** for updates
5. Implement **proper error handling**
6. Add **logging** for audit trails

## Why This Approach Works

1. **Unavoidable Validation**: Every write operation must pass validation
2. **Type Safety**: TypeScript ensures data structure integrity
3. **Atomic Operations**: Validation and writes are atomic
4. **Performance**: No additional network requests
5. **Simplicity**: Centralized validation logic
6. **Reliability**: Consistent validation for all operations

Remember: Security is about preventing unauthorized or invalid operations, not just making them difficult. Hooks provide the only guaranteed way to validate all data operations in Juno's Datastore. 