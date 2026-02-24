<script lang="ts" setup>
import app from "@/assets/general-card/app.svg"
import usage from "@/assets/general-card/usage.svg"
import { i18n } from "@/script/i18n.ts"
import { formatDuration, MILLISECONDS_PER_DAY } from "@/script/time-util.ts"
import {
  getAppDetail,
  getDurationById,
  queryDurationStatistic,
} from "@/script/cmd.ts"
import { endOfYear, startOfDay, startOfYear } from "date-fns"
import AppProgressGroupV2 from "@/components/statistic/AppProgressGroupV2.vue"
import { AppDuration } from "@/script/models.ts"

const yearData = ref<number[]>([])
const todayUseAppCount = ref("0")
const todayUseAppDuration = ref("0")
onMounted(async () => {
  await getYearData()
  await getDayData()
})
const progressData = ref<AppDuration[]>([])

async function getYearData() {
  const now = new Date()
  const yearStart = startOfYear(now)
  const yearEnd = endOfYear(now)
  const data = await queryDurationStatistic(
    yearStart.getTime(),
    now.getTime(),
    true,
    null,
    MILLISECONDS_PER_DAY,
    null
  )
  const durationByDateMap = toMap(data, (x) => [x.intervalStart, x.duration])
  const result = []
  for (
    let i = yearStart.getTime();
    i <= yearEnd.getTime();
    i += MILLISECONDS_PER_DAY
  ) {
    result.push(durationByDateMap.get(i)!)
  }
  yearData.value = result
}

async function getDayData() {
  const end = new Date()
  const dayStart = startOfDay(end)
  const records = await getDurationById(dayStart.getTime(), end.getTime())
  todayUseAppCount.value = records.length.toString()
  todayUseAppDuration.value = formatDuration(
    records.reduce((acc, cur) => acc + cur.duration, 0)
  )
  let progressDataResult: AppDuration[] = []
  for (const [id, durationList] of groupBy(records, (x) => [
    x.appId,
    x.duration,
  ])) {
    progressDataResult.push({
      app: await getAppDetail(id),
      duration: durationList.reduce((acc, cur) => acc + cur),
    })
  }
  progressDataResult.sort((a, b) => b.duration - a.duration)
  progressData.value = progressDataResult
}
</script>

<template>
  <div style="display: flex; flex-direction: column; row-gap: 16px">
    <div class="cards no-select">
      <general-card
        :content="todayUseAppCount + i18n.homePage.appsUnit"
        :icon="app"
        :illustration="i18n.homePage.apps"
      />
      <general-card
        :content="todayUseAppDuration"
        :icon="usage"
        :illustration="i18n.homePage.totalUse"
      />
    </div>
    <el-card class="heat-calendar-card">
      <heat-calendar v-if="yearData" :data="yearData" />
    </el-card>

    <el-card>
      <weekly-chart />
    </el-card>

    <div style="display: flex; flex-direction: row; gap: 16px">
      <el-card style="flex: 2">
        <p>{{ i18n.homePage.todayUsage }}</p>
        <app-progress-group-v2 :data="progressData" />
      </el-card>
      <el-card style="flex: 1; height: 400px">
        <weekly-polar />
      </el-card>
    </div>
  </div>
</template>

<style scoped>
@media (max-width: 1099px) {
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
