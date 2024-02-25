<script setup lang="ts">
defineProps<{
  icon: string,
  name: string,
  duration: string
}>()
</script>

<template>
  <div class="app-basic-card row">
    <img :src="icon" alt="app icon">
    <div class="container text-container">
      <div class="app-basic-card-name">{{ name }}</div>
      <div class="app-basic-card-duration">{{ duration }}</div>
    </div>
  </div>
</template>

<style scoped>
.app-basic-card {
  border-radius: 8px;
  padding: 16px;
  width: fit-content;
  background: var(--card-bg-color);
  gap: 16px;
  color: var(--font-color);
}

.text-container {
  text-align: left;
  justify-content: space-evenly;
  line-height: 1;
}

.app-basic-card-name {
  user-select: none;
  color: var(--font-color-bold);
}

.app-basic-card-duration{
  user-select: none;
  font-size: 12px;
}
</style>