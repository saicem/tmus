<script lang="ts" setup>
import { Chart } from "@antv/g2"
import { i18n } from "@/script/i18n.ts"
import {
  dayOfWeekOffset,
  formatDurationRough,
  MILLISECONDS_PER_DAY,
} from "@/script/time-util.ts"
import { queryDurationStatistic } from "@/script/cmd.ts"
import { configStore, passiveStore } from "@/script/state.ts"
import { addDays, startOfDay, subDays } from "date-fns"

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
  let resultMap = toMap(
    await queryDurationStatistic(
      lastWeekStart.getTime(),
      now.getTime(),
      true,
      null,
      MILLISECONDS_PER_DAY,
      null
    ),
    (x) => [x.intervalStart, x.duration]
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
            (idx + configStore.firstDayOfWeek) % 7
          ],
        duration: resultMap.get(addDays(lastWeekStart, idx).getTime()) ?? 0,
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
  chart.theme({ type: passiveStore.theme })
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
  [() => configStore.firstDayOfWeek, () => configStore.theme, i18n],
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
