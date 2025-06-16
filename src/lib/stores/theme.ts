import { writable } from 'svelte/store';
import { browser } from '$app/environment';

/**
 * Central theme management store
 * 
 * HOW IT WORKS:
 * - Svelte writable store triggers reactive updates across all subscribers
 * - localStorage persists user preference across browser sessions
 * - Layout reactive statement ($:) automatically updates DOM when store value changes
 */

function createThemeStore() {
	const { subscribe, set, update } = writable<'light' | 'dark'>('dark');

	return {
		subscribe,
		/**
         * Initialize theme - check localStorage only
		 * HOW: Queries browser localStorage API, then defaults to dark
		 * - localStorage.getItem() returns saved string or null
		 * - set() triggers Svelte's reactive system to update all subscribers
		 */
		init: () => {
			if (!browser) return;

			// HOW: Browser localStorage API persists data across sessions as strings
			const stored = localStorage.getItem('mode');
			if (stored) {
				set(stored as 'light' | 'dark');
				return;
			}

			// Default to dark mode
			set('dark');
		},

		/**
		 * HOW: Svelte's update() function receives current value, returns new value
		 * - update() automatically triggers reactive statements in components
		 * - localStorage.setItem() persists choice as string for next session
		 */
		toggle: () => {
			update(currentMode => {
				const newMode = currentMode === 'light' ? 'dark' : 'light';
				
				// HOW: localStorage.setItem() writes to browser's persistent storage
				if (browser) {
					localStorage.setItem('mode', newMode);
				}
				
				return newMode;
			});
		}
	};
}

export const themeStore = createThemeStore(); 