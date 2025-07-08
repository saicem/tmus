<script lang="ts" setup>
import app from "@/assets/general-card/app.svg"
import usage from "@/assets/general-card/usage.svg"
import up from "@/assets/general-card/up.svg"
import { onMounted, ref } from "vue"
import { i18n } from "@/script/i18n.ts"
import GeneralCard from "@/components/GeneralCard.vue"
import HeatCalendar from "@/components/HeatCalendar.vue"
import WeeklyChart from "@/components/WeeklyChart.vue"
import { formatDuration, MILLISECONDS_PER_DAY } from "@/script/time-util.ts"
import { getDurationByDate, getDurationById } from "@/script/cmd.ts"
import { endOfYear, startOfDay, startOfYear } from "date-fns"

const yearData = ref<number[]>([])
const appCount = ref("0")
const totalUse = ref("0")
const mostUse = ref("0")

onMounted(async () => {
  await getYearData()
  await getDayData()
})

async function getYearData() {
  const now = new Date()
  const yearStart = startOfYear(now)
  const yearEnd = endOfYear(now)
  const data = await getDurationByDate(yearStart.getTime(), now.getTime())
  const durationByDateMap = Object.fromEntries(
    data.map((x) => [x.date, x.duration])
  )
  const result = []
  for (
    let i = yearStart.getTime();
    i <= yearEnd.getTime();
    i += MILLISECONDS_PER_DAY
  ) {
    result.push(durationByDateMap[+i])
  }
  yearData.value = result
}

async function getDayData() {
  const end = new Date()
  const dayStart = startOfDay(end)
  const records = await getDurationById(dayStart.getTime(), end.getTime())
  appCount.value = records.length.toString()
  totalUse.value = formatDuration(
    records.reduce((acc, cur) => acc + cur.duration, 0)
  )
  mostUse.value = formatDuration(
    records.reduce((acc, cur) => (acc > cur.duration ? acc : cur.duration), 0)
  )
}
</script>

<template>
  <div style="display: flex; flex-direction: column; row-gap: 16px">
    <div class="cards no-select">
      <GeneralCard
        :content="appCount + i18n.homePage.appsUnit"
        :icon="app"
        :illustration="i18n.homePage.apps"
      />
      <GeneralCard
        :content="totalUse"
        :icon="usage"
        :illustration="i18n.homePage.totalUse"
      />
      <GeneralCard
        :content="mostUse"
        :icon="up"
        :illustration="i18n.homePage.mostUse"
      />
    </div>
    <el-card class="heat-calendar-card">
      <HeatCalendar v-if="yearData" :data="yearData" />
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
