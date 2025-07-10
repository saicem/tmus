<script lang="ts" setup>
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
            <router-link to="/">
              <el-menu-item index="1">
                {{ i18n.navigateMenu.home }}
              </el-menu-item>
            </router-link>
            <router-link to="/timeline">
              <el-menu-item index="2">
                {{ i18n.navigateMenu.timeline }}
              </el-menu-item>
            </router-link>
            <router-link to="/statistic">
              <el-menu-item index="3">
                {{ i18n.navigateMenu.statistic }}
              </el-menu-item>
            </router-link>
            <router-link to="/application">
              <el-menu-item index="4">
                {{ i18n.navigateMenu.application }}
              </el-menu-item>
            </router-link>
            <router-link to="/setting">
              <el-menu-item index="5">
                {{ i18n.navigateMenu.setting }}
              </el-menu-item>
            </router-link>
          </el-menu>
        </el-scrollbar>
      </el-aside>

      <el-main style="height: calc(100vh - 76px); padding: 0">
        <el-scrollbar style="padding: 0 20px">
          <router-view />
        </el-scrollbar>
      </el-main>
    </el-container>
  </el-container>
</template>

<style scoped></style>
