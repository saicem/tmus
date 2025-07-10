<script lang="ts" setup>
import { fetchUpdate, installUpdate } from "@/script/cmd.ts"
import { i18n } from "@/script/i18n.ts"
import { DownloadEvent, UpdateMetadata } from "@/script/models.ts"
import { ElNotification } from "element-plus"

const checkUpdateLoading = ref(false)
const dialogVisible = ref(false)
const updateMeta = ref<UpdateMetadata | null>(null)
const downloading = ref(false)
const contentLength = ref(1)
const downloadedLength = ref(0)

async function checkNeedUpdate() {
  if (updateMeta.value != null) {
    dialogVisible.value = true
    return
  }
  if (checkUpdateLoading.value) {
    return
  }
  checkUpdateLoading.value = true
  updateMeta.value = await fetchUpdate()
  if (updateMeta.value) {
    dialogVisible.value = true
  } else {
    ElNotification({
      message: i18n.value.configPage.currentVersionIsAlreadyTheLatestVersion,
      type: "success",
    })
  }
  checkUpdateLoading.value = false
}

async function confirmUpdate() {
  if (downloading.value) {
    return
  }
  downloading.value = true
  try {
    await installUpdate((event: DownloadEvent) => {
      if (event.event === "Started") {
        contentLength.value = event.data.contentLength ?? 1
        downloadedLength.value = 0
      } else if (event.event === "Progress") {
        downloadedLength.value += event.data.chunkLength ?? 0
      } else if (event.event === "Finished") {
        downloading.value = false
        dialogVisible.value = false
        updateMeta.value = null
      }
      console.log(
        event,
        (event.data.chunkLength ?? 0) / (event.data.contentLength ?? 1)
      )
    })
  } catch {
    downloading.value = false
  }
}
</script>

<template>
  <SettingItem
    :label="i18n.configPage.checkUpdate"
    :loading="checkUpdateLoading"
    @click="checkNeedUpdate"
  />
  <el-dialog
    v-model="dialogVisible"
    :title="i18n.configPage.versionUpdateTitle"
    width="500"
  >
    <span>{{
      i18n.configPage.updateAvailable(
        updateMeta?.version ?? "",
        updateMeta?.currentVersion ?? ""
      )
    }}</span>
    <el-progress
      v-if="downloading"
      :percentage="Math.round((downloadedLength / contentLength) * 100)"
    >
    </el-progress>
    <template #footer>
      <div v-if="!downloading" class="dialog-footer">
        <el-button @click="dialogVisible = false"
          >{{ i18n.common.cancel }}
        </el-button>
        <el-button type="primary" @click="confirmUpdate">
          {{ i18n.common.ok }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<style scoped></style>
