import { createApp } from 'vue'
import App from './App.vue'
import './index.css'
import vuetify from './plugins/vuetify'
import pinia from './plugins/pinia'
import router from './router'

const app = createApp(App)

app.use(pinia)
app.use(router)
app.use(vuetify)

app.mount('#app')
