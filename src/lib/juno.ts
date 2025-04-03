import { initJuno as initJunoCore } from '@junobuild/core';

// Satellite IDs for different environments
const SATELLITE_ID = {
    local: 'jx5yt-yyaaa-aaaal-abzbq-cai',    // Local emulator ID (fixed)
    production: 'rigfr-siaaa-aaaal-ab4fa-cai' // Your production satellite ID
};

export const initJuno = async () => {
    try {
        await initJunoCore({
            satelliteId: import.meta.env.DEV ? SATELLITE_ID.local : SATELLITE_ID.production,
            container: import.meta.env.DEV // Enable local container in dev mode
        });
        console.log(`Juno initialized successfully in ${import.meta.env.DEV ? 'local' : 'production'} mode`);
    } catch (err) {
        console.error('Failed to initialize Juno:', err);
    }
}; 