import { listen } from "@tauri-apps/api/event"
import { useColorMode } from "@vueuse/core"
import { ref, watch } from "vue"

export type LanguageEnum = "en" | "zh"
export type LanguageConfig = LanguageEnum | "system"
export type ThemeConfig = "dark" | "light" | "system"
export type Config = {
  lang: LanguageConfig
  theme: ThemeConfig
}

export const config = ref<Config>({
  lang: "system",
  theme: "system",
})
export const actualLang = ref<LanguageEnum>("en")

export function setThemeListener() {
  const themeMedia = window.matchMedia("(prefers-color-scheme: light)")
  themeMedia.addEventListener("change", (e) => {
    if (config.value.theme === "system") {
      colorMode.value = e.matches ? "light" : "dark"
    }
  })
}

watch(
  () => config.value.theme,
  () => {
    if (config.value.theme === "system") {
      const themeMedia = window.matchMedia("(prefers-color-scheme: light)")
      colorMode.value = themeMedia.matches ? "light" : "dark"
    } else {
      colorMode.value = config.value.theme
    }
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
