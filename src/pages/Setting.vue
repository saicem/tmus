<script lang="ts" setup>
import { onMounted, ref } from "vue"
import { i18n } from "@/script/i18n.ts"
import { configStore, passiveStore } from "@/script/state.ts"
import { getTmusMeta } from "@/script/cmd.ts"
import { disable, enable } from "@tauri-apps/plugin-autostart"
import { AppMeta } from "@/script/models.ts"

const dialogVisibleRule = ref(false)
const tmusMeta = ref<AppMeta>()

onMounted(async () => {
  tmusMeta.value = await getTmusMeta()
})
</script>

<template>
  <rule-dialog v-if="dialogVisibleRule" v-model="dialogVisibleRule" />
  <div class="root">
    <div class="column">
      <setting-group>
        <setting-item :label="i18n.configPage.language">
          <el-select v-model="configStore.lang" style="width: 100px">
            <el-option label="English" value="en" />
            <el-option label="简体中文" value="zh" />
            <el-option :label="i18n.configPage.langSystem" value="system" />
          </el-select>
        </setting-item>
        <setting-item :label="i18n.configPage.theme">
          <el-select v-model="configStore.theme" style="width: 100px">
            <el-option :label="i18n.configPage.themeSystem" value="system" />
            <el-option :label="i18n.configPage.themeLight" value="light" />
            <el-option :label="i18n.configPage.themeDark" value="dark" />
          </el-select>
        </setting-item>
        <setting-item :label="i18n.configPage.firstDayOfWeek">
          <el-select v-model="configStore.firstDayOfWeek" style="width: 100px">
            <el-option :label="i18n.configPage.monday" :value="0" />
            <el-option :label="i18n.configPage.tuesday" :value="1" />
            <el-option :label="i18n.configPage.wednesday" :value="2" />
            <el-option :label="i18n.configPage.thursday" :value="3" />
            <el-option :label="i18n.configPage.friday" :value="4" />
            <el-option :label="i18n.configPage.saturday" :value="5" />
            <el-option :label="i18n.configPage.sunday" :value="6" />
          </el-select>
        </setting-item>
        <setting-item :label="i18n.configPage.dateFormat">
          <el-select v-model="configStore.dateFormat" style="width: 100px">
            <el-option label="2025-07-05" value="yyyy-MM-dd" />
            <el-option label="2025/07/05" value="yyyy/MM/dd" />
          </el-select>
        </setting-item>
        <setting-item :label="i18n.configPage.timeFormat">
          <el-select v-model="configStore.timeFormat" style="width: 100px">
            <el-option label="9:40 / 14:40" value="H:mm:ss" />
            <el-option label="09:40 / 14:40" value="HH:mm:ss" />
          </el-select>
        </setting-item>
      </setting-group>
      <setting-group>
        <setting-item :label="i18n.configPage.autoStart">
          <el-switch
            v-model="passiveStore.autoStart"
            @change="(val) => (val ? enable() : disable())"
          />
        </setting-item>
        <setting-item :label="i18n.configPage.autoStartMcpServer">
          <el-switch v-model="configStore.autoStartMcpServer" />
        </setting-item>
        <mcp-setting-item />
      </setting-group>
    </div>
    <div class="column">
      <setting-group>
        <setting-item
          :label="i18n.configPage.appRule"
          :tip="i18n.configPage.appRuleTip"
          @click="dialogVisibleRule = true"
        />
        <setting-item :label="i18n.configPage.filterUninstalledApp">
          <el-switch v-model="configStore.filterUninstalledApp" />
        </setting-item>
      </setting-group>
      <setting-group>
        <setting-item :label="i18n.configPage.autoCheckUpdate">
          <el-switch v-model="configStore.autoCheckUpdate" />
        </setting-item>
        <update-setting-item />
        <setting-item :label="i18n.configPage.version">
          <el-text> {{ tmusMeta?.tmusVersion }}</el-text>
        </setting-item>
      </setting-group>
    </div>
  </div>
</template>
<style scoped>
.root {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
}

.column {
  display: flex;
  flex-direction: column;
  flex: 1 0;
  min-width: 320px;
  gap: 16px;
}
</style>
