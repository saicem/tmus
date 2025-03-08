<script setup lang="ts">
import { ref, watch } from "vue"
import { AppDuration } from "@/global/data.ts"
import AppCardGroup from "@/components/AppCardGroup.vue"
import moment, { Moment } from "moment-timezone"
import { appDetail, durationById } from "@/global/api.ts"

const shortcuts = [
  {
    text: "Last 1 day",
    value: (): [Date, Date] => {
      const end = new Date()
      const start = new Date()
      start.setDate(start.getDate() - 1)
      return [start, end]
    },
  },
  {
    text: "Last 3 days",
    value: (): [Date, Date] => {
      const end = new Date()
      const start = new Date()
      start.setDate(start.getDate() - 3)
      return [start, end]
    },
  },
  {
    text: "Last 1 week",
    value: (): [Date, Date] => {
      const end = new Date()
      const start = new Date()
      start.setDate(start.getDate() - 7)
      return [start, end]
    },
  },
  {
    text: "Last 1 month",
    value: (): [Date, Date] => {
      const end = new Date()
      const start = new Date()
      start.setMonth(start.getMonth() - 1)
      return [start, end]
    },
  },
  {
    text: "Last 3 months",
    value: (): [Date, Date] => {
      const end = new Date()
      const start = new Date()
      start.setMonth(start.getMonth() - 3)
      return [start, end]
    },
  },
  {
    text: "Last 1 year",
    value: (): [Date, Date] => {
      const end = new Date()
      const start = new Date()
      start.setMonth(start.getMonth() - 12)
      return [start, end]
    },
  },
]

const datetimeRange = ref<[Date, Date]>(shortcuts[0].value())
const data = ref<AppDuration[]>([])

const load = async (startDate: Moment, endDate: Moment) => {
  console.log("load", startDate, endDate)
  let result = await durationById(startDate, endDate)
  data.value = await Promise.all(
    Object.entries(result).map(async ([k, v]) => {
      return {
        app: await appDetail(+k),
        duration: moment.duration(v),
      }
    })
  )
}

watch(
  datetimeRange,
  ([startDate, endDate]) => load(moment(startDate), moment(endDate)),
  { immediate: true }
)
</script>

<template>
  <div style="display: flex; flex-wrap: wrap; gap: 8px">
    <el-date-picker
      v-model="datetimeRange"
      type="datetimerange"
      :shortcuts="shortcuts"
      range-separator="To"
      start-placeholder="Start date"
      end-placeholder="End date"
    />
    <app-card-group :data="data" />
  </div>
</template>

<style scoped></style>
