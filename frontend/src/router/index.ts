import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'

const router = createRouter({
  // GitHub Pages 用 hash 模式，不需要服务端配置
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/item',
      name: 'item',
      component: () => import('@/views/ItemView.vue'),
      props: route => ({ path: route.query.path, category: route.query.category }),
    },
    {
      path: '/raw',
      name: 'raw',
      component: () => import('@/views/RawView.vue'),
      props: route => ({ path: route.query.path }),
    },
    {
      path: '/category/:key',
      name: 'category',
      component: () => import('@/views/CategoryView.vue'),
      props: true,
    },
  ],
  scrollBehavior: () => ({ top: 0 }),
})

export default router
