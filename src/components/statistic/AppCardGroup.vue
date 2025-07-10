<script lang="ts" setup>
import { AppDuration } from "@/script/models.ts"

defineProps<{
  data: AppDuration[]
}>()
</script>

<template>
  <div style="display: flex; flex-wrap: wrap; gap: 16px">
    <router-link
      v-for="{ app: app, duration: duration } in data"
      :key="app.id"
      :to="'/detail/' + app.id"
    >
      <AppCard :detail="app" :duration="duration" />
    </router-link>
  </div>
</template>

<style scoped></style>
