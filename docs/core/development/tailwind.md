# Custom Tailwind 4 Setup

## Overview

This project uses a custom version of Tailwind CSS (version 4) specifically designed for Juno integration. The setup consists of several specialized packages that work together to provide optimized CSS processing for Juno's static site generation.

## Key Differences from Classic Tailwind

### 1. Single Plugin Architecture
Unlike classic Tailwind which uses multiple plugins (tailwindcss, autoprefixer, etc.), Juno's Tailwind 4 uses a single plugin approach:
- ✅ Use `@tailwindcss/postcss` as the only PostCSS plugin
- ❌ Don't use autoprefixer (handled internally)
- ❌ Don't use standard Tailwind plugins

### 2. Import Structure
Classic Tailwind typically uses global CSS imports in layout files. For Juno's Tailwind 4:
- ✅ Import app.css in the root page component (`src/routes/+page.svelte`)
- ❌ Don't import in layout files
- ❌ Don't use global stylesheets in SvelteKit config

### 3. Configuration Files
- ✅ Use minimal PostCSS config with only `@tailwindcss/postcss`
- ❌ Don't create tailwind.config.js (handled by Juno)
- ❌ Don't use standard Tailwind plugins or themes

## Correct Setup

### 1. Package Structure
```json
{
  "devDependencies": {
    "@tailwindcss/postcss": "^4.0.0",
    "@tailwindcss/vite": "^4.0.6",
    "tailwindcss": "^4.0.0"
  }
}
```

### 2. PostCSS Configuration
```javascript
// postcss.config.js
export default {
  plugins: {
    '@tailwindcss/postcss': {},  // Single plugin only
  },
};
```

### 3. CSS Import Location
```typescript
// src/routes/+page.svelte
import '../app.css';  // Import at root page level
```

### 4. CSS Structure
```css
/* src/app.css */
@tailwind base;
@tailwind components;
@tailwind utilities;

/* Custom styles below */
```

## Build Process

### 1. Development Build
The process differs from classic Tailwind:
1. Vite plugin (`@tailwindcss/vite`) initializes first
2. PostCSS plugin processes CSS through Rust-based oxide
3. Platform-specific optimizations are applied
4. No runtime processing or JIT compilation

### 2. Production Build
1. Static analysis of used classes
2. Single-pass CSS generation
3. Platform-specific optimizations
4. No purge step needed (handled internally)

## Common Mistakes to Avoid

### 1. Classic Tailwind Patterns
❌ Don't use these classic Tailwind patterns:
- Multiple PostCSS plugins
- Tailwind config file
- JIT mode configuration
- Purge configuration

### 2. Import Patterns
❌ Don't use these import patterns:
- Global CSS in app.html
- CSS in layout files
- Multiple CSS entry points

### 3. Plugin Usage
❌ Don't use these plugin patterns:
- Standard Tailwind plugins
- autoprefixer
- postcss-nesting
- Other PostCSS plugins

## Debugging

### 1. Build Issues
If CSS is not processing:
1. Check import location (should be in +page.svelte)
2. Verify PostCSS config has only @tailwindcss/postcss
3. Check package versions match (all 4.x)

### 2. Style Issues
If styles aren't applying:
1. Verify CSS import in root page
2. Check for classic Tailwind patterns
3. Remove any competing CSS processors

### 3. Performance Issues
If build is slow:
1. Check for multiple CSS entry points
2. Remove unnecessary PostCSS plugins
3. Verify platform-specific oxide package is installed

## Package Structure

### Core Packages
- `@tailwindcss/postcss` (v4.0.0): Custom PostCSS plugin for Tailwind 4
- `@tailwindcss/vite` (v4.0.6): Vite plugin for Tailwind 4 integration
- `tailwindcss` (v4.0.0): The base Tailwind 4 package

### Platform-Specific Packages
The setup includes platform-specific optimizations through the `@tailwindcss/oxide` packages:
- `@tailwindcss/oxide-android-arm64`
- `@tailwindcss/oxide-darwin-arm64`
- `@tailwindcss/oxide-darwin-x64`
- `@tailwindcss/oxide-freebsd-x64`
- `@tailwindcss/oxide-linux-arm-gnueabihf`
- `@tailwindcss/oxide-linux-arm64-gnu`
- `@tailwindcss/oxide-linux-arm64-musl`
- `@tailwindcss/oxide-linux-x64-gnu`
- `@tailwindcss/oxide-linux-x64-musl`
- `@tailwindcss/oxide-win32-arm64-msvc`
- `@tailwindcss/oxide-win32-x64-msvc`

## Configuration

### PostCSS Configuration
```javascript
// postcss.config.js
export default {
  plugins: {
    '@tailwindcss/postcss': {},
  },
};
```

### Vite Configuration
```typescript
// vite.config.ts
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  plugins: [
    sveltekit(),
    juno({ container: true }),
    tailwindcss()
  ],
  // ... other config
});
```

## How It Works

1. **Build Process**:
   - The Vite plugin (`@tailwindcss/vite`) integrates Tailwind with the build process
   - PostCSS plugin (`@tailwindcss/postcss`) processes Tailwind directives
   - Platform-specific optimizations are applied through the oxide packages

2. **CSS Processing**:
   - Tailwind directives in `app.css` are processed during build
   - The custom PostCSS plugin handles the transformation
   - Platform-specific optimizations are applied for better performance

3. **Integration with Juno**:
   - The setup is optimized for Juno's static site generation
   - CSS is processed at build time
   - No runtime CSS processing is needed

## Usage

1. **CSS Directives**:
   ```css
   @tailwind base;
   @tailwind components;
   @tailwind utilities;
   ```

2. **Tailwind Classes**:
   - Use standard Tailwind classes in your components
   - Classes are processed at build time
   - Only used classes are included in the final CSS

3. **Custom Styles**:
   - Add custom styles in `app.css`
   - Use `@apply` directive for custom components
   - Follow Tailwind's utility-first approach

## Important Notes

1. **Version Compatibility**:
   - All packages must be version 4.x
   - Mixing with standard Tailwind packages may cause issues
   - Keep all packages in sync

2. **Build Process**:
   - CSS is processed during build
   - No runtime CSS processing
   - Optimized for static site generation

3. **Platform Support**:
   - Includes optimizations for various platforms
   - Automatically selects appropriate optimizations
   - No manual configuration needed

## Troubleshooting

1. **CSS Not Processing**:
   - Check PostCSS configuration
   - Verify Vite plugin is properly configured
   - Ensure all packages are version 4.x

2. **Build Errors**:
   - Check for version mismatches
   - Verify platform-specific packages are installed
   - Check for syntax errors in CSS files

3. **Performance Issues**:
   - Monitor CSS bundle size
   - Check for unused Tailwind classes
   - Verify platform-specific optimizations are working 