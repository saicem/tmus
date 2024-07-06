<script setup lang="ts">
import TitleBar from "@/components/TitleBar.vue"
</script>

<template>
  <el-container>
    <el-header> <TitleBar /> </el-header>
    <el-main><RouterView /></el-main>
  </el-container>
</template>

<style scoped></style>
