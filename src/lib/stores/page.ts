import { writable } from 'svelte/store';

export interface PageMeta {
    title?: string;           // Used in browser tab title
    headerTitle?: string;     // Used in header display (falls back to title if not set)
    description?: string;
    author?: string;
    // Add more fields as needed
}

export const page = writable<PageMeta>({});

export function setPageMeta(meta: PageMeta) {
    page.set(meta);
} 