<script setup lang="ts">
import { computed, onMounted, ref } from "vue"
import { durationByDayId, getAppDetailMap } from "@/script/api.ts"
import moment, { Moment } from "moment-timezone"
import { AppDuration, DateGroup, FileDetail } from "@/script/data.ts"
import AppCardGroup from "@/components/statistic/AppCardGroup.vue"
import { getTmusMeta } from "@/script/cmd.ts"
import { config } from "@/script/state.ts"

const scrollDisable = computed(() => loading.value || noMore.value)
const noMore = ref(false)
const loading = ref(true)
const nextDate = ref<Moment>(moment())
const data = ref<DateGroup<AppDuration>[]>([])
const millisInDay = 1000 * 60 * 60 * 24
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
  const result = await durationByDayId(startDate, endDate)
  let ripeResult: DateGroup<AppDuration>[] = await Promise.all(
    Object.entries(result).map(async ([k, v]) => {
      return {
        moment: moment(millisInDay * +k),
        data: await Promise.all(
          Object.entries(v).map(async ([id, duration]) => {
            return {
              app: appDetailMap.value[+id],
              duration: moment.duration(duration),
            }
          })
        ),
      }
    })
  )
  ripeResult.sort((a, b) => b.moment.unix() - a.moment.unix())
  ripeResult.forEach((dg) => {
    dg.data = dg.data
      .filter((d) => !config.value.filterUninstalledApp || d.app.exist)
      .sort((a, b) => b.duration.asMilliseconds() - a.duration.asMilliseconds())
  })
  ripeResult = ripeResult.filter((dg) => dg.data.length > 0)
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
        placement="top"
        :timestamp="date.format('YYYY-MM-DD')"
      >
        <app-card-group :data="appData" />
      </el-timeline-item>
    </div>
  </el-timeline>
  <div style="text-align: center">No more</div>
</template>

<style scoped></style>
