<script setup lang="ts">
import app from "@/assets/general-card/app.svg"
import usage from "@/assets/general-card/usage.svg"
import up from "@/assets/general-card/up.svg"
import api from "@/global/api"
import { computed, ref } from "vue"
import moment, { Duration } from "moment"
import { languageStore } from "@/global/state"
import { messages } from "@/global/i18n.ts"

const msg = computed(() => messages[languageStore.language].homePage)
const duration = ref<Record<number, Duration>>()
const weeklyDurations = ref<Duration[]>()
const appCount = ref("0")
const totalUse = ref("0")
const mostUse = ref("0")

api.durationByDayInThisYear().then((res) => {
  console.log("durationByDayInThisYear", res)
  duration.value = res
  let startDayOfYear = moment().startOf("week").subtract(1, "week").dayOfYear()
  weeklyDurations.value = new Array(14).fill(null).map((_, idx) => {
    return res[startDayOfYear + idx] ?? moment.duration(0)
  })
})

api.todayAppGeneral().then((res) => {
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
        :unit="msg.appsUnit"
        :illustration="msg.apps"
      />
      <GeneralCard
        :icon="usage"
        :value="totalUse"
        :unit="msg.totalUseUnit"
        :illustration="msg.totalUse"
      />
      <GeneralCard
        :icon="up"
        :value="mostUse"
        :unit="msg.mostUseUnit"
        :illustration="msg.mostUse"
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
