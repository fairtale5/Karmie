# Skeleton UI Implementation Plan

## Table of Contents

1. [Introduction](#introduction)
2. [Layout Decisions](#layout-decisions)
3. [Current State Analysis](#current-state-analysis)
4. [Implementation Roadmap](#implementation-roadmap)
   - [Phase 1: Foundation Setup](#phase-1-foundation-setup)  
   - [Phase 2: Component Migration](#phase-2-component-migration)
   - [Phase 3: Layout & Page Structure](#phase-3-layout--page-structure)
   - [Phase 4: Advanced Features](#phase-4-advanced-features)
5. [Design Guidelines](#design-guidelines)
6. [Theme Management](#theme-management)
7. [Loading State Management](#loading-state-management)
8. [Error Handling](#error-handling)
9. [Additional Resources](#additional-resources)
10. [Open Questions & Future Considerations](#open-questions--future-considerations)

## Introduction

This document outlines the plan to fully implement Skeleton UI across the Reputator application. The goal is to move from the current mix of pure HTML/Svelte/Tailwind to a consistent UI framework based on Skeleton UI, leveraging our custom `repu-crimson` theme.

## Layout Decisions

### Summary

- **Homepage**: Single-column layout with a hero section at the top, followed by main content.
- **All Other Pages**: Two-column layout with a left sidebar for navigation or contextual content. Sidebar is implemented for flexibility, even if not always needed.
- **Mobile-First**: All layouts and components will be designed mobile-first, then enhanced for desktop. Responsive breakpoints will be prioritized.
- **No Modals**: Onboarding and similar flows will use dedicated pages, not modals, to ensure robust mobile support and accessibility.

### Rationale

- The left sidebar provides a consistent structure and makes it easy to add or remove navigation/contextual content as the app evolves.
- A hero section on the homepage aligns with common design patterns and provides a strong entry point for users.
- Mobile-first design ensures the best experience for the majority of users and avoids common mobile layout pitfalls.
- Dedicated pages for onboarding and similar flows are more robust and accessible than modals, especially on mobile devices.

---

## Current State Analysis

After examining the project's codebase, we've identified the current state of the frontend implementation:

### Implemented Components

- **Header**: Partially implemented with Skeleton UI
  - Uses Skeleton's `Switch` component for light/dark mode toggle
  - Incorporates Lucide icons (`Sun`, `Moon`)
  - Custom styling using CSS variables and some Skeleton classes like `hover:preset-tonal`
  - Custom navigation using standard buttons rather than Skeleton components

- **Background**: Custom CSS background using gradients in `repu-crimson-extensions.css`
  ```css
  body {
    background-attachment: fixed;
    background-image: radial-gradient(at 0% 25%, color-mix(in oklch, var(--color-primary-500) 10%, transparent) 0px, transparent 30%),
                      radial-gradient(at 15% 6%, color-mix(in oklch, var(--color-success-500) 10%, transparent) 0px, transparent 30%);
  }
  ```

- **Routes and Pages**:
  - Main page (`+page.svelte`): Using pure HTML/Tailwind classes without Skeleton components
  - Admin page: Complex UI with tables, forms, and interactive elements using standard HTML and Tailwind
  - Custom components like `Article.svelte`, `Hero.svelte`, and `Footer.svelte` use standard HTML/Tailwind

### Theme Configuration

- Custom `repu-crimson` theme created
- Proper data attributes for dark mode toggling
- Theme toggle implemented in Header using Skeleton's Switch component

### Missing Implementations

- **Form Controls**: Not using Skeleton's form components (inputs, selects, textareas)
- **Buttons**: Using Tailwind classes instead of Skeleton's button presets
- **Cards**: Not utilizing Skeleton card components or presets
- **Dialogs/Modals**: No implementation of Skeleton modal components
- **Tables**: Using standard HTML tables without Skeleton styling
- **Alerts/Notifications**: Using custom alert components with Tailwind
- **Loading States**: No proper loading indicators or feedback during async operations

### Error Handling

- All error feedback for unauthenticated actions (such as attempting to submit a form while not logged in) will be handled exclusively via toast notifications using Skeleton UI's feedback system.
- Redundant static warning divs (e.g., `<div class="text-center text-warning-700 mt-2">You must be logged in to set up your profile.</div>`) have been removed to avoid duplicate messaging and improve UX consistency.
- The authentication state flag has been renamed from `authUserInitialized` to `authUserDoneInitializing` to clarify that it is only set to `true` after the async user fetch is fully complete and the user state is final. This prevents race conditions and UI flashes for logged-in users.
- This approach aligns with Skeleton UI's best practices for transient, accessible feedback and keeps the UI clean.
- Persistent errors or warnings are handled with Skeleton's Alert component.

### Layout Structure

- Basic Tailwind container-based layout
- Not utilizing Skeleton's layout patterns or grid system effectively
- Header partially styled with Skeleton components but main content areas use standard HTML/Tailwind

### Known Issues

1. Inconsistent styling between components that use Skeleton UI and those that don't
2. Dark mode implementation works but styling across components is inconsistent in dark mode
3. Missing loading state indicators during backend operations
4. Forms lack proper validation feedback using Skeleton components
5. Notifications for successful operations are implemented inconsistently

This analysis confirms the need for a comprehensive implementation plan to fully leverage Skeleton UI across the application and create a consistent user experience.

## Implementation Roadmap

Based on Skeleton UI's best practices and the current state of the application, we'll implement a phased approach to fully adopt Skeleton UI across the entire application.

### Phase 1: Foundation Setup

1. **Refine Theme Configuration**
   - Ensure proper theme variable definitions in `repu-crimson.css`
   - Keep custom background gradient in `repu-crimson-extensions.css`
   - Add proper Skeleton class references in the theme files
   - Verify dark mode functionality works with the theme

2. **Layout Structure Implementation**
   - Implement proper layout patterns based on Skeleton's layout recommendations
   - Create reusable layout components for different page types:
     - Standard page layout with header and content
     - Admin page layout with sidebar navigation
     - Dashboard layout with multiple content sections
   - Use semantic HTML with Skeleton's utility classes for layouts

3. **Base Component Migration**
   - Migrate common UI elements to Skeleton components:
     - Replace Tailwind buttons with Skeleton's button presets
     - Update all links with Skeleton's anchor styling
     - Implement proper typography using Skeleton's text utilities
     - Update containers with Skeleton's card components

### Phase 2: Component Migration

1. **Forms and Inputs**
   - Convert all form elements to Skeleton components:
     - Input fields with proper validation
     - Select dropdowns
     - Textarea components
     - Radio and checkbox controls
     - Form layouts and grouping
   - Implement proper form validation using Skeleton's styles

2. **Interactive Elements**
   - Implement Skeleton components for all interactive elements:
     - Tabs for navigation between sections
     - Accordions for collapsible content
     - Modals for confirmation dialogs
     - Tooltips for help text
     - Popovers for additional information

3. **Data Display**
   - Convert data display elements to Skeleton components:
     - Tables with proper styling
     - Lists with Skeleton styling
     - Badges and chips for status indicators
     - Progress indicators

### Phase 3: Layout & Page Structure

1. **Navigation Components**
   - Implement Skeleton's navigation components:
     - App navigation with proper routing
     - Sidebar for admin navigation
     - Breadcrumbs for page hierarchy
     - Tabs for section navigation

2. **Page-Specific Components**
   - Implement components for specific page types:
     - Home page with hero section
     - Admin dashboard with metrics and controls
     - User profile page with details and actions
     - Settings page with form controls

3. **Responsive Design**
   - Ensure all pages are responsive using Skeleton's utilities:
     - Mobile navigation
     - Adaptive layouts
     - Responsive tables
     - Collapsible sections for small screens

### Phase 4: Advanced Features

1. **Loading States**
   - Implement loading indicators for all async operations:
     - Page loading skeletons
     - Button loading states
     - Form submission indicators
     - Data fetch progress indicators

2. **Error Handling**
   - Create consistent error handling using Skeleton components:
     - Form validation errors
     - API error displays
     - Toast notifications for errors
     - Error pages and states

3. **Animations and Transitions**
   - Add Skeleton-compatible animations:
     - Page transitions
     - Component mounting/unmounting
     - Loading state animations
     - Interactive element feedback

4. **Accessibility Improvements**
   - Ensure accessibility compliance:
     - Proper contrast in the theme
     - Keyboard navigation
     - Screen reader support
     - Focus indicators

By following this phased approach, we'll systematically migrate the entire application to Skeleton UI while maintaining functionality and ensuring a consistent user experience.

### Reputations Hub Page

**Purpose:**
The main landing page for logged-in users, providing an overview of all reputation communities (tags), the user's standing in each, recent activity, and a graph overview. All default styles should use the custom theme.

**Layout:**
- Single column, mobile-first (can expand to two-column later)
- Use Skeleton container and spacing utilities

**Sections & Components:**
1. Reputation Tag Selector
   - Large, prominent selector at the top (Skeleton Select, Tabs, or custom dropdown)
   - "All Communities" option for overview
2. Community Description
   - Brief, always visible but not dominant (Skeleton Card or subtle info box)
3. User's Reputation in Selected Tag
   - Card with user's score, rank, badges/levels
   - Prominent, but not larger than the selector
4. User's Recent Activity
   - List of the user's most recent votes/actions in this community
   - Compact, mobile-friendly
5. Top Users in Reputation
   - Top 10 users (with pagination)
   - Each row: username, score, horizontal bar graph (Skeleton Progress or custom)
   - Top 3 visually highlighted (e.g., gold/silver/bronze)
6. Most Recent Votes
   - List of recent votes in this community
   - Each row: author, target, colored indicator (green/red)
   - Compact, mobile-friendly
7. Graph Overview
   - sigma.js graph of the vote network for this tag
   - Responsive container
   - If empty: show a greyed-out placeholder graph
8. Call to Action
   - Context-aware: "Join Community" if not a member, "Contribute" if already active
   - Can be hidden/disabled if not relevant

**States & UX:**
- Empty States: Greyed-out graph, "No data yet" messages, or subtle placeholders for lists
- Loading States: Use Skeleton UI placeholders for all sections while loading
- Accessibility: All controls and graphs must be accessible (labels, alt text, keyboard navigation)
- Performance: Paginate/lazy-load lists and graphs (optimize after MVP)
- Error Handling: Use Skeleton Toast components for errors (catch and display backend errors)

**Default Styles:**
- All components and sections should use the default styles from the custom `repu-crimson` theme.

**Open Questions:**
- Should the graph overview be visible by default, or behind a "Show Graph" toggle for performance?
- Any specific text or style for empty/loading/error states, or use Skeleton's defaults and iterate later?
- Should the Call to Action be more prominent or context-sensitive in future iterations?

---

## Design Guidelines

### Core Design Principles

When implementing Skeleton UI across the application, follow these key principles:

1. **Utility-First Approach**
   - Use Skeleton's utility classes for styling rather than custom CSS
   - Follow the naming patterns and conventions established by Skeleton
   - Leverage the preset system for consistent component styling

2. **Semantic HTML Structure**
   - Use appropriate HTML elements for their intended purpose
   - Implement accessible markup patterns
   - Utilize Skeleton's recommended HTML structures for components

3. **Consistent Component Usage**
   - Use Skeleton components in their intended manner
   - Apply presets consistently across similar elements
   - Follow the component API documentation for proper implementation

4. **Adaptive Design**
   - Ensure designs work in both light and dark mode
   - Utilize color pairings for automatic adaptation
   - Test all implementations in both color schemes

### Color System

The `repu-crimson` theme should be used consistently with the following guidelines:

1. **Color Role Assignment**
   - **Primary**: Main brand color, used for primary actions and key UI elements
   - **Secondary**: Supporting color for secondary actions and accents
   - **Tertiary**: Used sparingly for highlights or specific UI elements
   - **Success/Warning/Error**: Reserved for feedback and status indicators
   - **Surface**: Background colors with appropriate contrast levels

2. **Color Application**
   - Apply colors using Skeleton's color utilities: `bg-primary-500`, `text-error-400`, etc.
   - Use color pairings for automatic dark mode adaptation: `bg-primary-50-950`
   - Maintain sufficient contrast for accessibility

3. **Custom Theme Integration**
   - Use theme variables consistently: `var(--color-primary-500)`
   - Keep custom extensions separate from the main theme definition
   - Document any custom color additions

### Typography

Follow Skeleton's typography system for consistent text styling:

1. **Text Scale**
   - Use Skeleton's predefined text sizes: `text-xs` through `text-9xl`
   - Follow the scaling pattern for maintaining visual hierarchy
   - Utilize `h1` through `h6` classes for headings

2. **Font Styling**
   - Apply font weights consistently: `font-thin`, `font-normal`, `font-bold`, etc.
   - Use appropriate line heights for readability
   - Maintain consistent letter spacing

3. **Text Utilities**
   - Use Skeleton's text utilities for alignment, decoration, and transformation
   - Apply text colors using theme variables: `text-primary-500`
   - Ensure proper contrast in both light and dark mode

### Spacing and Layout

Implement consistent spacing and layout patterns:

1. **Spacing Scale**
   - Use Tailwind's spacing scale for margin and padding
   - Maintain rhythm with consistent spacing between elements
   - Apply responsive spacing adjustments at appropriate breakpoints

2. **Layout Patterns**
   - Utilize Skeleton's layout recommendations for page structures
   - Implement grid and flex layouts consistently
   - Follow semantic layout structure with appropriate HTML elements

3. **Containers and Boundaries**
   - Use the `container` class for consistent maximum widths
   - Apply appropriate padding at different viewport sizes
   - Maintain consistent spacing between major page sections

### Component Styling Patterns

When implementing Skeleton components, follow these guidelines:

1. **Button Patterns**
   - Use appropriate presets for different button types:
     - `preset-filled` for primary actions
     - `preset-tonal` for secondary actions
     - `preset-outlined` for tertiary actions
   - Maintain consistent sizing across button types
   - Use appropriate icons and text combinations

2. **Form Patterns**
   - Use consistent input styling with Skeleton's form components
   - Implement clear label and input relationships
   - Provide appropriate validation feedback
   - Group related form elements logically

3. **Card Patterns**
   - Apply consistent card styling with appropriate presets
   - Use cards for containing related content
   - Maintain proper spacing within and between cards
   - Use appropriate card variants for different content types

4. **Interactive Element Patterns**
   - Apply consistent focus and hover states
   - Implement clear active state indicators
   - Ensure interactive elements are obviously clickable/tappable
   - Provide appropriate feedback for interaction

By following these design guidelines, we'll create a cohesive and consistent user interface that leverages the strengths of Skeleton UI while maintaining the application's unique identity through the custom `repu-crimson` theme.

## Theme Management

### Separating Custom Extensions

To maintain the ability to update the base theme while preserving custom extensions:

1. Create a separate file for extensions: `src/lib/themes/repu-crimson-extensions.css`
2. Update the import order in `app.css`:

```css
/* Import base Skeleton styles */
@import '@skeletonlabs/skeleton';

/* Import base theme */
@import '../src/lib/themes/repu-crimson.css';

/* Import theme extensions */
@import '../src/lib/themes/repu-crimson-extensions.css';

/* Import Skeleton presets */
@import '@skeletonlabs/skeleton/optional/presets';
```

This approach allows updating the base theme independently from custom extensions.

## Loading State Management

Each async operation should follow this pattern:

1. **Initial State**: Show UI in ready state
2. **Loading State**: 
   - Display appropriate loading indicator (progress bar, skeleton placeholder)
   - Disable interactive elements to prevent duplicate submissions
3. **Success State**: 
   - Display success notification using Toast component
   - Update UI to reflect new state
4. **Error State**:
   - Display error message using Alert component
   - Maintain form data where possible
   - Provide clear recovery instructions

Implementation example:

```svelte
<script>
  import { ProgressBar, Toast } from '@skeletonlabs/skeleton-svelte';
  import { toastStore } from '@skeletonlabs/skeleton';
  
  let loading = false;
  
  async function handleSubmit() {
    loading = true;
    
    try {
      await submitData();
      toastStore.trigger({
        message: 'Data submitted successfully!',
        background: 'preset-filled-success-500'
      });
    } catch (error) {
      toastStore.trigger({
        message: `Error: ${error.message}`,
        background: 'preset-filled-error-500'
      });
    } finally {
      loading = false;
    }
  }
</script>

<form on:submit|preventDefault={handleSubmit}>
  <!-- Form fields -->
  
  {#if loading}
    <ProgressBar />
  {:else}
    <button type="submit" class="btn preset-filled-primary-500">Submit</button>
  {/if}
</form>
```

## Error Handling

Implement a consistent error handling strategy:

1. **Backend Errors**:
   - Create a central error handling service
   - Transform backend error responses into user-friendly messages
   - Display errors using appropriate components (Toast, Alert, inline validation)

2. **Validation Errors**:
   - Use Skeleton's form validation patterns
   - Display errors inline with relevant form fields
   - Provide clear guidance on how to correct errors

3. **Network/System Errors**:
   - Implement retry mechanisms where appropriate
   - Provide clear fallback options
   - Store unsaved data where possible to prevent loss

Implementation example:

```svelte
<script>
  import { Input } from '@skeletonlabs/skeleton-svelte';
  
  let formData = { email: '', password: '' };
  let errors = {};
  
  function validate() {
    errors = {};
    
    if (!formData.email) {
      errors.email = 'Email is required';
    }
    
    if (!formData.password) {
      errors.password = 'Password is required';
    }
    
    return Object.keys(errors).length === 0;
  }
  
  async function handleSubmit() {
    if (!validate()) return;
    
    try {
      // Submit data
    } catch (error) {
      // Handle error
    }
  }
</script>

<form on:submit|preventDefault={handleSubmit}>
  <div class="space-y-4">
    <label class="label">
      <span>Email</span>
      <Input
        type="email"
        bind:value={formData.email}
        placeholder="Enter your email"
        class={errors.email ? 'input-error' : ''}
      />
      {#if errors.email}
        <span class="text-error-500">{errors.email}</span>
      {/if}
    </label>
    
    <!-- Additional fields -->
    
    <button type="submit" class="btn preset-filled-primary-500">Submit</button>
  </div>
</form>
```

## Additional Resources

- [Skeleton Documentation](https://www.skeleton.dev/docs/get-started/introduction)
- [Skeleton Fundamentals](https://www.skeleton.dev/docs/get-started/fundamentals)
- [Skeleton Layouts Guide](https://www.skeleton.dev/docs/guides/layouts)
- [Skeleton Presets](https://www.skeleton.dev/docs/design/presets)
- [Skeleton Components](https://www.skeleton.dev/docs/components)

## Open Questions & Future Considerations

- Should the sidebar be persistent or collapsible on desktop/mobile?
- What navigation items or contextual content should the sidebar contain by default?
- How should the hero section on the homepage be styled and what content should it include?
- Are there any additional onboarding steps or user profile fields needed?
- How should we handle avatar uploads and display (integration with Juno storage)?
- What are the accessibility requirements for all layouts and components?

---

(Please review and add any further questions or considerations as we proceed.)

## Authentication State & Header Button Implementation (2024-06-XX)

### Global Auth State
- Implemented a global Svelte store (`src/lib/stores/authUser.ts`) that subscribes to Juno's `authSubscribe`.
- All components can now import and use `$authUser` for reactive authentication state.
- Reference: [Juno Auth Subscription Docs](../../juno/docs/build/authentication/development.md#subscription)

### Header Login/Logout Button
- Header now uses `$authUser` to show either a "Login" (filled) or "Logout" (outlined) button, using Skeleton UI button classes.
- On login, calls `signIn()` and redirects to `LOGIN_REDIRECT_URL` (default: `/reputations`).
- On logout, calls `signOut()` and redirects to `LOGOUT_REDIRECT_URL` (default: `/`).
- Error handling and accessibility attributes included.
- Reference: [Skeleton UI Button Presets](https://www.skeleton.dev/docs/components/button#presets)

### Configurable Redirects
- Added `LOGIN_REDIRECT_URL` and `LOGOUT_REDIRECT_URL` to `src/lib/settings.ts` for easy future changes.

### Incremental Rollout
- Header implementation complete and ready for review.
- Next: Update homepage and other pages to use the global store, then remove old local auth logic.

### Toast Notifications for Auth Feedback
- Migrated from inline error messages in the header to Skeleton Toast notifications for login/logout feedback.
- Uses a shared toaster instance (`src/lib/toaster-skeleton.ts`) and the `<Toaster>` component in the root layout.
- Login and logout now use `toaster.promise()` for async feedback: loading, success, and error states.
- Reference: [Skeleton Toast Docs](https://www.skeleton.dev/docs/components/toast/svelte)

---

(Please review and add any further questions or considerations as we proceed.) 