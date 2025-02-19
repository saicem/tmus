<script setup lang="ts">
import app from "@/assets/general-card/app.svg"
import usage from "@/assets/general-card/usage.svg"
import up from "@/assets/general-card/up.svg"
import { ref } from "vue"
import moment, { Duration } from "moment"
import { i18n } from "@/global/i18n.ts"
import { durationByDayInThisYear, todayAppGeneral } from "@/global/api.ts"
import GeneralCard from "@/components/GeneralCard.vue"
import HeatCalendar from "@/components/HeatCalendar.vue"
import WeeklyChart from "@/components/WeeklyChart.vue"

const duration = ref<Record<number, Duration>>()
const weeklyDurations = ref<Duration[]>()
const appCount = ref("0")
const totalUse = ref("0")
const mostUse = ref("0")

function formatDuration(duration: Duration): string {
  let h = duration.hours()
  let m = duration.minutes()
  return h > 0 ? `${h}h${m}m` : `${m}m`
}

durationByDayInThisYear().then((res) => {
  console.log("durationByDayInThisYear", res)
  duration.value = res
  let startDayOfYear = moment().startOf("week").subtract(1, "week").dayOfYear()
  weeklyDurations.value = new Array(14).fill(0).map((_, idx) => {
    console.log(
      "idx",
      startDayOfYear + idx,
      res[startDayOfYear + idx],
      res[startDayOfYear + idx]?.asHours()
    )
    return res[startDayOfYear + idx] ?? moment.duration(0)
  })
})

todayAppGeneral().then((res) => {
  console.log("todayAppGeneral", res)
  if (res.length == 0) {
    return
  }
  appCount.value = res.length.toString()
  const durations = res.map((x) => x.duration)
  mostUse.value = formatDuration(
    moment.duration(
      Math.max(...durations.map((x) => x.asMilliseconds())),
      "milliseconds"
    )
  )
  totalUse.value = formatDuration(durations.reduce((x, y) => x.add(y)))
})
</script>

<template>
  <div style="display: flex; flex-direction: column; row-gap: 16px">
    <div class="cards">
      <GeneralCard
        :icon="app"
        :content="appCount + i18n.homePage.appsUnit"
        :illustration="i18n.homePage.apps"
      />
      <GeneralCard
        :icon="usage"
        :content="totalUse"
        :illustration="i18n.homePage.totalUse"
      />
      <GeneralCard
        :icon="up"
        :content="mostUse"
        :illustration="i18n.homePage.mostUse"
      />
    </div>
    <el-card>
      <HeatCalendar :data="duration" v-if="duration" />
    </el-card>
    <el-card>
      <WeeklyChart :durations="weeklyDurations" v-if="weeklyDurations" />
    </el-card>
  </div>
</template>

<style scoped>
.cards {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  gap: 16px;
}
</style>
