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
  const h = ms / 60 / 60 / 1000
  return `${h.toFixed(1)}h`
}

export const dayOfWeekOffset = (day: Date) => {
  return (getDay(day) - 1 - configStore.firstDayOfWeek + 14) % 7
}

export const timeZoneOffsetMillis = (): number =>
  new Date().getTimezoneOffset() * MILLISECONDS_PER_MINUTE

export const getTimeRangeByDays = (startDaysBefore: number, endDaysBefore: number = 0): [Date, Date] => {
  const end = new Date()
  end.setHours(0, 0, 0, 0)
  end.setDate(end.getDate() + 1)
  const start = new Date(end)
  start.setDate(start.getDate() - startDaysBefore)
  end.setDate(end.getDate() - endDaysBefore)
  return [start, end]
}

export const getTimeRangeByMonths = (startMonthsBefore: number, endMonthsBefore: number = 0): [Date, Date] => {
  const end = new Date()
  end.setHours(0, 0, 0, 0)
  end.setDate(end.getDate() + 1)
  const start = new Date(end)
  start.setMonth(start.getMonth() - startMonthsBefore)
  end.setMonth(end.getMonth() - endMonthsBefore)
  return [start, end]
}

export const minutesFormatInDay = (minutes: number) => {
  const h = Math.floor(minutes / 60)
  const m = minutes % 60
  return `${h}:${m.toString().padStart(2, "02")}`
}

export const minutesFormatInWeek = (minutes: number) => {
  const dayMinutes = 24 * 60
  const dayOfWeek = Math.floor(minutes / dayMinutes)
  return ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"][dayOfWeek] + " " + minutesFormatInDay(minutes % dayMinutes)
}
