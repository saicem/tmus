import "./styles.css"
import App from "./App.vue"
import { router } from "@/script/route"
import "element-plus/theme-chalk/dark/css-vars.css"
import "@/script/state.ts"
import { config, setThemeListener } from "@/script/state.ts"
import "element-plus/theme-chalk/el-notification.css"
import "element-plus/theme-chalk/el-message.css"
import { getAppConfig } from "@/script/cmd.ts"

async function loadConfig() {
  config.value = await getAppConfig()
}

setThemeListener()

loadConfig().then(() => createApp(App).use(router).mount("#app"))
