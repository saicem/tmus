<script lang="ts" setup>
import { FileDetail } from "@/script/models.ts"
import { formatDuration } from "@/script/time-util.ts"

defineProps<{
  app: FileDetail
  duration: number
  percentage: number
}>()
</script>

<template>
  <div
    style="
      display: flex;
      flex-direction: row;
      width: 100%;
      gap: 8px;
      padding: 8px;
    "
  >
    <app-icon :icon="app.icon" :size="32" />
    <div
      style="
        display: flex;
        flex-direction: column;
        width: max-content;
        color: var(--font-color-bold);
        flex: 1;
      "
    >
      <p class="app-title">{{ app.name }}</p>
      <el-progress
        :percentage="percentage"
        :stroke-width="16"
        :text-inside="true"
      >
        <span style="color: var(--font-color-bold)">{{
          formatDuration(duration)
        }}</span>
      </el-progress>
    </div>
  </div>
</template>

<style scoped>
.app-title {
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
  line-height: 1;
}
</style>
