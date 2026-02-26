<script lang="ts" setup>
import { Chart } from "@antv/g2"
import {
  MILLISECONDS_PER_DAY,
  MILLISECONDS_PER_MINUTE,
} from "@/script/time-util.ts"
import { startOfDay, subDays } from "date-fns"
import { queryDurationStatistic } from "@/script/cmd.ts"
import { i18n } from "@/script/i18n.ts"
import { configStore } from "@/script/state.ts"

interface ChartData {
  time: number
  tag: string
  duration: number
  tooltipTitle: string
}

const data = ref<ChartData[]>([])
const chartContainer = ref<HTMLDivElement | null>(null)

const granularityInMinute = 10
const granularity = MILLISECONDS_PER_MINUTE * granularityInMinute
const cycle = MILLISECONDS_PER_DAY / granularity
const cycleInHour = cycle / 24

async function loadData() {
  let now = new Date()
  let pastWeekEnd = startOfDay(now)
  let pastWeekStart = subDays(pastWeekEnd, 7)

  let newData: ChartData[] = []
  // The past week average, the statistical granularity is 10 minutes
  const pastWeekData = toMap(
    await queryDurationStatistic(
      pastWeekStart.getTime(),
      pastWeekEnd.getTime(),
      true,
      null,
      granularity,
      cycle
    ),
    (x) => [
      x.intervalStart / granularity,
      +(x.duration / MILLISECONDS_PER_MINUTE / 7).toFixed(1),
    ]
  )
  newData.push(
    ...range(0, cycle).map((x) => {
      return {
        time: x / cycleInHour,
        tag: i18n.value.homePage.pastWeekAverage,
        duration: pastWeekData.get(x) ?? 0,
        tooltipTitle: computeTooltipTitle(x),
      }
    })
  )

  // Today, as above
  const todayData = toMap(
    await queryDurationStatistic(
      pastWeekEnd.getTime(),
      now.getTime(),
      true,
      null,
      granularity,
      cycle
    ),
    (x) => [
      x.intervalStart / granularity,
      +(x.duration / MILLISECONDS_PER_MINUTE).toFixed(1),
    ]
  )
  newData.push(
    ...range(0, cycle).map((x) => {
      return {
        time: x / cycleInHour,
        tag: i18n.value.common.today,
        duration: todayData.get(x) ?? 0,
        tooltipTitle: computeTooltipTitle(x),
      }
    })
  )
  data.value = newData
}

function computeTooltipTitle(cycleNo: number) {
  return `${computeTooltipTitleItem(cycleNo)} - ${computeTooltipTitleItem(
    cycleNo + 1
  )}`
}

function computeTooltipTitleItem(cycleNo: number) {
  const cycleInHourNo = cycleNo % cycleInHour
  const h = (cycleNo - cycleInHourNo) / cycleInHour
  const m = cycleInHourNo * granularityInMinute
  return `${h}:${m.toString().padStart(2, "0")}`
}

function renderChart(container: HTMLElement) {
  const chart = new Chart({
    container,
    autoFit: true,
    height: 320,
  })

  chart
    .coordinate({ type: "polar" })
    .data(data.value)
    .scale("x", {
      padding: 0.5,
      align: 0,
      tickCount: 24,
      domain: [0, 24],
    })
    .scale("y", { domain: [0, granularityInMinute] })
    // Fuck! Is customizing the coordinate axis really this complicated?
    .axis("x", {
      tickFilter: (_: any, i: number) => i !== 24,
    })
    .axis("y", {
      zIndex: 1,
      title: false,
      label: false,
    })

  chart
    .area()
    .encode("x", "time")
    .encode("y", "duration")
    .encode("color", "tag")
    .encode("shape", "smooth")
    .style("fillOpacity", 0.5)
    .tooltip({
      title: (x: ChartData) => x.tooltipTitle,
      items: [
        {
          channel: "y",
          valueFormatter: (x) => `${x}m`,
        },
      ],
    })

  chart
    .line()
    .encode("x", "time")
    .encode("y", "duration")
    .encode("color", "tag")
    .encode("shape", "smooth")
    .style("lineWidth", 1)
    .tooltip(null)

  chart.interaction("tooltip", {
    crosshairsLineDash: [4, 4],
    filter: (d: any) => {
      return d.value != "0m"
    },
  })

  chart.render()
}

onMounted(async () => {
  if (chartContainer?.value) {
    await loadData()
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
    <h3 style="font-size: 16px; font-weight: bold; text-align: center">
      {{ i18n.homePage.dailyRhythm }}
    </h3>
    <div ref="chartContainer" style="flex: 1; height: 320px" />
  </div>
</template>

<style scoped></style>
