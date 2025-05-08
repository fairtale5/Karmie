# UI/UX Guidelines

## Overview

This document outlines the high-level UI/UX decisions and guidelines for the Reputator project. For specific component implementations, refer to the frontend code and its documentation.

All info about frontend found in docs/resources/llms-svelte.md

## Technical Stack

### SvelteKit + Tailwind CSS
- Using standard SvelteKit setup with Tailwind CSS
- Static site generation (no server-side rendering)
- Client-side routing and state management
- Modern Tailwind v4 utilities and features

### Important Technical Requirements

#### Static Site Generation
- Project uses static site generation with Juno
- All routes must be pre-rendered at build time
- Avoid using server-side only features:
  - No `+layout.server.ts` files
  - No `+page.server.ts` files
  - No `+layout.ts` files with server-side logic
  - No `actions` in forms
- Use client-side alternatives where possible

#### Route Configuration
All routes in a Juno project should use these settings:
```typescript
export const prerender = true;  // Enable static generation
export const ssr = false;       // Disable server-side rendering
export const csr = true;        // Enable client-side rendering
```

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
- Use Tailwind CSS for consistent styling
- Support light and dark modes through Tailwind
- Use CSS variables for custom theming
- Follow responsive design principles

### 2. Component Styling
- Use Tailwind utility classes for consistent styling
- Follow Tailwind's spacing scale
- Use Tailwind's color system
- Leverage Tailwind's responsive utilities

## CSS Structure

### 1. Global Styles
- Import Tailwind in `src/app.css`
- Define custom theme variables in `app.css`
- Keep global styles minimal
- Use Tailwind's configuration for customization

### 2. Component Styles
- Use Tailwind utility classes primarily
- Add custom styles through `<style>` when needed
- Use `@apply` for complex utility combinations
- Keep component styles scoped

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
