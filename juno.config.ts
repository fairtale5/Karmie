import {defineConfig} from '@junobuild/config';

export default defineConfig({
  satellite: {
    ids: {
      // development: 'jx5yt-yyaaa-aaaal-abzbq-cai', // For local development (using emulator)
      development: 'vjyvo-kyaaa-aaaal-asc5a-cai', // "Reputator2" Development Satellite
      production: 'rigfr-siaaa-aaaal-ab4fa-cai' // "Reputator" Production Satellite
    },
    source: 'build',
    predeploy: ['npm run build']
  }
});
