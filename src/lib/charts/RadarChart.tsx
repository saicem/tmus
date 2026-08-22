import { useMemo } from "react"
import {
  PolarAngleAxis,
  PolarGrid,
  PolarRadiusAxis,
  Radar,
  RadarChart as ReRadarChart,
  ResponsiveContainer,
  Tooltip,
} from "recharts"
import { getChartColors, getGridColor, getMutedColor } from "./colors"

export interface RadarPoint {
  time: number
  tag: string
  value: number
}

export interface RadarTick {
  value: number
  label: string
}

interface RadarPointXY {
  x: number
  y: number
}

/**
 * Convert a closed polygon of points into a smooth closed path using
 * Catmull-Rom splines converted to cubic bezier segments.
 */
function smoothClosedPath(points: RadarPointXY[]): string {
  if (points.length < 3) {
    return points.map((p) => `L ${p.x} ${p.y}`).join(" ")
  }
  let d = `M ${points[0].x} ${points[0].y}`
  for (let i = 0; i < points.length; i++) {
    const p0 = points[(i - 1 + points.length) % points.length]
    const p1 = points[i]
    const p2 = points[(i + 1) % points.length]
    const p3 = points[(i + 2) % points.length]
    const c1x = p1.x + (p2.x - p0.x) / 6
    const c1y = p1.y + (p2.y - p0.y) / 6
    const c2x = p2.x - (p3.x - p1.x) / 6
    const c2y = p2.y - (p3.y - p1.y) / 6
    d += ` C ${c1x} ${c1y}, ${c2x} ${c2y}, ${p2.x} ${p2.y}`
  }
  return `${d} Z`
}

/** Custom smooth shape for the Radar series (data points unchanged). */
function RadarShape(props: {
  points?: (RadarPointXY | null)[]
  fill?: string
  fillOpacity?: number
  stroke?: string
  strokeWidth?: number
}) {
  const points = (props.points ?? []).filter(
    (p): p is RadarPointXY => p != null
  )
  return (
    <path
      d={smoothClosedPath(points)}
      fill={props.fill}
      fillOpacity={props.fillOpacity}
      stroke={props.stroke}
      strokeWidth={props.strokeWidth}
    />
  )
}
interface RadarTooltipProps {
  active?: boolean
  payload?: {
    name: string
    value: number
    color: string
    payload: Record<string, unknown>
  }[]
  formatValue?: (value: number) => string
  timeFormatter?: (time: number) => string
}

/**
 * Custom tooltip: each data block is a time range (e.g. 10:00 - 10:10) shown
 * as the title, followed by the per-series values of that block.
 */
function RadarTooltip({
  active,
  payload,
  formatValue,
  timeFormatter,
}: RadarTooltipProps) {
  if (!active || !payload?.length) return null
  const row = payload[0].payload
  const rangeStart = row.rangeStart as number | undefined
  const rangeEnd = row.rangeEnd as number | undefined
  const range =
    rangeStart !== undefined && rangeEnd !== undefined
      ? `${timeFormatter ? timeFormatter(rangeStart) : String(rangeStart)} - ${
          timeFormatter ? timeFormatter(rangeEnd) : String(rangeEnd)
        }`
      : ""
  const raw = row.raw as Record<string, number> | undefined
  return (
    <div className="rounded-lg border bg-popover px-3 py-2 text-xs text-popover-foreground shadow-md">
      {range && <div className="mb-1 font-medium">{range}</div>}
      {payload.map((entry) => (
        <div key={entry.name} style={{ color: entry.color }}>
          {entry.name}:{" "}
          {formatValue
            ? formatValue(raw?.[entry.name] ?? entry.value)
            : String(raw?.[entry.name] ?? entry.value)}
        </div>
      ))}
    </div>
  )
}

/**
 * Polar radar chart built on Recharts. Each input block is a time range
 * (one vertex per block, connected with straight lines), and the tooltip
 * reports the block's time range and original value.
 */
export function RadarChart({
  data,
  domain = [0, 24],
  ticks,
  size = 400,
  formatValue,
  timeFormatter,
}: {
  data: RadarPoint[]
  domain?: [number, number]
  ticks?: RadarTick[]
  size?: number
  formatValue?: (value: number) => string
  timeFormatter?: (time: number) => string
}) {
  const colors = getChartColors()
  const muted = getMutedColor()
  const grid = getGridColor()

  const { tags, rows, labelMap } = useMemo(() => {
    const tags = Array.from(new Set(data.map((d) => d.tag)))
    const baseTimes = Array.from(new Set(data.map((d) => d.time))).sort(
      (a, b) => a - b
    )
    // Decimate dense input; keep whole hours so hourly ticks always exist.
    const step = Math.max(1, Math.ceil(baseTimes.length / 48))
    const isWholeHour = (t: number) => Number.isInteger(t)
    const times = baseTimes.filter((t, i) => isWholeHour(t) || i % step === 0)

    const rows = times.map((time, i) => {
      const rangeStart = time
      const rangeEnd = times[(i + 1) % times.length]
      const row: Record<string, unknown> = {
        time,
        rangeStart,
        rangeEnd,
        raw: {},
      }
      for (const tag of tags) {
        const point = data.find((d) => d.tag === tag && d.time === time)
        row[tag] = point?.value ?? 0
        ;(row.raw as Record<string, number>)[tag] = point?.value ?? 0
      }
      return row
    })

    const map = new Map<number, string>()
    const span = domain[1] - domain[0]
    const list =
      ticks ??
      (span <= 48
        ? // Hourly ticks (e.g. 0-23 for a day).
          Array.from({ length: span }, (_, i) => ({
            value: domain[0] + i,
            label: String(Math.round(domain[0] + i)),
          }))
        : Array.from({ length: 5 }, (_, i) => {
            const value = domain[0] + ((domain[1] - domain[0]) / 4) * i
            return { value, label: String(Math.round(value)) }
          }))
    for (const t of list) map.set(t.value, t.label)
    return { tags, rows, labelMap: map }
  }, [data, domain, ticks])

  const angleFormatter = (value: number) => labelMap.get(value) ?? ""

  return (
    <ResponsiveContainer width="100%" height={size}>
      <ReRadarChart data={rows} cx="50%" cy="50%" outerRadius="78%">
        <PolarGrid radialLines={false} stroke={grid} strokeOpacity={0.5} />
        <PolarAngleAxis
          dataKey="time"
          type="number"
          domain={domain}
          tickFormatter={angleFormatter}
          tick={{ fill: muted, fontSize: 9 }}
        />
        <PolarRadiusAxis domain={[0, "auto"]} tick={false} axisLine={false} />
        {tags.map((tag, i) => (
          <Radar
            key={tag}
            name={tag}
            dataKey={tag}
            stroke={colors[i % colors.length]}
            strokeWidth={1.5}
            fill={colors[i % colors.length]}
            fillOpacity={0.22}
            isAnimationActive={false}
            shape={<RadarShape />}
          />
        ))}
        <Tooltip
          content={
            <RadarTooltip
              formatValue={formatValue}
              timeFormatter={timeFormatter}
            />
          }
        />
      </ReRadarChart>
    </ResponsiveContainer>
  )
}
