<script lang="ts" setup>
import { FileDetail } from "@/script/data.ts"
import { Duration } from "moment-timezone"
import { formatDuration } from "@/script/time-util.ts"

defineProps<{
  app: FileDetail
  duration: Duration
  percentage: number
}>()
</script>

<template>
  <div
    style="
      display: grid;
      grid-template-columns: 64px minmax(100px, 300px) 1fr;
      width: 100%;
      gap: 8px;
      padding: 8px;
    "
  >
    <div class="app-header">
      <el-image :src="app.icon" alt="App Icon" class="app-icon" />
    </div>
    <div style="display: flex; flex-direction: column; width: max-content; color: var(--font-color-bold)">
      <p class="app-title">{{ app.name }}</p>
      <p class="app-duration">{{ formatDuration(duration) }}</p>
    </div>
    <el-progress
      :percentage="percentage"
      :stroke-width="16"
      :text-inside="true"
    >
      <span style="color: var(--font-color-bold)">{{ formatDuration(duration) }}</span>

    </el-progress>
  </div>
</template>

<style scoped>
.app-icon {
  width: 48px;
  height: 48px;
}

.app-title {
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}
</style>
