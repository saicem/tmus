import { createMemoryHistory, createRouter } from "vue-router"

import Overview from "@/pages/Overview.vue"
import Timeline from "@/pages/Timeline.vue"
import Setting from "@/pages/Setting.vue"
import Detail from "@/pages/Detail.vue"
import Statistic from "@/pages/Statistic.vue"
import Application from "@/pages/Application.vue"
import Category from "@/pages/Category.vue"

const routes = [
  {
    path: "/",
    name: "overview",
    component: Overview,
  },
  {
    path: "/timeline",
    name: "timeline",
    component: Timeline,
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
    path: "/category",
    name: "category",
    component: Category,
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
