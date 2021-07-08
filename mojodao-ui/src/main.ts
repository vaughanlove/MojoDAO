import { createApp } from 'vue'
import App from './App.vue'
import router from './router/index'
import './assets/global.css'
import VueScrollmagic from 'vue-scrollmagic'


createApp(App).use(router, VueScrollmagic).mount('#app')

