import {
  Cell,
  Pie,
  PieChart as RePieChart,
  type PieLabelRenderProps,
  ResponsiveContainer,
  Tooltip,
} from "recharts"
import { getChartColors, getMutedColor } from "./colors"

const RADIAN = Math.PI / 180

/**
 * Donut chart built on Recharts (the library behind shadcn's chart component).
 */
export function PieChart({
  data,
  size = 200,
  thickness = 26,
  formatValue,
  otherLabel,
}: {
  data: { name: string; value: number }[]
  size?: number
  thickness?: number
  formatValue?: (value: number) => string
  otherLabel?: string
}) {
  const colors = getChartColors()
  const muted = getMutedColor()
  // Merge slices whose share is below this threshold into one "Other" slice.
  const OTHER_SHARE_THRESHOLD = 0.01
  const total = data.reduce((sum, item) => sum + item.value, 0)
  const chartData =
    total > 0 && otherLabel
      ? (() => {
          const small: typeof data = []
          const large: typeof data = []
          for (const item of data) {
            if (item.value / total < OTHER_SHARE_THRESHOLD) {
              small.push(item)
            } else {
              large.push(item)
            }
          }
          if (small.length === 0) return data
          return [
            ...large,
            {
              name: otherLabel,
              value: small.reduce((sum, item) => sum + item.value, 0),
            },
          ]
        })()
      : data

  const renderLabel = (props: PieLabelRenderProps) => {
    const {
      cx = 0,
      cy = 0,
      x = 0,
      y = 0,
      midAngle = 0,
      outerRadius = 0,
      percent = 0,
      name,
      textAnchor,
    } = props
    // Skip tiny slices so their labels do not overlap the others, but always
    // keep the merged "Other" slice labeled.
    if (percent < OTHER_SHARE_THRESHOLD && name !== otherLabel) return null
    const lineStartX = cx + outerRadius * Math.cos(-midAngle * RADIAN)
    const lineStartY = cy + outerRadius * Math.sin(-midAngle * RADIAN)
    return (
      <g>
        <line
          x1={lineStartX}
          y1={lineStartY}
          x2={x}
          y2={y}
          stroke={muted}
          strokeWidth={1}
        />
        <text
          x={x}
          y={y}
          fill={muted}
          fontSize={12}
          textAnchor={textAnchor}
          dominantBaseline="central"
        >
          {name}
        </text>
      </g>
    )
  }

  return (
    <ResponsiveContainer width="100%" height={size + 56}>
      <RePieChart>
        <Pie
          data={chartData}
          dataKey="value"
          nameKey="name"
          cx="50%"
          cy="50%"
          innerRadius={size / 2 - thickness}
          outerRadius={size / 2}
          paddingAngle={1}
          stroke="transparent"
          isAnimationActive={false}
          labelLine={false}
          label={renderLabel}
        >
          {chartData.map((_, i) => (
            <Cell key={i} fill={colors[i % colors.length]} />
          ))}
        </Pie>
        <Tooltip
          contentStyle={{ borderRadius: 8, fontSize: 12 }}
          formatter={(value) =>
            formatValue ? formatValue(Number(value)) : String(value)
          }
        />
      </RePieChart>
    </ResponsiveContainer>
  )
}
