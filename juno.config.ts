import {defineConfig} from '@junobuild/config';

export default defineConfig({
  satellite: {
    // For local development (using emulator)
    // id: 'jx5yt-yyaaa-aaaal-abzbq-cai',
    
    // For production (uncomment when deploying)
    id: 'rigfr-siaaa-aaaal-ab4fa-cai',
    
    source: 'build',
    predeploy: ['npm run build']
  }
});
