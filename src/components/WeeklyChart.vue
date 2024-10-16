<script setup lang="ts">
import { Chart } from "@antv/g2"
import { onMounted, ref, onUpdated } from "vue"
import { Duration } from "moment"

const props = defineProps<{
  /**
   * 14 days duration, last week and this week.
   */
  durations: Duration[]
  theme: string
}>()
const root = ref<HTMLDivElement | null>(null)
let plot: Chart | null = null

const dayOfWeekName = ["周日", "周一", "周二", "周三", "周四", "周五", "周六"]
const data = ((durations) => {
  let lastWeek = durations.slice(0, 7)
  let thisWeek = durations.slice(7, 14)
  return thisWeek
    .map((d, i) => {
      return {
        week: "本周",
        dayOfWeek: dayOfWeekName[i % 7],
        duration: Number(d.asHours().toFixed(2)),
      }
    })
    .concat(
      lastWeek.map((d, i) => {
        return {
          week: "上周",
          dayOfWeek: dayOfWeekName[i % 7],
          duration: Number(d.asHours().toFixed(2)),
        }
      })
    )
})(props.durations)

function renderBarChart(container: HTMLElement) {
  const chart = new Chart({ container })
  chart.theme({ type: props.theme })
  chart.options({
    title: "周使用时长",
    type: "interval",
    autoFit: true,
    data: data,
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

onMounted(() => {
  if (root?.value) {
    renderBarChart(root.value)
  }
})

onUpdated(() => {
  if (root?.value) {
    plot?.theme({ type: props.theme })
    plot?.render()
  }
})
</script>

<template>
  <div ref="root"></div>
</template>
