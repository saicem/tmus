<script lang="ts" setup>
import { ref, onMounted, watch, onBeforeUnmount } from "vue"
import { Chart } from "@antv/g2"
import { formatDurationRough } from "@/script/time-util"
import { i18n } from "@/script/i18n"
import { passiveStore } from "@/script/state"

export interface StackBarChartData {
  label: string
  x: string
  y: number
}

const props = defineProps<{
  data: StackBarChartData[]
}>()

const containerRef = ref<HTMLDivElement | null>(null)
let chart: Chart | null = null

function renderChart(container: HTMLElement) {
  if (chart) {
    chart.destroy()
  }

  chart = new Chart({
    container,
    autoFit: true,
    height: 400,
  })
  chart.theme({ type: passiveStore.theme })

  chart.interval()
    .data(props.data)
    .encode("x", "x")
    .encode("y", "y")
    .encode("color", "label")
    .transform({ type: 'dodgeX' })
    .axis('x', {
      title: null,
    })
    .axis('y', {
      title: null,
      labelFormatter: formatDurationRough
    })
    .tooltip({
      items: [{
        name: i18n.value.statisticPage.types.duration,
        field: 'y',
        valueFormatter: formatDurationRough
      }],
    })

  chart.render()
}

onMounted(() => {
  if (containerRef.value) {
    renderChart(containerRef.value)
  }
})

watch(
  () => props.data,
  () => {
    if (containerRef.value) {
      renderChart(containerRef.value)
    }
  },
  { deep: true }
)

onBeforeUnmount(() => {
  if (chart) {
    chart.destroy()
    chart = null
  }
})
</script>

<template>
  <div ref="containerRef" style="width: 100%; height: 400px"></div>
</template>

<style scoped></style>