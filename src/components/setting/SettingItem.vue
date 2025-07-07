<script setup lang="ts">
import { InfoFilled, Loading } from "@element-plus/icons-vue"
import { useSlots } from "vue"

defineProps<{
  label: string
  tip?: string
  loading?: boolean
}>()

const slots = useSlots()
</script>

<template>
  <div
    :class="{ root: !slots.default }"
    style="
      display: flex;
      flex-direction: row;
      width: 100%;
      height: 48px;
      padding: 12px 12px;
    "
  >
    <span style="flex: 1 1 auto"
      >{{ label }}
      <el-tooltip v-if="tip" :content="tip">
        <el-icon style="vertical-align: middle">
          <InfoFilled />
        </el-icon>
      </el-tooltip>
    </span>
    <slot name="default">
      <div v-show="!loading" style="width: 1.5em; height: 1.5em">
        <svg viewBox="0 0 24 24">
          <path
            d="M9.29 6.71c-.39.39-.39 1.02 0 1.41L13.17 12l-3.88 3.88c-.39.39-.39 1.02 0 1.41s1.02.39 1.41 0l4.59-4.59c.39-.39.39-1.02 0-1.41L10.7 6.7c-.38-.38-1.02-.38-1.41.01"
          ></path>
        </svg>
      </div>
      <el-icon v-show="loading" class="is-loading">
        <Loading />
      </el-icon>
    </slot>
  </div>
</template>

<style scoped>
.root:hover {
  background-color: var(--el-color-primary-light-9);
}
</style>
