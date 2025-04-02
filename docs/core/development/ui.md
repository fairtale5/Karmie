# UI/UX Guidelines

## Overview

This document outlines the high-level UI/UX decisions and guidelines for the Reputator project. For specific component implementations, refer to the frontend code and its documentation.

## Important Technical Requirements

### No Server-Side Rendering
- The project uses static site generation with Juno
- All routes must be pre-rendered at build time
- Avoid using server-side only features:
  - No `+layout.server.ts` files
  - No `+page.server.ts` files
  - No `+layout.ts` files with server-side logic
  - No `actions` in forms
- Use client-side alternatives where possible
- Import global styles in the root page component (`src/routes/+page.svelte`)

## Design Principles

### 1. User Experience
- **Clarity**: Make reputation scores and voting actions immediately clear
- **Feedback**: Provide immediate visual feedback for all user actions
- **Consistency**: Maintain consistent patterns across all interfaces
- **Efficiency**: Minimize clicks needed for common actions

### 2. Accessibility
- **ARIA Labels**: All interactive elements must have descriptive ARIA labels
- **Keyboard Navigation**: Support full keyboard navigation
- **Color Contrast**: Maintain WCAG 2.1 AA compliance for color contrast
- **Screen Readers**: Ensure all content is accessible to screen readers

### 3. Performance
- **Loading States**: Show appropriate loading indicators
- **Optimistic Updates**: Update UI immediately, handle errors gracefully
- **Lazy Loading**: Load detailed information on demand
- **Virtual Scrolling**: Use for long lists of votes or users

## Component Guidelines

### 1. User Components
- Display user information clearly and consistently
- Show reputation scores prominently
- Include vote actions where appropriate
- Support inline editing for user details

### 2. Vote Components
- Make voting actions simple and clear
- Show vote weight previews
- Provide confirmation for negative votes
- Display vote history in an organized manner

### 3. Reputation Components
- Show reputation trends clearly
- Use appropriate visualizations for data
- Support filtering and sorting
- Include tooltips for detailed information

## State Management Guidelines

### 1. Data Flow
- Use centralized state management
- Implement optimistic updates
- Handle offline scenarios
- Maintain data consistency

### 2. Error Handling
- Show user-friendly error messages
- Provide recovery options
- Log errors appropriately
- Handle edge cases gracefully

## Styling Guidelines

### 1. Theme System
- Use CSS variables for consistent styling
- Support light and dark modes
- Maintain consistent spacing
- Follow responsive design principles

### 2. Component Styling
- Use consistent border radiuses
- Maintain consistent spacing
- Follow color system guidelines
- Support responsive layouts

## Future Improvements

### 1. Enhanced Visualizations
- Implement reputation history graphs
- Add vote weight distribution charts
- Create user activity heatmaps
- Support custom visualization options

### 2. Advanced Filtering
- Add date range selectors
- Implement tag-based filtering
- Support reputation threshold filters
- Add custom filter combinations

### 3. Mobile Optimization
- Optimize layouts for mobile
- Implement touch-friendly interactions
- Add mobile-specific features
- Support offline functionality

### 4. Accessibility Enhancements
- Add high contrast mode
- Improve keyboard navigation
- Enhance screen reader support
- Add focus management

## SvelteKit Integration Guidelines

### Important Limitations
- **No Server-Side Rendering (SSR)**
  - Juno requires static site generation
  - Avoid using server-side only features like:
    - `+layout.server.ts` files
    - `+page.server.ts` files
    - `+layout.ts` files with server-side logic
    - `actions` in forms
  - Use client-side alternatives where possible
  - All routes should be pre-rendered at build time

### Recommended Patterns
- Use static generation with `+page.ts` instead of server files
- Handle authentication and data fetching on the client side
- Use client-side routing for dynamic content

## SvelteKit Route Configuration

### Required Settings
All routes in a Juno project should use these settings:
```typescript
export const prerender = true;  // Enable static generation
export const ssr = false;       // Disable server-side rendering
export const csr = true;        // Enable client-side rendering
```

### Why These Settings?
- `prerender = true`: Required for Juno's static hosting
- `ssr = false`: Juno doesn't support server-side rendering
- `csr = true`: Enables client-side functionality

### Common Issues
- Never set `prerender = false` as it prevents static generation
- Always keep `ssr = false` for Juno compatibility
- Maintain consistent settings across all routes 

## CSS and Styling

### Tailwind Integration
This project uses Juno's custom Tailwind 4 setup, which differs significantly from standard Tailwind:

1. **Single Entry Point**
   - CSS must be imported in root page (`src/routes/+page.svelte`)
   - No global stylesheets in layout files
   - No CSS imports in `app.html`

2. **Build Process**
   - Uses Rust-based processing through oxide packages
   - No runtime CSS processing
   - No JIT compilation

3. **Configuration**
   - Minimal PostCSS setup with single plugin
   - No Tailwind config file needed
   - Platform-specific optimizations handled automatically

For detailed setup and troubleshooting, see `docs/core/development/tailwind.md`. 