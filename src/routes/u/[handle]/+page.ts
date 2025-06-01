import type { PageLoad } from './$types';

export const prerender = false; // Disable prerendering for dynamic routes

export const load: PageLoad = async ({ params }) => {
  return {
    handle: params.handle
  };
}; 