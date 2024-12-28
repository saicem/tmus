import { config, LanguageConfig } from "@/global/state.ts"
import { computed } from "vue"

export const i18n = computed(() => {
  return messages[config.value.lang]
})

type I18nMessageType = {
  navigateMenu: {
    home: string
    timeline: string
    tags: string
    statistic: string
    setting: string
  }
  homePage: {
    apps: string
    appsUnit: string
    totalUse: string
    totalUseUnit: string
    mostUse: string
    mostUseUnit: string
  }
  weeklyChart: {
    title: string
    thisWeek: string
    lastWeek: string
    dayOfWeekNames: [string, string, string, string, string, string, string]
  }
  configPage: {
    language: string
    theme: string
    themeAuto: string
    themeLight: string
    themeDark: string
  }
}

const messages: Record<LanguageConfig, I18nMessageType> = {
  en: {
    navigateMenu: {
      home: "Home",
      timeline: "Timeline",
      tags: "Tags",
      statistic: "Statistic",
      setting: "Setting",
    },
    homePage: {
      apps: "App Count",
      appsUnit: "",
      totalUse: "Total Use",
      totalUseUnit: " hrs",
      mostUse: "Most Use",
      mostUseUnit: " hrs",
    },
    weeklyChart: {
      title: "Weekly Usage",
      thisWeek: "This Week",
      lastWeek: "Last Week",
      dayOfWeekNames: [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
      ],
    },
    configPage: {
      language: "Language",
      theme: "Theme",
      themeAuto: "Auto",
      themeLight: "Light",
      themeDark: "Dark",
    },
  },
  zh: {
    navigateMenu: {
      home: "主页",
      timeline: "时间线",
      tags: "标签",
      statistic: "统计",
      setting: "设置",
    },
    homePage: {
      apps: "应用",
      appsUnit: "个",
      totalUse: "使用量",
      totalUseUnit: "小时",
      mostUse: "最常使用",
      mostUseUnit: "小时",
    },
    weeklyChart: {
      title: "周使用量",
      thisWeek: "本周",
      lastWeek: "上周",
      dayOfWeekNames: ["周日", "周一", "周二", "周三", "周四", "周五", "周六"],
    },
    configPage: {
      language: "语言",
      theme: "主题",
      themeAuto: "自动",
      themeLight: "浅色",
      themeDark: "深色",
    },
  },
}
