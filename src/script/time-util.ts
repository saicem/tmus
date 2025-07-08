import moment, { Duration, Moment } from "moment-timezone"
import { config } from "@/script/state.ts"

export const MILLISECONDS_PER_SECOND = 1000
export const MILLISECONDS_PER_MINUTE = 60 * MILLISECONDS_PER_SECOND
export const MILLISECONDS_PER_HOUR = 60 * MILLISECONDS_PER_MINUTE
export const MILLISECONDS_PER_DAY = 24 * MILLISECONDS_PER_HOUR

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

export const formatDurationRaw = (duration: number) => {
  let days = Math.floor(duration / MILLISECONDS_PER_DAY)
  duration = duration % MILLISECONDS_PER_DAY
  let hours = Math.floor(duration / MILLISECONDS_PER_HOUR)
  duration = duration % MILLISECONDS_PER_HOUR
  let minutes = Math.floor(duration / MILLISECONDS_PER_MINUTE)
  duration = duration % MILLISECONDS_PER_MINUTE
  let seconds = Math.floor(duration / MILLISECONDS_PER_SECOND)

  let parts = []
  if (days) {
    parts.push(days + "d")
  }
  if (hours) {
    parts.push(hours + "h")
  }
  if (minutes) {
    parts.push(minutes + "m")
  }
  if (seconds) {
    parts.push(seconds + "s")
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

export const timeZoneOffsetMillis = () =>
  moment().utcOffset() * moment.duration(1, "minute").asMilliseconds()
