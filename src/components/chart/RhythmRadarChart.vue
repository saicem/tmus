<script lang="ts" setup>
import { ref, computed, watch, onMounted, onBeforeUnmount } from "vue"
import { Chart } from "@antv/g2"
import { formatDurationRough, MILLISECONDS_PER_DAY, MILLISECONDS_PER_MINUTE } from "@/script/time-util"
import { i18n } from "@/script/i18n.ts"
import { TimeSpan } from "@/script/models"
import { passiveStore } from "@/script/state"

interface ChartData {
  index: number
  tag: string
  duration: number
  tooltipTitle: string
}

const props = defineProps<{
  type: TimeSpan
  granularity: number
  data: number[][]
}>()

const chartContainer = ref<HTMLDivElement | null>(null)
let chart: Chart | null = null
const domainMaximum = computed(() => {
  return props.data[0].length
})

const processedData = computed<ChartData[]>(() => {
  if (!props.data || props.data.length === 0) {
    return []
  }

  const result: ChartData[] = []
  console.log("chartData", props.data)
  props.data.forEach((groupData, groupIndex) => {
    groupData.forEach((averageDuration, periodIndex) => {
      result.push({
        index: periodIndex,
        tag: `Group ${groupIndex + 1}`,
        duration: averageDuration,
        tooltipTitle: computeTooltipTitle(periodIndex),
      })
    })
  })

  return result
})

function computeTooltipTitle(periodIndex: number) {
  if (props.type === "day") {
    return computeTooltipTitleItemInDay(periodIndex)
  } else if (props.type === "week") {
    return computeTooltipTitleItemInWeek(periodIndex)
  } else {
    return ""
  }
}

function computeTooltipTitleItemInDay(periodIndex: number) {
  const granularityByMinute = Math.round(props.granularity / MILLISECONDS_PER_MINUTE);
  const minutes = periodIndex * granularityByMinute;
  const item = (minutes: number) => {
    const h = Math.floor(minutes / 60);
    const m = minutes % 60;
    return `${h}:${m.toString().padStart(2, "0")}`
  }
  return `${item(minutes)} - ${item(minutes + granularityByMinute)}`
}

function computeTooltipTitleItemInWeek(periodIndex: number) {
  const granularityInDay = Math.round(MILLISECONDS_PER_DAY / props.granularity)
  const dayNo = Math.floor(periodIndex / granularityInDay)
  if (props.granularity === MILLISECONDS_PER_DAY) {
    return ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"][dayNo]
  }
  return `${["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"][dayNo]} ${computeTooltipTitleItemInDay(periodIndex % granularityInDay)}`
}

function labelFormatter(index: number) {
  if (props.type === "day") {
    const hourNo = Math.round(index * 24 / domainMaximum.value)
    return `${hourNo}`
  } else if (props.type === "week") {
    const dayNo = Math.round(index * 7 / domainMaximum.value)
    return ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"][dayNo]
  } else {
    return ""
  }
}

function scaleAndAxis() {
  return {
    scale: {
      x: {
        padding: 0.5,
        align: 0,
        tickCount: domainMaximum.value,
        domain: [0, domainMaximum.value],
      },
      y: {
        domain: [0, props.granularity],
      },
    },
    axis: {
      x: {
        tickFilter: (_: any, i: number) => i !== domainMaximum.value && i * (props.type === "day" ? 24 : 7) % domainMaximum.value === 0,
        labelFormatter: (index: number) => labelFormatter(index),
      },
      y: {
        zIndex: 1,
        title: false,
        label: false,
      },
    },
  }
}

function renderChart() {
  if (!chartContainer.value) {
    return
  }

  if (chart) {
    chart.destroy()
  }

  chart = new Chart({
    container: chartContainer.value,
    autoFit: true,
    height: 320,
  })
  chart.theme({ type: passiveStore.theme })


  chart.options({
    coordinate: {
      type: "polar",
    },
    data: processedData.value,
    ...scaleAndAxis(),
  })

  chart
    .area()
    .encode("x", "index")
    .encode("y", "duration")
    .encode("color", "tag")
    .encode("shape", "smooth")
    .style("fillOpacity", 0.3)
    .style("lineWidth", 1)
    .tooltip({
      title: (x: ChartData) => x.tooltipTitle,
      items: [
        {
          channel: "y",
          valueFormatter: (duration) => formatDurationRough(duration),
        },
      ],
    })

  chart.interaction("tooltip", {
    crosshairsLineDash: [4, 4],
    filter: (d: any) => {
      return d.value != "0m"
    },
  })

  chart.render()
}

watch(
  [() => props.data, () => props.type],
  () => {
    renderChart()
  },
  { deep: true }
)

onMounted(() => {
  renderChart()
})

onBeforeUnmount(() => {
  if (chart) {
    chart.destroy()
    chart = null
  }
})
</script>

<template>
  <div style="display: flex; flex-direction: column; overflow: hidden">
    <h3 style="font-size: 16px; font-weight: bold; text-align: center">
      {{ i18n.homePage.dailyRhythm }}
    </h3>
    <div ref="chartContainer" style="flex: 1; height: 320px"></div>
  </div>
</template>

<style scoped></style>