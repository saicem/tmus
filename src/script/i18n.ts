import { ref, watch } from "vue"
import { LanguageEnum, passiveStore } from "@/script/state.ts"

type I18nMessageType = {
  common: {
    cancel: string
    ok: string
    start: string
    stop: string
    thisWeek: string
    lastWeek: string
    pastWeek: string
    today: string
    copiedToClipboard: string
  }
  navigateMenu: {
    overview: string
    timeline: string
    tags: string
    statistic: string
    application: string
    category: string
    setting: string
  }
  homePage: {
    apps: string
    appsUnit: string
    totalUse: string
    pastWeekAverage: string
    dailyRhythm: string
    todayUsage: string
  }
  statisticPage: {
    label: {
      timeRange: string
      timeSpan: string
      granularity: string
      displayStyle: string
      group: string
    }
    shortcuts: {
      last1day: string
      last3days: string
      last1week: string
      last1month: string
      last3months: string
      last1year: string
    }
    displayStyle: {
      card: string
      progress: string
      pie: string
      radar: string
      bar: string
    }
    categories: {
      title: string
      placeholder: string
      all: string
      uncategorized: string
    }
    types: {
      title: string
      appDuration: string
      appDays: string
      categoryDuration: string
      categoryDays: string
      categoryRhythm: string
    }
    timeSpan: {
      day: string
      week: string
    }
    granularity: {
      day1: string
      hour1: string
      hour2: string
      hour3: string
      hour4: string
      hour6: string
      minute30: string
      minute15: string
      minute5: string
    }
    unit: {
      day: string
      hour: string
      minute: string
    }
    placeholder: {
      startDate: string
      endDate: string
      rangeSeparator: string
      schemeName: string
    }
    scheme: {
      title: string
      save: string
      delete: string
      saved: string
      deleted: string
      confirmDelete: string
      defaultName: string
    }
    group: {
      add: string
      remove: string
      timeRange: string
      category: string
    }
    validation: {
      noData: string
    }
  }
  applicationPage: {
    typeToSearchName: string
    typeToSearchCompany: string
  }
  weeklyChart: {
    title: string
    thisWeekTotalTime: string
    lastWeekTotalTime: string
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
    checkUpdate: string
    version: string
    currentVersionIsAlreadyTheLatestVersion: string
    updateAvailable: (version: string, currentVersion: string) => string
    versionUpdateTitle: string
    autoCheckUpdate: string
    autoStartMcpServer: string
    mcpServer: string
    usePort: string
  }
  detailPage: {
    icon: string
    name: string
    filePath: string
    productName: string
    fileDescription: string
    companyName: string
    durationDateAreaTab: string
    durationDayAreaTab: string
    durationAreaChart: {
      dateChart: {
        xName: string
        yName: string
      }
      dayChart: {
        xName: string
        yName: string
      }
    }
    fileHasBeenDeleted: string
  }
  categoryPage: {
    title: string
    addRootCategory: string
    categoryTree: string
    uncategorizedApplications: string
    applicationName: string
    path: string
    operation: string
    selectCategory: string
    confirmDelete: string
    confirm: string
    cancel: string
    addCategory: string
    addChild: string
    edit: string
    delete: string
    save: string
    cancelEdit: string
    enterCategoryName: string
    rootLevel: string
    parentCategory: string
    categoryName: string
    categoryNameCannotBeEmpty: string
    categoryAddedSuccessfully: string
    failedToAddCategory: string
    categoryUpdatedSuccessfully: string
    failedToUpdateCategory: string
    categoryDeletedSuccessfully: string
    failedToDeleteCategory: string
    applicationAssignedSuccessfully: string
    failedToAssignApplication: string
    applicationCategoryRemovedSuccessfully: string
    failedToRemoveCategoryFromApplication: string
    failedToLoadCategories: string
    failedToLoadApplications: string
    failedToLoadUncategorizedApplications: string
    searchPlaceholder: string
  }
  ruleDialog: {
    excludeApp: string
    includeApp: string
    mergeApp: string
    path: string
    mergedPath: string
    operation: string
    add: string
    remove: string
    modifiedTip: string
  }
}

const messages: Record<LanguageEnum, I18nMessageType> = {
  en: {
    common: {
      cancel: "Cancel",
      ok: "OK",
      start: "Start",
      stop: "Stop",
      thisWeek: "This Week",
      lastWeek: "Last Week",
      pastWeek: "Past Week",
      today: "Today",
      copiedToClipboard: "Copied to clipboard",
    },
    navigateMenu: {
      overview: "Overview",
      timeline: "Timeline",
      tags: "Tags",
      statistic: "Statistic",
      application: "Application",
      category: "Category",
      setting: "Setting",
    },
    homePage: {
      apps: "Today App Count",
      appsUnit: "",
      totalUse: "Today Total Duration",
      pastWeekAverage: "Past Week Average",
      dailyRhythm: "Daily Rhythm",
      todayUsage: "Today Usage",
    },
    statisticPage: {
      label: {
        timeRange: "Time Range",
        timeSpan: "Time Span",
        granularity: "Granularity",
        displayStyle: "Display Style",
        group: "Group",
      },
      shortcuts: {
        last1day: "Last 1 day",
        last3days: "Last 3 days",
        last1week: "Last 1 week",
        last1month: "Last 1 month",
        last3months: "Last 3 months",
        last1year: "Last 1 year",
      },
      displayStyle: {
        card: "Card Chart",
        progress: "Progress Chart",
        pie: "Pie Chart",
        radar: "Radar Chart",
        bar: "Bar Chart",
      },
      categories: {
        title: "Category Filter",
        placeholder: "Please select categories",
        all: "All Categories",
        uncategorized: "Uncategorized"
      },
      types: {
        title: "Statistic Type",
        appDuration: "App Total Duration",
        appDays: "App Usage Days",
        categoryDuration: "Category Total Duration",
        categoryDays: "Category Usage Days",
        categoryRhythm: "Category Usage Rhythm",
      },
      timeSpan: {
        day: "Day",
        week: "Week",
      },
      granularity: {
        day1: "1 Day",
        hour1: "1 Hour",
        hour2: "2 Hours",
        hour3: "3 Hours",
        hour4: "4 Hours",
        hour6: "6 Hours",
        minute30: "30 Minutes",
        minute15: "15 Minutes",
        minute5: "5 Minutes",
      },
      placeholder: {
        startDate: "Start date",
        endDate: "End date",
        rangeSeparator: "to",
        schemeName: "Plan name",
      },
      scheme: {
        title: "Save Plan",
        save: "Save",
        delete: "Delete",
        saved: "Plan saved successfully",
        deleted: "Plan deleted successfully",
        confirmDelete: "Are you sure you want to delete this plan?",
        defaultName: "New Plan",
      },
      group: {
        add: "Add Group",
        remove: "Remove Group",
        timeRange: "Group Time Range",
        category: "Group Category",
      },
      validation: {
        noData: "No data available",
      },
      unit: {
        day: "day",
        hour: "hour",
        minute: "minute",
      },
    },
    applicationPage: {
      typeToSearchName: "Type to search name",
      typeToSearchCompany: "Type to search company",
    },
    weeklyChart: {
      title: "Weekly Usage",
      thisWeekTotalTime: "This Week Total Duration",
      lastWeekTotalTime: "Last Week Total Duration",
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
      checkUpdate: "Check Update",
      version: "Tmus Version",
      currentVersionIsAlreadyTheLatestVersion:
        "Current version is already the latest version",
      updateAvailable: (version, currentVersion) =>
        `Update available: ${version} (current: ${currentVersion})`,
      versionUpdateTitle: "Version Update",
      autoCheckUpdate: "Auto Check Update",
      autoStartMcpServer: "Auto Start MCP Server",
      mcpServer: "MCP Server",
      usePort: "Use Port",
    },
    detailPage: {
      icon: "Icon",
      name: "Name",
      filePath: "File Path",
      productName: "Product Name",
      fileDescription: "File Description",
      companyName: "Company Name",
      durationDateAreaTab: "Daily Trends",
      durationDayAreaTab: "Intraday Rhythm",
      durationAreaChart: {
        dateChart: {
          xName: "Date",
          yName: "Duration",
        },
        dayChart: {
          xName: "Time",
          yName: "Days",
        },
      },
      fileHasBeenDeleted: "The file has been deleted",
    },
    ruleDialog: {
      excludeApp: "Exclude App",
      includeApp: "Include App",
      mergeApp: "Merge App",
      path: "Path",
      mergedPath: "Merged Path",
      operation: "Operation",
      add: "Add",
      remove: "Remove",
      modifiedTip:
        "You have modified the rule, are you sure to close without save?",
    },
    categoryPage: {
      title: "Application Categories",
      addRootCategory: "Add Root Category",
      categoryTree: "Category Tree",
      uncategorizedApplications: "Uncategorized Applications",
      applicationName: "Application Name",
      path: "Path",
      operation: "Operation",
      selectCategory: "Select category",
      confirmDelete: "Are you sure you want to delete this category?",
      confirm: "Confirm",
      cancel: "Cancel",
      addCategory: "Add Category",
      addChild: "Add Child",
      enterCategoryName: "Enter category name",
      edit: "Edit",
      delete: "Delete",
      save: "Save",
      cancelEdit: "Cancel",
      rootLevel: "Root Level",
      parentCategory: "Parent Category",
      categoryName: "Category Name",
      categoryNameCannotBeEmpty: "Category name cannot be empty",
      categoryAddedSuccessfully: "Category added successfully",
      failedToAddCategory: "Failed to add category",
      categoryUpdatedSuccessfully: "Category updated successfully",
      failedToUpdateCategory: "Failed to update category",
      categoryDeletedSuccessfully: "Category deleted successfully",
      failedToDeleteCategory: "Failed to delete category",
      applicationAssignedSuccessfully: "Application assigned successfully",
      failedToAssignApplication: "Failed to assign application",
      applicationCategoryRemovedSuccessfully: "Application category removed successfully",
      failedToRemoveCategoryFromApplication: "Failed to remove category from application",
      failedToLoadCategories: "Failed to load categories",
      failedToLoadApplications: "Failed to load applications",
      failedToLoadUncategorizedApplications: "Failed to load uncategorized applications",
      searchPlaceholder: "Search by name, company, or path...",
    },
  },
  zh: {
    common: {
      cancel: "取消",
      ok: "确定",
      start: "启动",
      stop: "停止",
      thisWeek: "本周",
      lastWeek: "上周",
      pastWeek: "近一周",
      today: "今天",
      copiedToClipboard: "已复制到剪贴板",
    },
    navigateMenu: {
      overview: "概览",
      timeline: "时间线",
      tags: "标签",
      statistic: "统计",
      application: "应用",
      category: "分类",
      setting: "设置",
    },
    statisticPage: {
      label: {
        timeRange: "时间范围",
        timeSpan: "时间跨度",
        granularity: "统计粒度",
        displayStyle: "展示样式",
        group: "分组",
      },
      shortcuts: {
        last1day: "最近 1 天",
        last3days: "最近 3 天",
        last1week: "最近 1 周",
        last1month: "最近 1 月",
        last3months: "最近 3 月",
        last1year: "最近 1 年",
      },
      displayStyle: {
        card: "卡片",
        progress: "进度图",
        pie: "饼图",
        radar: "雷达图",
        bar: "柱状图",
      },
      categories: {
        title: "分类筛选",
        placeholder: "请选择分类",
        all: "不区分分类",
        uncategorized: "未分类"
      },
      types: {
        title: "统计类型",
        appDuration: "应用使用总时长",
        appDays: "应用使用天数",
        categoryDuration: "分类使用总时长",
        categoryDays: "分类使用天数",
        categoryRhythm: "分类使用节律",
      },
      timeSpan: {
        day: "天",
        week: "周",
      },
      granularity: {
        day1: "1 天",
        hour1: "1 小时",
        hour2: "2 小时",
        hour3: "3 小时",
        hour4: "4 小时",
        hour6: "6 小时",
        minute30: "30 分钟",
        minute15: "15 分钟",
        minute5: "5 分钟",
      },
      placeholder: {
        startDate: "开始日期",
        endDate: "结束日期",
        rangeSeparator: "至",
        schemeName: "方案名称",
      },
      scheme: {
        title: "保存方案",
        save: "保存",
        delete: "删除",
        saved: "方案保存成功",
        deleted: "方案删除成功",
        confirmDelete: "确定要删除此方案吗？",
        defaultName: "新方案",
      },
      group: {
        add: "添加分组",
        remove: "移除分组",
        timeRange: "分组时间范围",
        category: "分组分类",
      },
      validation: {
        noData: "暂无数据",
      },
      unit: {
        day: "天",
        hour: "小时",
        minute: "分钟",
      },
    },
    homePage: {
      apps: "今日应用",
      appsUnit: "个",
      totalUse: "今日时长",
      pastWeekAverage: "近一周平均",
      dailyRhythm: "日内节律",
      todayUsage: "今日使用",
    },
    applicationPage: {
      typeToSearchName: "输入应用名称搜索",
      typeToSearchCompany: "输入公司名称搜索",
    },
    weeklyChart: {
      title: "周使用时长",
      thisWeekTotalTime: "本周总时长",
      lastWeekTotalTime: "上周总时长",
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
      checkUpdate: "检查更新",
      version: "Tmus 版本",
      currentVersionIsAlreadyTheLatestVersion: "当前版本已经是最新版本",
      updateAvailable: (version, currentVersion) =>
        `有新版本 ${version}，当前版本为 ${currentVersion}，是否更新？`,
      versionUpdateTitle: "版本更新",
      autoCheckUpdate: "自动检查更新",
      autoStartMcpServer: "自动启动 MCP 服务器",
      mcpServer: "MCP 服务器",
      usePort: "使用端口",
    },
    detailPage: {
      icon: "图标",
      name: "名称",
      filePath: "文件路径",
      productName: "产品名称",
      fileDescription: "文件描述",
      companyName: "公司名称",
      durationDateAreaTab: "每日趋势",
      durationDayAreaTab: "日内节律",
      durationAreaChart: {
        dateChart: {
          xName: "日期",
          yName: "时长",
        },
        dayChart: {
          xName: "时间",
          yName: "天数",
        },
      },
      fileHasBeenDeleted: "文件已被删除",
    },
    ruleDialog: {
      excludeApp: "排除应用",
      includeApp: "包含应用",
      mergeApp: "合并应用",
      path: "路径",
      mergedPath: "合并后路径",
      operation: "操作",
      add: "新增",
      remove: "删除",
      modifiedTip: "你已经修改了规则,确定不保存关闭吗?",
    },
    categoryPage: {
      enterCategoryName: "输入分类名称",
      title: "应用分类",
      addRootCategory: "添加根分类",
      categoryTree: "分类树",
      uncategorizedApplications: "未分类应用",
      applicationName: "应用名称",
      path: "路径",
      operation: "操作",
      selectCategory: "选择分类",
      confirmDelete: "确定要删除这个分类吗?",
      confirm: "确定",
      cancel: "取消",
      addCategory: "添加分类",
      addChild: "添加子分类",
      edit: "编辑",
      delete: "删除",
      save: "保存",
      cancelEdit: "取消",
      rootLevel: "根级别",
      parentCategory: "父分类",
      categoryName: "分类名称",
      categoryNameCannotBeEmpty: "分类名称不能为空",
      categoryAddedSuccessfully: "分类添加成功",
      failedToAddCategory: "添加分类失败",
      categoryUpdatedSuccessfully: "分类更新成功",
      failedToUpdateCategory: "更新分类失败",
      categoryDeletedSuccessfully: "分类删除成功",
      failedToDeleteCategory: "删除分类失败",
      applicationAssignedSuccessfully: "应用分配成功",
      failedToAssignApplication: "分配应用失败",
      applicationCategoryRemovedSuccessfully: "应用分类移除成功",
      failedToRemoveCategoryFromApplication: "移除应用分类失败",
      failedToLoadCategories: "加载分类失败",
      failedToLoadApplications: "加载应用失败",
      failedToLoadUncategorizedApplications: "加载未分类应用失败",
      searchPlaceholder: "按名称、公司或路径搜索...",
    },
  },
}

export const i18n = ref<I18nMessageType>(messages["en"])

watch(
  () => passiveStore.lang,
  async (lang) => {
    i18n.value = messages[lang]
  },
  { immediate: true }
)
