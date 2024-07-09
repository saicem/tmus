import { listen } from "@tauri-apps/api/event"
import { useColorMode } from "@vueuse/core"
import { reactive } from "vue"

export const languageStore = reactive({
  language: "zh-CN",
  setLanguage(language: string) {
    this.language = language
  },
})

export const colorMode = useColorMode({
  selector: "html",
  attribute: "class",
  modes: {
    dark: "dark",
    light: "light",
  },
})

await listen("menuItemClick", (e: { payload: string }) => {
  console.log("menuItemClick", e.payload)
  const id = e.payload
  if (id.startsWith("lang")) {
    console.log("Language change.")
  } else if (id.startsWith("theme")) {
    colorMode.value = id.substring("theme-".length) as "dark" | "light"
  }
})
