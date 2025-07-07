<script setup lang="ts">
import { Chart } from "@antv/g2"
import { onMounted, ref, watch } from "vue"
import { colorMode, config } from "@/script/state.ts"
import { i18n } from "@/script/i18n.ts"
import moment from "moment-timezone"
import { durationByDay } from "@/script/api.ts"
import { dayFromEpoch, dayOfWeekOffset } from "@/script/time-util.ts"

const chartContainer = ref<HTMLDivElement | null>(null)

onMounted(async () => {
  if (chartContainer?.value) {
    renderBarChart(chartContainer.value, await loadData())
  }
})

async function loadData() {
  let now = moment()
  let todayDayOfWeekOffset = dayOfWeekOffset(now)
  let lastWeekStart = now
    .clone()
    .startOf("day")
    .subtract(todayDayOfWeekOffset + 7, "days")
  let result = await durationByDay(lastWeekStart, now)
  let lastWeekDayFromEpoch = dayFromEpoch(lastWeekStart)
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
        duration: Number(
          moment
            .duration(result[lastWeekDayFromEpoch + idx] ?? 0)
            .asHours()
            .toFixed(2)
        ),
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
      y: { title: null, labelFormatter: (d: number) => `${d}h` },
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
          valueFormatter: (d: string) => `${d}h`,
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
