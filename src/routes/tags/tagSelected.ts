import { writable } from 'svelte/store';
import type { TagDocument } from '$lib/types';

export const selectedTag = $state<TagDocument | null>(null); 