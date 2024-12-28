<script setup lang="ts">
import { FileDetail } from "@/global/data.ts"
import { appDetail } from "@/global/api.ts"
import { ref } from "vue"

const props = defineProps<{
  id: number
}>()
const detail = ref<FileDetail | null>(null)
appDetail(props.id).then((res) => (detail.value = res))
</script>

<template>
  <el-card>
    <img :src="detail?.icon" alt="" />
    <h4>
      {{ detail?.name }}
    </h4>
  </el-card>
</template>

<style scoped></style>
