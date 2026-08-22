import { useMemo } from "react"
import {
  Bar,
  BarChart as ReBarChart,
  CartesianGrid,
  Legend,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts"
import { getChartColors, getGridColor, getMutedColor } from "./colors"

export interface BarDatum {
  label: string
  group?: string
  value: number
}

/**
 * Bar chart built on Recharts (the library behind shadcn's chart component),
 * supporting both single and grouped (dodged) series.
 */
export function BarChart({
  data,
  height = 280,
  formatValue,
}: {
  data: BarDatum[]
  height?: number
  formatValue?: (value: number) => string
}) {
  const colors = getChartColors()
  const muted = getMutedColor()
  const grid = getGridColor()

  const { groups, rows } = useMemo(() => {
    const groups = Array.from(
      new Set(data.map((d) => d.group).filter(Boolean))
    ) as string[]
    const labels = Array.from(new Set(data.map((d) => d.label)))
    const rows = labels.map((label) => {
      const row: Record<string, string | number> = { label }
      for (const item of data.filter((d) => d.label === label)) {
        row[item.group ?? "value"] = item.value
      }
      return row
    })
    return { groups, rows }
  }, [data])

  return (
    <ResponsiveContainer width="100%" height={height}>
      <ReBarChart data={rows} margin={{ top: 8, right: 8, left: 0, bottom: 0 }}>
        <CartesianGrid stroke={grid} strokeOpacity={0.4} vertical={false} />
        <XAxis
          dataKey="label"
          tick={{ fill: muted, fontSize: 12 }}
          tickLine={false}
          axisLine={false}
          interval="preserveStartEnd"
          minTickGap={8}
        />
        <YAxis
          tick={{ fill: muted, fontSize: 12 }}
          tickLine={false}
          axisLine={false}
          width={38}
          tickFormatter={(v: number) =>
            formatValue ? formatValue(v) : String(v)
          }
        />
        <Tooltip
          cursor={{ fill: grid, fillOpacity: 0.15 }}
          contentStyle={{ borderRadius: 8, fontSize: 12 }}
          formatter={(value) =>
            formatValue ? formatValue(Number(value)) : String(value)
          }
        />
        {groups.length === 0 ? (
          <Bar
            dataKey="value"
            fill={colors[0]}
            radius={[3, 3, 0, 0]}
            maxBarSize={40}
            isAnimationActive={false}
          />
        ) : (
          <>
            <Legend
              iconType="circle"
              wrapperStyle={{ fontSize: 12, color: muted }}
            />
            {groups.map((g, i) => (
              <Bar
                key={g}
                dataKey={g}
                fill={colors[i % colors.length]}
                radius={[3, 3, 0, 0]}
                maxBarSize={26}
                isAnimationActive={false}
              />
            ))}
          </>
        )}
      </ReBarChart>
    </ResponsiveContainer>
  )
}
