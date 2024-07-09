import moment, { Duration, Moment } from "moment-timezone"
import cmd from "./cmd"
import { FileDetail, FocusRecord } from "./data"

async function readReverse(cursor: number | null, count: number) {
  const result = await cmd.readReverse(cursor, count)
  const ret = await Promise.all(result[0].map((x) => refreshAppDetail(x.id)))
  console.log("detail", ret)
  console.log("readReverse", result)
  const records = result[0].map((x) => convertFocusData(x))
  return [records, result[1]]
}

async function todayAppGeneral() {
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

const fileDetailCache: Record<number, FileDetail> = {}

async function refreshAppDetail(id: number) {
  if (fileDetailCache[id]) {
    return fileDetailCache[id]
  }
  return (fileDetailCache[id] = await cmd.fileDetail(id))
}

function appDetail(id: number): FileDetail {
  return fileDetailCache[id]
}

async function durationByDayInThisYear() {
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

interface FocusData {
  id: number
  start: Moment
  end: Moment
  duration: Duration
}

function convertFocusData(data: FocusRecord): FocusData {
  return {
    id: data.id,
    start: moment(data.focusAt),
    end: moment(data.blurAt),
    duration: moment.duration(data.blurAt - data.focusAt),
  }
}

export default {
  readReverse,
  todayAppGeneral,
  appDetail,
  durationByDayInThisYear,
}
