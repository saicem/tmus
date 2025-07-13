<script lang="ts" setup>
import { installUpdate } from "@/script/cmd.ts"
import { i18n } from "@/script/i18n.ts"
import { DownloadEvent } from "@/script/models.ts"
import { updateDialogStore } from "@/script/state.ts"

const downloading = ref(false)
const contentLength = ref(1)
const downloadedLength = ref(0)

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
        updateDialogStore.show = false
        updateDialogStore.meta = null
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
  <el-dialog
    v-model="updateDialogStore.show"
    :title="i18n.configPage.versionUpdateTitle"
    width="500"
  >
    <span>{{
      i18n.configPage.updateAvailable(
        updateDialogStore.meta?.version ?? "",
        updateDialogStore.meta?.currentVersion ?? ""
      )
    }}</span>
    <el-progress
      v-if="downloading"
      :percentage="Math.round((downloadedLength / contentLength) * 100)"
    >
    </el-progress>
    <template #footer>
      <div v-if="!downloading" class="dialog-footer">
        <el-button @click="updateDialogStore.show = false"
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
