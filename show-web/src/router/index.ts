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
      redirect: '/wayfind/maps',
      component: () => import('../views/WayFindView.vue'),
      children: [
        {
          path: 'maps',
          name: 'wayfind-maps',
          component: () => import('../views/WayFindMaps.vue')
        },
        {
          path: 'maps/edit',
          name: 'wayfind-map-edit',
          component: () => import('../views/WayFindMapEditor.vue')
        },
        {
          path: 'tasks',
          name: 'wayfind-tasks',
          component: () => import('../views/WayFindTasks.vue')
        },
        {
          path: 'inference',
          name: 'wayfind-inference',
          component: () => import('../views/WayFindInference.vue')
        },
        {
          path: 'inference/detail',
          name: 'wayfind-inference-detail',
          component: () => import('../views/WayFindInferenceDetail.vue')
        },
        {
          path: 'algorithm',
          name: 'wayfind-algorithm',
          component: () => import('../views/WayFindAlgorithm.vue')
        }
      ]
    },
    {
      path: '/professor',
      name: 'professor',
      component: () => import('../views/ProfessorView.vue')
    },
    {
        path: '/mlearn',
        name: 'mlearn',
        component: () => import('../views/MLearnView.vue'),
        redirect: '/mlearn/task',
        children: [
          {
            path: 'task',
            name: 'mlearn-task',
            component: () => import('../views/MLearnTask.vue')
          },
          {
            path: 'regression',
            name: 'mlearn-regression',
            component: () => import('../views/MLearnRegression.vue')
          },
          {
            path: 'genetic',
            name: 'mlearn-genetic',
            component: () => import('../views/MLearnGenetic.vue')
          }
        ]
      },
    {
      path: '/:pathMatch(.*)*',
      name: 'not-found',
      component: () => import('../views/NotFoundView.vue')
    }
  ]
})

export default router
