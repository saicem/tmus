<script lang="ts" setup>
import { Chart } from "@antv/g2"
import { colorMode, config } from "@/script/state.ts"
import { i18n } from "@/script/i18n.ts"
import {
  dayOfWeekOffset,
  formatDurationRough,
  MILLISECONDS_PER_DAY,
} from "@/script/time-util.ts"
import { queryDurationStatistic } from "@/script/cmd.ts"

const chartContainer = ref<HTMLDivElement | null>(null)

onMounted(async () => {
  if (chartContainer?.value) {
    renderBarChart(chartContainer.value, await loadData())
  }
})

async function loadData() {
  let now = new Date()
  let todayDayOfWeekOffset = dayOfWeekOffset(now)
  let lastWeekStart = subDays(startOfDay(now), todayDayOfWeekOffset + 7)
  let result = Object.fromEntries(
    (
      await queryDurationStatistic(
        lastWeekStart.getTime(),
        now.getTime(),
        true,
        null,
        MILLISECONDS_PER_DAY,
        null
      )
    ).map((x) => [x.intervalStart, x.duration])
  )

  return Array(14)
    .fill(0)
    .map((_, idx) => {
      return {
        week:
          Math.trunc(idx / 7) == 0
            ? i18n.value.weeklyChart.lastWeek
            : i18n.value.weeklyChart.thisWeek,
        dayOfWeek:
          i18n.value.weeklyChart.dayOfWeekNames[
            (idx + config.value.firstDayOfWeek) % 7
          ],
        duration: result[addDays(lastWeekStart, idx).getTime()] ?? 0,
      }
    })
}

function renderBarChart(
  container: HTMLElement,
  chartData: {
    week: string
    dayOfWeek: string
    duration: number
  }[]
) {
  const chart = new Chart({ container })
  chart.theme({ type: colorMode.value })
  chart.options({
    title: i18n.value.weeklyChart.title,
    type: "interval",
    autoFit: true,
    data: chartData,
    encode: { x: "dayOfWeek", y: "duration", color: "week" },
    transform: [{ type: "dodgeX" }],
    axis: {
      x: { title: null },
      y: { title: null, labelFormatter: formatDurationRough },
    },
    interaction: {
      elementHighlight: { background: true },
      tooltip: {
        shared: true,
      },
    },
    tooltip: {
      items: [
        {
          channel: "y",
          valueFormatter: formatDurationRough,
        },
      ],
    },
  })
  chart.render()
}

watch(
  [() => config.value.firstDayOfWeek, () => config.value.theme, i18n],
  async (_old, _new) => {
    if (chartContainer?.value) {
      renderBarChart(chartContainer.value, await loadData())
    }
  },
  { deep: true }
)
</script>

<template>
  <div ref="chartContainer"></div>
</template>
