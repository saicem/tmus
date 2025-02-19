<script setup lang="ts">
import { i18n } from "@/global/i18n.ts"
import { config } from "@/global/state.ts"
import { setAppConfig } from "@/global/cmd.ts"

function configChange() {
  setAppConfig(config.value)
}
</script>

<template>
  <div style="display: flex; flex-direction: column; row-gap: 16px">
    <el-select
      v-model="config.lang"
      :placeholder="i18n.configPage.language"
      @change="configChange"
      style="width: 240px"
    >
      <el-option label="English" value="en" />
      <el-option label="简体中文" value="zh" />
    </el-select>
    <el-select
      v-model="config.theme"
      :placeholder="i18n.configPage.theme"
      @change="configChange"
      style="width: 240px"
    >
      <el-option :label="i18n.configPage.themeSystem" value="system" />
      <el-option :label="i18n.configPage.themeLight" value="light" />
      <el-option :label="i18n.configPage.themeDark" value="dark" />
    </el-select>
  </div>
</template>

<style scoped></style>
