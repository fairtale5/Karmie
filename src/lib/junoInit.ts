import { initSatellite } from '@junobuild/core';

// Satellite IDs for different environments
const SATELLITE_ID = {
    local: 'jx5yt-yyaaa-aaaal-abzbq-cai',    // Local emulator ID (fixed)
    production: 'rigfr-siaaa-aaaal-ab4fa-cai' // Your production satellite ID
};

export const initSatelliteConnection = async () => {
    try {
        await initSatellite({
            satelliteId: import.meta.env.DEV ? SATELLITE_ID.local : SATELLITE_ID.production,
            container: import.meta.env.DEV // Enable local container in dev mode
        });
        
        const envName = import.meta.env.DEV ? 'local' : 'production';
        console.log(`Initialized Juno in ${envName} mode`);
    } catch (err) {
        console.error('Satellite initialization error:', err);
    }
}; 