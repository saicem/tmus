<script lang="ts" setup>
import { ref, computed, onMounted, watch, onBeforeUnmount } from "vue"
import { Chart, TooltipComponent } from "@antv/g2"

export interface BarChartData {
  name: string
  value: number
}

const props = defineProps<{
  data: BarChartData[]
  tooltip?: TooltipComponent
  yAxisLabelFormat?: (value: number) => string
}>()

const containerRef = ref<HTMLDivElement | null>(null)
let chart: Chart | null = null

const processedData = computed<BarChartData[]>(() => {
  return [...props.data].sort((a, b) => b.value - a.value)
})

function renderChart(container: HTMLElement) {
  if (chart) {
    chart.destroy()
  }

  chart = new Chart({
    container,
    autoFit: true,
    height: 300,
  })

  const markNode = chart
    .interval()
    .data(processedData.value)
    .encode('x', 'name')
    .encode('y', 'value')
    .encode('color', 'name')
    .scale('color', {
      palette: 'spectral',
      offset: (t) => t * 0.8 + 0.1,
    })
    .axis('x', {
      title: null,
    })
    .axis('y', {
      title: null,
      labelFormatter: props.yAxisLabelFormat,
    })

  if (props.tooltip) {
    markNode.tooltip(props.tooltip)
  }

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
  <div ref="containerRef" style="width: 100%; height: 300px"></div>
</template>

<style scoped></style>
