import { useEffect, useState } from "react"
import { useNavigate, useParams } from "react-router-dom"
import { ArrowLeft, FileWarning } from "lucide-react"
import { AppIcon } from "@/components/common/AppIcon"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip"
import { AreaChart } from "@/lib/charts/AreaChart"
import { getAppDetail, getAppDurationArea, showInFolder } from "@/lib/cmd"
import { useI18n } from "@/lib/i18n"
import type { AppDurationAreaModel, FileDetail } from "@/lib/models"
import { useAppStore } from "@/lib/state"
import { formatDurationRough, timeZoneOffsetMillis } from "@/lib/time-util"

export function Detail() {
  const t = useI18n()
  const navigate = useNavigate()
  const { id } = useParams<{ id: string }>()
  const appId = Number(id)
  const [detail, setDetail] = useState<FileDetail | null>(null)
  const [area, setArea] = useState<AppDurationAreaModel | null>(null)
  const [activeTab, setActiveTab] = useState("durationDateArea")

  useEffect(() => {
    let cancelled = false
    void (async () => {
      const [d, a] = await Promise.all([
        getAppDetail(appId),
        getAppDurationArea({
          appId,
          startTimestamp: 0,
          endTimestamp: Date.now(),
          timezoneOffset:
            useAppStore.getState().config.timezoneOffset ??
            timeZoneOffsetMillis(),
        }),
      ])
      if (cancelled) return
      setDetail(d)
      setArea(a)
    })()
    return () => {
      cancelled = true
    }
  }, [appId])

  const fileDeleted = !!detail && !detail.exist

  const goBack = () => {
    if ((window.history.state as { idx?: number } | null)?.idx) {
      navigate(-1)
    } else {
      navigate("/application")
    }
  }

  return (
    <div className="flex flex-col gap-4 p-4">
      <Button
        variant="ghost"
        size="sm"
        className="w-fit gap-2 pl-2"
        onClick={goBack}
      >
        <ArrowLeft className="size-4" />
        {t.common.back}
      </Button>

      <Card>
        <CardHeader>
          <div className="flex items-center gap-4">
            <AppIcon icon={detail?.icon} exist={detail?.exist} size={32} />
            <CardTitle>{detail?.name}</CardTitle>
          </div>
        </CardHeader>
        <CardContent className="flex flex-col gap-3 text-sm">
          <div className="flex gap-2">
            <span className="w-32 shrink-0 text-muted-foreground">
              {t.detailPage.name}
            </span>
            <span>{detail?.name}</span>
          </div>
          <div className="flex gap-2">
            <span className="w-32 shrink-0 text-muted-foreground">
              {t.detailPage.productName}
            </span>
            <span>{detail?.version?.productName ?? "-"}</span>
          </div>
          <div className="flex gap-2">
            <span className="w-32 shrink-0 text-muted-foreground">
              {t.detailPage.fileDescription}
            </span>
            <span>{detail?.version?.fileDescription ?? "-"}</span>
          </div>
          <div className="flex gap-2">
            <span className="w-32 shrink-0 text-muted-foreground">
              {t.detailPage.companyName}
            </span>
            <span>{detail?.version?.companyName ?? "-"}</span>
          </div>
          <div className="flex items-center gap-2">
            <span className="w-32 shrink-0 text-muted-foreground">
              {t.detailPage.filePath}
            </span>
            {fileDeleted ? (
              <Tooltip>
                <TooltipTrigger asChild>
                  <Button variant="ghost" className="gap-2 text-destructive">
                    <FileWarning className="size-4" />
                    <span className="truncate">{detail?.path}</span>
                  </Button>
                </TooltipTrigger>
                <TooltipContent>
                  {t.detailPage.fileHasBeenDeleted}
                </TooltipContent>
              </Tooltip>
            ) : (
              <Button
                variant="link"
                className="h-auto p-0 text-foreground"
                onClick={() => detail?.path && showInFolder(detail.path)}
              >
                <span className="truncate">{detail?.path}</span>
              </Button>
            )}
          </div>
        </CardContent>
      </Card>

      <Card>
        <Tabs value={activeTab} onValueChange={setActiveTab}>
          <CardHeader>
            <TabsList>
              <TabsTrigger value="durationDateArea">
                {t.detailPage.durationDateAreaTab}
              </TabsTrigger>
              <TabsTrigger value="durationDayArea">
                {t.detailPage.durationDayAreaTab}
              </TabsTrigger>
            </TabsList>
          </CardHeader>
          <CardContent>
            <TabsContent value="durationDateArea">
              {area && (
                <AreaChart
                  data={area.dateArea.map((d) => ({
                    label: d.index,
                    value: d.value,
                  }))}
                  formatValue={formatDurationRough}
                  yName={t.detailPage.durationAreaChart.dateChart.yName}
                />
              )}
            </TabsContent>
            <TabsContent value="durationDayArea">
              {area && (
                <AreaChart
                  data={area.dayArea.map((d) => ({
                    label: d.index,
                    value: d.value,
                  }))}
                  formatValue={(v) =>
                    `${Math.round(v)} ${
                      t.detailPage.durationAreaChart.dayChart.dayUnit
                    }`
                  }
                  yName={t.detailPage.durationAreaChart.dayChart.yName}
                />
              )}
            </TabsContent>
          </CardContent>
        </Tabs>
      </Card>
    </div>
  )
}
