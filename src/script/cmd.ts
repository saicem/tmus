import {
  Channel,
  invoke,
  InvokeArgs,
  InvokeOptions,
} from "@tauri-apps/api/core"
import {
  AppDurationAreaModel,
  AppMeta,
  DateDuration,
  DownloadEvent,
  FileDetail,
  FileIndexRecord,
  IdDateDuration,
  IdDuration,
  RuleConfig,
  TagConfig,
  UpdateMetadata,
} from "./models.ts"
import { Config } from "@/script/state.ts"
import { ElMessage } from "element-plus"
import { timeZoneOffsetMillis } from "@/script/time-util.ts"

export async function getDurationById(
  startTimestamp: number,
  endTimestamp: number
): Promise<IdDuration[]> {
  return await ivk("get_duration_by_id", {
    startTimestamp,
    endTimestamp,
  })
}

export async function getDurationByDate(
  startTimestamp: number,
  endTimestamp: number
): Promise<DateDuration[]> {
  return await ivk("get_duration_by_date", {
    startTimestamp,
    endTimestamp,
    timezoneOffset: timeZoneOffsetMillis(),
  })
}

export async function getDurationByDateID(
  startTimestamp: number,
  endTimestamp: number
): Promise<IdDateDuration[]> {
  return await ivk("get_duration_by_date_id", {
    startTimestamp,
    endTimestamp,
    timezoneOffset: timeZoneOffsetMillis(),
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

export async function getAppDetailMap() {
  return (await getAllAppDetail()).reduce(
    (map, detail) => {
      map[detail.id] = detail
      return map
    },
    {} as Record<number, FileDetail>
  )
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
