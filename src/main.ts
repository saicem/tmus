import { createApp } from "vue"
import "./styles.css"
import App from "./App.vue"
import { router } from "@/script/route"
import "element-plus/theme-chalk/dark/css-vars.css"
import "@/script/state.ts"
import { reloadConfig } from "@/script/api.ts"
import { setThemeListener } from "@/script/state.ts"
import "element-plus/theme-chalk/el-notification.css"
import "element-plus/theme-chalk/el-message.css"

setThemeListener()
reloadConfig().then(() => createApp(App).use(router).mount("#app"))
