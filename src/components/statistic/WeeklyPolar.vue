<script lang="ts" setup>
import { Chart } from "@antv/g2"
import {
  MILLISECONDS_PER_HOUR,
  MILLISECONDS_PER_MINUTE,
} from "@/script/time-util.ts"
import { startOfDay, subDays } from "date-fns"
import { queryDurationStatistic } from "@/script/cmd.ts"
import { i18n } from "@/script/i18n.ts"
import { configStore } from "@/script/state.ts"

const data = ref<{ hour: string; week: string; duration: number }[]>([])
const chartContainer = ref<HTMLDivElement | null>(null)

async function loadData() {
  let now = new Date()
  let pastWeekEnd = startOfDay(now)
  let pastWeekStart = subDays(pastWeekEnd, 7)

  let newData: {
    hour: string
    week: string
    duration: number
  }[] = []
  // The past week average
  const pastWeekData = toMap(
    await queryDurationStatistic(
      pastWeekStart.getTime(),
      pastWeekEnd.getTime(),
      true,
      null,
      MILLISECONDS_PER_HOUR,
      24
    ),
    (x) => [
      x.intervalStart / MILLISECONDS_PER_HOUR,
      +(x.duration / (MILLISECONDS_PER_MINUTE * 7)).toFixed(1),
    ]
  )
  newData.push(
    ...range(0, 24).map((x) => {
      return {
        hour: x.toString(),
        week: i18n.value.homePage.pastWeekAverage,
        duration: pastWeekData.get(x) ?? 0,
      }
    })
  )

  // Today
  const todayData = toMap(
    await queryDurationStatistic(
      pastWeekEnd.getTime(),
      now.getTime(),
      true,
      null,
      MILLISECONDS_PER_HOUR,
      24
    ),
    (x) => [
      x.intervalStart / MILLISECONDS_PER_HOUR,
      +(x.duration / MILLISECONDS_PER_MINUTE).toFixed(1),
    ]
  )
  console.log("todayData", todayData, pastWeekData)
  newData.push(
    ...range(0, 24).map((x) => {
      return {
        hour: x.toString(),
        week: i18n.value.common.today,
        duration: todayData.get(x) ?? 0,
      }
    })
  )
  data.value = newData
}

function renderChart(container: HTMLElement) {
  const chart = new Chart({
    container,
    autoFit: true,
    height: 320,
  })

  chart.coordinate({ type: "polar" })

  chart
    .data(data.value)
    .scale("x", { padding: 0.5, align: 0 })
    .scale("y", { domain: [0, 60] })
    .axis("x", { grid: true })
    .axis("y", {
      zIndex: 1,
      title: false,
      label: false,
    })

  chart
    .area()
    .encode("x", "hour")
    .encode("y", "duration")
    .encode("color", "week")
    .encode("shape", "smooth")
    .style("fillOpacity", 0.5)
    .scale("x", { domainMax: 23 })

  chart
    .line()
    .encode("x", "hour")
    .encode("y", "duration")
    .encode("color", "week")
    .encode("shape", "smooth")
    .style("lineWidth", 2)

  chart.interaction("tooltip", { crosshairsLineDash: [4, 4] })

  chart.render()
}

onMounted(async () => {
  if (chartContainer?.value) {
    await loadData()
    console.log(data.value)
    renderChart(chartContainer.value)
  }
})

watch(
  [() => configStore.theme, i18n],
  async (_old, _new) => {
    if (chartContainer?.value) {
      await loadData()
      renderChart(chartContainer.value)
    }
  },
  { deep: true }
)
</script>

<template>
  <div style="display: flex; flex-direction: column; overflow: hidden">
    <div style="text-align: center">{{ i18n.homePage.dailyRhythm }}</div>
    <div ref="chartContainer" style="flex: 1; height: 320px" />
  </div>
</template>

<style scoped></style>
