<script lang="ts" setup>
import moment, { Duration } from "moment"
import HeatCalendarCell from "./HeatCalendarCell.vue"

const props = defineProps<{
  data: Duration[]
}>()

const now = moment()
const emptyCellCount = now.clone().startOf("year").day()
const daysInYearCount = now.clone().endOf("year").dayOfYear()
const weekCount = Math.ceil((emptyCellCount + daysInYearCount) / 7)
const monthWeeks = new Array(12).fill(null).map((_, idx) => {
  const monthFirstDayCellNum =
    now.clone().month(idx).date(1).dayOfYear() + emptyCellCount
  return Math.floor((monthFirstDayCellNum - 1) / 7)
})

const dayOfWeekList = ["Sun", "Mon", "Tues", "Wed", "Thur", "Fri", "Sat"]
const monthList = [
  "Jan",
  "Feb",
  "Mar",
  "Apr",
  "May",
  "Jun",
  "Jul",
  "Aug",
  "Sep",
  "Oct",
  "Nov",
  "Dec",
]

function dayOfYear(week: number, dow: number) {
  return week * 7 + dow + 1 - emptyCellCount
}

function showCell(week: number, dow: number) {
  const doy = dayOfYear(week, dow)
  return doy > 0 && doy <= daysInYearCount
}
</script>

<template>
  <table class="card heat-calendar-table">
    <thead>
      <tr style="height: 13px">
        <th></th>
        <th
          v-for="(_, i) in 12"
          :key="i"
          :colspan="
            i == 11
              ? weekCount - monthWeeks[i]
              : monthWeeks[i + 1] - monthWeeks[i]
          "
          class="month-label"
        >
          {{ monthList[i] }}
        </th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(_, dow) in 7" :key="dow">
        <th class="week-label">
          {{ dow % 2 == 1 ? dayOfWeekList[dow] : "" }}
        </th>
        <td
          v-for="(_, week) in weekCount"
          :key="week"
          style="height: 10px; width: 10px"
        >
          <HeatCalendarCell
            v-if="showCell(week, dow)"
            :day-of-year="dayOfYear(week, dow)"
            :duration="props.data[dayOfYear(week, dow)] ?? moment.duration(0)"
          />
        </td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped>
.week-label {
  width: 32px;
  user-select: none;
  color: var(--font-color-bold);
}

.month-label {
  user-select: none;
  color: var(--font-color-bold);
}

.heat-calendar-table {
  padding: 16px;
  font-size: 12px;
  line-height: 13px;
  text-align: left;
  font-weight: bold;
}
</style>
