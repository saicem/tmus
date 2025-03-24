<script setup lang="ts">
import SettingItem from "@/components/setting/SettingItem.vue"

defineProps<{
  label: string
  tip?: string
}>()
</script>

<template>
  <SettingItem class="root" :label="label" :tip="tip">
    <div style="width: 1.5em; height: 1.5em">
      <svg viewBox="0 0 24 24">
        <path
          d="M9.29 6.71c-.39.39-.39 1.02 0 1.41L13.17 12l-3.88 3.88c-.39.39-.39 1.02 0 1.41s1.02.39 1.41 0l4.59-4.59c.39-.39.39-1.02 0-1.41L10.7 6.7c-.38-.38-1.02-.38-1.41.01"
        ></path>
      </svg>
    </div>
  </SettingItem>
</template>

<style scoped>
.root:hover {
  background-color: var(--el-color-primary-light-9);
}
</style>
