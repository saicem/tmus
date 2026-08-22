import { getDay } from "date-fns"
import { useAppStore } from "@/lib/state"

export const MILLISECONDS_PER_SECOND = 1000
export const MILLISECONDS_PER_MINUTE = 60 * MILLISECONDS_PER_SECOND
export const MILLISECONDS_PER_HOUR = 60 * MILLISECONDS_PER_MINUTE
export const MILLISECONDS_PER_DAY = 24 * MILLISECONDS_PER_HOUR

export const formatDuration = (duration: number) => {
  const days = Math.floor(duration / MILLISECONDS_PER_DAY)
  duration = duration % MILLISECONDS_PER_DAY
  const hours = Math.floor(duration / MILLISECONDS_PER_HOUR)
  duration = duration % MILLISECONDS_PER_HOUR
  const minutes = Math.floor(duration / MILLISECONDS_PER_MINUTE)
  duration = duration % MILLISECONDS_PER_MINUTE
  const seconds = Math.floor(duration / MILLISECONDS_PER_SECOND)

  const parts = []
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
  const sign = ms < 0 ? "-" : ""
  const abs = Math.abs(ms)
  if (abs < 1000) return `${sign}${abs}ms`
  const s = abs / 1000
  if (s < 60) return `${sign}${s.toFixed(1)}s`
  const m = s / 60
  if (m < 60) return `${sign}${m.toFixed(1)}m`
  const h = m / 60
  if (h < 24) return `${sign}${h.toFixed(1)}h`
  const d = h / 24
  return `${sign}${d.toFixed(1)}d`
}

export const dayOfWeekOffset = (day: Date) => {
  const firstDayOfWeek = useAppStore.getState().config.firstDayOfWeek
  return (getDay(day) - 1 - firstDayOfWeek + 14) % 7
}

export const timeZoneOffsetMillis = (): number =>
  new Date().getTimezoneOffset() * MILLISECONDS_PER_MINUTE

export const getTimeRangeByDays = (
  startDaysBefore: number,
  endDaysBefore = 0
): [Date, Date] => {
  const end = new Date()
  end.setHours(0, 0, 0, 0)
  end.setDate(end.getDate() + 1)
  const start = new Date(end)
  start.setDate(start.getDate() - startDaysBefore)
  end.setDate(end.getDate() - endDaysBefore)
  return [start, end]
}

export const getTimeRangeByMonths = (
  startMonthsBefore: number,
  endMonthsBefore = 0
): [Date, Date] => {
  const end = new Date()
  end.setHours(0, 0, 0, 0)
  end.setDate(end.getDate() + 1)
  const start = new Date(end)
  start.setMonth(start.getMonth() - startMonthsBefore)
  end.setMonth(end.getMonth() - endMonthsBefore)
  return [start, end]
}

/** Format decimal hours (e.g. 10.5) as "HH:mm" (10:30). */
export const formatTimeOfDay = (hours: number): string => {
  const h = Math.floor(hours)
  const m = Math.round((hours - h) * 60)
  return `${String(h).padStart(2, "0")}:${String(m).padStart(2, "0")}`
}
