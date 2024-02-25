<script setup lang="ts">
import moment from "moment";

const now = moment()

const emptyCellCount = now.month(0).date(1).day()
const daysInYearCount = now.month(11).date(31).dayOfYear()
const weekCount = Math.ceil((emptyCellCount + daysInYearCount) / 7)
const monthWeeks = new Array(12).fill(null).map((_, idx) => {
  const monthFirstDay = now.month(idx).date(1).dayOfYear() + emptyCellCount
  return Math.ceil((monthFirstDay - 1) / 7)
})

const dayOfWeekList = ["Sun", "Mon", "Tues", "Wed", "Thur", "Fri", "Sat"]
const monthList = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]

function inYearDate(week: number, dow: number) {
  const d = week * 7 + dow
  return !(d < emptyCellCount || d >= daysInYearCount + emptyCellCount)

}

</script>

<template>
  <table class="card heat-calendar-table">
    <thead>
      <tr style="height: 13px">
        <th></th>
        <th v-for="(_, i) in 12" :key="i"
          :colspan="i == 11 ? weekCount - monthWeeks[i] : monthWeeks[i + 1] - monthWeeks[i]" style="user-select: none;">
          {{ monthList[i] }}
        </th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(_, dow) in 7" :key="dow">
        <td style="width: 32px;user-select: none;">{{ dow % 2 == 1 ? dayOfWeekList[dow] : "" }}</td>
        <td v-for="(_, week) in weekCount" :key="week" style="height: 10px;width: 10px">
          <a-tooltip v-if="inYearDate(week, dow)">
            <template #title>{{
              now.dayOfYear(week * 7 + dow - 1 - emptyCellCount).format("yyyy-MM-DD")
            }}
            </template>
            <div :data-tag="4" class="block-unit"></div>
          </a-tooltip>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped>
.heat-calendar-table {
  width: max-content;
  padding: 16px;
  font-size: 12px;
  line-height: 13px;
  text-align: left;
  font-weight: bold;
}

.block-unit {
  height: 10px;
  width: 10px;
  border-radius: 2px;
}

.block-unit[data-tag="0"] {
  background: var(--accent-color);
}

.block-unit[data-tag="1"] {
  background: var(--accent-color-1);
}

.block-unit[data-tag="2"] {
  background: var(--accent-color-2);
}

.block-unit[data-tag="3"] {
  background: var(--accent-color-3);
}

.block-unit[data-tag="4"] {
  background: var(--accent-color-4);
}

.block-unit:hover {
  box-shadow: 0 0 5px rgb(57, 120, 255);
}
</style>