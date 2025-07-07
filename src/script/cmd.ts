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
  FileDetail,
  FileIndexRecord,
  RuleConfig,
  TagConfig,
  UpdateMetadata,
} from "./data"
import { Config } from "@/script/state.ts"
import { ElMessage } from "element-plus"

async function durationById(
  startMillis: number,
  endMillis: number
): Promise<Record<number, number>> {
  return await ivk("duration_by_id", { startMillis, endMillis })
}

async function durationByDay(
  startMillis: number,
  endMillis: number,
  timeZoneOffset: number
): Promise<Record<number, number>> {
  return await ivk("duration_by_day", {
    startMillis,
    endMillis,
    timeZoneOffset,
  })
}

async function durationByDayId(
  startMillis: number,
  endMillis: number,
  timeZoneOffset: number
): Promise<Record<number, Record<number, number>>> {
  return await ivk("duration_by_day_id", {
    startMillis,
    endMillis,
    timeZoneOffset,
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

export async function getAllAppDetail(): Promise<FileDetail[]> {
  return await ivk("get_all_app_detail")
}

export async function fetch_update(): Promise<UpdateMetadata | null> {
  return await ivk("fetch_update")
}

export async function install_update(
  onMessage: (event: DownloadEvent) => void
): Promise<void> {
  let onEvent = new Channel<DownloadEvent>()
  onEvent.onmessage = onMessage
  return await ivk("install_update", { onEvent })
}

export async function getAppDurationArea(
  appId: number,
  startMillis: number,
  endMillis: number,
  timeZoneOffset: number
): Promise<AppDurationAreaModel> {
  return await ivk("get_app_duration_area", {
    appId,
    startMillis,
    endMillis,
    timeZoneOffset,
  })
}

export default {
  durationById,
  durationByDay,
  durationByDayId,
}

async function ivk<T>(
  cmd: string,
  args?: InvokeArgs,
  options?: InvokeOptions
): Promise<T> {
  try {
    return await invoke(cmd, args, options)
  } catch (error) {
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
