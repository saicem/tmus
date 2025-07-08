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
        height: 130px;
        width: 130px;
        flex: auto;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 8px;
      "
    >
      <div>
        <el-image :src="detail?.icon ?? ''" style="width: 64px; height: 64px" />
      </div>
      <p
        style="
          overflow: hidden;
          white-space: nowrap;
          text-overflow: ellipsis;
          width: 100%;
          font-weight: bold;
          text-align: center;
        "
      >
        {{ detail?.name }}
      </p>
      <p style="color: var(--accent-color-1)">{{ formatDuration(duration) }}</p>
    </div>
  </el-card>
</template>

<style scoped></style>
