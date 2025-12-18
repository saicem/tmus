<script lang="ts" setup>
import { open } from "@tauri-apps/plugin-dialog"
import { Folder } from "@element-plus/icons-vue"

const model = defineModel<string | null>()

defineProps<{
  name: string
  change?: (value: string | number) => void
}>()

const selectPath = async () => {
  let result = await open({
    multiple: false,
    directory: true,
  })
  if (result) {
    model.value = result
  }
}
</script>
<template>
  <div class="input-group">
    <el-input
      v-model="model"
      :name="name"
      input-style="border-radius: 0"
      style="border-radius: 0; outline: 0"
      type="text"
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
