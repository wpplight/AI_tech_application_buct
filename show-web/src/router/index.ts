import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('../views/HomeView.vue')
    },
    {
      path: '/wayfind',
      name: 'wayfind',
      component: () => import('../views/WayFindView.vue')
    },
    {
      path: '/professor',
      name: 'professor',
      component: () => import('../views/ProfessorView.vue')
    },
    {
      path: '/mlearn',
      name: 'mlearn',
      component: () => import('../views/MLearnView.vue')
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'not-found',
      component: () => import('../views/NotFoundView.vue')
    }
  ]
})

export default router
