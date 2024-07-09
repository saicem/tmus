import { createMemoryHistory, createRouter } from "vue-router"

import Home from "@/pages/Home.vue"
import Timeline from "@/pages/Timeline.vue"

const routes = [
  {
    path: "/",
    name: "home",
    component: Home,
  },
  {
    path: "/timeline",
    name: "timeline",
    component: Timeline,
  },
  {
    path: "/tags",
    name: "tags",
    component: Home,
  },
  {
    path: "/statistic",
    name: "statistic",
    component: Home,
  },
  {
    path: "/setting",
    name: "setting",
    component: Home,
  },
]

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
})
