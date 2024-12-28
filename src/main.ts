import { createApp } from "vue"
import "./styles.css"
import App from "./App.vue"
import { router } from "./global/route"
import "element-plus/theme-chalk/dark/css-vars.css"
import "./global/state.ts"
import { reloadConfig } from "@/global/api.ts"

reloadConfig().then(() => createApp(App).use(router).mount("#app"))
