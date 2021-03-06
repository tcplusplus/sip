import Vue from 'vue';
import VueRouter, { RouteConfig } from 'vue-router';

Vue.use(VueRouter);

const routes: RouteConfig[] = [
  {
    path: '/',
    name: 'Home',
    component: () => import(/* webpackChunkName: "home" */ '../views/Home.vue'),
  },
  {
    path: '/simulator',
    name: 'Simulator',
    component: () => import(/* webpackChunkName: "home" */ '../views/Simulator.vue'),
  },
  {
    path: '/sip/dist',
    name: 'Simulator',
    component: () => import(/* webpackChunkName: "home" */ '../views/Simulator.vue'),
  },
];

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes,
});

export default router;
