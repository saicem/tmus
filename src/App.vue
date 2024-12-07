<script setup lang="ts">
import TitleBar from "@/components/TitleBar.vue"
import { computed } from "vue"
import { messages } from "@/global/i18n.ts"
import { languageStore } from "@/global/state.ts"

const msg = computed(() => messages[languageStore.language].tab)
</script>

<template>
  <el-container>
    <el-header>
      <TitleBar />
    </el-header>
    <el-container>
      <el-aside>
        <el-scrollbar>
          <el-menu default-active="'1'">
            <RouterLink to="/">
              <el-menu-item index="1">{{ msg.home }}</el-menu-item>
            </RouterLink>
            <RouterLink to="/timeline">
              <el-menu-item index="2">{{ msg.timeline }}</el-menu-item>
            </RouterLink>
            <RouterLink to="/statistic">
              <el-menu-item index="3">{{ msg.statistic }}</el-menu-item>
            </RouterLink>
            <RouterLink to="/setting">
              <el-menu-item index="4">{{ msg.setting }}</el-menu-item>
            </RouterLink>
          </el-menu>
        </el-scrollbar>
      </el-aside>

      <el-main style="height: calc(100vh - 76px); padding: 0">
        <el-scrollbar style="padding: 0 20px">
          <RouterView />
        </el-scrollbar>
      </el-main>
    </el-container>
  </el-container>
</template>

<style scoped></style>
