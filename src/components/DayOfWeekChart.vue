<script setup lang="ts">
import {Chart} from "@antv/g2"
import {onMounted, ref} from "vue"
import {themeStore} from "@/global/state.ts"
import {latestTwoWeek} from "@/global/api.ts"

latestTwoWeek()

const root = ref<HTMLDivElement | null>(null)

function renderBarChart(container: HTMLElement) {
  const chart = new Chart({container})

  if (themeStore.isDark()) {
    chart.theme({type: "classicDark"})
  }

  chart.options({
    title: "周使用时长",
    type: "interval",
    autoFit: true,
    data: [
      {week: "本周", dayOfWeek: "周一", duration: 18.9},
      {week: "本周", dayOfWeek: "周二", duration: 28.8},
      {week: "本周", dayOfWeek: "周三", duration: 39.3},
      {week: "本周", dayOfWeek: "周四", duration: 81.4},
      {week: "本周", dayOfWeek: "周五", duration: 47},
      {week: "本周", dayOfWeek: "周六", duration: 20.3},
      {week: "本周", dayOfWeek: "周日", duration: 24},
      {week: "上周", dayOfWeek: "周一", duration: 18.9},
      {week: "上周", dayOfWeek: "周二", duration: 28.8},
      {week: "上周", dayOfWeek: "周三", duration: 39.3},
      {week: "上周", dayOfWeek: "周四", duration: 81.4},
      {week: "上周", dayOfWeek: "周五", duration: 47},
      {week: "上周", dayOfWeek: "周六", duration: 20.3},
      {week: "上周", dayOfWeek: "周日", duration: 24},
    ],
    encode: {x: "dayOfWeek", y: "duration", color: "week"},
    transform: [{type: "dodgeX"}],
    axis: {
      x: {title: null}, y: {title: null, labelFormatter: (d: number) => `${d}h`},
    },
    interaction: {
      elementHighlight: {background: true},
      tooltip: {
        shared: true,
      }
    },
    tooltip: {
      items: [
        {
          channel: "y",
          valueFormatter: (d: string) => `${d}h`
        }
      ]
    }
  })

  chart.render()
}

onMounted(() => {
  if (root?.value) {
    renderBarChart(root.value)
  }

})

</script>

<template>
  <div class="card">
    <div ref="root"></div>
  </div>
</template>
