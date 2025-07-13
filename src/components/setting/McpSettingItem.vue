<script lang="ts" setup>
import { i18n } from "@/script/i18n.ts"
import { configStore } from "@/script/state.ts"
import {
  getMcpServerStatus,
  startMcpServer,
  stopMcpServer,
} from "@/script/cmd.ts"

const dialogVisible = ref(false)
const running = ref(false)
const processing = ref(false)

onMounted(async () => {
  const status = await getMcpServerStatus()
  running.value = status.running
  if (status.running && status.port) {
    configStore.mcpServerPort = status.port
  }
})

async function toggleMcpServer() {
  if (processing.value) {
    return
  }
  processing.value = true
  try {
    if (running.value) {
      await stopMcpServer()
      running.value = false
    } else {
      await startMcpServer(configStore.mcpServerPort)
      running.value = true
    }
  } finally {
    processing.value = false
  }
}
</script>

<template>
  <setting-item
    :label="i18n.configPage.mcpServer"
    @click="dialogVisible = true"
  />
  <el-dialog
    v-model="dialogVisible"
    :title="i18n.configPage.mcpServer"
    width="500"
  >
    <el-form label-width="auto" style="max-width: 600px">
      <el-form-item :label="i18n.configPage.usePort">
        <el-input-number
          :disabled="running"
          v-model="configStore.mcpServerPort"
        />
      </el-form-item>
      <el-form-item label="SSE URL">{{ "http://127.0.0.1:" + configStore.mcpServerPort + "/sse" }}</el-form-item>
      <el-form-item>
        <el-button :loading="processing" @click="toggleMcpServer">
          {{ running ? i18n.common.stop : i18n.common.start }}
        </el-button>
      </el-form-item>
    </el-form>
  </el-dialog>
</template>

<style scoped></style>
