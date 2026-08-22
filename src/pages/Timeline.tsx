import { useCallback, useEffect, useRef, useState } from "react"
import { format, isBefore, startOfDay, subDays, subWeeks } from "date-fns"
import { AppIcon } from "@/components/common/AppIcon"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { getAppDetailMap, getTmusMeta, queryDurationStatistic } from "@/lib/cmd"
import { useI18n } from "@/lib/i18n"
import type { AppDuration, FileDetail } from "@/lib/models"
import { useAppStore } from "@/lib/state"
import { formatDuration, MILLISECONDS_PER_DAY } from "@/lib/time-util"
import { groupBy } from "@/lib/util"

interface DateGroup<T> {
  moment: Date
  data: T[]
}

/**
 * Chronological timeline of daily app usage, loaded one week at a time with
 * infinite scroll (newest first).
 */
export function Timeline() {
  const t = useI18n()
  const dateFormat = useAppStore((s) => s.config.dateFormat)
  const [loading, setLoading] = useState(true)
  const [noMore, setNoMore] = useState(false)
  const [groups, setGroups] = useState<DateGroup<AppDuration>[]>([])
  const nextDateRef = useRef(new Date())
  const metaStartRef = useRef(new Date())
  const appDetailMapRef = useRef<Record<number, FileDetail>>({})
  const loadingRef = useRef(true)
  const noMoreRef = useRef(false)
  const sentinelRef = useRef<HTMLDivElement>(null)

  const loadMore = useCallback(async () => {
    if (loadingRef.current || noMoreRef.current) return
    loadingRef.current = true
    setLoading(true)
    const endDate = nextDateRef.current
    const startDate = subWeeks(startOfDay(endDate), 1)
    const result = await queryDurationStatistic({
      startTimestamp: startDate.getTime(),
      endTimestamp: endDate.getTime(),
      mergeApps: false,
      appIds: null,
      granularity: MILLISECONDS_PER_DAY,
      cycle: null,
    })
    const dateMap = groupBy(result, (x) => [
      x.intervalStart,
      { app: appDetailMapRef.current[x.appId as number], duration: x.duration },
    ])
    const ripe: DateGroup<AppDuration>[] = Array.from(dateMap).map(
      ([k, v]) => ({
        moment: new Date(k),
        data: v,
      })
    )
    ripe.sort((a, b) => b.moment.valueOf() - a.moment.valueOf())
    ripe.forEach((dg) => dg.data.sort((a, b) => b.duration - a.duration))
    setGroups((prev) => [...prev, ...ripe])
    nextDateRef.current = subDays(startDate, 1)
    if (isBefore(nextDateRef.current, metaStartRef.current)) {
      noMoreRef.current = true
      setNoMore(true)
    }
    loadingRef.current = false
    setLoading(false)
  }, [])

  useEffect(() => {
    let cancelled = false
    void (async () => {
      const meta = await getTmusMeta()
      if (cancelled) return
      metaStartRef.current = new Date(meta.initialTimestamp)
      appDetailMapRef.current = await getAppDetailMap()
      if (cancelled) return
      loadingRef.current = false
      setLoading(false)
      void loadMore()
    })()
    return () => {
      cancelled = true
    }
  }, [loadMore])

  useEffect(() => {
    const sentinel = sentinelRef.current
    if (!sentinel) return
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) void loadMore()
      },
      { rootMargin: "800px" }
    )
    observer.observe(sentinel)
    return () => observer.disconnect()
  }, [loadMore])

  return (
    <div className="flex flex-col gap-4 p-4">
      {groups.map(({ moment, data }, i) => (
        <Card key={i}>
          <CardHeader>
            <div className="flex items-center justify-between gap-2">
              <CardTitle className="text-base">
                {format(moment, dateFormat)}
              </CardTitle>
              <span className="shrink-0 text-sm font-medium text-muted-foreground">
                {formatDuration(
                  data.reduce((acc, item) => acc + item.duration, 0)
                )}
              </span>
            </div>
          </CardHeader>
          <CardContent className="flex flex-col gap-2">
            {data.map(({ app, duration }) => (
              <div
                key={app.id}
                className="flex items-center justify-between gap-2 text-sm"
              >
                <span className="flex min-w-0 items-center gap-2">
                  <AppIcon icon={app.icon} exist={app.exist} size={16} />
                  <span className="truncate">{app.name}</span>
                </span>
                <span className="shrink-0 text-muted-foreground">
                  {formatDuration(duration)}
                </span>
              </div>
            ))}
          </CardContent>
        </Card>
      ))}
      {noMore ? (
        <p className="py-4 text-center text-sm text-muted-foreground">
          {t.common.noMore}
        </p>
      ) : (
        <div ref={sentinelRef} />
      )}
      {loading && groups.length === 0 && (
        <p className="text-center text-sm text-muted-foreground">
          {t.common.loading}
        </p>
      )}
    </div>
  )
}
