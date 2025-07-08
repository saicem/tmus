<script lang="ts" setup>
import { onMounted, ref } from "vue"
import { i18n } from "@/script/i18n.ts"
import { autoStart, config } from "@/script/state.ts"
import { getTmusMeta, setAppConfig } from "@/script/cmd.ts"
import { disable, enable } from "@tauri-apps/plugin-autostart"
import SettingItem from "@/components/setting/SettingItem.vue"
import SettingGroup from "@/components/setting/SettingGroup.vue"
import RuleDialog from "@/components/setting/RuleDialog.vue"
import TagDialog from "@/components/setting/TagDialog.vue"
import { AppMeta } from "@/script/models.ts"
import UpdateSettingItem from "@/components/setting/UpdateSettingItem.vue"

const dialogVisibleRule = ref(false)
const dialogVisibleTag = ref(false)
const tmusMeta = ref<AppMeta>()

onMounted(async () => {
  tmusMeta.value = await getTmusMeta()
})

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
    <RuleDialog v-if="dialogVisibleRule" v-model="dialogVisibleRule" />
    <TagDialog v-if="dialogVisibleTag" v-model="dialogVisibleTag" />
    <SettingGroup>
      <SettingItem :label="i18n.configPage.language">
        <el-select
          v-model="config.lang"
          style="width: 100px"
          @change="configChange"
        >
          <el-option label="English" value="en" />
          <el-option label="简体中文" value="zh" />
          <el-option :label="i18n.configPage.langSystem" value="system" />
        </el-select>
      </SettingItem>
      <SettingItem :label="i18n.configPage.theme">
        <el-select
          v-model="config.theme"
          style="width: 100px"
          @change="configChange"
        >
          <el-option :label="i18n.configPage.themeSystem" value="system" />
          <el-option :label="i18n.configPage.themeLight" value="light" />
          <el-option :label="i18n.configPage.themeDark" value="dark" />
        </el-select>
      </SettingItem>
      <SettingItem :label="i18n.configPage.firstDayOfWeek">
        <el-select
          v-model="config.firstDayOfWeek"
          style="width: 100px"
          @change="configChange"
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
          style="width: 100px"
          @change="configChange"
        >
          <el-option label="2025-07-05" value="YYYY-MM-DD" />
          <el-option label="2025/07/05" value="YYYY/MM/DD" />
        </el-select>
      </SettingItem>
      <SettingItem :label="i18n.configPage.timeFormat">
        <el-select
          v-model="config.timeFormat"
          style="width: 100px"
          @change="configChange"
        >
          <el-option label="9:40 / 14:40" value="H:mm:ss" />
          <el-option label="09:40 / 14:40" value="HH:mm:ss" />
        </el-select>
      </SettingItem>
    </SettingGroup>
    <SettingGroup>
      <SettingItem
        :label="i18n.configPage.appRule"
        :tip="i18n.configPage.appRuleTip"
        @click="dialogVisibleRule = true"
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
      <UpdateSettingItem />
      <SettingItem :label="i18n.configPage.version">
        <el-text> {{ tmusMeta?.tmusVersion }}</el-text>
      </SettingItem>
    </SettingGroup>
  </div>
</template>
<style scoped></style>
