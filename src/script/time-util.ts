import { configStore } from "@/script/state.ts"
import { getDay } from "date-fns"

export const MILLISECONDS_PER_SECOND = 1000
export const MILLISECONDS_PER_MINUTE = 60 * MILLISECONDS_PER_SECOND
export const MILLISECONDS_PER_HOUR = 60 * MILLISECONDS_PER_MINUTE
export const MILLISECONDS_PER_DAY = 24 * MILLISECONDS_PER_HOUR

export const formatDuration = (duration: number) => {
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

export const formatDurationRough = (ms: number) => {
  if (ms < 1000) return `${ms}ms`
  const s = ms / 1000
  if (s < 60) return `${s.toFixed(1)}s`
  const m = s / 60
  if (m < 60) return `${m.toFixed(1)}m`
  const h = m / 60
  if (h < 24) return `${h.toFixed(1)}h`
  const d = h / 24
  return `${d.toFixed(1)}d`
}

export const formatDurationInHours = (ms: number) => {
  const h = ms / 60 /60 / 1000
  return `${h.toFixed(1)}h`
}

export const dayOfWeekOffset = (day: Date) => {
  return (getDay(day) - 1 - configStore.firstDayOfWeek + 14) % 7
}

export const timeZoneOffsetMillis = (): number =>
  new Date().getTimezoneOffset() * MILLISECONDS_PER_MINUTE
