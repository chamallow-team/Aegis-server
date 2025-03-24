import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('@/views/HomeView.vue')
    },
    {
      path: '/editor',
      name: 'editor',
      component: () => import('@/views/editor/EditorView.vue'),
      children: [
        {
          path: 'entites',
          name: 'editor_entities',
          components: {
            body: () => import('@/views/editor/EntitiesView.vue')
          }
        }
      ]
    }
  ]
})

export default router
