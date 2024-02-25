import {createApp} from "vue";
import "./styles.css";
import App from "./App.vue";
import {themeStore} from "@/global/state.ts";

themeStore.setTheme("light");

createApp(App).mount("#app");
