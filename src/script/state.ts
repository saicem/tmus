import { listen } from "@tauri-apps/api/event"
import { isEnabled } from "@tauri-apps/plugin-autostart"
import { locale } from "@tauri-apps/plugin-os"
import { getAppConfig } from "@/script/cmd.ts"

export type LanguageEnum = "en" | "zh"
export type ThemeEnum = "dark" | "light"
export type LanguageConfig = LanguageEnum | "system"
export type ThemeConfig = ThemeEnum | "system"
export type Config = {
  lang: LanguageConfig
  theme: ThemeConfig
  filterUninstalledApp: boolean
  firstDayOfWeek: 0 | 1 | 2 | 3 | 4 | 5 | 6
  dateFormat: string
  timeFormat: string
}

export const statisticStore = reactive<{
  statisticType: "Progress" | "Card"
}>({
  statisticType: "Progress",
})

export const configStore = reactive<Config>({
  lang: "system",
  theme: "system",
  filterUninstalledApp: true,
  firstDayOfWeek: 0,
  dateFormat: "yyyy-MM-dd",
  timeFormat: "HH:mm:ss",
})

export const passiveStore = reactive<{
  autoStart: boolean
  lang: LanguageEnum
  theme: ThemeEnum
}>({
  autoStart: false,
  lang: "en",
  theme: "light",
})

watch(
  () => configStore.lang,
  async (newValue) => {
    if (newValue != "system") {
      passiveStore.lang = newValue
    } else {
      const lang = await locale()
      passiveStore.lang = lang?.startsWith("zh") ? "zh" : "en"
    }
  },
  {
    immediate: true,
  }
)

watch(
  () => configStore.theme,
  (newValue) => {
    let theme: ThemeEnum
    if (newValue != "system") {
      theme = newValue
    } else {
      theme = window.matchMedia("(prefers-color-scheme: light)").matches
        ? "light"
        : "dark"
    }
    passiveStore.theme = theme
  },
  {
    immediate: true,
  }
)

watch(
  () => passiveStore.theme,
  (newValue) => {
    updateHtmlTheme(newValue)
  }
)

// --- listen ---
export async function init() {
  Object.assign(configStore, await getAppConfig())

  await listen("menuItemClick", (e: { payload: string }) => {
    console.log("try menu click", e.payload)
    const id = e.payload
    if (id.startsWith("lang")) {
      configStore.lang = id.substring("lang_".length) as LanguageConfig
    } else if (id.startsWith("theme")) {
      configStore.theme = id.substring("theme_".length) as ThemeConfig
    }
  })

  window
    .matchMedia("(prefers-color-scheme: light)")
    .addEventListener("change", (e) => {
      if (configStore.theme === "system") {
        passiveStore.theme = e.matches ? "light" : "dark"
      }
    })

  isEnabled().then((value) => (passiveStore.autoStart = value))
}

// --- helper ---

function updateHtmlTheme(theme: ThemeEnum) {
  if (typeof document !== "undefined") {
    const htmlEl = document.documentElement
    if (theme === "dark") {
      htmlEl.classList.remove("light")
      htmlEl.classList.add("dark")
    } else {
      htmlEl.classList.remove("dark")
      htmlEl.classList.add("light")
    }
  }
}
