<script lang="ts" setup>
import TitleBar from "@/components/TitleBar.vue"
import { i18n } from "@/script/i18n.ts"
</script>

<template>
  <el-container>
    <el-header>
      <TitleBar />
    </el-header>
    <el-container>
      <el-aside width="200px">
        <el-scrollbar>
          <el-menu default-active="'1'">
            <RouterLink to="/">
              <el-menu-item index="1">
                {{ i18n.navigateMenu.home }}
              </el-menu-item>
            </RouterLink>
            <RouterLink to="/timeline">
              <el-menu-item index="2">
                {{ i18n.navigateMenu.timeline }}
              </el-menu-item>
            </RouterLink>
            <RouterLink to="/statistic">
              <el-menu-item index="3">
                {{ i18n.navigateMenu.statistic }}
              </el-menu-item>
            </RouterLink>
            <RouterLink to="/application">
              <el-menu-item index="4">
                {{ i18n.navigateMenu.application }}
              </el-menu-item>
            </RouterLink>
            <RouterLink to="/setting">
              <el-menu-item index="5">
                {{ i18n.navigateMenu.setting }}
              </el-menu-item>
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
