# User Profile Implementation

## 🎯 Current Status Summary

**MAJOR MILESTONE: Profile page architecture and data loading strategy completed!**

✅ **Core Implementation Complete:**
- Dynamic routing with `/u/[userHandle]` parameter
- Complete 3-case data loading strategy in `+page.ts`
- Error handling (404/500) with proper user feedback
- Demo user integration with `dummyProfileData`
- Current user profile (zero-latency with `authUserDoc` store)
- Other user profiles (async fetch with `queryDocsByKey`)

🚧 **Currently Working On:**
- Connecting profile components to real data sources  
- Component-level data integration for reputation stats, communities, activity

📋 **Next Priority:**
- Complete real data integration for remaining profile sections
- Implement reputation calculations and vote history
- Add Sigma.js graph visualization for reputation networks

---

## ✅ Completed
- [x] Set up URL structure (`/u/[userHandle]`) - **Updated to use userHandle parameter**
- [x] Create profile page layout with 3-column grid
- [x] Design profile card with avatar and basic info
- [x] Add edit button for own profile
- [x] Create reputation overview section
- [x] Add trusted communities display
- [x] Implement recent activity table
- [x] Add reviews section
- [x] Create community stats card
- [x] Add active reputations list
- [x] Update navigation to use user handle
- [x] Fix TypeScript errors in Avatar components
- [x] Configure dynamic route prerendering (disabled for dynamic routes)
- [x] Implement `BaseCard.svelte` on a test component (Active Reputations on profile page) and verify it works.
- [x] **Complete BaseCard.svelte implementation with full Svelte 5 syntax, proper TypeScript props, under-construction functionality, and Popover integration**
- [x] Implement proper data loading with Juno initialization
- [x] Add loading states and error handling
- [x] Set up data flow for demo, current user, and other user cases
- [x] Fix avatar sizing and styling consistency
- [x] Create and implement core profile components:
  - [x] ProfileHeader.svelte (avatar, name, handle, stats) - now includes reputation overview
  - [x] TrustedCommunities.svelte (community list)
  - [x] ~~ReputationOverview.svelte~~ (merged into ProfileHeader for cleaner design)
  - [x] ActiveReputations.svelte (active reputation list)
  - [x] RecentActivity.svelte (activity feed)
  - [x] BaseCard.svelte (shared card component)
- [x] Remove redundant "Users" navigation option
- [x] Update profile paths to use /u/demo_user for logged-out state
- [x] Delete old profile components and routes
- [x] Implement basic user data fetching (handle, name, avatar)
- [x] Update sidebar to use `profileLink` derived store
- [x] Implement reactive profile navigation to handle URL parameter changes
- [x] Replace server-side page loader with client-side reactive data loading
- [x] Fix race condition between Juno initialization and auth user document loading
- [x] Add case-insensitive handle matching for user document queries
- [x] **Implement complete PageLoad function with 3-case data loading strategy**
- [x] **Add proper error handling (404/500) in page loader**
- [x] **Integrate with dummyProfileData for consistent demo experience**
- [x] **Set up async user data fetching with queryDocsByKey integration**
- [x] **Configure route parameter handling for [userHandle] dynamic route**

## 🚧 In Progress
- [ ] Connect remaining components to real data using `query_by_key.ts`
- [ ] Add reputation calculations
- [ ] Set up vote history display
- [ ] Plan reputation graph visualization using Sigma.js
- [ ] Refactor existing modules into the shared card; add Construction icon & popup where `outlined` is true
- [ ] QA for dark/light themes, responsiveness, and focus states
- [ ] Mark components with dummy data as "under construction"
- [ ] Update component status:
  - [x] ProfileHeader.svelte - basic user data integrated, includes reputation overview
  - [ ] TrustedCommunities.svelte - needs real data integration
  - [ ] ActiveReputations.svelte - needs real data integration
  - [ ] RecentActivity.svelte - needs real data integration

## 📋 Todo
- [ ] Implement profile editing functionality
- [ ] Add reputation score calculations
- [ ] Create vote history pagination
- [x] Add loading states for main profile section
- [x] Implement error handling for profile loading
- [ ] Add user search functionality
- [ ] Create reputation graph visualization
  - [ ] Install Sigma.js, Graphology, and ForceAtlas2
  - [ ] Create Graph.svelte component
  - [ ] Convert VoteData[] to Graphology graph
  - [ ] Implement node/edge styling based on reputation
  - [ ] Add interactive features (hover, click, zoom)
- [ ] Add user activity timeline
- [ ] Implement favorite reputations feature
- [ ] Add user settings integration
- [ ] Create profile completion progress
- [ ] Add reputation badges/achievements
- [ ] Implement user statistics
- [ ] Add social sharing features
- [ ] Implement `BaseCard` across all pages:
  - [ ] Dashboard components (user stats, recent activities)
  - [ ] Tag-related components
  - [ ] Onboarding components
  - [ ] Home page components
  - [ ] Tag creation and management
  - [ ] All user profile sections
  - [ ] Settings and preferences
  - [ ] Notification components
  - [ ] Search result components
  - [ ] Help and support sections

## 🔍 Future Considerations
- [ ] Profile customization options
- [ ] Reputation comparison tools
- [ ] User activity analytics
- [ ] Community contribution metrics
- [ ] Reputation trend visualization
  - [ ] Use ForceAtlas2 for dynamic layouts
  - [ ] Implement time-based graph filtering
  - [ ] Add reputation score animations
- [ ] User interaction history
- [ ] Profile privacy settings
- [ ] Reputation export functionality
- [ ] User verification system
- [ ] Profile completion rewards

## 📚 Related Files
- `src/routes/u/[userHandle]/+page.svelte` - Main profile page
- `src/routes/u/[userHandle]/+page.ts` - **Page loader with 3-case data strategy**
- `src/lib/stores/authUserDoc.ts` - Profile navigation store (corrected filename)
- `src/lib/data/dummyProfileData.ts` - **Demo user data source**
- `src/lib/types.ts` - Type definitions
- `src/lib/docs-crud/query_by_key.ts` - Data fetching
- `src/lib/docs-crud/user_update.ts` - Profile updates
- `src/lib/components/layout/SidebarNavLeftRail.svelte` - Navigation
- `src/lib/components/layout/SidebarNavBottomBar.svelte` - Mobile navigation
- `docs/core/todo/frontend_sigmajs.md` - Graph visualization reference

## 🎯 Implementation Notes
- Using Skeleton UI v3 for consistent design
- Following existing page patterns (dashboard, tags)
- Graph visualization will use Sigma.js for interactive reputation networks
- Reputation calculations will be based on vote history and community trust
- Profile editing will reuse components from onboarding flow 


---

# Profile & Component Implementation — Comprehensive Plan

## 1 · Core Requirements
1. Single-page architecture with dynamic data sources
2. Efficient, state-aware data fetching (dummy vs. real)
3. **Consistent Skeleton-UI design** across all cards
4. Clear, standardized visual indicator for *under-construction* modules
5. Zero redundant network calls (use `$authUserDoc` whenever possible)

---

## 2 · Routing & Data Flow ✅ **IMPLEMENTED**

| Route | Login State | Data Source | Notes |
|-------|-------------|-------------|-------|
| `/u/demo_user` | any | `dummyProfileData` static JSON | Never queries backend - **COMPLETED** |
| `/u/[userHandle]`  | logged-out | `queryDocsByKey('users', 'hdl_${userHandle}_')` via `fetchUserData()` | Async fetch after Juno init - **COMPLETED** |
| `/u/[userHandle]`  | logged-in & **userHandle == `$authUserDoc.data.user_handle`** | `$authUserDoc` (already in store) | Zero extra latency - **COMPLETED** |

*Implementation:* All cases handled in `+page.ts` with proper error handling (404/500) and fallback to async data fetching when needed.

---

## 3 · Shared Card Component Styles

All profile modules (cards) inherit a common "base card" class so that color tokens, shadows, spacing, and borders stay perfectly aligned with existing pages (`QuickActionsTags.svelte`, onboarding `+page.svelte`, etc.).

```svelte
<!-- src/lib/components/common/BaseCard.svelte -->
<script lang="ts">
  export let classes = '';          // extra Tailwind / Skeleton classes
  export let underConstruction = false; // true = under construction variant
  let showPopup = false;
</script>

<div class={`card shadow bg-surface-100-900 border border-surface-200-800 p-4 ${underConstruction ? 'preset-outlined-error-500' : ''} ${classes}`}>
  <div class="flex justify-between items-center mb-4">
    <slot />
    {#if underConstruction}
      <div class="flex items-center gap-2">
        <button
          class="chip-icon preset-tonal-surface"
          title="Under Construction"
          on:click={() => showPopup = true}
        >
          <Construction class="text-error-500" size={16}/>
        </button>
      </div>
    {/if}
  </div>
  {#if showPopup}
    <HelperPopup on:close={() => (showPopup = false)}>
      Under Construction
    </HelperPopup>
  {/if}
</div>
```

### Under-Construction Variant
```svelte
<BaseCard outlined>
  <div class="flex justify-between items-center mb-4">
    <h2 class="text-lg font-bold">Component Title</h2>

    <div class="flex items-center gap-2">
      <!-- Expand button exactly like existing pages -->
      <button class="chip-icon preset-tonal-surface" title="View All">
        <Expand size={16}/>
      </button>

      <!-- Construction icon (error color) -->
      <button
        class="chip-icon preset-tonal-surface"
        title="Under Construction"
        on:click={() => showPopup = true}
      >
        <Construction class="text-error-500" size={16}/>
      </button>
    </div>
  </div>

  <!-- Existing content / placeholders here -->

  {#if showPopup}
    <HelperPopup on:close={() => (showPopup = false)}>
      Under Construction
    </HelperPopup>
  {/if}
</BaseCard>
```

* Keys:
  - Icon uses `text-error-500`
  - Error outline comes from `preset-outlined-error-500`
  - No other color/spacing changes, so cards still blend into overall theme

---

## 4 · Component Inventory & Status

| Module | Real Data Source | Under-Construction Flag |
|--------|-----------------|-------------------------|
| User Header (avatar, handle, display name) | `$authUserDoc` **or** fetched doc | ❌ (already real) |
| Active Reputations list | 🚧 placeholder | ✅ |
| Recent Votes | 🚧 placeholder | ✅ |
| Tag Creation Stats | 🚧 placeholder | ✅ |
| Quick Actions (already working) | `QuickActionsTags.svelte` | ❌ |
| …add more rows as modules are planned |

Each ✅ entry imports `Construction` icon & uses `outlined` variant.

---

## 5 · Sidebar Navigation Logic

```ts
// src/lib/navigation/userLink.ts
import { derived } from 'svelte/store';
import { authUserDoc } from '$lib/stores/authUserDoc';

export const profileLink = derived(authUserDoc, ($doc) =>
  $doc ? `/u/${$doc.data.user_handle}` : '/u/demo_user'
);
```
Sidebar components simply bind to `profileLink`.

---

## 6 · Page-Level Data Fetching (+page.ts) ✅ **IMPLEMENTED**

**Current implementation:** `src/routes/u/[userHandle]/+page.ts`

```ts
export const load: PageLoad = async ({ params }) => {
  const handle = params.userHandle;
  const currentUserDoc = get(authUserDoc);

  // Case 1: Demo user - return dummy data
  if (handle === 'demo_user') {
    return {
      handle,
      user: dummyProfileData.user,
      stats: dummyProfileData.communityStats,
      trustedCommunities: dummyProfileData.trustedCommunities,
      reputationStats: dummyProfileData.reputationStats,
      activeReputations: dummyProfileData.activeReputations,
      recentReviews: dummyProfileData.recentReviews
    };
  }

  // Case 2: Current user viewing own profile
  if (currentUserDoc && handle === currentUserDoc.data.user_handle) {
    return {
      handle,
      user: currentUserDoc,
      // ... other dummy data for now
    };
  }

  // Case 3: Async fetch for other users
  return {
    handle,
    fetchUserData: async () => {
      const results = await queryDocsByKey<UserDocument>('users', `hdl_${handle}_`);
      if (!results.items.length) throw error(404, 'User not found');
      return { user: results.items[0], /* ... other data */ };
    }
  };
};
```

**Features completed:**
- ✅ Parameter handling with `userHandle` 
- ✅ 3-case data loading strategy
- ✅ Error handling (404/500)
- ✅ Integration with `dummyProfileData`
- ✅ Async data fetching with `queryDocsByKey`
- ✅ Prerendering disabled for dynamic routes

---

## 7 · Loading & Error States

* **Skeleton placeholders** identical to onboarding page while data loads
* 404 card if handle not found (styled with `preset-outlined-error-500`)
* No "flash-from-dummy-to-real" because data source is selected *before* render

---

## 8 · Reference Checklist

1. Color tokens: `repu-crimson.css`
2. Type safety: `src/lib/types.ts`
3. Store single source: `src/lib/stores/authUserDoc.ts`
4. Query helper: `src/lib/docs-crud/query_by_key.ts`
5. Example patterns:  
   • `QuickActionsTags.svelte` (card style & internal layout)  
   • Onboarding `+page.svelte` (helper popup / helper icon pattern)

---

## 9 · Next Steps

1. ✅ **Implement `BaseCard.svelte`** and replace hard-coded card wrappers in profile modules. - **COMPLETED**
2. ✅ **BaseCard component fully implemented** with under-construction indicators, Popover integration, and Svelte 5 syntax. - **COMPLETED**
3. ✅ Create `/src/routes/u/[userHandle]/+page.svelte` that consumes the loader above and renders cards. - **COMPLETED**
4. ✅ Single route handles both demo and real users via loader logic. - **COMPLETED**
5. ✅ Update sidebar to use `profileLink` derived store. - **COMPLETED**
6. 🚧 QA for dark/light themes, responsiveness, and focus states. - **NEEDS TESTING**

---

### Outcome

A seamless profile experience:
• Logged-out visitors jump directly to the dummy profile.  
• Logged-in users see real data instantly with no double-render.  
• In-progress modules are clearly marked yet visually consistent with the rest of the UI.

## Shared Card Component for Consistent Styling

To ensure consistent card styles across all pages like `@+page.svelte`, `@tags.svelte`, and others, we should establish a shared component or style that can be reused.

### Proposed Solution
1. **Create a BaseCard Component**: We can create a `BaseCard.svelte` component that encapsulates the common styles and structure for cards. This component can then be used across different pages to ensure consistency.

2. **Component Structure**:
   - **Props for Customization**: Allow props for additional classes, outlined styles, and other customizations.
   - **Slot for Content**: Use a slot to allow different content to be injected into the card.

3. **Styling**:
   - **Use Tailwind/Skeleton Classes**: Apply consistent Tailwind or Skeleton UI classes for shadows, borders, and padding.
   - **Error Variant**: Include a variant for "under construction" or error states using a specific class like `preset-outlined-error-500`.

### Implementation Steps
1. **Define the BaseCard Component**:
   - Create a new file `BaseCard.svelte` in a common components directory.
   - Implement the component with props for customization and a slot for content.

2. **Refactor Existing Pages**:
   - Replace hard-coded card styles in existing pages with the `BaseCard` component.
   - Ensure that all pages import and use this component for card elements.

3. **Test Across Pages**:
   - Verify that the card styles are consistent across all pages.
   - Check for responsiveness and theme compatibility (light/dark modes).

### Example Implementation
Here's a basic structure for the `BaseCard.svelte` component:

```svelte
<!-- src/lib/components/common/BaseCard.svelte -->
<script lang="ts">
  export let classes = '';          // extra Tailwind / Skeleton classes
  export let underConstruction = false; // true = under construction variant
  let showPopup = false;
</script>

<div class={`card shadow bg-surface-100-900 border border-surface-200-800 p-4 ${underConstruction ? 'preset-outlined-error-500' : ''} ${classes}`}>
  <div class="flex justify-between items-center mb-4">
    <slot />
    {#if underConstruction}
      <div class="flex items-center gap-2">
        <button
          class="chip-icon preset-tonal-surface"
          title="Under Construction"
          on:click={() => showPopup = true}
        >
          <Construction class="text-error-500" size={16}/>
        </button>
      </div>
    {/if}
  </div>
  {#if showPopup}
    <HelperPopup on:close={() => (showPopup = false)}>
      Under Construction
    </HelperPopup>
  {/if}
</div>
```

### Next Steps
- **Create the `BaseCard.svelte` component**.
- **Refactor existing pages** to use this component.
- **Test and validate** the design consistency across different pages.

## Integration of Under Construction Indicator

To ensure that the "under construction" indicator is consistently applied whenever a card is in this state, we will integrate the Lucide icon and the on-click "helper text" directly into the `BaseCard` component. This approach will streamline the process and ensure uniformity across all instances where the card is marked as "under construction."

### Changes to BaseCard Component:

1. **Rename the `outlined` Prop**: 
   - Change the `outlined` prop to `underConstruction` to better reflect its purpose.

2. **Integrate the Lucide Icon and Helper Text**:
   - Add the Lucide icon and the on-click "helper text" directly into the `BaseCard` component.
   - Use conditional rendering to display these elements only when the `underConstruction` prop is true.

### Example Implementation:

```svelte
<!-- src/lib/components/common/BaseCard.svelte -->
<script lang="ts">
  export let classes = '';          // extra Tailwind / Skeleton classes
  export let underConstruction = false; // true = under construction variant
  let showPopup = false;
</script>

<div class={`card shadow bg-surface-100-900 border border-surface-200-800 p-4 ${underConstruction ? 'preset-outlined-error-500' : ''} ${classes}`}>
  <div class="flex justify-between items-center mb-4">
    <slot />
    {#if underConstruction}
      <div class="flex items-center gap-2">
        <button
          class="chip-icon preset-tonal-surface"
          title="Under Construction"
          on:click={() => showPopup = true}
        >
          <Construction class="text-error-500" size={16}/>
        </button>
      </div>
    {/if}
  </div>
  {#if showPopup}
    <HelperPopup on:close={() => (showPopup = false)}>
      Under Construction
    </HelperPopup>
  {/if}
</div>
```

### Next Steps:
- Implement these changes in the `BaseCard.svelte` component.
- Refactor existing modules to use the updated `BaseCard` with the `underConstruction` prop.

## Objective
- Create a base style using `BaseCard` to ensure consistent styling across all components.
- Successfully tested `BaseCard` in the 'Active Reputations' section.
- Next steps include applying `BaseCard` to new profile pages and eventually to all components on all pages, including dashboard, tags, profiles, onboarding, and more.
