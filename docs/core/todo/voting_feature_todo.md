# Voting Feature Integration Todo

## Overview
This document outlines the tasks for integrating a user voting feature into the existing tags and dashboard pages. The purpose is to provide users with multiple ways to vote on each other within the app, ensuring the process is easy and the experience is obstruction-free.
This will involve using `src/lib/docs-crud/vote_create.ts` for creating vote documents and `src/lib/docs-crud/vote_update.ts` for updating vote documents.

## Proposed Implementation Order

**Phase 1: Dashboard Quick Actions Menu**
[x] **Dashboard "Quick Actions" - Voting Tab:**
    *   Refactor "Quick Actions" to use Skeleton UI Tabs.
    *   "Vote on Users" tab:
        *   Tag Selection: Input field (e.g., Skeleton Input Chips - investigate suggestion support). Suggest top 3 tags.
        *   User Search: Input field. Suggest last 5 voted/top 5 most voted users.
        *   Reuse/adapt User-ID-Card modal (needs to handle tag selection within modal).

**Phase 2: Core Voting on Tags Page**
[x] **Quick Action Menu with Tabs (Tags Page):**
    *   Implement a quick action menu with tabs on the Tags page.
    *   Add a "Vote" action that allows users to vote on other users.
    *   Pre-fill tag selection when opened from tags page.
    *   Maintain consistent layout across mobile, tablet, and desktop views.

[ ] **User Profiles:**
    *   Create user profiles to display user information and allow voting.
[ ] **Quick Action on Dashboard:**
    *   Implement a quick action on the dashboard for voting on users.
[ ] **Connect Frontend to Real Data:**
    *   Update the frontend to display real users and vote search results in the UI, replacing dummy data.
[ ] **User-ID-Card (Modal for Voting on Tags Page):**
    *   Implement as a modal triggered by clicking a user on the Tags page.
    *   Layout: Horizontal - avatar (left), user info (right).
    *   User Info: Name (bold), reputation (smaller). Defer "recent activity"; use a "View Full Profile" button instead for now.
    *   Vote Buttons: triggering `createVoteDoc` using the `selectedTagKey`. Use Lucide Icons for the buttons. https://lucide.dev/icons/circle-plus or https://lucide.dev/icons/diamond-plus and https://lucide.dev/icons/circle-minus or https://lucide.dev/icons/diamond-minus.
[ ] **Integrate User-ID-Card into "Top Users" List (Tags Page):**
    *   Make users in the "Top Users" list clickable to open the User-ID-Card modal.
[ ] **Search Functionality in "Top Users" Component (Tags Page):**
    *   Add search input (with Lucide icon) to filter/fetch users.
    *   Initial search by handle; display handle in results. Full name can be in User-ID-Card.
    *   Search results are clickable to open the User-ID-Card.

**Phase 3: Enhancing Voting Context (Tags Page)**
[ ] **Data Model & `vote_create.ts` Update (Prerequisite):**
    *   Modify the "votes" collection schema and update `src/lib/docs-crud/vote_create.ts` to include:
        *   `context: string | null` (optional, for the vote reason).
        *   `related_vote_key: string | null` (optional, ULID of the parent contextual vote if this is a "+1" to a review-like vote).
[ ] **Adding Context to Votes:**
    *   In User-ID-Card modal, add an optional text area for `context` when voting.
[ ] **"Highlighted Votes with Context" Component (Tags Page):**
    *   New component to display votes with `context` (voter, target, context, vote value).
    *   Place below "Recent Votes". Defer upvoting/downvoting of these contextual votes.
    *   When a user votes through a "review" vote, the author of the "review" vote will earn a reputation boost. (needs to be implemented in the backend reputation calculations).

**Phase 4: Create Profile Pages**
[ ] **Profile Page Voting:**
    *   Create user profile pages.
    *   Add vote buttons (similar to User-ID-Card). Optionally show voting history.
    *   The url of profile pages should be the user's handle.
    *   The profile page should have the following sections:
        *   Component: User Info (same as User-ID-Card?)
        *   Component: Voting History
        *   Component: Top Reputation Tags and his Reputation in each of them
        *   Component: Recent Activity (same as on the Tags page: displays history of votes all/in/out/positive/negative).
        *   Component: Connections Graph (same as on the Tags page: displays connections graph using sigmajs).
        *   Component: Activity Feed: Not entirely sure about this one. If we go the router of a social media app, this would make sense. But if we focus solely on reputations, this wouldn't make sense. Maybe a social feed of "reviews" and allow users to comment on each other's reviews?

**Phase 5: Advanced Features & Polish**
[ ] Refine User-ID-Card (re-evaluate "recent activity", implement tag selection if opened outside tag context).
[ ] Implement upvoting/downvoting of contextual "review" votes.
[ ] "User List with Quick Actions" (Low Priority, Tags Page): Direct hover vote buttons.
[ ] Dashboard User Milestones, Smart Suggestions, Mobile-Friendly Enhancements, Social Features, Accessibility, Anti-Abuse Measures.


# **Brainstorming, notes and ideas**

## Tags Page Enhancements

1.  **User List with Quick Actions**
    *   Add quick action buttons next to each user in the top users list for upvoting or downvoting. (low priority)
    *   Show a list of users in the current tag. (Partially covered by "Top Users")
    *   User name/avatar
    *   Current reputation score
    *   Quick vote buttons (➕✅/➖⛔) that expand on hover
    *   Clicking user opens user-Id-card layout (replaces "Option to view full profile").

2.  **User Card with More Info (user-Id-card layout)**
    *   Implement a modal or overlay that opens when a user is clicked, showing information in a user-Id-card layout.
    *   Layout: Horizontal - user's avatar (left, circular image), user's information (right).
    *   User's Information: User's name (bold), reputation (smaller font).
    *   Defer "recent activity" for now. Consider a "View Full Profile" button instead or if modal becomes too complex.
    *   Buttons for voting (➕✅/➖⛔) or viewing the full profile (smaller font). Use Lucide Icons for the buttons. https://lucide.dev/icons/circle-plus or https://lucide.dev/icons/diamond-plus and https://lucide.dev/icons/circle-minus or https://lucide.dev/icons/diamond-minus.
    *   If the user votes using this quick-action id-card from the Tags page, the vote will be cast in the tag the user is currently in (`selectedTagKey`).
    *   (Deferred) If the user is not viewing a tag (e.g., card opened from dashboard), the Id-card UI will change to allow the user to select a tag first.

3.  **Search Bar in Top Users Component**
    *   Add a search bar within the "Top Users" component.
    *   When the user starts typing, the "top users" list will be replaced/filtered with users that match the search query.
    *   Layout: Horizontal, with Lucide search icon (`<Search />`) on the left and text input on the right.
    *   Placeholder: "Search for a user".
    *   Searches for users by handle (initially) and full name. Display handle in search results; full name can be confirmed in User-ID-Card. Question about space for handle and full name in list results can be addressed by prioritizing handle.
    *   As users type, show matching users in a dynamic list/dropdown.
    *   Each result shows: User name/handle, Current reputation, Quick vote buttons (or makes item clickable for User-ID-Card).

4.  **Voting Context and Highlighted Votes**
    *   Add an optional text area (e.g., in User-ID-Card modal) for users to provide `context` when voting.
    *   Create a new component to display only recent/top votes that have `context`, allowing other users to explore those votes like "reviews".
    *   (Deferred) Allow users to upvote/downvote these contextual votes, which results in them creating their own vote on the target user, copying the sentiment and linking to the reviewed vote (`related_vote_key`).
    *   Place this component below the "Recent Votes" section.
    *   When a user votes through a "review" vote, the author of the "review" vote will earn a reputation boost. (needs to be implemented in the backend reputation calculations).

## Dashboard Page Enhancements

1.  **Quick Actions for Voting**
    *   In the "Quick Actions" section, replace existing buttons with Skeleton UI tabs. (Reference `src/routes/tags/+page.svelte` for Tabs implementation, noting height/scroll behavior).
    *   Add a "Vote on Users" tab.
    *   Tag Selection: It asks the user to select a tag first. Suggest the user's top 3 tags for selection. Use Skeleton UI Input Chips (`<InputChip>`). Investigate support for auto-suggestions (e.g., type "IC", suggest "ICP").
    *   User Search: It asks the user to search for users by handle and full name. Suggest the last 5 users that the user has voted on and the 5 users that they have voted on the most until now.
    *   Voting itself can utilize the User-ID-Card modal, adapted for tag selection if needed.

2. **User Milestones and Voting Context**
    - Include milestones related to voting, such as "Most Contextual Votes" or "Top Voter in [Tag]".

## Additional Considerations

- **Data Model Changes**: 
    - Add fields to the vote data model for context and related votes, including a `context` field for the text and a `relatedVoteId` field to link votes.
  
- **UI/UX Enhancements**:
    - Ensure the UI is intuitive and users can easily find and use the new features.
    - Consider accessibility and mobile responsiveness when designing the new components.

## Ideas from Previous Discussions

- **Tag Page Voting Integration**
    - User List with Quick Actions
    - Search + Quick Vote

- **Dashboard Voting Integration**
    - Tag Selection First
    - Quick Vote Widget

- **User Profile Integration**
    - Profile Page Voting
    - Profile Cards

- **Smart Suggestions**
    - Vote Recommendations
    - Voting History

- **Mobile-Friendly Considerations**
    - Swipe Actions
    - Bottom Sheet

- **Social Features**
    - Voting Context
    - Voting Communities

- **Accessibility Considerations**
    - Keyboard Navigation
    - Clear Feedback

- **Anti-Abuse Measures**
    - Vote Limits
    - Vote Validation

This document serves as a technical guide for implementing the voting feature, ensuring all aspects are considered and integrated smoothly into the existing system. 



Original Answer with more suggestions:

I'll help brainstorm some UI/UX ideas for the voting system. Let me break this down into different approaches and considerations.


2. **Dashboard Voting Integration**

A. **Tag Selection First**
- Start with a tag selector (dropdown or cards)
- After selecting a tag, show the user list/search interface
- This maintains context while adding the extra tag selection step

3. **User Profile Integration**

A. **Profile Page Voting** <- this is very cool, i want this>
- Add vote buttons to user profiles 
- Show voting history/statistics
- Allow voting directly from profile views

B. **Profile Cards** <- this we already explained above>
- When clicking a user's name anywhere in the app
- Show a profile card overlay with:
    - Basic user info
    - Current reputation
    - Vote buttons
    - Link to full profile

4. **Smart Suggestions**

A. **Vote Recommendations**
- Show "Users you might want to vote on" section
- Based on:
    - Users you've interacted with
    - Users in tags you frequent
    - Users with similar interests

B. **Voting History**
- Show users you've previously voted on
- Allow quick re-voting or vote changes
- Track voting patterns

5. **Mobile-Friendly Considerations**

A. **Swipe Actions**
- On mobile, allow swipe gestures for voting
- Swipe right for positive, left for negative
- Quick and intuitive on touch devices

B. **Bottom Sheet**
- Use bottom sheets for voting interfaces
- Easy to dismiss and return to previous context
- Good for showing additional options

6. **Social Features**

A. **Voting Context**
- Allow adding a reason/comment with votes
- Show voting trends in tags
- Highlight recent votes

B. **Voting Communities**
- Show who else has voted on a user
- Display voting patterns in tags
- Create voting leaderboards

7. **Accessibility Considerations**

A. **Keyboard Navigation**
- Ensure all voting actions are keyboard accessible
- Clear focus states
- Screen reader friendly

B. **Clear Feedback**
- Visual feedback for votes
- Confirmation dialogs for important actions
- Clear success/error states

8. **Anti-Abuse Measures**

A. **Vote Limits**
- Show remaining votes per day
- Display cooldown periods
- Prevent vote manipulation

B. **Vote Validation**
- Require minimum interaction before voting
- Prevent self-voting
- Track voting patterns for abuse

Would you like me to elaborate on any of these approaches? We can also discuss specific implementation details for any of these ideas that resonate with you.
