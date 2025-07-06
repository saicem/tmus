import { config, LanguageEnum } from "@/script/state.ts"
import { locale } from "@tauri-apps/plugin-os"
import { ref, watch } from "vue"

type I18nMessageType = {
  navigateMenu: {
    home: string
    timeline: string
    tags: string
    statistic: string
    application: string
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
    filterUninstalledApp: string
    firstDayOfWeek: string
    monday: string
    tuesday: string
    wednesday: string
    thursday: string
    friday: string
    saturday: string
    sunday: string
    dateFormat: string
    timeFormat: string
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
      application: "Application",
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
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
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
      appRuleTip:
        "Application rules. Exclude applications, all applications containing the configured prefix will be excluded. Include applications, takes precedence over excluded applications, preventing them from being excluded. Merge applications, converts all applications under a specific path to another path, allowing for unified statistics of application duration under a directory. Restart Tmus takes effect.",
      filterUninstalledApp: "Filter Uninstalled App",
      firstDayOfWeek: "First Day Of Week",
      monday: "Monday",
      tuesday: "Tuesday",
      wednesday: "Wednesday",
      thursday: "Thursday",
      friday: "Friday",
      saturday: "Saturday",
      sunday: "Sunday",
      dateFormat: "Date Format",
      timeFormat: "Time Format",
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
        "You have modified the rule, are you sure to close without save?",
    },
  },
  zh: {
    navigateMenu: {
      home: "主页",
      timeline: "时间线",
      tags: "标签",
      statistic: "统计",
      application: "应用",
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
      totalUse: "使用时长",
      mostUse: "最常使用",
    },
    weeklyChart: {
      title: "周使用时长",
      thisWeek: "本周",
      lastWeek: "上周",
      dayOfWeekNames: [
        "星期一",
        "星期二",
        "星期三",
        "星期四",
        "星期五",
        "星期六",
        "星期日",
      ],
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
        "应用规则。排除应用，所有包含其中配置前缀的应用都会被排除。包含应用，优先级高于排除应用，避免应用被排除。合并应用，将某路径下的所有应用转化为另一路径，以便某一目录下的应用统一统计时长。重启 Tmus 生效。",
      filterUninstalledApp: "过滤已卸载应用",
      firstDayOfWeek: "一周的第一天",
      monday: "星期一",
      tuesday: "星期二",
      wednesday: "星期三",
      thursday: "星期四",
      friday: "星期五",
      saturday: "星期六",
      sunday: "星期日",
      dateFormat: "日期格式",
      timeFormat: "时间格式",
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
      modifiedTip: "你已经修改了规则,确定不保存关闭吗?",
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
