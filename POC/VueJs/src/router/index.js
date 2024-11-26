import { createRouter, createWebHistory } from 'vue-router'
import JsonApiView from '@/views/JsonApiView.vue'
import PostJsonApiView from '@/views/PostJsonApiView.vue'
import PutJsonApiView from '@/views/PutJsonApiView.vue'
import DeleteJsonApiView from '@/views/DeleteJsonApiView.vue'
import GetByIdJsonApiView from '@/views/GetByIdJsonApiView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/get',
      name: 'get',
      component: JsonApiView,
    },
    {
      path: '/post',
      name: 'post',
      component: PostJsonApiView
    },
    {
      path: '/put',
      name: 'put',
      component: PutJsonApiView
    },
    {
      path: '/delete',
      name: 'delete',
      component: DeleteJsonApiView
    },
    {
      path: '/getById',
      name: 'getById',
      component: GetByIdJsonApiView
    }
  ],
})

export default router
