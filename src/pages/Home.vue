<script setup lang="ts">
import app from "@/assets/general-card/app.svg"
import usage from "@/assets/general-card/usage.svg"
import up from "@/assets/general-card/up.svg"
import { onMounted, ref } from "vue"
import moment, { Duration } from "moment"
import { i18n } from "@/script/i18n.ts"
import { durationByDayInThisYear, todayAppGeneral } from "@/script/api.ts"
import GeneralCard from "@/components/GeneralCard.vue"
import HeatCalendar from "@/components/HeatCalendar.vue"
import WeeklyChart from "@/components/WeeklyChart.vue"
import { formatDuration } from "@/script/time-util.ts"

const duration = ref<Record<number, Duration>>({})
const appCount = ref("0")
const totalUse = ref("0")
const mostUse = ref("0")

onMounted(async () => {
  duration.value = await durationByDayInThisYear()
})

todayAppGeneral().then((res) => {
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
    <div class="cards no-select">
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
    <el-card class="heat-calendar-card">
      <HeatCalendar :data="duration" v-if="duration" />
    </el-card>

    <el-card>
      <WeeklyChart />
    </el-card>
  </div>
</template>

<style scoped>
@media (max-width: 1100px) {
  .heat-calendar-card {
    display: none;
  }
}

.cards {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  gap: 16px;
}
</style>
