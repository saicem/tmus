import { useCallback, useEffect, useMemo, useState } from "react"
import { Plus, Trash2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Empty } from "@/components/ui/empty"
import { Progress } from "@/components/ui/progress"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { BarChart } from "@/lib/charts/BarChart"
import { PieChart } from "@/lib/charts/PieChart"
import { RadarChart } from "@/lib/charts/RadarChart"
import {
  getAllCategories,
  getAppTotalDuration,
  getAppUsageDays,
  getCategoryTotalDuration,
  getCategoryUsageDays,
  getCategoryUsageRhythm,
} from "@/lib/cmd"
import { useI18n } from "@/lib/i18n"
import type { CategorySimple, TimeSpan } from "@/lib/models"
import {
  formatDurationRough,
  formatTimeOfDay,
  getTimeRangeByDays,
  getTimeRangeByMonths,
  MILLISECONDS_PER_DAY,
  MILLISECONDS_PER_HOUR,
  MILLISECONDS_PER_MINUTE,
} from "@/lib/time-util"

type StatisticType =
  | "AppDuration"
  | "AppDays"
  | "CategoryDuration"
  | "CategoryDays"
  | "CategoryRhythm"
type DisplayStyle = "Pie" | "Bar" | "radar"

function dateToInput(d: Date): string {
  const y = d.getFullYear()
  const m = String(d.getMonth() + 1).padStart(2, "0")
  const day = String(d.getDate()).padStart(2, "0")
  return `${y}-${m}-${day}`
}

function inputToStart(s: string): number {
  const [y, m, d] = s.split("-").map(Number)
  return new Date(y, m - 1, d).getTime()
}

function inputToEnd(s: string): number {
  return inputToStart(s) + MILLISECONDS_PER_DAY - 1
}

function startToInput(ms: number): string {
  return dateToInput(new Date(ms))
}

function endToInput(ms: number): string {
  return dateToInput(new Date(ms))
}

function TimeRangePicker({
  value,
  onChange,
}: {
  value: [number, number]
  onChange: (v: [number, number]) => void
}) {
  const t = useI18n()
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
  return (
    <div className="flex flex-wrap items-center gap-2">
      <input
        type="date"
        className="h-9 rounded-md border bg-transparent px-3 text-sm"
        value={startToInput(value[0])}
        onChange={(e) => onChange([inputToStart(e.target.value), value[1]])}
      />
      <span className="text-muted-foreground">~</span>
      <input
        type="date"
        className="h-9 rounded-md border bg-transparent px-3 text-sm"
        value={endToInput(value[1])}
        onChange={(e) => onChange([value[0], inputToEnd(e.target.value)])}
      />
      <Select
        onValueChange={(idx) => {
          const [s, e] = shortcuts[Number(idx)].range()
          onChange([s.getTime(), e.getTime()])
        }}
      >
        <SelectTrigger className="h-9 w-36">
          <SelectValue placeholder={t.statisticPage.label.timeRange} />
        </SelectTrigger>
        <SelectContent>
          {shortcuts.map((s, i) => (
            <SelectItem key={i} value={String(i)}>
              {s.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  )
}

function StylePicker({
  value,
  options,
  onChange,
}: {
  value: DisplayStyle
  options: { label: string; value: DisplayStyle }[]
  onChange: (v: DisplayStyle) => void
}) {
  return (
    <div className="flex gap-1">
      {options.map((opt) => (
        <Button
          key={opt.value}
          size="sm"
          variant={value === opt.value ? "default" : "outline"}
          onClick={() => onChange(opt.value)}
        >
          {opt.label}
        </Button>
      ))}
    </div>
  )
}

function ProgressList({
  items,
  format,
}: {
  items: { id: number; label: string; value: number; percentage: number }[]
  format: (v: number) => string
}) {
  const max = items[0]?.percentage ?? 0
  return (
    <div className="flex flex-col gap-4">
      {items.map((item) => (
        <div key={item.id} className="flex flex-col gap-1.5">
          <div className="flex items-center justify-between text-sm">
            <span className="truncate">{item.label}</span>
            <span className="text-muted-foreground">
              {format(item.value)} ({item.percentage}%)
            </span>
          </div>
          <Progress value={(item.percentage / Math.max(max, 1)) * 100} />
        </div>
      ))}
    </div>
  )
}

interface DurationItem {
  id: number
  name: string
  duration: number
  proportion: number
}

function useCategories() {
  const [categories, setCategories] = useState<CategorySimple[]>([])
  useEffect(() => {
    let cancelled = false
    void getAllCategories().then((result) => {
      if (!cancelled) setCategories(result)
    })
    return () => {
      cancelled = true
    }
  }, [])
  return categories
}

function AppDurationPanel({ t }: { t: ReturnType<typeof useI18n> }) {
  const categories = useCategories()
  const [range, setRange] = useState<[number, number]>(() => {
    const [s, e] = getTimeRangeByDays(1)
    return [s.getTime(), e.getTime()]
  })
  const [categoryId, setCategoryId] = useState<string>("")
  const [style, setStyle] = useState<DisplayStyle>("Pie")
  const [items, setItems] = useState<DurationItem[]>([])

  const load = useCallback(async () => {
    const response = await getAppTotalDuration({
      startTime: range[0],
      endTime: range[1],
      categoryId: categoryId ? Number(categoryId) : undefined,
    })
    const total = response.detail.reduce((sum, item) => sum + item.value, 0)
    setItems(
      response.detail.map((item) => ({
        id: item.app.id,
        name: item.app.name,
        duration: item.value,
        proportion: total > 0 ? item.value / total : 0,
      }))
    )
  }, [range, categoryId])

  useEffect(() => {
    void load()
  }, [load])

  return (
    <>
      <Card>
        <CardContent className="flex flex-col gap-4 pt-6">
          <div className="flex flex-wrap items-end gap-4">
            <div className="flex flex-col gap-2">
              <span className="text-sm font-medium">
                {t.statisticPage.label.timeRange}
              </span>
              <TimeRangePicker value={range} onChange={setRange} />
            </div>
            <div className="flex flex-col gap-2">
              <span className="text-sm font-medium">
                {t.statisticPage.categories.title}
              </span>
              <Select value={categoryId} onValueChange={setCategoryId}>
                <SelectTrigger className="h-9 w-48">
                  <SelectValue
                    placeholder={t.statisticPage.categories.placeholder}
                  />
                </SelectTrigger>
                <SelectContent>
                  {categories.map((c) => (
                    <SelectItem key={c.id} value={String(c.id)}>
                      {c.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div className="flex flex-col gap-2">
              <span className="text-sm font-medium">
                {t.statisticPage.label.displayStyle}
              </span>
              <StylePicker
                value={style}
                options={[
                  { label: t.statisticPage.displayStyle.pie, value: "Pie" },
                  { label: t.statisticPage.displayStyle.bar, value: "Bar" },
                ]}
                onChange={setStyle}
              />
            </div>
          </div>
          {items.length > 0 && style === "Pie" && (
            <PieChart
              data={items.map((i) => ({ name: i.name, value: i.duration }))}
              formatValue={formatDurationRough}
            />
          )}
          {items.length > 0 && style === "Bar" && (
            <BarChart
              data={items.map((i) => ({ label: i.name, value: i.duration }))}
              formatValue={formatDurationRough}
            />
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t.statisticPage.detail}</CardTitle>
        </CardHeader>
        <CardContent>
          {items.length === 0 ? (
            <Empty />
          ) : (
            <ProgressList
              items={items.map((i) => ({
                id: i.id,
                label: i.name,
                value: i.duration,
                percentage: Math.round(i.proportion * 1000) / 10,
              }))}
              format={formatDurationRough}
            />
          )}
        </CardContent>
      </Card>
    </>
  )
}

function AppDaysPanel({ t }: { t: ReturnType<typeof useI18n> }) {
  const categories = useCategories()
  const [range, setRange] = useState<[number, number]>(() => {
    const [s, e] = getTimeRangeByDays(1)
    return [s.getTime(), e.getTime()]
  })
  const [categoryId, setCategoryId] = useState<string>("")
  const [style, setStyle] = useState<DisplayStyle>("Pie")
  const [items, setItems] = useState<DurationItem[]>([])

  const load = useCallback(async () => {
    const response = await getAppUsageDays({
      startTime: range[0],
      endTime: range[1],
      categoryId: categoryId ? Number(categoryId) : undefined,
    })
    const total = response.detail.reduce((sum, item) => sum + item.value, 0)
    setItems(
      response.detail.map((item) => ({
        id: item.app.id,
        name: item.app.name,
        duration: item.value,
        proportion: total > 0 ? item.value / total : 0,
      }))
    )
  }, [range, categoryId])

  useEffect(() => {
    void load()
  }, [load])

  return (
    <>
      <Card>
        <CardContent className="flex flex-col gap-4 pt-6">
          <div className="flex flex-wrap items-end gap-4">
            <div className="flex flex-col gap-2">
              <span className="text-sm font-medium">
                {t.statisticPage.label.timeRange}
              </span>
              <TimeRangePicker value={range} onChange={setRange} />
            </div>
            <div className="flex flex-col gap-2">
              <span className="text-sm font-medium">
                {t.statisticPage.categories.title}
              </span>
              <Select value={categoryId} onValueChange={setCategoryId}>
                <SelectTrigger className="h-9 w-48">
                  <SelectValue
                    placeholder={t.statisticPage.categories.placeholder}
                  />
                </SelectTrigger>
                <SelectContent>
                  {categories.map((c) => (
                    <SelectItem key={c.id} value={String(c.id)}>
                      {c.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div className="flex flex-col gap-2">
              <span className="text-sm font-medium">
                {t.statisticPage.label.displayStyle}
              </span>
              <StylePicker
                value={style}
                options={[
                  { label: t.statisticPage.displayStyle.pie, value: "Pie" },
                  { label: t.statisticPage.displayStyle.bar, value: "Bar" },
                ]}
                onChange={setStyle}
              />
            </div>
          </div>
          {items.length > 0 && style === "Pie" && (
            <PieChart
              data={items.map((i) => ({ name: i.name, value: i.duration }))}
              formatValue={(v) => `${v} ${t.statisticPage.unit.day}`}
            />
          )}
          {items.length > 0 && style === "Bar" && (
            <BarChart
              data={items.map((i) => ({ label: i.name, value: i.duration }))}
              formatValue={(v) => `${v} ${t.statisticPage.unit.day}`}
            />
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t.statisticPage.detail}</CardTitle>
        </CardHeader>
        <CardContent>
          {items.length === 0 ? (
            <Empty />
          ) : (
            <ProgressList
              items={items.map((i) => ({
                id: i.id,
                label: i.name,
                value: i.duration,
                percentage: Math.round(i.proportion * 1000) / 10,
              }))}
              format={(v) => `${v.toFixed(0)}${t.statisticPage.unit.day}`}
            />
          )}
        </CardContent>
      </Card>
    </>
  )
}

function CategoryPanel({
  t,
  kind,
}: {
  t: ReturnType<typeof useI18n>
  kind: "duration" | "days"
}) {
  const categories = useCategories()
  const [range, setRange] = useState<[number, number]>(() => {
    const [s, e] = getTimeRangeByDays(1)
    return [s.getTime(), e.getTime()]
  })
  const [categoryIds, setCategoryIds] = useState<string[]>([])
  const [style, setStyle] = useState<DisplayStyle>("Pie")
  const [items, setItems] = useState<DurationItem[]>([])

  const load = useCallback(async () => {
    if (categoryIds.length === 0) {
      setItems([])
      return
    }
    const ids = categoryIds.map(Number)
    const response =
      kind === "duration"
        ? await getCategoryTotalDuration({
            startTime: range[0],
            endTime: range[1],
            categoryIds: ids,
          })
        : await getCategoryUsageDays({
            startTime: range[0],
            endTime: range[1],
            categoryIds: ids,
          })
    const total = response.detail.reduce((sum, item) => sum + item.value, 0)
    setItems(
      response.detail.map((item) => ({
        id: item.category.id,
        name: item.category.name,
        duration: item.value,
        proportion: total > 0 ? item.value / total : 0,
      }))
    )
  }, [range, categoryIds, kind])

  useEffect(() => {
    void load()
  }, [load])

  const formatValue =
    kind === "duration"
      ? formatDurationRough
      : (v: number) => `${v} ${t.statisticPage.unit.day}`
  const formatList =
    kind === "duration"
      ? formatDurationRough
      : (v: number) => `${v.toFixed(0)}${t.statisticPage.unit.day}`

  return (
    <>
      <Card>
        <CardContent className="flex flex-col gap-4 pt-6">
          <div className="flex flex-wrap items-end gap-4">
            <div className="flex flex-col gap-2">
              <span className="text-sm font-medium">
                {t.statisticPage.label.timeRange}
              </span>
              <TimeRangePicker value={range} onChange={setRange} />
            </div>
            <div className="flex flex-col gap-2">
              <span className="text-sm font-medium">
                {t.statisticPage.categories.title}
              </span>
              <div className="flex max-w-md flex-wrap gap-2">
                {categories.map((c) => {
                  const checked = categoryIds.includes(String(c.id))
                  return (
                    <label
                      key={c.id}
                      className="flex cursor-pointer items-center gap-1.5 text-sm"
                    >
                      <input
                        type="checkbox"
                        className="size-4"
                        checked={checked}
                        onChange={() =>
                          setCategoryIds((prev) =>
                            checked
                              ? prev.filter((id) => id !== String(c.id))
                              : [...prev, String(c.id)]
                          )
                        }
                      />
                      {c.name}
                    </label>
                  )
                })}
              </div>
            </div>
            <div className="flex flex-col gap-2">
              <span className="text-sm font-medium">
                {t.statisticPage.label.displayStyle}
              </span>
              <StylePicker
                value={style}
                options={[
                  { label: t.statisticPage.displayStyle.pie, value: "Pie" },
                  { label: t.statisticPage.displayStyle.bar, value: "Bar" },
                ]}
                onChange={setStyle}
              />
            </div>
          </div>
          {items.length > 0 && style === "Pie" && (
            <PieChart
              data={items.map((i) => ({ name: i.name, value: i.duration }))}
              formatValue={formatValue}
            />
          )}
          {items.length > 0 && style === "Bar" && (
            <BarChart
              data={items.map((i) => ({ label: i.name, value: i.duration }))}
              formatValue={formatValue}
            />
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t.statisticPage.detail}</CardTitle>
        </CardHeader>
        <CardContent>
          {items.length === 0 ? (
            <Empty />
          ) : (
            <ProgressList
              items={items.map((i) => ({
                id: i.id,
                label: i.name,
                value: i.duration,
                percentage: Math.round(i.proportion * 1000) / 10,
              }))}
              format={formatList}
            />
          )}
        </CardContent>
      </Card>
    </>
  )
}

interface RhythmGroupState {
  start: number
  end: number
  categoryId: string
}

function CategoryRhythmPanel({ t }: { t: ReturnType<typeof useI18n> }) {
  const categories = useCategories()
  const [timeSpan, setTimeSpan] = useState<TimeSpan>("day")
  const [granularity, setGranularity] = useState(MILLISECONDS_PER_HOUR)
  const [style, setStyle] = useState<DisplayStyle>("radar")
  const [groups, setGroups] = useState<RhythmGroupState[]>(() => {
    const end = new Date()
    end.setHours(24, 0, 0, 0)
    const start = new Date(end.getTime() - 7 * MILLISECONDS_PER_DAY)
    return [{ start: start.getTime(), end: end.getTime(), categoryId: "" }]
  })
  const [chartData, setChartData] = useState<number[][]>([])

  const granularityOptions = useMemo(() => {
    const day = [
      {
        label: t.statisticPage.granularity.minute5,
        value: 5 * MILLISECONDS_PER_MINUTE,
      },
      {
        label: t.statisticPage.granularity.minute15,
        value: 15 * MILLISECONDS_PER_MINUTE,
      },
      {
        label: t.statisticPage.granularity.minute30,
        value: 30 * MILLISECONDS_PER_MINUTE,
      },
      {
        label: t.statisticPage.granularity.hour1,
        value: MILLISECONDS_PER_HOUR,
      },
      {
        label: t.statisticPage.granularity.hour2,
        value: 2 * MILLISECONDS_PER_HOUR,
      },
      {
        label: t.statisticPage.granularity.hour3,
        value: 3 * MILLISECONDS_PER_HOUR,
      },
      {
        label: t.statisticPage.granularity.hour4,
        value: 4 * MILLISECONDS_PER_HOUR,
      },
      {
        label: t.statisticPage.granularity.hour6,
        value: 6 * MILLISECONDS_PER_HOUR,
      },
    ]
    const week = [
      {
        label: t.statisticPage.granularity.hour1,
        value: MILLISECONDS_PER_HOUR,
      },
      {
        label: t.statisticPage.granularity.hour2,
        value: 2 * MILLISECONDS_PER_HOUR,
      },
      {
        label: t.statisticPage.granularity.hour3,
        value: 3 * MILLISECONDS_PER_HOUR,
      },
      {
        label: t.statisticPage.granularity.hour4,
        value: 4 * MILLISECONDS_PER_HOUR,
      },
      {
        label: t.statisticPage.granularity.hour6,
        value: 6 * MILLISECONDS_PER_HOUR,
      },
      { label: t.statisticPage.granularity.day1, value: MILLISECONDS_PER_DAY },
    ]
    return timeSpan === "day" ? day : week
  }, [timeSpan, t])

  useEffect(() => {
    setGranularity(
      timeSpan === "day" ? MILLISECONDS_PER_HOUR : MILLISECONDS_PER_DAY
    )
  }, [timeSpan])

  const load = useCallback(async () => {
    if (groups.length === 0) {
      setChartData([])
      return
    }
    const response = await getCategoryUsageRhythm({
      groups: groups.map((g) => ({
        startTime: g.start,
        endTime: g.end,
        categoryId: g.categoryId ? Number(g.categoryId) : undefined,
      })),
      span: timeSpan,
      granularity,
    })
    setChartData(response.values)
  }, [groups, timeSpan, granularity])

  useEffect(() => {
    void load()
  }, [load])

  const points = useMemo(() => {
    return chartData.flatMap((row, gi) =>
      row.map((value, i) => ({ group: gi, index: i, value }))
    )
  }, [chartData])

  return (
    <Card>
      <CardContent className="flex flex-col gap-4 pt-6">
        <div className="flex flex-wrap items-end gap-4">
          <div className="flex flex-col gap-2">
            <span className="text-sm font-medium">
              {t.statisticPage.label.timeSpan}
            </span>
            <div className="flex gap-1">
              <Button
                size="sm"
                variant={timeSpan === "day" ? "default" : "outline"}
                onClick={() => setTimeSpan("day")}
              >
                {t.statisticPage.timeSpan.day}
              </Button>
              <Button
                size="sm"
                variant={timeSpan === "week" ? "default" : "outline"}
                onClick={() => setTimeSpan("week")}
              >
                {t.statisticPage.timeSpan.week}
              </Button>
            </div>
          </div>
          <div className="flex flex-col gap-2">
            <span className="text-sm font-medium">
              {t.statisticPage.label.granularity}
            </span>
            <Select
              value={String(granularity)}
              onValueChange={(v) => setGranularity(Number(v))}
            >
              <SelectTrigger className="h-9 w-36">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {granularityOptions.map((opt) => (
                  <SelectItem key={opt.value} value={String(opt.value)}>
                    {opt.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div className="flex flex-col gap-2">
            <span className="text-sm font-medium">
              {t.statisticPage.label.displayStyle}
            </span>
            <StylePicker
              value={style}
              options={[
                { label: t.statisticPage.displayStyle.radar, value: "radar" },
                { label: t.statisticPage.displayStyle.bar, value: "Bar" },
              ]}
              onChange={setStyle}
            />
          </div>
          <Button
            size="sm"
            variant="outline"
            onClick={() => {
              const end = new Date()
              end.setHours(24, 0, 0, 0)
              const start = new Date(end.getTime() - 7 * MILLISECONDS_PER_DAY)
              setGroups((prev) => [
                ...prev,
                { start: start.getTime(), end: end.getTime(), categoryId: "" },
              ])
            }}
          >
            <Plus data-icon="inline-start" />
            {t.statisticPage.group.add}
          </Button>
        </div>

        <div className="flex flex-col gap-2">
          {groups.map((group, gi) => (
            <div
              key={gi}
              className="flex flex-wrap items-center gap-3 rounded-md border p-3"
            >
              <span className="text-sm text-muted-foreground">
                {t.statisticPage.group.timeRange}
              </span>
              <input
                type="date"
                className="h-8 rounded-md border bg-transparent px-2 text-sm"
                value={startToInput(group.start)}
                onChange={(e) =>
                  setGroups((prev) =>
                    prev.map((g, i) =>
                      i === gi
                        ? { ...g, start: inputToStart(e.target.value) }
                        : g
                    )
                  )
                }
              />
              <span className="text-muted-foreground">~</span>
              <input
                type="date"
                className="h-8 rounded-md border bg-transparent px-2 text-sm"
                value={endToInput(group.end)}
                onChange={(e) =>
                  setGroups((prev) =>
                    prev.map((g, i) =>
                      i === gi ? { ...g, end: inputToEnd(e.target.value) } : g
                    )
                  )
                }
              />
              <span className="text-sm text-muted-foreground">
                {t.statisticPage.group.category}
              </span>
              <Select
                value={group.categoryId}
                onValueChange={(v) =>
                  setGroups((prev) =>
                    prev.map((g, i) => (i === gi ? { ...g, categoryId: v } : g))
                  )
                }
              >
                <SelectTrigger className="h-8 w-40">
                  <SelectValue placeholder={t.statisticPage.categories.all} />
                </SelectTrigger>
                <SelectContent>
                  {categories.map((c) => (
                    <SelectItem key={c.id} value={String(c.id)}>
                      {c.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              <Button
                size="sm"
                variant="ghost"
                className="text-destructive"
                onClick={() =>
                  setGroups((prev) => prev.filter((_, i) => i !== gi))
                }
              >
                <Trash2 className="size-4" />
              </Button>
            </div>
          ))}
        </div>

        {points.length === 0 ? (
          <Empty />
        ) : style === "Bar" ? (
          <BarChart
            data={points.map((p) => ({
              label: String(p.index),
              group: String(p.group),
              value: p.value,
            }))}
            formatValue={(v) =>
              formatDurationRough(v * MILLISECONDS_PER_MINUTE)
            }
          />
        ) : (
          <RadarChart
            data={points.map((p) => ({
              time:
                (p.index /
                  ((timeSpan === "day"
                    ? MILLISECONDS_PER_DAY
                    : 7 * MILLISECONDS_PER_DAY) /
                    granularity)) *
                (timeSpan === "day" ? 24 : 168),
              tag: String(p.group),
              value: p.value,
            }))}
            domain={timeSpan === "day" ? [0, 24] : [0, 168]}
            formatValue={(v) =>
              formatDurationRough(v * MILLISECONDS_PER_MINUTE)
            }
            timeFormatter={
              timeSpan === "day"
                ? formatTimeOfDay
                : (t) => `${Math.floor(t / 24)}d ${formatTimeOfDay(t % 24)}`
            }
            ticks={
              timeSpan === "day"
                ? undefined
                : Array.from({ length: 8 }, (_, i) => ({
                    value: i * 24,
                    label: `${i}d`,
                  }))
            }
          />
        )}
      </CardContent>
    </Card>
  )
}

export function Statistic() {
  const t = useI18n()
  const [type, setType] = useState<StatisticType>("AppDuration")

  return (
    <div className="mx-auto flex max-w-6xl flex-col gap-4 p-4">
      <Card>
        <CardHeader>
          <CardTitle className="text-base">
            {t.statisticPage.types.title}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <Select
            value={type}
            onValueChange={(v) => setType(v as StatisticType)}
          >
            <SelectTrigger className="w-56">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="AppDuration">
                {t.statisticPage.types.appDuration}
              </SelectItem>
              <SelectItem value="AppDays">
                {t.statisticPage.types.appDays}
              </SelectItem>
              <SelectItem value="CategoryDuration">
                {t.statisticPage.types.categoryDuration}
              </SelectItem>
              <SelectItem value="CategoryDays">
                {t.statisticPage.types.categoryDays}
              </SelectItem>
              <SelectItem value="CategoryRhythm">
                {t.statisticPage.types.categoryRhythm}
              </SelectItem>
            </SelectContent>
          </Select>
        </CardContent>
      </Card>

      {type === "AppDuration" && <AppDurationPanel t={t} />}
      {type === "AppDays" && <AppDaysPanel t={t} />}
      {(type === "CategoryDuration" || type === "CategoryDays") && (
        <CategoryPanel
          t={t}
          kind={type === "CategoryDuration" ? "duration" : "days"}
        />
      )}
      {type === "CategoryRhythm" && <CategoryRhythmPanel t={t} />}
    </div>
  )
}
