# Frontend Setup Guide

## Overview

This guide covers the setup of the frontend stack for the Reputator project, which uses SvelteKit with Tailwind CSS for static site generation.

## Prerequisites

- Node.js installed in WSL
- Project directory mounted in WSL
- Basic familiarity with SvelteKit and Tailwind CSS

## Initial Setup

### 1. Verify Dependencies

Current project uses:
- Tailwind CSS v4.0.17
- @tailwindcss/vite v4.0.17
- PostCSS v8.5.3

To check your installed versions:
```bash
npm list tailwindcss @tailwindcss/vite postcss
```

### 2. Install Required Packages

If starting from scratch:
```bash
npm install tailwindcss@4.0.17 @tailwindcss/vite@4.0.17 postcss@8.5.3
```

### 3. Configure Vite

Update `vite.config.ts`:
```typescript
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  plugins: [
    tailwindcss(),
    sveltekit(),
  ],
});
```

## CSS Setup

### 1. Create Global CSS File

Create `src/app.css`:
```css
@import 'tailwindcss';

/* Custom theme variables */
@theme {
  /* Add your theme variables here */
}

/* Global styles */
html {
  /* Add your global styles here */
}
```

### 2. Import CSS in Layout

Create/update `src/routes/+layout.svelte`:
```svelte
<script>
  import "../app.css";
</script>

<slot />
```

## Route Configuration

### 1. Static Generation Setup

For each route, add these exports:
```typescript
// In +page.ts or +layout.ts
export const prerender = true;
export const ssr = false;
export const csr = true;
```

### 2. Client-Side Data Fetching

Example data fetching pattern:
```typescript
// In +page.ts
export const load = async () => {
  // Client-side data fetching
  return {
    // Your data here
  };
};
```

## Component Development

### 1. Using Tailwind Classes

Example component:
```svelte
<script lang="ts">
  export let title: string;
</script>

<div class="p-4 bg-white rounded-lg shadow">
  <h2 class="text-xl font-bold text-gray-800">{title}</h2>
  <slot />
</div>
```

### 2. Custom Styles

When needed, add component-specific styles:
```svelte
<style lang="postcss">
  @reference "tailwindcss";
  
  /* Your custom styles here */
</style>
```

## Development Workflow

### 1. Start Development Server

```bash
npm run dev
```

### 2. Build for Production

```bash
npm run build
```

### 3. Preview Production Build

```bash
npm run preview
```

## Best Practices

### 1. CSS Organization
- Use Tailwind utility classes as primary styling method
- Keep custom CSS minimal
- Use CSS variables for theming
- Follow Tailwind's responsive design patterns

### 2. Component Structure
- Keep components small and focused
- Use TypeScript for type safety
- Follow SvelteKit's file-based routing
- Implement proper prop validation

### 3. Performance
- Use static generation for all routes
- Implement lazy loading where appropriate
- Optimize images and assets
- Monitor bundle sizes

## Troubleshooting

### Common Issues

1. **Build Errors**
   - Verify Tailwind version compatibility
   - Check for proper plugin configuration
   - Ensure all dependencies are installed

2. **Styling Issues**
   - Verify Tailwind import in app.css
   - Check for proper class names
   - Validate PostCSS configuration

3. **Route Issues**
   - Confirm static generation settings
   - Verify client-side data fetching
   - Check for proper file naming

## Additional Resources

- [SvelteKit Documentation](https://kit.svelte.dev/docs)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)
- [Juno Documentation](https://juno.build) 