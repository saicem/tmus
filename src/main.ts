import { createApp } from "vue"
import "./styles.css"
import App from "./App.vue"
import { router } from "./global/route"
import "element-plus/theme-chalk/dark/css-vars.css"

createApp(App).use(router).mount("#app")
