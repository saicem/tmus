<script lang="ts" setup>
import { i18n } from "@/script/i18n.ts"
import zhCn from "element-plus/es/locale/lang/zh-cn"
import en from "element-plus/es/locale/lang/en"
import { configStore, passiveStore, updateDialogStore } from "@/script/state.ts"
import { fetchUpdate } from "@/script/cmd.ts"

onMounted(async () => {
  if (configStore.autoCheckUpdate) {
    let update = await fetchUpdate()
    if (update != null) {
      updateDialogStore.meta = update
      updateDialogStore.show = true
    }
  }
})
</script>

<template>
  <update-dialog />
  <el-config-provider :locale="passiveStore.lang === 'en' ? en : zhCn">
    <el-container :class="passiveStore.theme">
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
  </el-config-provider>
</template>

<style scoped></style>
