<script lang="ts" setup>
import { fetchUpdate } from "@/script/cmd.ts"
import { i18n } from "@/script/i18n.ts"
import { ElNotification } from "element-plus"
import { updateDialogStore } from "@/script/state.ts"

const checkUpdateLoading = ref(false)

async function checkNeedUpdate() {
  if (updateDialogStore.meta != null) {
    updateDialogStore.show = true
    return
  }
  if (checkUpdateLoading.value) {
    return
  }
  checkUpdateLoading.value = true
  const updateMeta = await fetchUpdate()
  if (updateMeta) {
    updateDialogStore.meta = updateMeta
    updateDialogStore.show = true
  } else {
    ElNotification({
      message: i18n.value.configPage.currentVersionIsAlreadyTheLatestVersion,
      type: "success",
    })
  }
  checkUpdateLoading.value = false
}
</script>

<template>
  <setting-item
    :label="i18n.configPage.checkUpdate"
    :loading="checkUpdateLoading"
    @click="checkNeedUpdate"
  />
</template>

<style scoped></style>
