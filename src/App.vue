<script setup lang="ts">
import TitleBar from "@/components/TitleBar.vue"
import TabBar from "@/components/TabBar.vue"
import HomePage from "@/components/PageHome.vue"
import { themeStore } from "@/global/state.ts"
</script>

<template>
  <div class="container" :class="{ dark: themeStore.isDark() }">
    <TitleBar class="title-bar" />
    <TabBar class="tab-bar" />
    <div class="frame">
      <HomePage />
    </div>
  </div>
</template>

<style scoped>
.title-bar {
  background-color: var(--base-color-2);
  position: fixed;
  border-bottom: 1px solid var(--base-color-3);
}

.tab-bar {
  margin-top: 30px;
  position: fixed;
  height: 100%;
  background-color: var(--base-color-1);
}

.frame {
  display: flex;
  margin-top: 30px;
  margin-left: 64px;
  background-color: var(--base-color-1);
}
</style>
