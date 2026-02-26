<script lang="ts" setup>
import { FileDetail } from "@/script/models.ts"
import { formatDuration } from "@/script/time-util.ts"

defineProps<{
  duration: number
  detail: FileDetail
}>()
</script>

<template>
  <el-card>
    <div
      style="
        height: 36px;
        width: 150px;
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 8px;
      "
    >
      <app-icon style="flex: 0 0 auto" :icon="detail.icon" :size="32" />
      <div
        style="
          flex: 1;
          display: flex;
          flex-direction: column;
          justify-content: center;
        "
      >
        <p
          style="
            font-weight: bold;
            text-align: center;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            width: 110px;
          "
        >
          {{ detail?.name }}
        </p>
        <p style="color: var(--accent-color-1); text-align: center">
          {{ formatDuration(duration) }}
        </p>
      </div>
    </div>
  </el-card>
</template>

<style scoped></style>
