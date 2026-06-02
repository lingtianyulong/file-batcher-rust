import { createRouter, createWebHashHistory } from "vue-router";

const EmptyRouteView = {
  template: "<div></div>",
};

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      redirect: "/home",
    },
    {
      path: "/home",
      name: "home",
      component: () => import("../pages/home.vue"),
    },
    {
      path: "/settings",
      name: "settings",
      component: EmptyRouteView,
    },
    {
      path: "/batch",
      name: "batch",
      component: () => import("../pages/batch.vue"),
    },
    {
      path: "/search-config",
      name: "search-config",
      component: () => import("../pages/SearchConfig.vue"),
    },
    {
      path: "/rename",
      name: "rename",
      component: () => import("../pages/rename.vue"),
    },
  ],
});

export default router;
