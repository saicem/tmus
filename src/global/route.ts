import { createMemoryHistory, createRouter } from "vue-router"

import PageHome from "@/components/PageHome.vue"
import PageTimeline from "@/components/PageTimeline.vue"

const routes = [
  {
    path: "/",
    name: "home",
    component: PageHome,
  },
  {
    path: "/timeline",
    name: "timeline",
    component: PageTimeline,
  },
]

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
})