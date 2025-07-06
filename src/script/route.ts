import { createMemoryHistory, createRouter } from "vue-router"

import Home from "@/pages/Home.vue"
import Timeline from "@/pages/Timeline.vue"
import Setting from "@/pages/Setting.vue"
import Detail from "@/pages/Detail.vue"
import Statistic from "@/pages/Statistic.vue"
import Application from "@/pages/Application.vue"

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
    component: Statistic,
  },
  {
    path: "/application",
    name: "application",
    component: Application,
  },
  {
    path: "/setting",
    name: "setting",
    component: Setting,
  },
  {
    path: "/detail/:id",
    name: "detail",
    component: Detail,
    props: (route: { params: { id: string } }) => ({
      id: +route.params.id,
    }),
  },
]

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
})
