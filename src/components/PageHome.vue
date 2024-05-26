<script setup lang="ts">
import app from "@/assets/general-card/app.svg"
import usage from "@/assets/general-card/usage.svg"
import up from "@/assets/general-card/up.svg"
import GeneralCard from "@/components/GeneralCard.vue"
import DayOfWeekChart from "@/components/DayOfWeekChart.vue"
import HeatCalendar from "@/components/HeatCalendar.vue"
import { durationByDayInThisYear } from "@/global/command"
import { ref } from "vue"
import { Duration } from "moment"

const duration = ref<Record<number, Duration>>()

durationByDayInThisYear().then((res) => {
  duration.value = res
})
</script>

<template>
  <div class="container" style="gap: 16px; padding: 8px">
    <div class="cards">
      <GeneralCard :icon="app" value="16" unit="个" illustration="应用量" />
      <GeneralCard :icon="usage" value="3.89" unit="小时" illustration="总计" />
      <GeneralCard
        :icon="up"
        value="2.13"
        unit="小时"
        illustration="最常使用"
      />
    </div>
    <HeatCalendar :data="duration" v-if="duration" />
    <DayOfWeekChart style="flex: 1 0; height: 200px" />
  </div>
</template>

<style scoped>
.cards {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  gap: 16px;
}
</style>
