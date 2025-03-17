<script setup lang="ts">
import { i18n } from "@/global/i18n.ts"
import { config } from "@/global/state.ts"
import { setAppConfig } from "@/global/cmd.ts"
import { ref } from "vue"
import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart"
import SettingItem from "@/components/setting/SettingItem.vue"
import SettingGroup from "@/components/setting/SettingGroup.vue"

function configChange() {
  setAppConfig(config.value)
}

const autoStart = ref<boolean>(false)

isEnabled().then((value) => (autoStart.value = value))
</script>

<template>
  <div style="flex: 1; display: flex; flex-wrap: wrap; padding: 16px">
    <SettingGroup>
      <SettingItem :label="i18n.configPage.language">
        <el-select
          v-model="config.lang"
          @change="configChange"
          style="width: 100px"
        >
          <el-option label="English" value="en" />
          <el-option label="简体中文" value="zh" />
          <el-option :label="i18n.configPage.langSystem" value="system" />
        </el-select>
      </SettingItem>
      <SettingItem :label="i18n.configPage.theme">
        <el-select
          v-model="config.theme"
          @change="configChange"
          style="width: 100px"
        >
          <el-option :label="i18n.configPage.themeSystem" value="system" />
          <el-option :label="i18n.configPage.themeLight" value="light" />
          <el-option :label="i18n.configPage.themeDark" value="dark" />
        </el-select>
      </SettingItem>
      <SettingItem :label="i18n.configPage.autoStart">
        <el-switch
          v-model="autoStart"
          @change="(val: boolean) => (val ? enable() : disable())"
        />
      </SettingItem>
    </SettingGroup>
  </div>
</template>
<style scoped></style>
