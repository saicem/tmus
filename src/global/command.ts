import moment from "moment-timezone"
import { durationAggregate, fileDetail, durationByDay } from "./api"

export async function todayAppGeneral() {
    let end = moment()
    let start = end.clone().startOf('day')
    let records = await durationAggregate(start.valueOf(), end.valueOf())
    let result = Object.entries(records).map(async ([k, v]) => {
        return {
            file: await fileDetail(Number.parseInt(k)),
            duration: moment.duration(v)
        }
    })
    return Promise.all(result)
}

const fileDetailCache: { [key: number]: FileDetail } = {}

export async function appDetail(id: number): Promise<FileDetail> {
    if (fileDetailCache[id]) {
        return fileDetailCache[id]
    }
    return fileDetailCache[id] = await fileDetail(id)
}

export async function durationByDayInThisYear() {
    const dayMillis = moment.duration(1, 'day').asMilliseconds();
    const minMillis = moment.duration(1, 'minute').asMilliseconds();
    let end = moment()
    let start = end.clone().startOf('year')
    let offset = end.utcOffset() * minMillis;
    let records = await durationByDay(start.valueOf(), end.valueOf(), offset);
    let result = Object.entries(records).map(async ([k, v]) => {
        return {
            date: moment.unix(Number.parseInt(k) * dayMillis),
            duration: moment.duration(v)
        }
    })
    return Promise.all(result)
}