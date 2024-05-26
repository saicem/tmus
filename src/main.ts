import { createApp } from "vue"
import "./styles.css"
import App from "./App.vue"
import { themeStore } from "@/global/state.ts"
import Antd from "ant-design-vue"

themeStore.setTheme("light")

createApp(App).use(Antd).mount("#app")
