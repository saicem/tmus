import { config, LanguageEnum } from "@/global/state.ts"
import { locale } from "@tauri-apps/plugin-os"
import { ref, watch } from "vue"

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
    mostUse: string
  }
  statisticPage: {
    shortcuts: {
      last1day: string
      last3days: string
      last1week: string
      last1month: string
      last3months: string
      last1year: string
    }
    type: {
      card: string
      progress: string
    }
  }
  weeklyChart: {
    title: string
    thisWeek: string
    lastWeek: string
    dayOfWeekNames: [string, string, string, string, string, string, string]
  }
  configPage: {
    language: string
    langSystem: string
    theme: string
    autoStart: string
    themeSystem: string
    themeLight: string
    themeDark: string
    appRule: string
    appTag: string
    appRuleTip: string
  }
  detailPage: {
    icon: string
    exist: string
    name: string
    filePath: string
    productName: string
    fileDescription: string
    companyName: string
  }
  ruleDialog: {
    excludeApp: string
    includeApp: string
    mergeApp: string
    path: string
    mergedPath: string
    operation: string
    add: string
    cancel: string
    ok: string
    remove: string
    modifiedTip: string
  }
}

const messages: Record<LanguageEnum, I18nMessageType> = {
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
      mostUse: "Most Use",
    },
    statisticPage: {
      shortcuts: {
        last1day: "Last 1 Day",
        last3days: "Last 3 Days",
        last1week: "Last 1 Week",
        last1month: "Last 1 Month",
        last3months: "Last 3 Months",
        last1year: "Last 1 Year",
      },
      type: {
        card: "Card",
        progress: "Progress",
      },
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
      langSystem: "System",
      theme: "Theme",
      autoStart: "Auto Start",
      themeSystem: "System",
      themeLight: "Light",
      themeDark: "Dark",
      appRule: "App Rule",
      appTag: "App Tag",
      appRuleTip: "Application rules. Exclude applications, all applications containing the configured prefix will be excluded. Include applications, takes precedence over excluded applications, preventing them from being excluded. Merge applications, converts all applications under a specific path to another path, allowing for unified statistics of application duration under a directory. Restarting the application takes effect.",
    },
    detailPage: {
      icon: "Icon",
      exist: "Exist",
      name: "Name",
      filePath: "File Path",
      productName: "Product Name",
      fileDescription: "File Description",
      companyName: "Company Name",
    },
    ruleDialog: {
      excludeApp: "Exclude App",
      includeApp: "Include App",
      mergeApp: "Merge App",
      path: "Path",
      mergedPath: "Merged Path",
      operation: "Operation",
      add: "Add",
      cancel: "Cancel",
      ok: "OK",
      remove: "Remove",
      modifiedTip:
        "You have modified the rule, are you sure to close this dialog?",
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
    statisticPage: {
      shortcuts: {
        last1day: "最近 1 天",
        last3days: "最近 3 天",
        last1week: "最近 1 周",
        last1month: "最近 1 月",
        last3months: "最近 3 月",
        last1year: "最近 1 年",
      },
      type: {
        card: "卡片",
        progress: "进度条",
      },
    },
    homePage: {
      apps: "应用",
      appsUnit: "个",
      totalUse: "使用量",
      mostUse: "最常使用",
    },
    weeklyChart: {
      title: "周使用量",
      thisWeek: "本周",
      lastWeek: "上周",
      dayOfWeekNames: ["周日", "周一", "周二", "周三", "周四", "周五", "周六"],
    },
    configPage: {
      language: "语言",
      langSystem: "系统",
      theme: "主题",
      autoStart: "开机自启",
      themeSystem: "系统",
      themeLight: "浅色",
      themeDark: "深色",
      appRule: "应用规则",
      appTag: "应用标签",
      appRuleTip:
        "应用规则。排除应用，所有包含其中配置前缀的应用都会被排除。包含应用，优先级高于排除应用，避免应用被排除。合并应用，将某路径下的所有应用转化为另一路径，以便某一目录下的应用统一统计时长。重启应用生效。",
    },
    detailPage: {
      icon: "图标",
      exist: "文件存在",
      name: "名称",
      filePath: "文件路径",
      productName: "产品名称",
      fileDescription: "文件描述",
      companyName: "公司名称",
    },
    ruleDialog: {
      excludeApp: "排除应用",
      includeApp: "包含应用",
      mergeApp: "合并应用",
      path: "路径",
      mergedPath: "合并后路径",
      operation: "操作",
      add: "新增",
      cancel: "取消",
      ok: "确定",
      remove: "删除",
      modifiedTip: "你已经修改了规则,确定关闭吗?",
    },
  },
}

export const i18n = ref<I18nMessageType>(messages["en"])

const getLang = async (): Promise<LanguageEnum> => {
  if (config.value.lang === "system") {
    return await getLocaleLang()
  } else {
    return config.value.lang
  }
}

const getLocaleLang = async () => {
  const lang = await locale()
  if (lang?.startsWith("zh")) {
    return "zh"
  } else {
    return "en"
  }
}

watch(
  () => config.value.lang,
  async () => {
    i18n.value = messages[await getLang()]
  },
  { immediate: true }
)
