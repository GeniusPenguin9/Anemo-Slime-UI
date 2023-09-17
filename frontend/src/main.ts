/**
 * main.ts
 *
 * Bootstraps Vuetify and other plugins then mounts the App`
 */

// Components
import App from './App.vue'
import AsButton from "./components/AsButton.vue"
import AsLabel from "./components/AsLabel.vue"

// Composables
import { createApp } from 'vue'

// Plugins
import { registerPlugins } from '@/plugins'

const app = createApp(App)

registerPlugins(app)

app
    .component("AsButton", AsButton)
    .component("AsLabel", AsLabel)
    .mount('#app')
