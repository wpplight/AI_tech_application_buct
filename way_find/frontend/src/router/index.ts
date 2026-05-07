import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: () => import('@/views/DashboardView.vue')
    },
    {
      path: '/maps',
      name: 'maps',
      component: () => import('@/views/MapsListView.vue')
    },
    {
      path: '/maps/new',
      name: 'map-new',
      component: () => import('@/views/MapEditorView.vue')
    },
    {
      path: '/maps/:id/edit',
      name: 'map-edit',
      component: () => import('@/views/MapEditorView.vue')
    },
    {
      path: '/displays',
      name: 'displays',
      component: () => import('@/views/DisplaysListView.vue')
    },
    {
      path: '/displays/new',
      name: 'display-new',
      component: () => import('@/views/DisplaySetupView.vue')
    },
    {
      path: '/displays/:id',
      name: 'display',
      component: () => import('@/views/DisplayView.vue')
    },
  ],
})

export default router
