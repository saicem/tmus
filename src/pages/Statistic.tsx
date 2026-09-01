import { useCallback, useEffect, useMemo, useState } from "react"
import { Plus, Trash2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { Empty } from "@/components/ui/empty"
import { Progress } from "@/components/ui/progress"
import { CategoryTreeSelect } from "@/components/common/CategoryTreeSelect"
import { DateRangePicker } from "@/components/common/DateRangePicker"
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { BarChart } from "@/lib/charts/BarChart"
import { PieChart } from "@/lib/charts/PieChart"
import { RadarChart } from "@/lib/charts/RadarChart"
import {
  getAppTotalDuration,
  getAppUsageDays,
  getCategoryTree,
  getCategoryTotalDuration,
  getCategoryUsageDays,
  getCategoryUsageRhythm,
} from "@/lib/cmd"
import { useI18n } from "@/lib/i18n"
import type { Category, TimeSpan } from "@/lib/models"
import { useAppStore } from "@/lib/state"
import {
  formatDurationRough,
  formatTimeOfDay,
  getTimeRangeByDays,
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

function StatisticTypeSelect({
  value,
  onChange,
}: {
  value: StatisticType
  onChange: (v: StatisticType) => void
}) {
  const t = useI18n()
  return (
    <Select value={value} onValueChange={(v) => onChange(v as StatisticType)}>
      <SelectTrigger className="h-9 w-56">
        <SelectValue placeholder={t.statisticPage.types.title} />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectLabel>{t.statisticPage.types.title}</SelectLabel>
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
        </SelectGroup>
      </SelectContent>
    </Select>
  )
}

interface DurationItem {
  id: number
  name: string
  duration: number
  proportion: number
}

function useCategories() {
  const [categories, setCategories] = useState<Category[]>([])
  useEffect(() => {
    let cancelled = false
    void getCategoryTree().then((result) => {
      if (!cancelled) setCategories(result.children)
    })
    return () => {
      cancelled = true
    }
  }, [])
  return categories
}

function AppDurationPanel({
  t,
  type,
  onTypeChange,
}: {
  t: ReturnType<typeof useI18n>
  type: StatisticType
  onTypeChange: (v: StatisticType) => void
}) {
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
            <StatisticTypeSelect value={type} onChange={onTypeChange} />
          </div>
          <div className="flex flex-wrap items-end gap-4">
            <DateRangePicker
              value={range}
              onChange={setRange}
              placeholder={t.statisticPage.label.timeRange}
            />
            <CategoryTreeSelect
              categories={categories}
              value={categoryId}
              onChange={(v) => setCategoryId(v as string)}
              placeholder={t.statisticPage.categories.title}
              triggerClassName="h-9 w-48"
            />
            <Select
              value={style}
              onValueChange={(v) => setStyle(v as DisplayStyle)}
            >
              <SelectTrigger className="h-9 w-44">
                <SelectValue placeholder={t.statisticPage.label.displayStyle} />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectLabel>
                    {t.statisticPage.label.displayStyle}
                  </SelectLabel>
                  <SelectItem value="Pie">
                    {t.statisticPage.displayStyle.pie}
                  </SelectItem>
                  <SelectItem value="Bar">
                    {t.statisticPage.displayStyle.bar}
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent>
          {items.length === 0 ? (
            <Empty />
          ) : style === "Pie" ? (
            <PieChart
              data={items.map((i) => ({ name: i.name, value: i.duration }))}
              formatValue={formatDurationRough}
              otherLabel={t.statisticPage.other}
            />
          ) : (
            <BarChart
              data={items.map((i) => ({ label: i.name, value: i.duration }))}
              formatValue={formatDurationRough}
            />
          )}
        </CardContent>
      </Card>

      {items.length > 0 && (
        <Card>
          <CardContent>
            <ProgressList
              items={items.map((i) => ({
                id: i.id,
                label: i.name,
                value: i.duration,
                percentage: Math.round(i.proportion * 1000) / 10,
              }))}
              format={formatDurationRough}
            />
          </CardContent>
        </Card>
      )}
    </>
  )
}

function AppDaysPanel({
  t,
  type,
  onTypeChange,
}: {
  t: ReturnType<typeof useI18n>
  type: StatisticType
  onTypeChange: (v: StatisticType) => void
}) {
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
      timezoneOffset: useAppStore.getState().config.timezoneOffset,
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
            <StatisticTypeSelect value={type} onChange={onTypeChange} />
          </div>
          <div className="flex flex-wrap items-end gap-4">
            <DateRangePicker
              value={range}
              onChange={setRange}
              placeholder={t.statisticPage.label.timeRange}
            />
            <CategoryTreeSelect
              categories={categories}
              value={categoryId}
              onChange={(v) => setCategoryId(v as string)}
              placeholder={t.statisticPage.categories.title}
              triggerClassName="h-9 w-48"
            />
            <Select
              value={style}
              onValueChange={(v) => setStyle(v as DisplayStyle)}
            >
              <SelectTrigger className="h-9 w-44">
                <SelectValue placeholder={t.statisticPage.label.displayStyle} />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectLabel>
                    {t.statisticPage.label.displayStyle}
                  </SelectLabel>
                  <SelectItem value="Pie">
                    {t.statisticPage.displayStyle.pie}
                  </SelectItem>
                  <SelectItem value="Bar">
                    {t.statisticPage.displayStyle.bar}
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent>
          {items.length === 0 ? (
            <Empty />
          ) : style === "Pie" ? (
            <PieChart
              data={items.map((i) => ({ name: i.name, value: i.duration }))}
              formatValue={(v) => `${v} ${t.statisticPage.unit.day}`}
              otherLabel={t.statisticPage.other}
            />
          ) : (
            <BarChart
              data={items.map((i) => ({ label: i.name, value: i.duration }))}
              formatValue={(v) => `${v} ${t.statisticPage.unit.day}`}
            />
          )}
        </CardContent>
      </Card>

      {items.length > 0 && (
        <Card>
          <CardContent>
            <ProgressList
              items={items.map((i) => ({
                id: i.id,
                label: i.name,
                value: i.duration,
                percentage: Math.round(i.proportion * 1000) / 10,
              }))}
              format={(v) => `${v.toFixed(0)}${t.statisticPage.unit.day}`}
            />
          </CardContent>
        </Card>
      )}
    </>
  )
}

function CategoryPanel({
  t,
  kind,
  type,
  onTypeChange,
}: {
  t: ReturnType<typeof useI18n>
  kind: "duration" | "days"
  type: StatisticType
  onTypeChange: (v: StatisticType) => void
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
            timezoneOffset: useAppStore.getState().config.timezoneOffset,
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
            <StatisticTypeSelect value={type} onChange={onTypeChange} />
          </div>
          <div className="flex flex-wrap items-end gap-4">
            <DateRangePicker
              value={range}
              onChange={setRange}
              placeholder={t.statisticPage.label.timeRange}
            />
            <CategoryTreeSelect
              multiple
              categories={categories}
              value={categoryIds}
              onChange={(v) => setCategoryIds(v as string[])}
              placeholder={t.statisticPage.categories.title}
              triggerClassName="h-9 w-64"
            />
            <Select
              value={style}
              onValueChange={(v) => setStyle(v as DisplayStyle)}
            >
              <SelectTrigger className="h-9 w-44">
                <SelectValue placeholder={t.statisticPage.label.displayStyle} />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectLabel>
                    {t.statisticPage.label.displayStyle}
                  </SelectLabel>
                  <SelectItem value="Pie">
                    {t.statisticPage.displayStyle.pie}
                  </SelectItem>
                  <SelectItem value="Bar">
                    {t.statisticPage.displayStyle.bar}
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent>
          {items.length === 0 ? (
            <Empty />
          ) : style === "Pie" ? (
            <PieChart
              data={items.map((i) => ({ name: i.name, value: i.duration }))}
              formatValue={formatValue}
              otherLabel={t.statisticPage.other}
            />
          ) : (
            <BarChart
              data={items.map((i) => ({ label: i.name, value: i.duration }))}
              formatValue={formatValue}
            />
          )}
        </CardContent>
      </Card>

      {items.length > 0 && (
        <Card>
          <CardContent>
            <ProgressList
              items={items.map((i) => ({
                id: i.id,
                label: i.name,
                value: i.duration,
                percentage: Math.round(i.proportion * 1000) / 10,
              }))}
              format={formatList}
            />
          </CardContent>
        </Card>
      )}
    </>
  )
}

interface RhythmGroupState {
  start: number
  end: number
  categoryId: string
}

function CategoryRhythmPanel({
  t,
  type,
  onTypeChange,
}: {
  t: ReturnType<typeof useI18n>
  type: StatisticType
  onTypeChange: (v: StatisticType) => void
}) {
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
      timezoneOffset: useAppStore.getState().config.timezoneOffset,
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
    <>
      <Card>
        <CardContent className="flex flex-col gap-4 pt-6">
          <div className="flex flex-wrap items-end gap-4">
            <StatisticTypeSelect value={type} onChange={onTypeChange} />
          </div>
          <div className="flex flex-wrap items-end gap-4">
            <Select
              value={timeSpan}
              onValueChange={(v) => setTimeSpan(v as TimeSpan)}
            >
              <SelectTrigger className="h-9 w-36">
                <SelectValue placeholder={t.statisticPage.label.timeSpan} />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectLabel>{t.statisticPage.label.timeSpan}</SelectLabel>
                  <SelectItem value="day">
                    {t.statisticPage.timeSpan.day}
                  </SelectItem>
                  <SelectItem value="week">
                    {t.statisticPage.timeSpan.week}
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
            <Select
              value={String(granularity)}
              onValueChange={(v) => setGranularity(Number(v))}
            >
              <SelectTrigger className="h-9 w-44">
                <SelectValue placeholder={t.statisticPage.label.granularity} />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectLabel>{t.statisticPage.label.granularity}</SelectLabel>
                  {granularityOptions.map((opt) => (
                    <SelectItem key={opt.value} value={String(opt.value)}>
                      {opt.label}
                    </SelectItem>
                  ))}
                </SelectGroup>
              </SelectContent>
            </Select>
            <Select
              value={style}
              onValueChange={(v) => setStyle(v as DisplayStyle)}
            >
              <SelectTrigger className="h-9 w-44">
                <SelectValue placeholder={t.statisticPage.label.displayStyle} />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectLabel>
                    {t.statisticPage.label.displayStyle}
                  </SelectLabel>
                  <SelectItem value="radar">
                    {t.statisticPage.displayStyle.radar}
                  </SelectItem>
                  <SelectItem value="Bar">
                    {t.statisticPage.displayStyle.bar}
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
            <Button
              size="sm"
              variant="outline"
              onClick={() => {
                const end = new Date()
                end.setHours(24, 0, 0, 0)
                const start = new Date(end.getTime() - 7 * MILLISECONDS_PER_DAY)
                setGroups((prev) => [
                  ...prev,
                  {
                    start: start.getTime(),
                    end: end.getTime(),
                    categoryId: "",
                  },
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
                <DateRangePicker
                  value={[group.start, group.end]}
                  showShortcuts={false}
                  placeholder={t.statisticPage.group.timeRange}
                  onChange={([start, end]) =>
                    setGroups((prev) =>
                      prev.map((g, i) => (i === gi ? { ...g, start, end } : g))
                    )
                  }
                />
                <CategoryTreeSelect
                  categories={categories}
                  value={group.categoryId}
                  onChange={(v) =>
                    setGroups((prev) =>
                      prev.map((g, i) =>
                        i === gi ? { ...g, categoryId: v as string } : g
                      )
                    )
                  }
                  placeholder={t.statisticPage.group.category}
                  triggerClassName="h-8 w-44"
                />
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
        </CardContent>
      </Card>

      <Card>
        <CardContent>
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
    </>
  )
}

export function Statistic() {
  const t = useI18n()
  const [type, setType] = useState<StatisticType>("AppDuration")

  return (
    <div className="mx-auto flex max-w-6xl flex-col gap-4 p-4">
      {type === "AppDuration" && (
        <AppDurationPanel t={t} type={type} onTypeChange={setType} />
      )}
      {type === "AppDays" && (
        <AppDaysPanel t={t} type={type} onTypeChange={setType} />
      )}
      {(type === "CategoryDuration" || type === "CategoryDays") && (
        <CategoryPanel
          t={t}
          kind={type === "CategoryDuration" ? "duration" : "days"}
          type={type}
          onTypeChange={setType}
        />
      )}
      {type === "CategoryRhythm" && (
        <CategoryRhythmPanel t={t} type={type} onTypeChange={setType} />
      )}
    </div>
  )
}
