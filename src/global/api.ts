import moment, { Duration, Moment } from "moment-timezone"
import cmd, { getAppConfig } from "@/global/cmd.ts"
import { FileDetail, FocusData, FocusRecord } from "./data"
import { config } from "@/global/state.ts"
import { getApp, saveApp } from "@/global/db.ts"

const appDetailCache: Map<number, Promise<FileDetail>> = new Map<
  number,
  Promise<FileDetail>
>()

const minMillis = moment.duration(1, "minute").asMilliseconds()
const dayMillis = moment.duration(1, "day").asMilliseconds()

export async function todayAppGeneral() {
  const end = moment()
  const start = end.clone().startOf("day")
  const records = await cmd.durationById(start.valueOf(), end.valueOf())
  const result = Object.entries(records).map(async ([k, v]) => {
    return {
      file: await appDetail(Number.parseInt(k)),
      duration: moment.duration(v),
    }
  })
  return Promise.all(result)
}

export async function appDetail(id: number): Promise<FileDetail> {
  let app = await getApp(id)
  if (app != null) {
    return app
  }
  return requestAppDetail(id)
}

export async function requestAppDetail(id: number): Promise<FileDetail> {
  if (appDetailCache.has(id)) {
    return appDetailCache.get(id)!
  }
  const promise = cmd.fileDetail(id)
  appDetailCache.set(id, promise)
  const app = await promise
  await saveApp(app)
  appDetailCache.delete(id)
  return app!
}

export async function durationByDayInThisYear() {
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

export async function durationById(start: Moment, end: Moment) {
  return await cmd.durationById(start.valueOf(), end.valueOf())
}

export async function durationByDayId(start: Moment, end: Moment) {
  return await cmd.durationByDayId(
    start.valueOf(),
    end.valueOf(),
    end.utcOffset() * minMillis
  )
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
  config.value = await getAppConfig()
}
