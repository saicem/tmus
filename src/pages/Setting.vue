<script setup lang="ts">
import { ref } from "vue"
import { i18n } from "@/script/i18n.ts"
import { autoStart, config } from "@/script/state.ts"
import { setAppConfig } from "@/script/cmd.ts"
import { disable, enable } from "@tauri-apps/plugin-autostart"
import SettingItem from "@/components/setting/SettingItem.vue"
import SettingGroup from "@/components/setting/SettingGroup.vue"
import SettingMoreItem from "@/components/setting/SettingMoreItem.vue"
import RuleDialog from "@/components/setting/RuleDialog.vue"
import TagDialog from "@/components/setting/TagDialog.vue"

const dialogVisibleRule = ref(false)
const dialogVisibleTag = ref(false)

function configChange() {
  setAppConfig(config.value)
}
</script>

<template>
  <div
    style="
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
      grid-gap: 16px;
    "
  >
    <RuleDialog v-model="dialogVisibleRule" v-if="dialogVisibleRule" />
    <TagDialog v-model="dialogVisibleTag" v-if="dialogVisibleTag" />
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
      <SettingItem :label="i18n.configPage.firstDayOfWeek">
        <el-select
          v-model="config.firstDayOfWeek"
          @change="configChange"
          style="width: 100px"
        >
          <el-option :label="i18n.configPage.monday" :value="0" />
          <el-option :label="i18n.configPage.tuesday" :value="1" />
          <el-option :label="i18n.configPage.wednesday" :value="2" />
          <el-option :label="i18n.configPage.thursday" :value="3" />
          <el-option :label="i18n.configPage.friday" :value="4" />
          <el-option :label="i18n.configPage.saturday" :value="5" />
          <el-option :label="i18n.configPage.sunday" :value="6" />
        </el-select>
      </SettingItem>
      <SettingItem :label="i18n.configPage.dateFormat">
        <el-select
          v-model="config.dateFormat"
          @change="configChange"
          style="width: 100px"
        >
          <el-option label="2025-07-05" value="YYYY-MM-DD" />
          <el-option label="2025/07/05" value="YYYY/MM/DD" />
        </el-select>
      </SettingItem>
      <SettingItem :label="i18n.configPage.timeFormat">
        <el-select
          v-model="config.timeFormat"
          @change="configChange"
          style="width: 100px"
        >
          <el-option label="9:40 / 14:40" value="H:mm:ss" />
          <el-option label="09:40 / 14:40" value="HH:mm:ss" />
        </el-select>
      </SettingItem>
    </SettingGroup>
    <SettingGroup>
      <SettingMoreItem
        :label="i18n.configPage.appRule"
        @click="dialogVisibleRule = true"
        :tip="i18n.configPage.appRuleTip"
      />
      <!--      <SettingMoreItem-->
      <!--        :label="i18n.configPage.appTag"-->
      <!--        @click="dialogVisibleTag = true"-->
      <!--      />-->
      <SettingItem :label="i18n.configPage.filterUninstalledApp">
        <el-switch v-model="config.filterUninstalledApp" />
      </SettingItem>
      <SettingItem :label="i18n.configPage.autoStart">
        <el-switch
          v-model="autoStart"
          @change="(val) => (val ? enable() : disable())"
        />
      </SettingItem>
    </SettingGroup>
  </div>
</template>
<style scoped></style>
