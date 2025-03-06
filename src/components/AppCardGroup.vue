<script setup lang="ts">
import AppCard from "@/components/AppCard.vue"
import { AppDuration } from "@/global/data.ts"

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
