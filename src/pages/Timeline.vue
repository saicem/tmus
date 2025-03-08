<script setup lang="ts">
import { computed, ref } from "vue"
import { appDetail, durationByDayId } from "@/global/api.ts"
import moment, { Moment } from "moment-timezone"
import { AppDuration, DateGroup } from "@/global/data.ts"
import AppCardGroup from "@/components/AppCardGroup.vue"

const scrollDisable = computed(() => loading.value || noMore.value)
const noMore = ref(false)
const loading = ref(false)
const nextDate = ref<Moment>(moment())
const data = ref<DateGroup<AppDuration>[]>([])
const millisInDay = 1000 * 60 * 60 * 24

const load = async () => {
  console.log("load")
  loading.value = true
  const endDate = nextDate.value
  const startDate = endDate.clone().subtract(1, "week")
  const result = await durationByDayId(startDate, endDate)
  const ripeResult = await Promise.all(
    Object.entries(result)
      .sort((a, b) => {
        return +b[0] - +a[0]
      })
      .map(async ([k, v]) => {
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
  data.value.push(...ripeResult)
  console.log("data", data)
  // TODO judge no more
  if (!ripeResult || ripeResult.length == 0) {
    console.log("no more data")
    noMore.value = true
    return
  }
  nextDate.value = startDate.clone().subtract(1, "day")
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
