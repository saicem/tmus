import { listen } from "@tauri-apps/api/event"
import { isEnabled } from "@tauri-apps/plugin-autostart"
import { locale } from "@tauri-apps/plugin-os"
import { create } from "zustand"
import { getAppConfig, setAppConfig } from "@/lib/cmd"
import type { UpdateMetadata } from "@/lib/models"

export type LanguageEnum = "en" | "zh"
export type ThemeEnum = "dark" | "light"
export type LanguageConfig = LanguageEnum | "system"
export type ThemeConfig = ThemeEnum | "system"
export type DateFormatEnum = "yyyy-MM-dd" | "yyyy/MM/dd"
export type TimeFormatEnum = "H:mm:ss" | "HH:mm:ss"

export interface Config {
  lang: LanguageConfig
  theme: ThemeConfig
  filterUninstalledApp: boolean
  firstDayOfWeek: 0 | 1 | 2 | 3 | 4 | 5 | 6
  dateFormat: DateFormatEnum
  timeFormat: TimeFormatEnum
  autoCheckUpdate: boolean
  /** UTC - local offset in ms; null means auto-infer from the system. */
  timezoneOffset: number | null
}

const defaultConfig: Config = {
  lang: "system",
  theme: "system",
  filterUninstalledApp: true,
  firstDayOfWeek: 0,
  dateFormat: "yyyy-MM-dd",
  timeFormat: "H:mm:ss",
  autoCheckUpdate: true,
  timezoneOffset: null,
}

export interface UpdateDialogState {
  show: boolean
  meta: UpdateMetadata | null
}

interface AppState {
  config: Config
  lang: LanguageEnum
  systemLang: LanguageEnum
  theme: ThemeEnum
  autoStart: boolean
  updateDialog: UpdateDialogState
  init: () => Promise<void>
  setConfig: (patch: Partial<Config>) => void
  setAutoStart: (value: boolean) => void
  showUpdateDialog: (meta: UpdateMetadata) => void
  closeUpdateDialog: () => void
}

function resolveLang(
  lang: LanguageConfig,
  systemLang: LanguageEnum
): LanguageEnum {
  if (lang !== "system") return lang
  return systemLang
}

/**
 * Detect the system display language from the WebView (UI language), falling
 * back to the OS plugin locale which reads the region format on Windows.
 */
function detectSystemLang(): LanguageEnum {
  const detected = typeof navigator !== "undefined" ? navigator.language : ""
  return detected.startsWith("zh") ? "zh" : "en"
}

function resolveTheme(theme: ThemeConfig, prefersLight: boolean): ThemeEnum {
  if (theme !== "system") return theme
  return prefersLight ? "light" : "dark"
}

function applyTheme(theme: ThemeEnum) {
  const el = document.documentElement
  el.classList.toggle("dark", theme === "dark")
  el.classList.toggle("light", theme === "light")
}

export const useAppStore = create<AppState>((set, get) => ({
  config: defaultConfig,
  lang: detectSystemLang(),
  systemLang: detectSystemLang(),
  theme: "light",
  autoStart: false,
  updateDialog: { show: false, meta: null },

  init: async () => {
    const config = await getAppConfig()
    const detected = navigator.language || (await locale()) || "en"
    const systemLang: LanguageEnum = detected.startsWith("zh") ? "zh" : "en"
    const prefersLight = window.matchMedia(
      "(prefers-color-scheme: light)"
    ).matches
    const lang = resolveLang(config.lang, systemLang)
    const theme = resolveTheme(config.theme, prefersLight)
    applyTheme(theme)
    set({ config, lang, systemLang, theme })

    await listen("menuItemClick", (e: { payload: string }) => {
      const payload: string = e.payload
      if (payload.startsWith("lang")) {
        get().setConfig({
          lang: payload.substring("lang_".length) as LanguageConfig,
        })
      } else if (payload.startsWith("theme")) {
        get().setConfig({
          theme: payload.substring("theme_".length) as ThemeConfig,
        })
      }
    })

    window
      .matchMedia("(prefers-color-scheme: light)")
      .addEventListener("change", (e) => {
        const { config } = get()
        if (config.theme === "system") {
          const theme = resolveTheme("system", e.matches)
          applyTheme(theme)
          set({ theme })
        }
      })

    isEnabled().then((value) => set({ autoStart: value }))
  },

  setConfig: (patch) => {
    const config = { ...get().config, ...patch }
    set({ config })
    void setAppConfig(config)
    const lang = resolveLang(config.lang, get().systemLang)
    const theme = resolveTheme(
      config.theme,
      window.matchMedia("(prefers-color-scheme: light)").matches
    )
    applyTheme(theme)
    set({ lang, theme })
  },

  setAutoStart: (value) => set({ autoStart: value }),

  showUpdateDialog: (meta) => set({ updateDialog: { show: true, meta } }),
  closeUpdateDialog: () => set({ updateDialog: { show: false, meta: null } }),
}))
