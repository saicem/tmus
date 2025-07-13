import { listen } from "@tauri-apps/api/event"
import { isEnabled } from "@tauri-apps/plugin-autostart"
import { locale } from "@tauri-apps/plugin-os"
import { getAppConfig, setAppConfig } from "@/script/cmd.ts"
import { UpdateMetadata } from "@/script/models.ts"

export type LanguageEnum = "en" | "zh"
export type ThemeEnum = "dark" | "light"
export type DateFormatEnum = "yyyy-MM-dd" | "yyyy/MM/dd"
export type TimeFormatEnum = "H:mm:ss" | "HH:mm:ss"
export type LanguageConfig = LanguageEnum | "system"
export type ThemeConfig = ThemeEnum | "system"
export type Config = {
  lang: LanguageConfig
  theme: ThemeConfig
  filterUninstalledApp: boolean
  firstDayOfWeek: 0 | 1 | 2 | 3 | 4 | 5 | 6
  dateFormat: DateFormatEnum
  timeFormat: TimeFormatEnum
  autoCheckUpdate: boolean
  autoStartMcpServer: boolean
  mcpServerPort: number
}

export const updateDialogStore = reactive<{
  show: boolean
  meta: UpdateMetadata | null
}>({
  show: false,
  meta: null,
})

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
  timeFormat: "H:mm:ss",
  autoCheckUpdate: true,
  autoStartMcpServer: false,
  mcpServerPort: 2371,
})

export const passiveStore = reactive<{
  autoStart: boolean
  mcpServerRunning: boolean
  lang: LanguageEnum
  theme: ThemeEnum
}>({
  autoStart: false,
  mcpServerRunning: true,
  lang: "en",
  theme: "light",
})

watch(configStore, async (config) => {
  await setAppConfig(config)
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
  assignConfig(await getAppConfig())

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

function assignConfig(config: Config) {
  if (config.dateFormat != "yyyy-MM-dd" && config.dateFormat != "yyyy/MM/dd") {
    config.dateFormat = "yyyy-MM-dd"
  }
  if (config.timeFormat != "H:mm:ss" && config.timeFormat != "HH:mm:ss") {
    config.timeFormat = "H:mm:ss"
  }
  Object.assign(configStore, config)
}
