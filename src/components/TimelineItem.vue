<script setup lang="ts">
import { FileDetail, FocusData } from "@/global/data.ts"
import { appDetail } from "@/global/api.ts"
import { ref } from "vue"

const props = defineProps<{
  data: FocusData
}>()
const detail = ref<FileDetail | null>(null)
appDetail(props.data.id).then((res) => (detail.value = res))
</script>

<template>
  <el-card>
    <img :src="detail?.icon" alt="" />
    <h4>
      {{ detail?.name }}
    </h4>
    <p>{{ props.data.duration.toISOString() }}</p>
  </el-card>
</template>

<style scoped></style>
