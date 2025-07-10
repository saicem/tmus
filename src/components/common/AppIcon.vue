<script lang="ts" setup>
import { Box } from "@element-plus/icons-vue"

defineProps({
  icon: {
    type: String,
    default: null,
  },
  size: {
    type: Number,
    default: 32,
  },
})
</script>

<template>
  <div :style="{ height: size + 'px', width: size + 'px' }">
    <el-image v-if="icon" :src="icon" class="icon" />
    <Box v-if="!icon" class="icon" />
  </div>
</template>

<style scoped>
.icon {
  width: 100%;
  height: 100%;
}
</style>
