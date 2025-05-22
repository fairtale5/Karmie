import { writable } from 'svelte/store';

export interface PageMeta {
    title?: string;
    description?: string;
    author?: string;
    // Add more fields as needed
}

export const page = writable<PageMeta>({});

export function setPageMeta(meta: PageMeta) {
    page.set(meta);
} 