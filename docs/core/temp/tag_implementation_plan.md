# Tag Implementation Plan

## 1. Tag Document Structure

```typescript
// Simple branded type for pure ULIDs
type ULID = string & { readonly __brand: unique symbol };

interface TagDocument {
    // Standard Juno fields (automatically managed)
    key: string;              // Composite key: usr_{userUlid}_tag_{tagUlid}_tagName_{tagName}_
    owner: Principal;         // Automatically set to document creator's Principal
    created_at: bigint;      // Creation timestamp in nanoseconds
    updated_at: bigint;      // Last update timestamp in nanoseconds
    version: bigint;         // Document version for concurrency control
    
    // Tag-specific data
    data: {
        name: string;         // Display name of the tag (original case preserved)
        usr_key: ULID;       // Pure ULID of the user who created the tag
        tag_key: ULID;       // Pure ULID of this tag
        
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

## 2. Use Tag Key Formatting Function (`src/lib/keys/format_key_tag.ts`)

The formatTagKey function has been updated to include the tag name in the key format:

```typescript
export function formatTagKey(userUlid: ULID, tagUlid: ULID, tagName: string): string {
    if (!validateUlid(userUlid) || !validateUlid(tagUlid)) {
        throw new Error('Invalid ULID provided for tag key formatting');
    }
    
    // Convert tag name to lowercase and sanitize for key usage
    const sanitizedTagName = tagName.toLowerCase();
    
    return `usr_${userUlid}_tag_${tagUlid}_tagName_${sanitizedTagName}_`;
}
```

## 3. Cleanup Tasks
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
let tagBeingEdited = {
  key: '',
  name: '',
  description: '',
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
    console.log('[Admin] Saving tag:', tagBeingEdited);

    // Validate inputs
    if (!tagBeingEdited.data.name || !tagBeingEdited.data.description || !selectedAuthorKey) {
      errorGlobal = 'Please fill in all required fields, including selecting an author';
      return;
    }

    // Get selected user
    const selectedUser = users.find(u => u.key === selectedAuthorKey);
    if (!selectedUser || !selectedUser.data.usr_key) {
      errorGlobal = 'Selected user not found or missing ULID';
      return;
    }

    // For new documents: generate ULID and format key
    let tagDocKey: string;
    let tagDocUlid: ULID | null = null;
    let version: bigint | undefined;

    if (tagBeingEdited.key) {
      // Updating existing tag - need to get version
      try {
        const existingDoc = await getDoc({
          collection: COLLECTIONS.TAGS,
          key: tagBeingEdited.key
        });
        if (!existingDoc) {
          errorGlobal = 'Tag not found';
          return;
        }
        tagDocKey = tagBeingEdited.key;
        version = existingDoc.version;
        // When updating, we use the existing tag_key from the document
        tagDocUlid = existingDoc.data.tag_key;
      } catch (e) {
        console.error('[Admin] Error fetching existing tag:', e);
        errorGlobal = 'Failed to fetch existing tag version';
        return;
      }
    } else {
      // Creating new tag - generate new ULID and format key
      tagDocUlid = createUlid();
      tagDocKey = formatTagKey(selectedUser.data.usr_key, tagDocUlid, tagBeingEdited.data.name);
    }

    // Create or update tag document
    await setDoc({
      collection: COLLECTIONS.TAGS,
      doc: {
        key: tagDocKey,
        data: {
          name: tagBeingEdited.data.name,
          description: tagBeingEdited.data.description,
          usr_key: selectedUser.data.usr_key,  // Pure ULID
          tag_key: tagDocUlid!,               // Pure ULID
          time_periods: tagBeingEdited.data.time_periods,
          reputation_threshold: tagBeingEdited.data.reputation_threshold,
          vote_reward: tagBeingEdited.data.vote_reward,
          min_users_for_threshold: tagBeingEdited.data.min_users_for_threshold
        },
        ...(version && { version })
      }
    });

    // Reset form
    tagBeingEdited = {
      key: '',
      name: '',
      description: '',
      selectedAuthorKey: '',
      time_periods: [...REPUTATION_SETTINGS.DEFAULT_TIME_PERIODS],
      reputation_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.REPUTATION_THRESHOLD,
      vote_reward: REPUTATION_SETTINGS.DEFAULT_TAG.VOTE_REWARD,
      min_users_for_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.MIN_USERS_FOR_THRESHOLD
    };
    
    successGlobal = 'Tag saved successfully';
    errorGlobal = '';
    await loadTags();
  } catch (e) {
    console.error('[Admin] Error saving tag:', e);
    
    // Enhanced error handling
    if (e instanceof Error) {
      if (e.message.includes('Tag name')) {
        errorGlobal = 'This tag name is already taken. Please choose a different one.';
      } else if (e.message.includes('Invalid ULID')) {
        errorGlobal = 'Invalid ULID format detected. Please check user selection.';
      } else {
        errorGlobal = e.message;
      }
    } else {
      errorGlobal = 'Failed to save tag. Please try again.';
    }
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
            <div class="font-mono text-xs">User ULID: {tag.data.usr_key || 'N/A'}</div>
            <div class="font-mono text-xs">Tag ULID: {tag.data.tag_key || 'N/A'}</div>
            <div class="text-xs">Created: {new Date(Number(tag.created_at) / 1_000_000).toLocaleString()}</div>
          </div>
        </td>
        <td>
          <div class="space-y-1">
            <div class="font-bold">{tag.data.name}</div>
            <div class="text-sm opacity-75">{tag.data.description}</div>
            <div class="text-xs">Author: {users.find(u => u.data.usr_key === tag.data.usr_key)?.data.username || 'Unknown'}</div>
          </div>
        </td>
        <!-- ... rest of the display ... -->
      </tr>
    {/each}
  </tbody>
</table>
```

## 5. Implementation Order

1. Update `formatTagKey()` function to include tag name (already done)
2. Remove `createTagDescription()` function and its imports
3. Update admin page:
   - Add user selection UI
   - Update tag form state
   - Modify `saveTag()` function to use `createUlid()` and `formatTagKey()`
   - Update tag list display
4. Test tag creation with new structure
5. Test tag updates with new structure
6. Verify key format and data storage
7. Update loadTags() function to handle both old and new formats during transition

## 6. Removal of `author_key`
- Removed all references to `author_key` in the tag data structure.
- Updated all logic to use `usr_key` with ULIDs for user identification.
- Ensured backward compatibility is not required. 