import { listen } from "@tauri-apps/api/event"
import { useColorMode } from "@vueuse/core"
import { ref, watch } from "vue"

export type LanguageConfig = "en" | "zh"
export type ThemeConfig = "dark" | "light"
export type Config = {
  lang: LanguageConfig
  theme: ThemeConfig
}

export const config = ref<Config>({
  lang: "en",
  theme: "light",
})

watch(
  () => config.value.theme,
  () => {
    colorMode.value = config.value.theme
  }
)

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
    config.value.lang = id.substring("lang_".length) as LanguageConfig
  } else if (id.startsWith("theme")) {
    config.value.theme = id.substring("theme_".length) as ThemeConfig
  }
})
