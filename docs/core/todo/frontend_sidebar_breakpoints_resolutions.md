https://github.com/fairtale5/Karmie/issues/25

# Fix: Dynamic Responsive Breakpoints for Sidebar-Aware Layout

## Problem Description

Our component breakpoints are inconsistent because the available content width changes dramatically based on sidebar state, but our responsive breakpoints are fixed to viewport width instead of available content width.

## Current Behavior

**Sidebar Dimensions:**
- **Narrow/Collapsed**: 72px width
- **Expanded**: 256px width
- **Difference**: 184px less available content space when expanded

**Issue Manifestation:**
- When sidebar is **narrow (72px)**: Components have ample space, breakpoints work perfectly
- When sidebar is **expanded (256px)**: 184px less content space causes layout breaks
- Components like `QuickActions` (using `grid-cols-4`) and dashboard grids break on certain screen sizes when sidebar is expanded

**Root Cause:**
Our responsive breakpoints (`md:`, `lg:`, `xl:`) are based on viewport width, not available content width. A 1024px viewport gives different effective content space depending on sidebar state:
- Narrow sidebar: ~952px content width (1024 - 72)
- Expanded sidebar: ~768px content width (1024 - 256)

This 184px difference means components that work at "md" breakpoint with narrow sidebar fail with expanded sidebar.

## Technical Context

**Current Implementation:**
- Sidebar state managed in `src/lib/components/layout/AppShell.svelte`
- Sidebar width controlled via `style="width: {isExpanded ? '256px' : '72px'}"`
- Components use standard Tailwind breakpoints: `md:grid-cols-2`, `lg:grid-cols-3`, etc.
- Main content area uses `grid grid-cols-1 md:grid-cols-[auto_1fr]`

**Affected Components:**
- Dashboard grid layouts (`grid grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3`)
- QuickActions component (`grid grid-cols-4`)
- Profile page layouts
- Tag management interfaces
- Any component using responsive grid/flex layouts

## Solution Options Considered

### Option 1: Dual Breakpoint Sets ❌
Create two sets of breakpoints (`md:` and `sidebar-md:`) and manually choose which to use.
- **Pros**: Simple Tailwind configuration
- **Cons**: Maintenance nightmare, manual decision-making, inconsistent application

### Option 2: Container Queries ❌
Use CSS container queries to make components responsive to parent container size.
- **Pros**: Modern CSS approach
- **Cons**: Browser support concerns, requires plugin, adds complexity

### Option 3: CSS Custom Properties ❌
Dynamic CSS variables that calculate available content width.
- **Pros**: Flexible
- **Cons**: Complex setup, custom CSS required

### Option 4: Dynamic Breakpoint Adjustment ✅ **CHOSEN SOLUTION**
Dynamically adjust the default breakpoints by subtracting current sidebar width.
- **Pros**: Single source of truth, zero component changes, system-wide consistency
- **Cons**: Requires custom implementation

## Chosen Solution: Dynamic Breakpoint Adjustment

**Core Concept:**
Instead of having two sets of breakpoints, dynamically adjust the effective breakpoints based on sidebar state:
```
effectiveBreakpoint = defaultBreakpoint - currentSidebarWidth
```

**Core Technical Challenge:**
The sidebar state (`isExpanded`) is currently a local variable in `AppShell.svelte`, but the global CSS system needs access to this state to calculate dynamic breakpoints.

**Implementation Strategy:**
1. **Bridge Component State to Global CSS**: Use CSS Custom Properties on `:root` to expose sidebar state
2. **JavaScript Calculation**: AppShell reactively sets CSS variables when sidebar state changes
3. **CSS Custom Properties**: Set calculated breakpoint values as CSS variables
4. **Tailwind Override**: Make default breakpoints (`md:`, `lg:`) use dynamic variables
5. **Automatic Propagation**: All existing responsive classes automatically get adjusted behavior

**State Bridge Implementation:**
```css
:root {
  --sidebar-width: 256px; /* or 72px based on isExpanded */
  --md-breakpoint: calc(768px + var(--sidebar-width));
  --lg-breakpoint: calc(1024px + var(--sidebar-width));
  --xl-breakpoint: calc(1280px + var(--sidebar-width));
}
```

**How State Flows:**
1. `AppShell.svelte` has `isExpanded` boolean
2. Svelte reactive statement (`$:`) updates CSS custom properties on document root
3. CSS media queries use calculated breakpoint values
4. All responsive classes automatically adjust to new breakpoints

**Benefits:**
- **Zero Component Changes**: All existing `md:`, `lg:` classes work automatically
- **Single Source of Truth**: Sidebar width change affects entire system
- **Invisible to Developers**: No special syntax or decision-making needed
- **Maintainable**: Change sidebar dimensions in one place, everything adjusts
- **Consistent**: Can't be applied inconsistently because it's system-wide

## Expected Outcome

After implementation:
- Components will be responsive to **available content width** instead of viewport width
- Same responsive behavior regardless of sidebar state
- No component-level changes required
- Sidebar width changes automatically propagate to all responsive layouts

## Implementation Notes

- Implementation should be in `src/lib/components/layout/AppShell.svelte`
- May require custom CSS and JavaScript to override Tailwind's default media queries
- Should maintain compatibility with existing responsive classes
- Consider performance implications of dynamic CSS custom property updates

## Priority

**High** - This affects user experience across all major application interfaces and creates inconsistent responsive behavior. 