// Composables
import { createRouter, createWebHistory } from 'vue-router'
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
    redirect: "/view/ExampleView"
  }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
})

export default router
