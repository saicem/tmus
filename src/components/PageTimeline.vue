<script setup lang="ts">
import AppBasicCard from "@/components/AppBasicCard.vue"
import moment from "moment"

defineProps<{
  usageDaily: {
    date: string
    appList: {
      icon: string
      name: string
      duration: string
    }[]
  }[]
}>()

const now = moment()

function dateText(date: string) {
  const target = moment(date)
  const diffDays = now.diff(target, "day")
  if (diffDays == 0) {
    return "今天"
  } else if (diffDays == 1) {
    return "昨天"
  } else {
    return target.format("YYYY-MM-DD")
  }
}
</script>

<template>
  <div
    class="container usage-timeline"
    v-for="(usage, index) in usageDaily"
    :key="index"
  >
    <h3 style="text-align: left; color: var(--font-color-bold)">
      {{ dateText(usage.date) }}
    </h3>
    <div class="usage-timeline-app-box">
      <AppBasicCard
        v-for="(app, index) in usage.appList"
        :key="index"
        :duration="app.duration"
        :icon="app.icon"
        :name="app.name"
      />
    </div>
  </div>
</template>

<style scoped>
.usage-timeline {
  gap: 12px;
}

.usage-timeline-app-box {
  display: flex;
  flex-flow: row wrap;
  gap: 12px;
}
</style>
