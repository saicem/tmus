import { invoke } from "@tauri-apps/api/core"
import { FileDetail, FocusRecord } from "./data"
import { Config } from "@/global/state.ts"

async function rawRecord(
  startMillis: number,
  endMillis: number
): Promise<FocusRecord[]> {
  return await invoke("raw_record", { startMillis, endMillis })
}

async function readReverse(
  cursor: number | null,
  count: number
): Promise<[FocusRecord[], number | null]> {
  return await invoke("read_reverse", { cursor, count })
}

async function durationAggregate(
  startMillis: number,
  endMillis: number
): Promise<Record<number, number>> {
  return await invoke("duration_aggregate", { startMillis, endMillis })
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

async function fileDetail(id: number): Promise<FileDetail> {
  return await invoke("file_detail", { id: id })
}

async function getAppConfig(): Promise<Config> {
  return await invoke("get_app_config")
}

export async function setAppConfig(config: Config): Promise<void> {
  return await invoke("set_app_config", { config })
}

export default {
  rawRecord,
  readReverse,
  durationAggregate,
  durationByDay,
  fileDetail,
  getAppConfig,
  setAppConfig,
}
