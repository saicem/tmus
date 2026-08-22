import {
  Area,
  AreaChart as ReAreaChart,
  CartesianGrid,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts"
import { getChartColors, getGridColor, getMutedColor } from "./colors"

/**
 * Area chart built on Recharts (the library behind shadcn's chart component).
 */
export function AreaChart({
  data,
  height = 300,
  formatValue,
  yName,
}: {
  data: { label: string; value: number }[]
  height?: number
  formatValue?: (value: number) => string
  yName?: string
}) {
  const colors = getChartColors()
  const muted = getMutedColor()
  const grid = getGridColor()

  return (
    <ResponsiveContainer width="100%" height={height}>
      <ReAreaChart
        data={data}
        margin={{ top: 8, right: 8, left: 0, bottom: 0 }}
      >
        <CartesianGrid stroke={grid} strokeOpacity={0.4} vertical={false} />
        <XAxis
          dataKey="label"
          tick={{ fill: muted, fontSize: 12 }}
          tickLine={false}
          axisLine={false}
          interval="preserveStartEnd"
          minTickGap={40}
        />
        <YAxis
          tick={{ fill: muted, fontSize: 12 }}
          tickLine={false}
          axisLine={false}
          width={64}
          allowDecimals={false}
          tickFormatter={(v: number) =>
            formatValue ? formatValue(v) : String(v)
          }
          label={
            yName
              ? {
                  value: yName,
                  angle: -90,
                  position: "insideLeft",
                  style: { fill: muted, fontSize: 12, textAnchor: "middle" },
                }
              : undefined
          }
        />
        <Tooltip
          contentStyle={{ borderRadius: 8, fontSize: 12 }}
          formatter={(value) =>
            formatValue ? formatValue(Number(value)) : String(value)
          }
        />
        <Area
          type="monotone"
          dataKey="value"
          stroke={colors[0]}
          strokeWidth={2}
          fill={colors[0]}
          fillOpacity={0.15}
          isAnimationActive={false}
        />
      </ReAreaChart>
    </ResponsiveContainer>
  )
}
