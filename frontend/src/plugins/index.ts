/**
 * plugins/index.ts
 *
 * Automatically included in `./src/main.ts`
 */

// Plugins
import vuetify from './vuetify'
import router from '../router'
import AsFramework from './AsFramework'

// Types
import type { App } from 'vue'

export function registerPlugins(app: App) {
  app
    .use(AsFramework)
    .use(vuetify)
    .use(router)
}
