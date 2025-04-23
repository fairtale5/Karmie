# Tag Implementation Plan

## 1. Tag Document Structure

```typescript
// Simple branded type for pure ULIDs
type ULID = string & { readonly __brand: unique symbol };

interface TagDocument {
    // Standard Juno fields (automatically managed)
    key: string;              // Composite key: USR_{userUlid}_TAG_{tagUlid}_{username}_
    owner: Principal;         // Automatically set to document creator's Principal
    created_at: bigint;      // Creation timestamp in nanoseconds
    updated_at: bigint;      // Last update timestamp in nanoseconds
    version: bigint;         // Document version for concurrency control
    
    // Tag-specific data
    data: {
        name: string;         // Display name of the tag
        usr_key: ULID;       // Pure ULID of the user who created the tag
        tag_key: ULID;       // Pure ULID of this tag
        username: string;     // For key reconstruction if needed
        
        // Time periods for vote decay multipliers
        time_periods: Array<{
            months: number;     // Duration in months (1-999)
            multiplier: number; // Weight multiplier (0.05-1.5)
        }>;
        
        reputation_threshold: number;     // Minimum reputation needed for voting power
        vote_reward: number;             // Reputation points given for casting votes
        min_users_for_threshold: number; // Minimum users needed before vote rewards are restricted
    }
}
```

## 2. Update Tag Key Generation (`src/lib/keys/tag.ts`)

```typescript
interface CreateTagKeyResult {
  fullKey: string;      // Composite key for document
  tagUlid: ULID;       // Pure ULID for references
}

export function createTagKey(userUlid: ULID, username: string): CreateTagKeyResult {
  if (!validateUlid(userUlid)) {
    throw new Error('Invalid ULID provided for tag key generation');
  }
  const tagUlid = generateUlid();
  return {
    fullKey: `USR_${userUlid}_TAG_${tagUlid}_${username}_`,
    tagUlid
  };
}
```

## 3. Cleanup Tasks
- Move ULID type to central types file
- Delete `createTagDescription()` function
- Remove any imports of this function
- Update any code that expects the old key format

## 4. Admin Page Updates (`src/routes/admin/+page.svelte`)

### A. Add User Selection
```svelte
<div class="mb-4">
  <label for="tagAuthor" class="block text-sm font-medium text-gray-700">
    Tag Author
  </label>
  <select
    id="tagAuthor"
    bind:value={selectedAuthorKey}
    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm"
    required
  >
    <option value="">Select Author</option>
    {#each users as user}
      <option value={user.key}>
        {user.data.display_name} ({user.data.username})
      </option>
    {/each}
  </select>
</div>
```

### B. Update Tag Form State
```typescript
let newTag = {
  key: '',
  name: '',
  selectedAuthorKey: '',  // New field for user selection
  time_periods: [...REPUTATION_SETTINGS.DEFAULT_TIME_PERIODS],
  reputation_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.REPUTATION_THRESHOLD,
  vote_reward: REPUTATION_SETTINGS.DEFAULT_TAG.VOTE_REWARD,
  min_users_for_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.MIN_USERS_FOR_THRESHOLD
};
```

### C. Update `saveTag()` Function
```typescript
async function saveTag() {
  try {
    if (!newTag.name || !newTag.selectedAuthorKey) {
      error = 'Please fill in all required fields';
      return;
    }

    // Get selected user
    const selectedUser = users.find(u => u.key === newTag.selectedAuthorKey);
    if (!selectedUser) {
      error = 'Selected user not found';
      return;
    }

    // Generate tag key
    const { fullKey, tagUlid } = createTagKey(selectedUser.data.usr_key, selectedUser.data.username);

    // If updating existing tag, get current version
    let version;
    if (newTag.key) {
      const existingDoc = await getDoc({
        collection: COLLECTIONS.TAGS,
        key: newTag.key
      });
      if (!existingDoc) {
        error = 'Tag not found';
        return;
      }
      version = existingDoc.version;
    }

    // Create or update tag document
    await setDoc({
      collection: COLLECTIONS.TAGS,
      doc: {
        key: fullKey,
        data: {
          name: newTag.name,
          usr_key: selectedUser.data.usr_key,  // Pure ULID
          tag_key: tagUlid,                    // Pure ULID
          username: selectedUser.data.username,
          time_periods: newTag.time_periods,
          reputation_threshold: newTag.reputation_threshold,
          vote_reward: newTag.vote_reward,
          min_users_for_threshold: newTag.min_users_for_threshold
        },
        ...(version && { version })
      }
    });

    // Reset form
    newTag = {
      key: '',
      name: '',
      selectedAuthorKey: '',
      time_periods: [...REPUTATION_SETTINGS.DEFAULT_TIME_PERIODS],
      reputation_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.REPUTATION_THRESHOLD,
      vote_reward: REPUTATION_SETTINGS.DEFAULT_TAG.VOTE_REWARD,
      min_users_for_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.MIN_USERS_FOR_THRESHOLD
    };

    success = 'Tag saved successfully';
    await loadTags();
  } catch (e) {
    console.error('[Admin] Error saving tag:', e);
    error = e instanceof Error ? e.message : 'Failed to save tag';
  }
}
```

### D. Update Tag Display in List
```svelte
<table class="table table-zebra w-full">
  <thead>
    <tr>
      <th>Document Info</th>
      <th>Tag Data</th>
      <th>Settings</th>
      <th>Actions</th>
    </tr>
  </thead>
  <tbody>
    {#each tags as tag}
      <tr>
        <td>
          <div class="space-y-1">
            <div class="font-mono text-xs">Key: {tag.key}</div>
            <div class="font-mono text-xs">User ULID: {tag.data.usr_key}</div>
            <div class="font-mono text-xs">Tag ULID: {tag.data.tag_key}</div>
            <div class="text-xs">Created: {new Date(Number(tag.created_at) / 1_000_000).toLocaleString()}</div>
          </div>
        </td>
        <td>
          <div class="space-y-1">
            <div class="font-bold">{tag.data.name}</div>
            <div class="text-sm">Created by: {tag.data.username}</div>
          </div>
        </td>
        <!-- ... rest of the display ... -->
      </tr>
    {/each}
  </tbody>
</table>
```

## 5. Implementation Order

1. Move ULID type to central types file
2. Update tag document interface with new ULID types
3. Create and test new `createTagKey()` function
4. Remove `createTagDescription()` function and its imports
5. Update admin page:
   - Add user selection UI
   - Update tag form state
   - Modify `saveTag()` function
   - Update tag list display
6. Test tag creation with new structure
7. Test tag updates with new structure
8. Verify key format and data storage 