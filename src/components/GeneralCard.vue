<script setup lang="ts">
const props = defineProps<{
  icon: string
  value: string
  unit: string
  illustration: string
}>()
</script>

<template>
  <div class="card container root">
    <div class="icon">
      <img :src="icon" alt="icon" />
    </div>
    <div class="container" style="align-items: start">
      <div class="vice-info">{{ props.illustration }}</div>
      <div>
        <span class="value">{{ props.value }}</span>
        <span style="user-select: none">{{ props.unit }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.root {
  flex-direction: row;
  justify-content: start;
  padding: 16px 24px;
  color: var(--font-color);
  width: 100%;
  gap: 16px;
  background: var(--card-bg-color);
}

.value {
  color: var(--font-color-bold);
  font-size: x-large;
  margin-right: 6px;
  user-select: none;
}

.vice-info {
  margin-bottom: 4px;
  user-select: none;
}
</style>
