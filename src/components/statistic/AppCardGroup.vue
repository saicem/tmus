<script lang="ts" setup>
import { FileDetail } from "@/script/models.ts"

defineProps<{
  data: { app: FileDetail, value: string }[]
}>()
</script>

<template>
  <div style="display: flex; flex-wrap: wrap; gap: 16px">
    <router-link v-for="{ app: app, value: value } in data" :key="app.id" :to="'/detail/' + app.id">
      <AppCard :detail="app" :value="value" />
    </router-link>
  </div>
</template>

<style scoped></style>
