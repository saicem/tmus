import moment, { Duration } from "moment-timezone"
import cmd from "./cmd"
import { FileDetail, FocusData, FocusRecord } from "./data"
import { config } from "@/global/state.ts"

const exeDetailCache: Map<number, Promise<FileDetail>> = new Map<
  number,
  Promise<FileDetail>
>()

export async function readReverse(
  cursor: number | null,
  count: number
): Promise<[FocusData[], number | null]> {
  const [rawRecords, newCursor] = await cmd.readReverse(cursor, count)
  console.log("readReverse", rawRecords, newCursor)
  const records = rawRecords.map((x) => convertFocusData(x))
  return [records, newCursor]
}

export async function todayAppGeneral() {
  const end = moment()
  const start = end.clone().startOf("day")
  const records = await cmd.durationAggregate(start.valueOf(), end.valueOf())
  const result = Object.entries(records).map(async ([k, v]) => {
    return {
      file: await appDetail(Number.parseInt(k)),
      duration: moment.duration(v),
    }
  })
  return Promise.all(result)
}

export async function appDetail(id: number): Promise<FileDetail> {
  if (!exeDetailCache.has(id)) {
    const promise = cmd.fileDetail(id)
    exeDetailCache.set(id, promise)
    console.log("appDetail", await promise)
  }
  return await exeDetailCache.get(id)!
}

export async function durationByDayInThisYear() {
  const dayMillis = moment.duration(1, "day").asMilliseconds()
  const minMillis = moment.duration(1, "minute").asMilliseconds()
  const end = moment()
  const start = end.clone().startOf("year")
  const offset = end.utcOffset() * minMillis
  const records = await cmd.durationByDay(
    start.valueOf(),
    end.valueOf(),
    offset
  )
  const startDay = Math.floor((start.valueOf() + offset) / dayMillis)
  console.log("records", records)

  const ret: Record<number, Duration> = {}

  Object.entries(records).forEach(([k, v]) => {
    ret[Number.parseInt(k) - startDay + 1] = moment.duration(v)
  })
  return ret
}

export function convertFocusData(data: FocusRecord): FocusData {
  return {
    id: data.id,
    start: moment(data.focusAt),
    end: moment(data.blurAt),
    duration: moment.duration(data.blurAt - data.focusAt),
  }
}

export async function reloadConfig() {
  config.value = await cmd.getAppConfig()
}
