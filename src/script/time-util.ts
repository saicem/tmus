import { Duration, Moment } from "moment-timezone"
import { config } from "@/script/state.ts"

export const formatDuration = (duration: Duration) => {
  let parts = []
  if (duration.days() > 0) {
    parts.push(duration.days() + "d")
  }
  if (duration.hours() > 0) {
    parts.push(duration.hours() + "h")
  }
  if (duration.minutes() > 0) {
    parts.push(duration.minutes() + "m")
  }
  if (duration.seconds() > 0) {
    parts.push(duration.seconds() + "s")
  }
  return parts.join(" ")
}

/**
 * In moment js, day of week start from 0 to 6. 0 is Sunday, 6 is Saturday.
 * But in windows, day of week start from 0 to 6. Monday to Sunday.
 * @param day The day need to get the offset from the config start day of week
 */
export const dayOfWeekOffset = (day: Moment) => {
  return (day.day() - 1 - config.value.firstDayOfWeek + 7) % 7
}

/**
 * Calculates the number of days since the epoch (January 1, 1970) for a given moment,
 * adjusted for the local timezone offset.
 *
 * @param day - The Moment object representing the date and time.
 * @returns The number of days since the epoch.
 */
export const dayFromEpoch = (day: Moment) => {
  return Math.trunc((day.valueOf() / 1000 / 60 + day.utcOffset()) / 60 / 24)
}
