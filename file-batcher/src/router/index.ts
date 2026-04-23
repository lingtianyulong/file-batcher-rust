import { createRouter, createWebHashHistory } from "vue-router";

const EmptyRouteView = {
  template: "<div></div>",
};

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      redirect: "/rename",
    },
    {
      path: "/rename",
      name: "rename",
      component: EmptyRouteView,
    },
    {
      path: "/settings",
      name: "settings",
      component: EmptyRouteView,
    },
    {
      path: "/batch",
      name: "batch",
      component: EmptyRouteView,
    },
  ],
});

export default router;
