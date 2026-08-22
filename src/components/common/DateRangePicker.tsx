import { useEffect, useState } from "react"
import { format } from "date-fns"
import { enUS, zhCN } from "date-fns/locale"
import { Calendar as CalendarIcon } from "lucide-react"
import type { DateRange } from "react-day-picker"
import { Button } from "@/components/ui/button"
import { Calendar } from "@/components/ui/calendar"
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"
import { useI18n } from "@/lib/i18n"
import { useAppStore } from "@/lib/state"
import {
  getTimeRangeByDays,
  getTimeRangeByMonths,
  MILLISECONDS_PER_DAY,
} from "@/lib/time-util"

/** Start of the local day for a millisecond timestamp. */
function dayStart(ts: number): Date {
  const d = new Date(ts)
  return new Date(d.getFullYear(), d.getMonth(), d.getDate())
}

/**
 * Convert the app convention (start at 00:00, end at 23:59:59.999) into the
 * midnight-to-midnight range expected by the calendar.
 */
function millisToRange(value: [number, number]): DateRange {
  return { from: dayStart(value[0]), to: dayStart(value[1]) }
}

/** Convert a completed calendar range back to the app's millisecond convention. */
function rangeToMillis(range: DateRange): [number, number] {
  const from = range.from as Date
  const to = range.to as Date
  const start = new Date(
    from.getFullYear(),
    from.getMonth(),
    from.getDate()
  ).getTime()
  const end = new Date(to.getFullYear(), to.getMonth(), to.getDate()).getTime()
  return [start, end + MILLISECONDS_PER_DAY - 1]
}

/**
 * Single-button range picker built from a Popover and a Calendar in range
 * mode, so both the start and end dates are chosen in one calendar and the
 * range can never become invalid.
 */
export function DateRangePicker({
  value,
  onChange,
  showShortcuts = true,
  placeholder,
}: {
  value: [number, number]
  onChange: (v: [number, number]) => void
  showShortcuts?: boolean
  placeholder?: string
}) {
  const t = useI18n()
  const lang = useAppStore((s) => s.lang)
  const dateLocale = lang === "zh" ? zhCN : enUS
  const [open, setOpen] = useState(false)
  const [internal, setInternal] = useState<DateRange | undefined>(() =>
    millisToRange(value)
  )

  // Reset the in-progress selection to the committed value whenever the popover
  // is closed or the committed range changes (e.g. via a shortcut).
  useEffect(() => {
    setInternal(millisToRange(value))
  }, [value, open])

  const shortcuts: { label: string; range: () => [Date, Date] }[] = [
    {
      label: t.statisticPage.shortcuts.last1day,
      range: () => getTimeRangeByDays(1),
    },
    {
      label: t.statisticPage.shortcuts.last3days,
      range: () => getTimeRangeByDays(3),
    },
    {
      label: t.statisticPage.shortcuts.last1week,
      range: () => getTimeRangeByDays(7),
    },
    {
      label: t.statisticPage.shortcuts.last1month,
      range: () => getTimeRangeByMonths(1),
    },
    {
      label: t.statisticPage.shortcuts.last3months,
      range: () => getTimeRangeByMonths(3),
    },
    {
      label: t.statisticPage.shortcuts.last1year,
      range: () => getTimeRangeByMonths(12),
    },
  ]

  const handleSelect = (range: DateRange | undefined) => {
    setInternal(range)
    if (range?.from && range?.to) {
      onChange(rangeToMillis(range))
    }
  }

  const from = internal?.from
  const to = internal?.to
  const label = from
    ? to
      ? `${format(from, "PPP", { locale: dateLocale })} – ${format(to, "PPP", {
          locale: dateLocale,
        })}`
      : format(from, "PPP", { locale: dateLocale })
    : placeholder ?? t.statisticPage.label.pickDateRange

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          className="h-9 w-[300px] justify-start gap-2 text-left font-normal"
        >
          <CalendarIcon className="size-4 shrink-0 text-muted-foreground" />
          <span className="truncate">{label}</span>
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-auto p-0">
        <div className="flex items-stretch">
          {showShortcuts && (
            <div className="flex flex-col gap-1 border-r p-2">
              {shortcuts.map((s, i) => (
                <Button
                  key={i}
                  variant="ghost"
                  size="sm"
                  className="justify-start"
                  onClick={() => {
                    const [start, end] = s.range()
                    onChange([start.getTime(), end.getTime()])
                    setOpen(false)
                  }}
                >
                  {s.label}
                </Button>
              ))}
            </div>
          )}
          <Calendar
            mode="range"
            selected={internal}
            onSelect={handleSelect}
            defaultMonth={internal?.from}
            numberOfMonths={2}
            locale={dateLocale}
          />
        </div>
      </PopoverContent>
    </Popover>
  )
}
