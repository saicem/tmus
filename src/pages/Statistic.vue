<script setup lang="ts">
import { ref } from "vue"

const appShowStyleSelectValue = ref<"Card" | "Bar">("Card")
const datetimeRange = ref()
const queryData = () => {


}
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
      onchange="queryData()"
    />
  </div>
</template>

<style scoped></style>
