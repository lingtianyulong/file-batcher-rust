import 'element-plus/dist/index.css';
import 'primeicons/primeicons.css';

import * as ElementPlusIconsVue from '@element-plus/icons-vue';
import Aura from '@primevue/themes/aura';
import ElementPlus from 'element-plus';
import PrimeVue from 'primevue/config';
import Tooltip from 'primevue/tooltip'
import {createApp} from 'vue';

import App from './App.vue';
import router from './router';


const app = createApp(App);
app.use(ElementPlus).use(router).use(PrimeVue, {
  theme: {
    preset: Aura,
  },
});
app.directive('tooltip', Tooltip);
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}
app.mount('#app');