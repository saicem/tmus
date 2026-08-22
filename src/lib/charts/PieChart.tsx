import {
  Cell,
  Pie,
  PieChart as RePieChart,
  ResponsiveContainer,
  Tooltip,
} from "recharts"
import { getChartColors } from "./colors"

/**
 * Donut chart built on Recharts (the library behind shadcn's chart component).
 */
export function PieChart({
  data,
  size = 200,
  thickness = 26,
  formatValue,
}: {
  data: { name: string; value: number }[]
  size?: number
  thickness?: number
  formatValue?: (value: number) => string
}) {
  const colors = getChartColors()
  return (
    <ResponsiveContainer width="100%" height={size}>
      <RePieChart>
        <Pie
          data={data}
          dataKey="value"
          nameKey="name"
          cx="50%"
          cy="50%"
          innerRadius={size / 2 - thickness}
          outerRadius={size / 2}
          paddingAngle={1}
          stroke="transparent"
          isAnimationActive={false}
        >
          {data.map((_, i) => (
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
