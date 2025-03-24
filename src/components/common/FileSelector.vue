<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog"
import { Folder } from "@element-plus/icons-vue"

const model = defineModel<string | null>()

defineProps<{
  name: string
  change?: (value: string | number) => void
}>()

const selectPath = async () => {
  model.value = await open({
    multiple: false,
    directory: true,
  })
}
</script>
<template>
  <div class="input-group">
    <el-input
      input-style="border-radius: 0"
      style="border-radius: 0; outline: 0"
      type="text"
      :name="name"
      v-model="model"
      @change="
        (value) => {
          change?.(value)
        }
      "
    >
      <template #append>
        <el-button @click="selectPath">
          <el-icon>
            <Folder />
          </el-icon>
        </el-button>
      </template>
    </el-input>
  </div>
</template>

<style scoped></style>
