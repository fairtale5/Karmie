import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: adapter({
			// Enable SPA mode with fallback for client-side routing
			fallback: 'index.html',
			// Output directories for the build
			pages: 'build',
			assets: 'build',
			// Disable precompression for better compatibility
			precompress: false,
			// Allow dynamic routes in static adapter
			strict: false
		}),
		// Configure prerendering to handle dynamic routes
		prerender: {
			handleMissingId: 'ignore' // Don't fail on dynamic routes
		}
	}
};

export default config;
