<script setup lang="ts">

import TabItem from "@/components/MenuItem.vue"
</script>

<template>
  <div class="tab-bar">
    <TabItem text="😊" tag="home"/>
    <TabItem text="😁" tag="history"/>
    <TabItem text="👍" tag="category"/>
  </div>
</template>

<style scoped>
.tab-bar {
  padding: 8px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
</style>