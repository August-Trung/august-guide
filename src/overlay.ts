import { createApp } from 'vue'
import OverlayApp from './OverlayApp.vue'
import vuetify from './plugins/vuetify'
import pinia from './plugins/pinia'

const app = createApp(OverlayApp)

app.use(pinia)
app.use(vuetify)

app.mount('#app')
