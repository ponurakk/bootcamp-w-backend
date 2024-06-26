import { createPinia } from 'pinia';
import { createApp } from 'vue';
import "./index.css";
import './index.scss';
import App from './App.vue';

createApp(App).use(createPinia()).mount('#app');
