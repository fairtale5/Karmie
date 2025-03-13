import {defineConfig} from '@junobuild/config';

export default defineConfig({
  satellite: {
    id: 'rigfr-siaaa-aaaal-ab4fa-cai',
    source: 'build',
    predeploy: ['npm run build']
  }
});
