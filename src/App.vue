<script setup lang="ts">
import TitleBar from "@/components/TitleBar.vue"
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
              <el-menu-item index="1"> Home</el-menu-item>
            </RouterLink>
            <RouterLink to="/timeline">
              <el-menu-item index="2">Timeline</el-menu-item>
            </RouterLink>
            <RouterLink to="/tags">
              <el-menu-item index="3"> Tags</el-menu-item>
            </RouterLink>
            <RouterLink to="/statistic">
              <el-menu-item index="4"> Statistic</el-menu-item>
            </RouterLink>
            <RouterLink to="/setting">
              <el-menu-item index="5"> Setting</el-menu-item>
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
