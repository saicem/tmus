import { useEffect, useState } from "react"
import { open } from "@tauri-apps/plugin-dialog"
import { disable, enable } from "@tauri-apps/plugin-autostart"
import { Folder, Plus } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"

import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Separator } from "@/components/ui/separator"
import { Switch } from "@/components/ui/switch"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { fetchUpdate, getAppRule, getTmusMeta, setAppRule } from "@/lib/cmd"
import { toast } from "sonner"
import { useI18n } from "@/lib/i18n"
import type { RuleConfig } from "@/lib/models"
import { useAppStore } from "@/lib/state"
import { timeZoneOffsetMillis } from "@/lib/time-util"

/** Format an offset (UTC - local, ms) as "UTC+08:00". */
function formatTimezone(offsetMs: number): string {
  const minutes = Math.round(-offsetMs / 60000)
  const sign = minutes < 0 ? "-" : "+"
  const abs = Math.abs(minutes)
  const hh = String(Math.floor(abs / 60)).padStart(2, "0")
  const mm = String(abs % 60).padStart(2, "0")
  return `UTC${sign}${hh}:${mm}`
}

/** Current UTC - local offset of an IANA timezone at the given instant (ms). */
function zoneOffsetAt(zone: string, at: number): number {
  const formatter = new Intl.DateTimeFormat("en-US", {
    timeZone: zone,
    hourCycle: "h23",
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  })
  const fields: Record<string, string> = {}
  for (const part of formatter.formatToParts(new Date(at))) {
    fields[part.type] = part.value
  }
  const asUtc = Date.UTC(
    Number(fields.year),
    Number(fields.month) - 1,
    Number(fields.day),
    Number(fields.hour),
    Number(fields.minute),
    Number(fields.second)
  )
  // Real offsets are minute-aligned; round away Intl formatting noise
  // (fractional seconds) so the stored value is an exact offset.
  return Math.round((asUtc - at) / 60000) * 60000
}

/**
 * Every offset currently in use by any IANA timezone (deduped). Labels show
 * only the offset: the representative zone is engine-dependent and confusing
 * (e.g. "UTC+08:00" could otherwise show Antarctica/Casey or another arbitrary
 * zone). Falls back to 15-minute steps when the tz database is unavailable.
 */
const TIMEZONE_OFFSETS: { offset: number; label: string }[] = (() => {
  const now = Date.now()
  const zones =
    typeof Intl.supportedValuesOf === "function"
      ? Intl.supportedValuesOf("timeZone")
      : []
  const offsets = new Set<number>()
  for (const zone of zones) {
    const offset = zoneOffsetAt(zone, now)
    offsets.add(offset)
  }
  if (offsets.size > 0) {
    return [...offsets]
      .map((offset) => ({ offset, label: formatTimezone(offset) }))
      .sort((a, b) => a.offset - b.offset)
  }
  return Array.from({ length: 105 }, (_, i) => {
    const minutes = -720 + i * 15
    const offset = -minutes * 60000
    return { offset, label: formatTimezone(offset) }
  })
})()

function SettingRow({
  label,
  tip,
  children,
}: {
  label: string
  tip?: string
  children?: React.ReactNode
}) {
  return (
    <div className="flex items-center justify-between gap-4 py-3">
      <div>
        <p className="text-sm font-medium">{label}</p>
        {tip && <p className="mt-1 text-xs text-muted-foreground">{tip}</p>}
      </div>
      {children}
    </div>
  )
}

function FileSelector({
  value,
  onChange,
}: {
  value: string
  onChange: (v: string) => void
}) {
  const selectPath = async () => {
    const result = await open({ multiple: false, directory: true })
    if (result) onChange(String(result))
  }
  return (
    <div className="flex gap-0">
      <Input
        value={value}
        onChange={(e) => onChange(e.target.value)}
        className="rounded-r-none"
      />
      <Button
        variant="outline"
        size="icon"
        className="h-9 w-9 rounded-l-none border-l-0"
        onClick={() => void selectPath()}
      >
        <Folder className="size-4" />
      </Button>
    </div>
  )
}

function RuleDialog({
  open,
  onOpenChange,
}: {
  open: boolean
  onOpenChange: (v: boolean) => void
}) {
  const t = useI18n()
  const [tab, setTab] = useState("exclude")
  const [rule, setRule] = useState<RuleConfig>({
    exclude: [],
    include: [],
    merge: [],
  })
  const [modified, setModified] = useState(false)

  useEffect(() => {
    if (!open) return
    let cancelled = false
    void (async () => {
      const result = await getAppRule()
      result.exclude.sort((a, b) => a.path.localeCompare(b.path))
      result.include.sort((a, b) => a.path.localeCompare(b.path))
      result.merge.sort((a, b) => a.path.localeCompare(b.path))
      if (!cancelled) setRule(result)
    })()
    return () => {
      cancelled = true
    }
  }, [open])

  const addItem = () => {
    setModified(true)
    setRule((prev) => {
      if (tab === "exclude")
        return { ...prev, exclude: [...prev.exclude, { path: "" }] }
      if (tab === "include")
        return { ...prev, include: [...prev.include, { path: "" }] }
      return { ...prev, merge: [...prev.merge, { path: "", toPath: "" }] }
    })
  }

  const save = () => {
    void setAppRule(rule)
    onOpenChange(false)
  }

  return (
    <Dialog
      open={open}
      onOpenChange={(v) => {
        if (!v && modified) {
          if (window.confirm(t.ruleDialog.modifiedTip)) onOpenChange(false)
          return
        }
        onOpenChange(v)
      }}
    >
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle>{t.configPage.appRule}</DialogTitle>
        </DialogHeader>
        <Tabs value={tab} onValueChange={setTab}>
          <TabsList>
            <TabsTrigger value="exclude">{t.ruleDialog.excludeApp}</TabsTrigger>
            <TabsTrigger value="include">{t.ruleDialog.includeApp}</TabsTrigger>
            <TabsTrigger value="merge">{t.ruleDialog.mergeApp}</TabsTrigger>
          </TabsList>
          <TabsContent
            value="exclude"
            className="max-h-72 overflow-auto rounded-md border"
          >
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t.ruleDialog.path}</TableHead>
                  <TableHead className="w-24">
                    {t.ruleDialog.operation}
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {rule.exclude.map((item, i) => (
                  <TableRow key={i}>
                    <TableCell>
                      <FileSelector
                        value={item.path}
                        onChange={(v) => {
                          setModified(true)
                          setRule((prev) => ({
                            ...prev,
                            exclude: prev.exclude.map((x, j) =>
                              j === i ? { ...x, path: v } : x
                            ),
                          }))
                        }}
                      />
                    </TableCell>
                    <TableCell>
                      <Button
                        variant="ghost"
                        size="sm"
                        className="text-destructive"
                        onClick={() => {
                          setModified(true)
                          setRule((prev) => ({
                            ...prev,
                            exclude: prev.exclude.filter((_, j) => j !== i),
                          }))
                        }}
                      >
                        {t.ruleDialog.remove}
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TabsContent>
          <TabsContent
            value="include"
            className="max-h-72 overflow-auto rounded-md border"
          >
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t.ruleDialog.path}</TableHead>
                  <TableHead className="w-24">
                    {t.ruleDialog.operation}
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {rule.include.map((item, i) => (
                  <TableRow key={i}>
                    <TableCell>
                      <FileSelector
                        value={item.path}
                        onChange={(v) => {
                          setModified(true)
                          setRule((prev) => ({
                            ...prev,
                            include: prev.include.map((x, j) =>
                              j === i ? { ...x, path: v } : x
                            ),
                          }))
                        }}
                      />
                    </TableCell>
                    <TableCell>
                      <Button
                        variant="ghost"
                        size="sm"
                        className="text-destructive"
                        onClick={() => {
                          setModified(true)
                          setRule((prev) => ({
                            ...prev,
                            include: prev.include.filter((_, j) => j !== i),
                          }))
                        }}
                      >
                        {t.ruleDialog.remove}
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TabsContent>
          <TabsContent
            value="merge"
            className="max-h-72 overflow-auto rounded-md border"
          >
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t.ruleDialog.path}</TableHead>
                  <TableHead>{t.ruleDialog.mergedPath}</TableHead>
                  <TableHead className="w-24">
                    {t.ruleDialog.operation}
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {rule.merge.map((item, i) => (
                  <TableRow key={i}>
                    <TableCell>
                      <FileSelector
                        value={item.path}
                        onChange={(v) => {
                          setModified(true)
                          setRule((prev) => ({
                            ...prev,
                            merge: prev.merge.map((x, j) =>
                              j === i ? { ...x, path: v } : x
                            ),
                          }))
                        }}
                      />
                    </TableCell>
                    <TableCell>
                      <FileSelector
                        value={item.toPath}
                        onChange={(v) => {
                          setModified(true)
                          setRule((prev) => ({
                            ...prev,
                            merge: prev.merge.map((x, j) =>
                              j === i ? { ...x, toPath: v } : x
                            ),
                          }))
                        }}
                      />
                    </TableCell>
                    <TableCell>
                      <Button
                        variant="ghost"
                        size="sm"
                        className="text-destructive"
                        onClick={() => {
                          setModified(true)
                          setRule((prev) => ({
                            ...prev,
                            merge: prev.merge.filter((_, j) => j !== i),
                          }))
                        }}
                      >
                        {t.ruleDialog.remove}
                      </Button>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TabsContent>
        </Tabs>
        <DialogFooter>
          <Button variant="outline" onClick={addItem}>
            <Plus data-icon="inline-start" />
            {t.ruleDialog.add}
          </Button>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            {t.common.cancel}
          </Button>
          <Button onClick={save}>{t.common.ok}</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}

export function Setting() {
  const t = useI18n()
  const config = useAppStore((s) => s.config)
  const setConfig = useAppStore((s) => s.setConfig)
  const autoStart = useAppStore((s) => s.autoStart)
  const setAutoStart = useAppStore((s) => s.setAutoStart)
  const showUpdateDialog = useAppStore((s) => s.showUpdateDialog)
  const [ruleOpen, setRuleOpen] = useState(false)
  const [checking, setChecking] = useState(false)
  const [version, setVersion] = useState("")

  useEffect(() => {
    let cancelled = false
    void getTmusMeta().then((meta) => {
      if (!cancelled) setVersion(meta.tmusVersion)
    })
    return () => {
      cancelled = true
    }
  }, [])

  const checkUpdate = async () => {
    if (checking) return
    setChecking(true)
    try {
      const meta = await fetchUpdate()
      if (meta) {
        showUpdateDialog(meta)
      } else {
        toast.success(t.configPage.currentVersionIsAlreadyTheLatestVersion)
      }
    } finally {
      setChecking(false)
    }
  }

  return (
    <div className="mx-auto flex max-w-5xl flex-wrap gap-4 p-4">
      <RuleDialog open={ruleOpen} onOpenChange={setRuleOpen} />
      <Card className="min-w-80 flex-1">
        <CardHeader>
          <CardTitle className="text-base">{t.configPage.general}</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col">
          <SettingRow label={t.configPage.language}>
            <Select
              value={config.lang}
              onValueChange={(v) =>
                setConfig({ lang: v as typeof config.lang })
              }
            >
              <SelectTrigger className="w-32">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="en">English</SelectItem>
                <SelectItem value="zh">简体中文</SelectItem>
                <SelectItem value="system">
                  {t.configPage.langSystem}
                </SelectItem>
              </SelectContent>
            </Select>
          </SettingRow>
          <Separator />
          <SettingRow label={t.configPage.theme}>
            <Select
              value={config.theme}
              onValueChange={(v) =>
                setConfig({ theme: v as typeof config.theme })
              }
            >
              <SelectTrigger className="w-32">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="system">
                  {t.configPage.themeSystem}
                </SelectItem>
                <SelectItem value="light">{t.configPage.themeLight}</SelectItem>
                <SelectItem value="dark">{t.configPage.themeDark}</SelectItem>
              </SelectContent>
            </Select>
          </SettingRow>
          <Separator />
          <SettingRow
            label={t.configPage.timezone}
            tip={
              config.timezoneOffset === null
                ? `${t.configPage.timezoneAuto}：${formatTimezone(
                    timeZoneOffsetMillis()
                  )}`
                : undefined
            }
          >
            <Select
              value={
                config.timezoneOffset === null
                  ? "auto"
                  : String(config.timezoneOffset)
              }
              onValueChange={(v) =>
                setConfig({
                  timezoneOffset: v === "auto" ? null : Number(v),
                })
              }
            >
              <SelectTrigger className="w-36">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="auto">
                  {t.configPage.timezoneAuto}
                </SelectItem>
                {TIMEZONE_OFFSETS.map(({ offset, label }) => (
                  <SelectItem key={offset} value={String(offset)}>
                    {label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </SettingRow>
          <Separator />
          <SettingRow label={t.configPage.firstDayOfWeek}>
            <Select
              value={String(config.firstDayOfWeek)}
              onValueChange={(v) =>
                setConfig({
                  firstDayOfWeek: Number(v) as typeof config.firstDayOfWeek,
                })
              }
            >
              <SelectTrigger className="w-32">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="0">{t.configPage.monday}</SelectItem>
                <SelectItem value="1">{t.configPage.tuesday}</SelectItem>
                <SelectItem value="2">{t.configPage.wednesday}</SelectItem>
                <SelectItem value="3">{t.configPage.thursday}</SelectItem>
                <SelectItem value="4">{t.configPage.friday}</SelectItem>
                <SelectItem value="5">{t.configPage.saturday}</SelectItem>
                <SelectItem value="6">{t.configPage.sunday}</SelectItem>
              </SelectContent>
            </Select>
          </SettingRow>
          <Separator />
          <SettingRow label={t.configPage.dateFormat}>
            <Select
              value={config.dateFormat}
              onValueChange={(v) =>
                setConfig({ dateFormat: v as typeof config.dateFormat })
              }
            >
              <SelectTrigger className="w-32">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="yyyy-MM-dd">2025-07-05</SelectItem>
                <SelectItem value="yyyy/MM/dd">2025/07/05</SelectItem>
              </SelectContent>
            </Select>
          </SettingRow>
          <Separator />
          <SettingRow label={t.configPage.timeFormat}>
            <Select
              value={config.timeFormat}
              onValueChange={(v) =>
                setConfig({ timeFormat: v as typeof config.timeFormat })
              }
            >
              <SelectTrigger className="w-32">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="H:mm:ss">9:40 / 14:40</SelectItem>
                <SelectItem value="HH:mm:ss">09:40 / 14:40</SelectItem>
              </SelectContent>
            </Select>
          </SettingRow>
          <Separator />
          <SettingRow label={t.configPage.autoStart}>
            <Switch
              checked={autoStart}
              onCheckedChange={(v) => {
                setAutoStart(v)
                if (v) void enable()
                else void disable()
              }}
            />
          </SettingRow>
        </CardContent>
      </Card>

      <Card className="min-w-80 flex-1">
        <CardHeader>
          <CardTitle className="text-base">
            {t.configPage.rulesUpdates}
          </CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col">
          <SettingRow
            label={t.configPage.appRule}
            tip={t.configPage.appRuleTip}
          >
            <Button variant="outline" onClick={() => setRuleOpen(true)}>
              {t.configPage.edit}
            </Button>
          </SettingRow>
          <Separator />
          <SettingRow label={t.configPage.filterUninstalledApp}>
            <Switch
              checked={config.filterUninstalledApp}
              onCheckedChange={(v) => setConfig({ filterUninstalledApp: v })}
            />
          </SettingRow>
          <Separator />
          <SettingRow label={t.configPage.autoCheckUpdate}>
            <Switch
              checked={config.autoCheckUpdate}
              onCheckedChange={(v) => setConfig({ autoCheckUpdate: v })}
            />
          </SettingRow>
          <Separator />
          <SettingRow label={t.configPage.checkUpdate}>
            <Button
              variant="outline"
              disabled={checking}
              onClick={() => void checkUpdate()}
            >
              {checking ? "..." : t.configPage.checkUpdate}
            </Button>
          </SettingRow>
          <Separator />
          <SettingRow label={t.configPage.version}>
            <span className="text-sm text-muted-foreground">{version}</span>
          </SettingRow>
        </CardContent>
      </Card>
    </div>
  )
}
