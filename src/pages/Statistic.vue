<script setup lang="ts">
import { computed, ref, watch } from "vue"
import { AppDuration } from "@/script/data.ts"
import moment, { Moment } from "moment-timezone"
import { appDetail, durationById } from "@/script/api.ts"
import { i18n } from "@/script/i18n.ts"
import AppProgressGroup from "@/components/statistic/AppProgressGroup.vue"
import AppCardGroup from "@/components/statistic/AppCardGroup.vue"
import { statisticStore } from "@/script/state.ts"

const shortcuts = computed(() => [
  {
    text: i18n.value.statisticPage.shortcuts.last1day,
    value: (): [Date, Date] => {
      const end = new Date()
      const start = new Date()
      start.setDate(start.getDate() - 1)
      return [start, end]
    },
  },
  {
    text: i18n.value.statisticPage.shortcuts.last3days,
    value: (): [Date, Date] => {
      const end = new Date()
      const start = new Date()
      start.setDate(start.getDate() - 3)
      return [start, end]
    },
  },
  {
    text: i18n.value.statisticPage.shortcuts.last1week,
    value: (): [Date, Date] => {
      const end = new Date()
      const start = new Date()
      start.setDate(start.getDate() - 7)
      return [start, end]
    },
  },
  {
    text: i18n.value.statisticPage.shortcuts.last1month,
    value: (): [Date, Date] => {
      const end = new Date()
      const start = new Date()
      start.setMonth(start.getMonth() - 1)
      return [start, end]
    },
  },
  {
    text: i18n.value.statisticPage.shortcuts.last3months,
    value: (): [Date, Date] => {
      const end = new Date()
      const start = new Date()
      start.setMonth(start.getMonth() - 3)
      return [start, end]
    },
  },
  {
    text: i18n.value.statisticPage.shortcuts.last1year,
    value: (): [Date, Date] => {
      const end = new Date()
      const start = new Date()
      start.setMonth(start.getMonth() - 12)
      return [start, end]
    },
  },
])

const datetimeRange = ref<[Date, Date]>(
  ((): [Date, Date] => {
    const end = new Date()
    const start = new Date()
    start.setDate(start.getDate() - 1)
    return [start, end]
  })()
)
const data = ref<AppDuration[]>([])

const load = async (startDate: Moment, endDate: Moment) => {
  let result = await durationById(startDate, endDate)
  data.value = (
    await Promise.all(
      Object.entries(result).map(async ([k, v]) => {
        return {
          app: await appDetail(+k),
          duration: moment.duration(v),
        }
      })
    )
  ).sort((a, b) => b.duration.asMilliseconds() - a.duration.asMilliseconds())
}

watch(
  datetimeRange,
  ([startDate, endDate]) => load(moment(startDate), moment(endDate)),
  { immediate: true }
)
</script>

<template>
  <div style="display: flex; flex-direction: column; flex-wrap: wrap; gap: 8px">
    <div style="display: flex; flex-direction: row; gap: 8px">
      <el-date-picker
        v-model="datetimeRange"
        type="datetimerange"
        :shortcuts="shortcuts"
        range-separator="To"
        start-placeholder="Start date"
        end-placeholder="End date"
        style="flex: 1 0 360px"
      />
      <el-select
        style="flex: 1 0 100px"
        default-first-option
        v-model="statisticStore"
      >
        <el-option :label="i18n.statisticPage.type.card" value="Card" />
        <el-option :label="i18n.statisticPage.type.progress" value="Progress" />
      </el-select>
    </div>
    <app-card-group v-if="statisticStore == 'Card'" :data="data" />
    <app-progress-group v-if="statisticStore == 'Progress'" :data="data" />
  </div>
</template>

<style scoped></style>
