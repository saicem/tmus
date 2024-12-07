import { listen } from "@tauri-apps/api/event"
import { useColorMode } from "@vueuse/core"
import { reactive } from "vue"

export type LanguageType = "en" | "zh"

export const languageStore = reactive<{ language: LanguageType }>({
  language: "en",
})

export type ColorModeType = "dark" | "light"

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
    languageStore.language = id.substring("lang_".length) as LanguageType
  } else if (id.startsWith("theme")) {
    colorMode.value = id.substring("theme_".length) as ColorModeType
  }
})
