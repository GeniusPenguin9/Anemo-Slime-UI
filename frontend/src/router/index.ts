// Composables
import { createRouter, createWebHashHistory } from 'vue-router'
import { views } from "@/views/index"

function createViewRoutes() {
  const result = [];
  for (const [viewName, comp] of Object.entries(views)) {
    result.push({
      path: viewName,
      name: viewName,
      component: async () => await comp
    })
  }
  return result;
}

const routes = [
  {
    path: '/view',
    component: () => import('@/layouts/default/Default.vue'),
    children: createViewRoutes(),
  },
  {
    path: '/',
    redirect: "/view/example"
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
