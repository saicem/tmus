import { config, LanguageEnum } from "@/global/state.ts"
import { locale } from "@tauri-apps/plugin-os";
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
  },
}

export const i18n = ref<I18nMessageType>(messages["en"]);

const getLang = async (): Promise<LanguageEnum> => {
  if (config.value.lang === "system") {
    return await getLocaleLang();
  } else {
    return config.value.lang;
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
    i18n.value = messages[await getLang()];
  },
  { immediate: true }
);