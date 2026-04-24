import {
  Channel,
  invoke,
  InvokeArgs,
  InvokeOptions,
} from "@tauri-apps/api/core"
import {
  AppDurationAreaModel,
  AppMeta,
  DownloadEvent,
  DurationStat,
  FileDetail,
  FileIndexRecord,
  IdDuration, McpServerStatus,
  RuleConfig,
  TagConfig,
  UpdateMetadata,
  Category,
  AddCategoryRequest,
  UpdateCategoryRequest,
  DeleteCategoryRequest,
  AssignCategoryRequest,
  RemoveAppCategoryRequest,
  AppDurationRequest,
  AppDurationResponse,
  AppDayCountRequest,
  AppDayCountResponse,
  CategoryDurationRequest,
  CategoryDurationResponse,
  CategoryDayCountRequest,
  CategoryDayCountResponse,
  RhythmRequest,
  RhythmDataResponse,
  StatisticScheme,
  AddSchemeItemRequest,
  AddSchemeItemResponse,
  CategorySimple
} from "./models.ts"
import { Config } from "@/script/state.ts"
import { ElMessage } from "element-plus"

export async function getDurationById(
  startTimestamp: number,
  endTimestamp: number
): Promise<IdDuration[]> {
  return await ivk("get_duration_by_id", {
    startTimestamp,
    endTimestamp,
  })
}

export async function queryDurationStatistic(
  startTimestamp: number,
  endTimestamp: number,
  mergeApps: boolean,
  appIds: number[] | null,
  granularity: number,
  cycle: number | null
): Promise<DurationStat[]> {
  return await ivk("query_duration_statistic", {
    startTimestamp,
    endTimestamp,
    mergeApps,
    appIds,
    granularity,
    cycle,
  })
}

export async function showInFolder(path: string | undefined) {
  if (!path) return
  return await ivk("show_in_folder", { path })
}

export async function getAppConfig(): Promise<Config> {
  return await ivk("get_app_config")
}

export async function setAppConfig(config: Config): Promise<void> {
  return await ivk("set_app_config", { config })
}

export async function getAppRule(): Promise<RuleConfig> {
  return await ivk("get_app_rule")
}

export async function setAppRule(config: RuleConfig): Promise<void> {
  return await ivk("set_app_rule", { config })
}

export async function getAppVersion(): Promise<TagConfig> {
  return await ivk("get_app_version")
}

export async function setAppVersion(config: TagConfig): Promise<void> {
  return await ivk("set_app_version", { config })
}

export async function getTmusMeta(): Promise<AppMeta> {
  return await ivk("get_tmus_meta")
}

export async function focusIndexRecord(): Promise<FileIndexRecord[]> {
  return await ivk("focus_index_record")
}

export async function getAppDetail(id: number): Promise<FileDetail> {
  return await ivk("get_app_detail", { id: id })
}

export async function getAppDetailMap(): Promise<Record<number, FileDetail>> {
  return await ivk("get_all_app_detail")
}

export async function fetchUpdate(): Promise<UpdateMetadata | null> {
  return await ivk("fetch_update")
}

export async function installUpdate(
  onMessage: (event: DownloadEvent) => void
): Promise<void> {
  let onEvent = new Channel<DownloadEvent>()
  onEvent.onmessage = onMessage
  return await ivk("install_update", { onEvent })
}

export async function getAppDurationArea(
  appId: number,
  startTimestamp: number,
  endTimestamp: number,
  timezoneOffset: number
): Promise<AppDurationAreaModel> {
  return await ivk("get_app_duration_area", {
    appId,
    startTimestamp,
    endTimestamp,
    timezoneOffset,
  })
}

export async function startMcpServer(port: number) {
  return await ivk("start_mcp_server", {
    port,
  })
}

export async function stopMcpServer() {
  return await ivk("stop_mcp_server")
}

export async function getMcpServerStatus(): Promise<McpServerStatus> {
  return await invoke("get_mcp_server_status")
}

export async function getCategoryTree(): Promise<Category> {
  return await ivk("get_category_tree")
}

export async function getAllCategories(): Promise<CategorySimple[]> {
  return await ivk("get_all_categories")
}

export async function addCategory(request: AddCategoryRequest): Promise<void> {
  await ivk<void>("add_category", { request })
}

export async function updateCategory(request: UpdateCategoryRequest): Promise<void> {
  await ivk("update_category", { request })
}

export async function deleteCategory(request: DeleteCategoryRequest): Promise<void> {
  await ivk("delete_category", { request })
}

export async function setAppCategory(request: AssignCategoryRequest): Promise<void> {
  await ivk("set_app_category", { request })
}

export async function removeAppFromCategory(request: RemoveAppCategoryRequest): Promise<void> {
  await ivk("remove_app_from_category", { request })
}

export async function getUncategorizedApps(offset: number = 0, limit: number = 100, keyword?: string): Promise<FileDetail[]> {
  const result = await ivk<{ apps: FileDetail[], total: number, has_more: boolean }>("get_uncategorized_apps", { request: { offset, limit, keyword } })
  return result.apps
}

export async function getCategoryApps(categoryId: number): Promise<FileDetail[]> {
  return await ivk<FileDetail[]>("get_category_apps", { categoryId: categoryId })
}

async function ivk<T>(
  cmd: string,
  args?: InvokeArgs,
  options?: InvokeOptions
): Promise<T> {
  console.log(`[ivk] Calling command: ${cmd}`, { args, options })
  const startTime = performance.now()

  try {
    const result = await invoke(cmd, args, options)
    const endTime = performance.now()
    console.log(`[ivk] Command ${cmd} completed in ${endTime - startTime}ms`, { result })
    return result as T
  } catch (error) {
    const endTime = performance.now()
    console.error(`[ivk] Command ${cmd} failed in ${endTime - startTime}ms`, { error })

    let errorMessage = "An unknown error occurred."
    if (typeof error === "string") {
      errorMessage = error
    } else if (error instanceof Error) {
      errorMessage = error.message
    }

    ElMessage({
      message: errorMessage,
      type: "error",
      duration: 5000,
      showClose: true,
    })
    throw new Error(errorMessage)
  }
}

export async function getBaseTime(): Promise<number> {
  return await ivk("get_base_time")
}

export async function getAppTotalDuration(request: AppDurationRequest): Promise<AppDurationResponse> {
  return await ivk("get_app_total_duration", { request })
}

export async function getAppUsageDays(request: AppDayCountRequest): Promise<AppDayCountResponse> {
  return await ivk("get_app_usage_days", { request })
}

export async function getCategoryTotalDuration(request: CategoryDurationRequest): Promise<CategoryDurationResponse> {
  return await ivk("get_category_total_duration", { request })
}

export async function getCategoryUsageDays(request: CategoryDayCountRequest): Promise<CategoryDayCountResponse> {
  return await ivk("get_category_usage_days", { request })
}

export async function getCategoryUsageRhythm(request: RhythmRequest): Promise<RhythmDataResponse> {
  return await ivk("get_category_usage_rhythm", { request })
}

export async function getStatisticSchemeList(): Promise<StatisticScheme> {
  return await ivk("get_statistic_scheme_list")
}

export async function addStatisticScheme(request: AddSchemeItemRequest): Promise<AddSchemeItemResponse> {
  return await ivk("add_statistic_scheme", { request })
}

export async function deleteStatisticScheme(id: number): Promise<void> {
  return await ivk("delete_statistic_scheme", { id })
}

export async function saveStatisticSchemeManual(): Promise<void> {
  return await ivk("save_statistic_scheme_manual")
}
