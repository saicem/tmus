<script lang="ts" setup>
import { AppDuration } from "@/script/models.ts"
import { i18n } from "@/script/i18n.ts"
import { statisticStore } from "@/script/state.ts"
import { getAppDetailMap, getDurationById } from "@/script/cmd.ts"

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

const load = async (startDate: Date, endDate: Date) => {
  const result = await getDurationById(startDate.getTime(), endDate.getTime())
  const appDetailMap = await getAppDetailMap()
  data.value = result
    .map((x) => {
      return {
        app: appDetailMap[x.appId],
        duration: x.duration,
      }
    })
    .sort((a, b) => b.duration - a.duration)
}

watch(datetimeRange, ([startDate, endDate]) => load(startDate, endDate), {
  immediate: true,
})
</script>

<template>
  <div style="display: flex; flex-direction: column; flex-wrap: wrap; gap: 8px">
    <div style="display: flex; flex-direction: row; gap: 8px">
      <el-date-picker
        v-model="datetimeRange"
        :shortcuts="shortcuts"
        end-placeholder="End date"
        range-separator="To"
        start-placeholder="Start date"
        style="flex: 1 0 360px"
        type="datetimerange"
      />
      <el-select
        v-model="statisticStore"
        default-first-option
        style="flex: 1 0 100px"
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
