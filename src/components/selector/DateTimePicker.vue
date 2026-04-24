<template>
  <el-date-picker style="width: 360px" v-model="model" :shortcuts="shortcuts"
    :end-placeholder="i18n.statisticPage.placeholder.endDate"
    :range-separator="i18n.statisticPage.placeholder.rangeSeparator"
    :start-placeholder="i18n.statisticPage.placeholder.startDate" :type="props.type ?? 'datetimerange'" />
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { i18n } from "@/script/i18n.ts"
import { getTimeRangeByDays, getTimeRangeByMonths } from '@/script/time-util';

const model = defineModel<[Date, Date]>({
  default: () => [new Date(), new Date()]
})

const props = defineProps<{ type?: "daterange" | "datetimerange" }>()

const shortcuts = ref([
  {
    text: i18n.value.statisticPage.shortcuts.last1day,
    value: () => getTimeRangeByDays(1)
  },
  {
    text: i18n.value.statisticPage.shortcuts.last1week,
    value: () => getTimeRangeByDays(7)
  },
  {
    text: i18n.value.statisticPage.shortcuts.last1month,
    value: () => getTimeRangeByMonths(1)
  },
  {
    text: i18n.value.statisticPage.shortcuts.last3months,
    value: () => getTimeRangeByMonths(3)
  },
  {
    text: i18n.value.statisticPage.shortcuts.last1year,
    value: () => getTimeRangeByMonths(12)
  }
])
</script>
