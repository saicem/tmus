<script lang="ts" setup>
import { AppDuration, DateGroup, FileDetail } from "@/script/models.ts"
import {
  getAppDetailMap,
  getTmusMeta,
  queryDurationStatistic,
} from "@/script/cmd.ts"
import { config } from "@/script/state.ts"
import { MILLISECONDS_PER_DAY } from "@/script/time-util.ts"
import { format } from "date-fns"

const scrollDisable = computed(() => loading.value || noMore.value)
const noMore = ref(false)
const loading = ref(true)
const nextDate = ref<Date>(new Date())
const data = ref<DateGroup<AppDuration>[]>([])
const metaStartDate = ref<Date>(new Date())
const appDetailMap = ref<Record<number, FileDetail>>({})

onMounted(async () => {
  metaStartDate.value = new Date((await getTmusMeta()).initialTimestamp)
  appDetailMap.value = await getAppDetailMap()
  loading.value = false
})

const load = async () => {
  loading.value = true
  const endDate = nextDate.value
  const startDate = subWeeks(startOfDay(endDate), 1)
  const result = await queryDurationStatistic(
    startDate.getTime(),
    endDate.getTime(),
    false,
    null,
    MILLISECONDS_PER_DAY,
    null
  )
  const dateMap: Record<string, AppDuration[]> = {}
  result.forEach((x) => {
    const detail = appDetailMap.value[x.appId!]
    if (config.value.filterUninstalledApp && !detail.exist) {
      return
    }
    if (dateMap[x.intervalStart] == undefined) {
      dateMap[x.intervalStart] = []
    }
    dateMap[x.intervalStart].push({
      app: detail,
      duration: x.duration,
    })
  })
  let ripeResult: DateGroup<AppDuration>[] = Object.entries(dateMap).map(
    ([k, v]) => {
      return { moment: new Date(+k), data: v }
    }
  )
  ripeResult.sort((a, b) => b.moment.valueOf() - a.moment.valueOf())
  ripeResult.forEach((dg) => {
    dg.data = dg.data.sort((a, b) => b.duration - a.duration)
  })
  data.value.push(...ripeResult)
  nextDate.value = subDays(startDate, 1)
  if (isBefore(nextDate.value, metaStartDate.value)) {
    noMore.value = true
    return
  }
  loading.value = false
}
</script>

<template>
  <el-timeline
    v-infinite-scroll="load"
    :infinite-scroll-disabled="scrollDisable"
    infinite-scroll-distance="1000"
  >
    <div>
      <el-timeline-item
        v-for="({ moment: date, data: appData }, i) in data"
        :key="i"
        :timestamp="format(date, config.dateFormat)"
        placement="top"
      >
        <app-card-group :data="appData" />
      </el-timeline-item>
    </div>
  </el-timeline>
  <div style="text-align: center">No more</div>
</template>

<style scoped></style>
