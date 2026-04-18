import "./styles.css"
import App from "./App.vue"
import { router } from "@/script/route"
import "element-plus/theme-chalk/dark/css-vars.css"
import "@/script/state.ts"
import { init } from "@/script/state.ts"
import "element-plus/theme-chalk/el-notification.css"
import "element-plus/theme-chalk/el-message.css"
import 'element-plus/theme-chalk/src/message.scss'
import 'element-plus/theme-chalk/src/message-box.scss'

init().then(() => createApp(App).use(router).mount("#app"))
