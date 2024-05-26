<script setup lang="ts">
import TitleBar from "@/components/TitleBar.vue"
import TabBar from "@/components/TabBar.vue"
import HomePage from "@/components/PageHome.vue"
import { themeStore } from "@/global/state.ts"
</script>

<template>
  <div
    class="container"
    :class="{ dark: themeStore.isDark() }"
    style="background-color: var(--app-bg-color)"
  >
    <TitleBar />
    <div class="frame">
      <TabBar />
      <HomePage />
    </div>
  </div>
</template>

<style scoped>
.frame {
  display: flex;
}
</style>
