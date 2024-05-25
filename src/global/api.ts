import moment from "moment"
import { durationAggregate, fileVersionById } from "./command"

export async function latestTwoWeek() {
    let end = moment()
    let start = end.clone().startOf('week').subtract(7, 'days')
    let records = await durationAggregate(start, end)
    let result = Object.entries(records).map(async ([k, v]) => {
        return {
            file: await fileVersionById(Number.parseInt(k)),
            duration: moment.duration(v).toISOString()
        }
    })
    console.log(await Promise.all(result))
}