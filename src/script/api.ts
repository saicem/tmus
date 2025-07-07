import moment, { Duration, Moment } from "moment-timezone"
import cmd, { getAllAppDetail, getAppConfig } from "@/script/cmd.ts"
import { FileDetail, FocusData, FocusRecord } from "./data"
import { config } from "@/script/state.ts"
import { timeZoneOffsetMillis } from "@/script/time-util.ts"

const dayMillis = moment.duration(1, "day").asMilliseconds()

export async function getAppDetailMap() {
  return (await getAllAppDetail()).reduce(
    (map, detail) => {
      map[detail.id] = detail
      return map
    },
    {} as Record<number, FileDetail>
  )
}

export async function todayAppGeneral() {
  const appDetailMap = await getAppDetailMap()
  const end = moment()
  const start = end.clone().startOf("day")
  const records = await cmd.durationById(start.valueOf(), end.valueOf())
  const result = Object.entries(records).map(async ([k, v]) => {
    return {
      file: appDetailMap[Number.parseInt(k)],
      duration: moment.duration(v),
    }
  })
  return Promise.all(result)
}

export async function durationByDayInThisYear() {
  const end = moment()
  const start = end.clone().startOf("year")
  const offset = timeZoneOffsetMillis()
  const records = await cmd.durationByDay(
    start.valueOf(),
    end.valueOf(),
    offset
  )
  const startDay = Math.floor((start.valueOf() + offset) / dayMillis)
  const ret: Record<number, Duration> = {}

  Object.entries(records).forEach(([k, v]) => {
    ret[Number.parseInt(k) - startDay + 1] = moment.duration(v)
  })
  return ret
}

export async function durationByDay(start: Moment, end: Moment) {
  return await cmd.durationByDay(
    start.valueOf(),
    end.valueOf(),
    timeZoneOffsetMillis()
  )
}

export async function durationById(start: Moment, end: Moment) {
  return await cmd.durationById(start.valueOf(), end.valueOf())
}

export async function durationByDayId(start: Moment, end: Moment) {
  return await cmd.durationByDayId(
    start.valueOf(),
    end.valueOf(),
    timeZoneOffsetMillis()
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
