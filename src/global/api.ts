import { invoke } from "@tauri-apps/api"

export async function durationAggregate(
  startMillis: number,
  endMillis: number
): Promise<Record<number, number>> {
  return await invoke("duration_aggregate", { startMillis, endMillis })
}

export async function fileDetail(id: number): Promise<FileDetail> {
  return await invoke("file_detail", { id: id })
}

export async function durationByDay(
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
