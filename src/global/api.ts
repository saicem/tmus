import { invoke } from "@tauri-apps/api";

export async function durationAggregate(startMillis: number, endMillis: number): Promise<Record<number, number>> {
    let ret = await invoke('duration_aggregate', { startMillis, endMillis });
    return ret as any
}

export async function fileDetail(id: number): Promise<FileDetail> {
    let ret = await invoke('file_version_by_id', { id: id })
    return ret as any
}

export async function durationByDay(startMillis: number, endMillis: number, offsetTimeZone: number): Promise<Record<number, number>> {
    let ret = await invoke('duration_by_day', { startMillis, endMillis, offsetTimeZone });
    return ret as any
}