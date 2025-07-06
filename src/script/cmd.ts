import { invoke } from "@tauri-apps/api/core"
import {
  AppMeta,
  FileDetail,
  FileIndexRecord,
  RuleConfig,
  TagConfig,
} from "./data"
import { Config } from "@/script/state.ts"

async function durationById(
  startMillis: number,
  endMillis: number
): Promise<Record<number, number>> {
  return await invoke("duration_by_id", { startMillis, endMillis })
}

async function durationByDay(
  startMillis: number,
  endMillis: number,
  timeZoneOffset: number
): Promise<Record<number, number>> {
  return await invoke("duration_by_day", {
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
  return await invoke("duration_by_day_id", {
    startMillis,
    endMillis,
    timeZoneOffset,
  })
}

async function fileDetail(id: number): Promise<FileDetail> {
  return await invoke("file_detail", { id: id })
}

export async function showInFolder(path: string | undefined) {
  if (!path) return
  return await invoke("show_in_folder", { path })
}

export async function getAppConfig(): Promise<Config> {
  return await invoke("get_app_config")
}

export async function setAppConfig(config: Config): Promise<void> {
  return await invoke("set_app_config", { config })
}

export async function getAppRule(): Promise<RuleConfig> {
  return await invoke("get_app_rule")
}

export async function setAppRule(config: RuleConfig): Promise<void> {
  return await invoke("set_app_rule", { config })
}

export async function getAppVersion(): Promise<TagConfig> {
  return await invoke("get_app_version")
}

export async function setAppVersion(config: TagConfig): Promise<void> {
  return await invoke("set_app_version", { config })
}

export async function tmusMeta(): Promise<AppMeta> {
  return await invoke("tmus_meta")
}

export async function focusIndexRecord(): Promise<FileIndexRecord[]> {
  return await invoke("focus_index_record")
}

export async function getAllApp(): Promise<FileDetail[]> {
  return await invoke("get_all_app")
}

export default {
  durationById,
  durationByDay,
  durationByDayId,
  fileDetail,
}
