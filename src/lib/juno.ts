import { initJuno as initJunoCore } from '@junobuild/core';

export const initJuno = async () => {
    try {
        await initJunoCore({
            satelliteId: 'rigfr-siaaa-aaaal-ab4fa-cai'
        });
        console.log('Juno initialized successfully');
    } catch (err) {
        console.error('Failed to initialize Juno:', err);
    }
}; 