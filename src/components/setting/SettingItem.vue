<script setup lang="ts">
import { InfoFilled } from "@element-plus/icons-vue"

defineProps<{
  label: string
  tip?: string
}>()
</script>

<template>
  <div
    style="display: flex; flex-direction: row; width: 100%; padding: 12px 12px"
  >
    <span style="flex: 1 1 auto"
      >{{ label }}
      <el-tooltip v-if="tip" :content="tip">
        <el-icon style="vertical-align: middle">
          <InfoFilled />
        </el-icon>
      </el-tooltip>
    </span>

    <slot name="default" />
  </div>
</template>

<style scoped></style>
