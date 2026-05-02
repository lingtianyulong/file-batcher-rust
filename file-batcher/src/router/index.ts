import {createRouter, createWebHashHistory} from 'vue-router';

const EmptyRouteView = {
  template: '<div></div>',
};

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      redirect: '/home',
    },
    {
      path: '/home',
      name: 'home',
      component: EmptyRouteView,
    },
    {
      path: '/settings',
      name: 'settings',
      component: EmptyRouteView,
    },
    {
      path: '/batch',
      name: 'batch',
      component: EmptyRouteView,
    },
  ],
});

export default router;
