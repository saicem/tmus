import moment, { Duration } from "moment-timezone"
import { durationAggregate, fileDetail, durationByDay } from "./api"

export async function todayAppGeneral() {
  const end = moment()
  const start = end.clone().startOf("day")
  const records = await durationAggregate(start.valueOf(), end.valueOf())
  const result = Object.entries(records).map(async ([k, v]) => {
    return {
      file: await fileDetail(Number.parseInt(k)),
      duration: moment.duration(v),
    }
  })
  return Promise.all(result)
}

const fileDetailCache: Record<number, FileDetail> = {}

export async function appDetail(id: number): Promise<FileDetail> {
  if (fileDetailCache[id]) {
    return fileDetailCache[id]
  }
  return (fileDetailCache[id] = await fileDetail(id))
}

export async function durationByDayInThisYear() {
  const dayMillis = moment.duration(1, "day").asMilliseconds()
  const minMillis = moment.duration(1, "minute").asMilliseconds()
  const end = moment()
  const start = end.clone().startOf("year")
  const offset = end.utcOffset() * minMillis
  const records = await durationByDay(start.valueOf(), end.valueOf(), offset)
  const startDay = Math.floor((start.valueOf() + offset) / dayMillis)

  const ret: Record<number, Duration> = {}

  Object.entries(records).forEach(([k, v]) => {
    ret[Number.parseInt(k) - startDay + 1] = moment.duration(v)
    console.log(moment().dayOfYear(Number.parseInt(k) - startDay + 1))
  })
  return ret
}
