<script lang="ts" setup>
import { computed, onMounted, ref } from "vue"
import moment, { Moment } from "moment-timezone"
import { AppDuration, DateGroup, FileDetail } from "@/script/models.ts"
import AppCardGroup from "@/components/statistic/AppCardGroup.vue"
import {
  getAppDetailMap,
  getDurationByDateID,
  getTmusMeta,
} from "@/script/cmd.ts"
import { config } from "@/script/state.ts"

const scrollDisable = computed(() => loading.value || noMore.value)
const noMore = ref(false)
const loading = ref(true)
const nextDate = ref<Moment>(moment())
const data = ref<DateGroup<AppDuration>[]>([])
const metaStartDate = ref<Moment>(moment())
const appDetailMap = ref<Record<number, FileDetail>>({})

onMounted(async () => {
  metaStartDate.value = moment((await getTmusMeta()).startMsEpoch)
  appDetailMap.value = await getAppDetailMap()
  loading.value = false
})

const load = async () => {
  loading.value = true
  const endDate = nextDate.value
  const startDate = endDate.clone().subtract(1, "week").startOf("day")
  const result = await getDurationByDateID(startDate, endDate)
  const dateMap: Record<string, AppDuration[]> = {}
  result.forEach((x) => {
    const detail = appDetailMap.value[x.appId]
    if (config.value.filterUninstalledApp && !detail.exist) {
      return
    }
    if (dateMap[x.date] == undefined) {
      dateMap[x.date] = []
    }
    dateMap[x.date].push({
      app: detail,
      duration: moment.duration(x.duration),
    })
  })
  let ripeResult: DateGroup<AppDuration>[] = Object.entries(dateMap).map(
    ([k, v]) => {
      return { moment: moment(+k), data: v }
    }
  )
  ripeResult.sort((a, b) => b.moment.valueOf() - a.moment.valueOf())
  ripeResult.forEach((dg) => {
    dg.data = dg.data.sort(
      (a, b) => b.duration.asMilliseconds() - a.duration.asMilliseconds()
    )
  })
  data.value.push(...ripeResult)
  nextDate.value = startDate.clone().subtract(1, "day")
  if (nextDate.value.isBefore(metaStartDate.value)) {
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
        :timestamp="date.format('YYYY-MM-DD')"
        placement="top"
      >
        <app-card-group :data="appData" />
      </el-timeline-item>
    </div>
  </el-timeline>
  <div style="text-align: center">No more</div>
</template>

<style scoped></style>
