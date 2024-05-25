import { invoke } from "@tauri-apps/api";
import { Moment } from "moment";

export async function durationAggregate(start: Moment, end: Moment): Promise<Record<number, number>> {
    let ret = await invoke('duration_aggregate', { startStr: start.format(), endStr: end.format() });
    return ret as any
}

export async function fileVersionById(id: number): Promise<FileVersion> {
    let ret = await invoke('file_version_by_id', { id: id })
    return ret as any
}