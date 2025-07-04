<script setup lang="ts">
import { computed, onMounted, ref, toRaw } from "vue"
import { appDetail, durationByDayId } from "@/global/api.ts"
import moment, { Moment } from "moment-timezone"
import { AppDuration, DateGroup } from "@/global/data.ts"
import AppCardGroup from "@/components/statistic/AppCardGroup.vue"
import { appMeta } from "@/global/cmd.ts"
import { config } from "@/global/state.ts"

const scrollDisable = computed(() => loading.value || noMore.value)
const noMore = ref(false)
const loading = ref(false)
const nextDate = ref<Moment>(moment())
const data = ref<DateGroup<AppDuration>[]>([])
const millisInDay = 1000 * 60 * 60 * 24
const metaStartDate = ref<Moment>(moment())

onMounted(async () => {
  metaStartDate.value = moment((await appMeta()).startMsEpoch)
})

const load = async () => {
  loading.value = true
  const endDate = nextDate.value
  const startDate = endDate.clone().subtract(1, "week").startOf("day")
  const result = await durationByDayId(startDate, endDate)
  console.log("result", result)
  const ripeResult: DateGroup<AppDuration>[] = await Promise.all(
    Object.entries(result).map(async ([k, v]) => {
      return {
        moment: moment(millisInDay * +k),
        data: await Promise.all(
          Object.entries(v).map(async ([id, duration]) => {
            return {
              app: await appDetail(+id),
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
  data.value.push(...ripeResult)
  console.log(
    "timeline data",
    startDate.format("YYYY-MM-DD HH:mm:ss"),
    "to",
    endDate.format("YYYY-MM-DD HH:mm:ss"),
    toRaw(data.value)
  )
  nextDate.value = startDate.clone().subtract(1, "day")
  if (nextDate.value.isBefore(metaStartDate.value)) {
    console.log(
      "timeline no more data",
      nextDate.value.format("YYYY-MM-DD HH:mm:ss"),
      metaStartDate.value.format("YYYY-MM-DD HH:mm:ss")
    )
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
    :infinite-scroll-delay="400"
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
