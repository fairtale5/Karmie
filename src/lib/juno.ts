import { initJuno as initJunoCore } from '@junobuild/core';
import { junoStatus } from './stores/junoStore';

export const initJuno = async () => {
    // Reset status before attempting initialization
    junoStatus.set({ initialized: false, error: null });

    try {
        await initJunoCore({
            satelliteId: 'rigfr-siaaa-aaaal-ab4fa-cai'
        });
        junoStatus.set({ initialized: true, error: null });
        console.log('Juno initialized successfully');
    } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Failed to initialize Juno';
        junoStatus.set({ initialized: false, error: errorMessage });
        console.error('Failed to initialize Juno:', err);
    }
}; 