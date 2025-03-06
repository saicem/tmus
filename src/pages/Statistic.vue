<script setup lang="ts">
import { ref, watch } from "vue"
import { AppDuration } from "@/global/data.ts"
import AppCardGroup from "@/components/AppCardGroup.vue"
import moment, { Moment } from "moment-timezone"
import { appDetail, durationById } from "@/global/api.ts"

const appShowStyleSelectValue = ref<"Card" | "Bar">("Card")
const datetimeRange = ref()
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

watch(datetimeRange, ([startDate, endDate]) => load(startDate, endDate))

const shortcuts = [
  {
    text: "Last 1 day",
    value: () => {
      const end = new Date()
      const start = new Date()
      start.setDate(start.getDate() - 1)
      return [start, end]
    },
  },
  {
    text: "Last 3 days",
    value: () => {
      const end = new Date()
      const start = new Date()
      start.setDate(start.getDate() - 3)
      return [start, end]
    },
  },
  {
    text: "Last 1 week",
    value: () => {
      const end = new Date()
      const start = new Date()
      start.setDate(start.getDate() - 7)
      return [start, end]
    },
  },
  {
    text: "Last 1 month",
    value: () => {
      const end = new Date()
      const start = new Date()
      start.setMonth(start.getMonth() - 1)
      return [start, end]
    },
  },
  {
    text: "Last 3 months",
    value: () => {
      const end = new Date()
      const start = new Date()
      start.setMonth(start.getMonth() - 3)
      return [start, end]
    },
  },
  {
    text: "Last 1 year",
    value: () => {
      const end = new Date()
      const start = new Date()
      start.setMonth(start.getMonth() - 12)
      return [start, end]
    },
  },
]
</script>

<template>
  <div style="display: flex; flex-wrap: wrap; gap: 8px">
    <el-select
      ref="appShowStyleSelect"
      v-model="appShowStyleSelectValue"
      style="width: 240px"
    >
      <el-option value="Card">Card</el-option>
      <el-option value="Bar">Bar</el-option>
    </el-select>
    <el-date-picker
      v-model="datetimeRange"
      type="datetimerange"
      :shortcuts="shortcuts"
      range-separator="To"
      start-placeholder="Start date"
      end-placeholder="End date"
    />
    <app-card-group v-if="appShowStyleSelectValue == 'Card'" :data="data" />
  </div>
</template>

<style scoped></style>
