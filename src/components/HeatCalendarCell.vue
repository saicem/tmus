<script lang="ts" setup>
import { formatDuration, MILLISECONDS_PER_HOUR } from "@/script/time-util.ts"
import { config } from "@/script/state.ts"
import { format, setDayOfYear } from "date-fns"

defineProps<{
  dayOfYear: number
  duration: number
}>()
</script>

<template>
  <el-tooltip :hide-after="200" :show-after="200">
    <template #content>
      <div style="text-align: center">
        {{
          `${format(setDayOfYear(new Date(), dayOfYear), config.dateFormat)} `
        }}
        <br />{{ formatDuration(duration) }}
      </div>
    </template>
    <div
      :data-tag="Math.min(4, Math.ceil(duration / MILLISECONDS_PER_HOUR / 4))"
      class="block-unit"
    ></div>
  </el-tooltip>
</template>

<style scoped>
.block-unit {
  height: 10px;
  width: 10px;
  border-radius: 2px;
}

.block-unit[data-tag="4"] {
  background: var(--el-color-primary-dark-2);
}

.block-unit[data-tag="3"] {
  background: var(--el-color-primary-light-3);
}

.block-unit[data-tag="2"] {
  background: var(--el-color-primary-light-5);
}

.block-unit[data-tag="1"] {
  background: var(--el-color-primary-light-7);
}

.block-unit[data-tag="0"] {
  background: var(--el-color-primary-light-9);
}

.block-unit:hover {
  box-shadow: 0 0 5px rgb(57, 120, 255);
}
</style>
