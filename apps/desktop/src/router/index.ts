import { createRouter, createWebHashHistory } from "vue-router";
import HomeView from "@/views/HomeView.vue";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("@/views/SettingsView.vue"),
    },
    {
      path: "/history",
      name: "history",
      component: () => import("@/views/HistoryView.vue"),
    },
    {
      path: "/providers",
      name: "providers",
      component: () => import("@/views/ProvidersView.vue"),
    },
    {
      path: "/providers/:providerId",
      name: "provider-detail",
      component: () => import("@/views/ProviderDetailView.vue"),
      props: true,
    },
  ],
});

export { router };
