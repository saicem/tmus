import { useCallback, useEffect, useState } from "react"
import { AppWindow, CalendarRange, Clock } from "lucide-react"
import { startOfDay, startOfYear, subDays } from "date-fns"

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Empty } from "@/components/ui/empty"
import { Progress } from "@/components/ui/progress"
import { BarChart } from "@/lib/charts/BarChart"
import { RadarChart } from "@/lib/charts/RadarChart"
import { AppIcon } from "@/components/common/AppIcon"
import { HeatCalendar } from "@/components/statistic/HeatCalendar"

import {
  getAppDetailMap,
  getDurationById,
  queryDurationStatistic,
} from "@/lib/cmd"
import { useI18n } from "@/lib/i18n"
import type { AppDuration } from "@/lib/models"
import { useAppStore } from "@/lib/state"
import {
  dayOfWeekOffset,
  formatDuration,
  formatDurationRough,
  formatTimeOfDay,
  MILLISECONDS_PER_DAY,
  MILLISECONDS_PER_MINUTE,
} from "@/lib/time-util"
import { groupBy, range, toMap } from "@/lib/util"

interface WeeklyPoint {
  week: string
  dayOfWeek: string
  minutes: number
}

interface RhythmPoint {
  time: number
  tag: string
  duration: number
}

export function Overview() {
  const t = useI18n()
  const [yearData, setYearData] = useState<number[]>([])
  const [todayAppCount, setTodayAppCount] = useState("0")
  const [todayValue, setTodayValue] = useState(0)
  const [yesterdayValue, setYesterdayValue] = useState(0)
  const [weekValue, setWeekValue] = useState(0)
  const [lastWeekValue, setLastWeekValue] = useState(0)
  const [weekAppCount, setWeekAppCount] = useState(0)
  const [progressData, setProgressData] = useState<AppDuration[]>([])
  const [weeklyData, setWeeklyData] = useState<WeeklyPoint[]>([])
  const [rhythmData, setRhythmData] = useState<RhythmPoint[]>([])

  const loadYearData = useCallback(async () => {
    const now = new Date()
    const yearStart = startOfYear(now)
    const yearEnd = new Date(yearStart.getFullYear(), 11, 31)
    const data = await queryDurationStatistic({
      startTimestamp: yearStart.getTime(),
      endTimestamp: now.getTime(),
      mergeApps: true,
      appIds: null,
      granularity: MILLISECONDS_PER_DAY,
      cycle: null,
      timezoneOffset: useAppStore.getState().config.timezoneOffset,
    })
    const durationByDateMap = toMap(data, (x) => [x.intervalStart, x.duration])
    const result: number[] = []
    for (
      let i = yearStart.getTime();
      i <= yearEnd.getTime();
      i += MILLISECONDS_PER_DAY
    ) {
      result.push(durationByDateMap.get(i) ?? 0)
    }
    setYearData(result)
  }, [])

  const loadDayData = useCallback(async () => {
    const end = new Date()
    const dayStart = startOfDay(end)
    const yesterdayStart = subDays(dayStart, 1)
    const weekStart = subDays(dayStart, dayOfWeekOffset(end))
    const lastWeekStart = subDays(weekStart, 7)
    const [records, yesterdayRecords, weekRecords, lastWeekRecords] =
      await Promise.all([
        getDurationById({
          startTimestamp: dayStart.getTime(),
          endTimestamp: end.getTime(),
        }),
        getDurationById({
          startTimestamp: yesterdayStart.getTime(),
          endTimestamp: dayStart.getTime(),
        }),
        getDurationById({
          startTimestamp: weekStart.getTime(),
          endTimestamp: end.getTime(),
        }),
        getDurationById({
          startTimestamp: lastWeekStart.getTime(),
          endTimestamp: weekStart.getTime(),
        }),
      ])
    const sumDuration = (xs: { duration: number }[]) =>
      xs.reduce((acc, cur) => acc + cur.duration, 0)
    setTodayAppCount(records.length.toString())
    setTodayValue(sumDuration(records))
    setYesterdayValue(sumDuration(yesterdayRecords))
    setWeekValue(sumDuration(weekRecords))
    setLastWeekValue(sumDuration(lastWeekRecords))
    setWeekAppCount(weekRecords.length)
    const appDetailMap = await getAppDetailMap()
    const progressDataResult: AppDuration[] = []
    for (const [id, durationList] of groupBy(records, (x) => [
      x.appId,
      x.duration,
    ])) {
      progressDataResult.push({
        app: appDetailMap[id],
        duration: durationList.reduce((acc, cur) => acc + cur),
      })
    }
    progressDataResult.sort((a, b) => b.duration - a.duration)
    setProgressData(progressDataResult)
  }, [])

  const loadWeekly = useCallback(async () => {
    const now = new Date()
    const lastWeekStart = subDays(startOfDay(now), dayOfWeekOffset(now) + 7)
    const resultMap = toMap(
      await queryDurationStatistic({
        startTimestamp: lastWeekStart.getTime(),
        endTimestamp: now.getTime(),
        mergeApps: true,
        appIds: null,
        granularity: MILLISECONDS_PER_DAY,
        cycle: null,
        timezoneOffset: useAppStore.getState().config.timezoneOffset,
      }),
      (x) => [x.intervalStart, x.duration]
    )
    const firstDayOfWeek = useAppStore.getState().config.firstDayOfWeek
    const ret: WeeklyPoint[] = Array(14)
      .fill(0)
      .map((_, idx) => {
        const date = new Date(
          lastWeekStart.getTime() + idx * MILLISECONDS_PER_DAY
        )
        const duration = resultMap.get(date.getTime()) ?? 0
        return {
          week:
            Math.trunc(idx / 7) === 0 ? t.common.lastWeek : t.common.thisWeek,
          dayOfWeek: t.weeklyChart.dayOfWeekNames[(idx + firstDayOfWeek) % 7],
          minutes: Math.round(duration / MILLISECONDS_PER_MINUTE),
        }
      })
    setWeeklyData(ret)
  }, [t])

  const loadRhythm = useCallback(async () => {
    const now = new Date()
    const pastWeekEnd = startOfDay(now)
    const pastWeekStart = subDays(pastWeekEnd, 7)
    const granularity = MILLISECONDS_PER_MINUTE * 10
    const cycle = MILLISECONDS_PER_DAY / granularity
    const cycleInHour = cycle / 24

    const pastWeekData = toMap(
      await queryDurationStatistic({
        startTimestamp: pastWeekStart.getTime(),
        endTimestamp: pastWeekEnd.getTime(),
        mergeApps: true,
        appIds: null,
        granularity,
        cycle,
        timezoneOffset: useAppStore.getState().config.timezoneOffset,
      }),
      (x) => [
        x.intervalStart / granularity,
        +(x.duration / MILLISECONDS_PER_MINUTE / 7).toFixed(1),
      ]
    )
    const todayData = toMap(
      await queryDurationStatistic({
        startTimestamp: pastWeekEnd.getTime(),
        endTimestamp: now.getTime(),
        mergeApps: true,
        appIds: null,
        granularity,
        cycle,
        timezoneOffset: useAppStore.getState().config.timezoneOffset,
      }),
      (x) => [
        x.intervalStart / granularity,
        +(x.duration / MILLISECONDS_PER_MINUTE).toFixed(1),
      ]
    )
    const newData: RhythmPoint[] = []
    for (const x of range(0, cycle)) {
      newData.push({
        time: x / cycleInHour,
        tag: t.homePage.pastWeekAverage,
        duration: pastWeekData.get(x) ?? 0,
      })
      newData.push({
        time: x / cycleInHour,
        tag: t.common.today,
        duration: todayData.get(x) ?? 0,
      })
    }
    setRhythmData(newData)
  }, [t])

  const load = useCallback(async () => {
    await Promise.all([
      loadYearData(),
      loadDayData(),
      loadWeekly(),
      loadRhythm(),
    ])
  }, [loadYearData, loadDayData, loadWeekly, loadRhythm])

  useEffect(() => {
    void load()
    const onRefresh = () => void load()
    window.addEventListener("tmus:refresh", onRefresh)
    return () => window.removeEventListener("tmus:refresh", onRefresh)
  }, [load])

  const maxDuration = progressData[0]?.duration ?? 1

  return (
    <div className="mx-auto flex max-w-5xl flex-col gap-4 p-4">
      <div className="grid grid-cols-2 gap-3 md:grid-cols-3">
        <Card className="p-3">
          <div className="flex items-start justify-between">
            <CardHeader className="p-0">
              <CardTitle className="whitespace-nowrap text-xs font-medium text-muted-foreground">
                {t.homePage.apps}
              </CardTitle>
            </CardHeader>
            <div className="flex size-7 items-center justify-center rounded-md bg-primary/10 text-primary">
              <AppWindow className="size-4" />
            </div>
          </div>
          <CardContent className="p-0 pt-2">
            <div className="text-xl font-bold">
              {todayAppCount}
              {t.homePage.appsUnit}
            </div>
            <p className="mt-1 text-xs text-muted-foreground">
              {t.homePage.thisWeekPrefix} {weekAppCount} {t.homePage.appsUnit}
            </p>
          </CardContent>
        </Card>
        <Card className="p-3">
          <div className="flex items-start justify-between">
            <CardHeader className="p-0">
              <CardTitle className="whitespace-nowrap text-xs font-medium text-muted-foreground">
                {t.homePage.totalUse}
              </CardTitle>
            </CardHeader>
            <div className="flex size-7 items-center justify-center rounded-md bg-primary/10 text-primary">
              <Clock className="size-4" />
            </div>
          </div>
          <CardContent className="p-0 pt-2">
            <div className="text-xl font-bold">
              {formatDuration(todayValue)}
            </div>
            <p className="mt-1 text-xs text-muted-foreground">
              {t.homePage.vsYesterday}{" "}
              {formatDurationRough(todayValue - yesterdayValue)}
            </p>
          </CardContent>
        </Card>

        <Card className="p-3">
          <div className="flex items-start justify-between">
            <CardHeader className="p-0">
              <CardTitle className="whitespace-nowrap text-xs font-medium text-muted-foreground">
                {t.homePage.weekTotal}
              </CardTitle>
            </CardHeader>
            <div className="flex size-7 items-center justify-center rounded-md bg-primary/10 text-primary">
              <CalendarRange className="size-4" />
            </div>
          </div>
          <CardContent className="p-0 pt-2">
            <div className="text-xl font-bold">{formatDuration(weekValue)}</div>
            <p className="mt-1 text-xs text-muted-foreground">
              {t.homePage.vsLastWeek}{" "}
              {formatDurationRough(weekValue - lastWeekValue)}
            </p>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardContent>
          <HeatCalendar data={yearData} />
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>{t.weeklyChart.title}</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="mx-auto max-w-2xl">
            <BarChart
              data={weeklyData.map((d) => ({
                label: d.dayOfWeek,
                group: d.week,
                value: d.minutes,
              }))}
              formatValue={(v) =>
                formatDurationRough(v * MILLISECONDS_PER_MINUTE)
              }
            />
          </div>
        </CardContent>
      </Card>

      <div className="flex flex-col gap-4 lg:flex-row">
        <Card className="flex-1">
          <CardHeader>
            <CardTitle>{t.homePage.todayUsage}</CardTitle>
          </CardHeader>
          <CardContent>
            {progressData.length === 0 ? (
              <Empty />
            ) : (
              <div className="flex flex-col gap-4">
                {progressData.map(({ app, duration }) => (
                  <div key={app.id} className="flex flex-col gap-1.5">
                    <div className="flex items-center justify-between gap-2 text-sm">
                      <span className="flex min-w-0 items-center gap-2">
                        <AppIcon icon={app.icon} exist={app.exist} size={16} />
                        <span className="truncate">{app.name}</span>
                      </span>
                      <span className="shrink-0 text-muted-foreground">
                        {formatDuration(duration)}
                      </span>
                    </div>
                    <Progress value={(duration / maxDuration) * 100} />
                  </div>
                ))}
              </div>
            )}
          </CardContent>
        </Card>

        <Card className="lg:w-1/2">
          <CardHeader>
            <CardTitle className="text-center">
              {t.homePage.dailyRhythm}
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="mx-auto w-full max-w-[400px]">
              <RadarChart
                data={rhythmData.map((d) => ({
                  time: d.time,
                  tag: d.tag,
                  value: d.duration,
                }))}
                formatValue={(v) =>
                  formatDurationRough(v * MILLISECONDS_PER_MINUTE)
                }
                timeFormatter={formatTimeOfDay}
              />
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
