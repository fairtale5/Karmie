// File will be renamed to junoStore.ts
import { writable } from 'svelte/store';

interface JunoStatus {
    initialized: boolean;
    error: string | null;
}

export const junoStatus = writable<JunoStatus>({
    initialized: false,
    error: null
}); 