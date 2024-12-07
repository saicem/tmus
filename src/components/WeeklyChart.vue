<script setup lang="ts">
import { Chart } from "@antv/g2"
import { onMounted, ref, watch } from "vue"
import { Duration } from "moment"
import { colorMode, config } from "@/global/state.ts"
import { msg } from "@/global/i18n.ts"

const props = defineProps<{
  /**
   * 14 days duration, last week and this week.
   */
  durations: Duration[]
}>()
const root = ref<HTMLDivElement | null>(null)
let plot: Chart | null = null

function convertData(durations: Duration[]) {
  let lastWeek = durations.slice(0, 7)
  let thisWeek = durations.slice(7, 14)
  return thisWeek
    .map((d, i) => {
      return {
        week: msg.value.weeklyChart.thisWeek,
        dayOfWeek: msg.value.weeklyChart.dayOfWeekNames[i % 7],
        duration: Number(d.asHours().toFixed(2)),
      }
    })
    .concat(
      lastWeek.map((d, i) => {
        return {
          week: msg.value.weeklyChart.lastWeek,
          dayOfWeek: msg.value.weeklyChart.dayOfWeekNames[i % 7],
          duration: Number(d.asHours().toFixed(2)),
        }
      })
    )
}

function renderBarChart(container: HTMLElement) {
  console.log("renderBarChart")
  const chart = new Chart({ container })
  chart.theme({ type: colorMode.value })
  chart.options({
    title: msg.value.weeklyChart.title,
    type: "interval",
    autoFit: true,
    data: convertData(props.durations),
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
  plot = chart
  plot.render()
}

watch(
  config,
  (_old, _new) => {
    renderBarChart(root.value!)
  },
  { deep: true }
)

onMounted(() => {
  if (root?.value) {
    renderBarChart(root.value)
  }
})
</script>

<template>
  <div ref="root"></div>
</template>
