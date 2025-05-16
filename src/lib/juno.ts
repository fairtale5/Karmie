import { initSatellite } from '@junobuild/core';

// Satellite IDs for different environments
const SATELLITE_ID = {
    // Used when running `npm run dev` (development server with hot reload)
    // dev: 'jx5yt-yyaaa-aaaal-abzbq-cai', // Local emulator
    // dev: 'vjyvo-kyaaa-aaaal-asc5a-cai', // "Reputator2" Development Satellite
    dev: 'rigfr-siaaa-aaaal-ab4fa-cai', // "Reputator" Production Satellite
    // Used when running `npm run preview` (production build preview)
    preview: 'rigfr-siaaa-aaaal-ab4fa-cai'
};

export const initJuno = async () => {
    try {
        await initSatellite({
            // DEV is true for `npm run dev`, false for `npm run preview`
            satelliteId: import.meta.env.DEV ? SATELLITE_ID.dev : SATELLITE_ID.preview,
            // container: import.meta.env.DEV // Enable local container in dev mode (emulator)
            container: false // Always use remote satellite, never local emulator
        });
        console.log(`Juno initialized successfully in ${import.meta.env.DEV ? 'dev' : 'preview'} mode`);
    } catch (err) {
        console.error('Failed to initialize Juno:', err);
    }
}; 