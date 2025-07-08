<script lang="ts" setup>
import AppCard from "@/components/statistic/AppCard.vue"
import { AppDuration } from "@/script/models.ts"

defineProps<{
  data: AppDuration[]
}>()
</script>

<template>
  <div style="display: flex; flex-wrap: wrap; gap: 16px">
    <RouterLink
      v-for="{ app: app, duration: duration } in data"
      :key="app.id"
      :to="'/detail/' + app.id"
    >
      <AppCard :detail="app" :duration="duration" />
    </RouterLink>
  </div>
</template>

<style scoped></style>
