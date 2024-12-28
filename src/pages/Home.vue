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
  mostUse.value = Math.max(...res.map((x) => x.duration.asHours())).toFixed(2)
  totalUse.value = res
    .map((x) => x.duration.asHours())
    .reduce((acc, x) => acc + x)
    .toFixed(2)
})
</script>

<template>
  <div style="display: flex; flex-direction: column; row-gap: 16px">
    <div class="cards">
      <GeneralCard
        :icon="app"
        :value="appCount"
        :unit="i18n.homePage.appsUnit"
        :illustration="i18n.homePage.apps"
      />
      <GeneralCard
        :icon="usage"
        :value="totalUse"
        :unit="i18n.homePage.totalUseUnit"
        :illustration="i18n.homePage.totalUse"
      />
      <GeneralCard
        :icon="up"
        :value="mostUse"
        :unit="i18n.homePage.mostUseUnit"
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
