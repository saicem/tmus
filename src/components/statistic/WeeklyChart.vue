<script lang="ts" setup>
import { Chart } from "@antv/g2"
import { i18n } from "@/script/i18n.ts"
import {
  dayOfWeekOffset,
  formatDurationInHours,
  formatDurationRough,
  MILLISECONDS_PER_DAY,
} from "@/script/time-util.ts"
import { queryDurationStatistic } from "@/script/cmd.ts"
import { configStore, passiveStore } from "@/script/state.ts"
import { addDays, startOfDay, subDays } from "date-fns"

const chartContainer = ref<HTMLDivElement | null>(null)
const thisWeekTotalTime = ref("0")
const lastWeekTotalTime = ref("0")

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

  let ret = Array(14)
    .fill(0)
    .map((_, idx) => {
      return {
        week:
          Math.trunc(idx / 7) == 0
            ? i18n.value.common.lastWeek
            : i18n.value.common.thisWeek,
        dayOfWeek:
          i18n.value.weeklyChart.dayOfWeekNames[
            (idx + configStore.firstDayOfWeek) % 7
          ],
        duration: resultMap.get(addDays(lastWeekStart, idx).getTime()) ?? 0,
      }
    })
  thisWeekTotalTime.value = formatDurationInHours(
    ret.slice(7, 14).reduce((acc, cur) => acc + cur.duration, 0)
  )
  lastWeekTotalTime.value = formatDurationInHours(
    ret.slice(0, 7).reduce((acc, cur) => acc + cur.duration, 0)
  )
  return ret
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
      legendFilter: false,
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
  <div style="display: flex; flex-direction: row">
    <div ref="chartContainer" style="flex: 1; height: 320px"></div>
    <div class="chart-legend">
      <div class="chart-legend-item">
        <p class="legend-label">{{ i18n.weeklyChart.thisWeekTotalTime }}</p>
        <p class="legend-data">{{ thisWeekTotalTime }}</p>
      </div>
      <div class="chart-legend-item">
        <p class="legend-label">{{ i18n.weeklyChart.lastWeekTotalTime }}</p>
        <p class="legend-data">{{ lastWeekTotalTime }}</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.chart-legend {
  width: 150px;
  display: flex;
  flex-direction: column;
}

.chart-legend-item {
  display: flex;
  flex-direction: column;
  margin-bottom: 8px;
  flex: 1;
}

.legend-label {
  margin-top: 24px;
  font-size: small;
  color: var(--font-color);
}

.legend-data {
  margin-top: 8px;
  color: var(--font-color-bold);
  font-size: larger;
}
</style>
