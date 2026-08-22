import {
  format,
  getDay,
  getDayOfYear,
  isLeapYear,
  setDayOfYear,
  setMonth,
  startOfMonth,
  startOfYear,
} from "date-fns"
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip"
import { useAppStore } from "@/lib/state"
import { formatDuration, MILLISECONDS_PER_HOUR } from "@/lib/time-util"

const dayOfWeekList = ["Sun", "Mon", "Tues", "Wed", "Thur", "Fri", "Sat"]
const monthList = [
  "Jan",
  "Feb",
  "Mar",
  "Apr",
  "May",
  "Jun",
  "Jul",
  "Aug",
  "Sep",
  "Oct",
  "Nov",
  "Dec",
]

/**
 * GitHub-style yearly usage heatmap, faithfully ported from the original
 * Element Plus table implementation:
 * - one column per week, one row per day of week (Sun first)
 * - month labels with colspan spanning the weeks each month covers
 * - day-of-week labels on odd rows
 * - 5-level blue intensity: 1 level per 4 hours of usage
 */
export function HeatCalendar({ data }: { data: number[] }) {
  const dateFormat = useAppStore((s) => s.config.dateFormat)
  const now = new Date()
  const emptyCellCount = getDay(startOfYear(now))
  const daysInYearCount = isLeapYear(now) ? 366 : 365
  const weekCount = Math.ceil((emptyCellCount + daysInYearCount) / 7)
  const monthWeeks = Array.from({ length: 12 }, (_, idx) => {
    const monthFirstDayCellNum =
      getDayOfYear(startOfMonth(setMonth(now, idx))) + emptyCellCount
    return Math.floor((monthFirstDayCellNum - 1) / 7)
  })

  const dayOfYear = (week: number, dow: number) =>
    week * 7 + dow + 1 - emptyCellCount
  const showCell = (week: number, dow: number) => {
    const doy = dayOfYear(week, dow)
    return doy > 0 && doy <= daysInYearCount
  }
  const getCellDate = (week: number, dow: number) =>
    format(setDayOfYear(new Date(), dayOfYear(week, dow)), dateFormat)

  const tagOf = (duration: number) =>
    Math.min(4, Math.ceil(duration / MILLISECONDS_PER_HOUR / 4))

  return (
    <div className="overflow-x-auto">
      <table className="border-separate" style={{ borderSpacing: "2px" }}>
        <thead>
          <tr>
            <th className="h-3.5" />
            {Array.from({ length: 12 }, (_, i) => (
              <th
                key={i}
                colSpan={
                  i === 11
                    ? weekCount - monthWeeks[i]
                    : monthWeeks[i + 1] - monthWeeks[i]
                }
                className="text-left text-xs font-bold text-muted-foreground"
              >
                {monthList[i]}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {Array.from({ length: 7 }, (_, dow) => (
            <tr key={dow}>
              <th className="w-8 pr-1 text-right text-xs font-bold text-muted-foreground">
                {dow % 2 === 1 ? dayOfWeekList[dow] : ""}
              </th>
              {Array.from({ length: weekCount }, (_, week) => {
                if (!showCell(week, dow)) {
                  return <td key={week} className="p-0" />
                }
                const doy = dayOfYear(week, dow)
                const duration = data[doy] ?? 0
                return (
                  <td key={week} className="p-0">
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <div data-tag={tagOf(duration)} className="heat-cell" />
                      </TooltipTrigger>
                      <TooltipContent className="text-center">
                        <div>{getCellDate(week, dow)}</div>
                        <div>{formatDuration(duration)}</div>
                      </TooltipContent>
                    </Tooltip>
                  </td>
                )
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
