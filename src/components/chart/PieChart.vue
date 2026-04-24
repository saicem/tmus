<script lang="ts" setup>
import { Chart, TooltipComponent } from "@antv/g2"

export interface PieChartData {
  name: string
  value: number
  percentage: number
}

const props = defineProps<{
  data: PieChartData[]
  tooltip?: TooltipComponent
}>()

const containerRef = ref<HTMLDivElement | null>(null)
let chart: Chart | null = null

const THRESHOLD = 3

const processedData = computed<PieChartData[]>(() => {
  const result: PieChartData[] = []
  let otherSum = 0
  let exceptOtherPercentage = 0

  for (const item of props.data) {
    item.percentage = item.percentage
    if (item.percentage < THRESHOLD && result.length > 0) {
      otherSum += item.value
    } else {
      exceptOtherPercentage += item.percentage
      result.push(item)
    }
  }

  if (otherSum > 0) {
    result.push({
      name: "Other",
      value: otherSum,
      percentage: Math.round((100 - exceptOtherPercentage) * 10) / 10,
    })
  }

  return result
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

  chart.coordinate({ type: 'theta', innerRadius: 0.5, outerRadius: 0.8 });

  const markNode = chart
    .interval()
    .data(processedData.value)
    .transform({ type: 'stackY' })
    .encode('y', 'value')
    .encode('color', 'name')
    .scale('color', {
      palette: 'spectral',
      offset: (t) => t * 0.8 + 0.1,
    })
    .label({
      position: 'outside',
      text: (data: PieChartData) => `${data.name}: ${data.percentage}%`,
      transform: [{ type: 'overlapDodgeY' }]
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
