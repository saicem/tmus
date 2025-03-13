<script setup lang="ts">
defineProps<{
  label: string
}>()
</script>

<template>
  <div style="display: flex; flex-direction: row; width: 100%; margin: 8px">
    <span style="flex: 1 1 auto">{{ label }}</span>
    <slot name="default" />
  </div>
</template>

<style scoped></style>
