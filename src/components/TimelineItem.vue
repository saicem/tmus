<script setup lang="ts">
import { FileDetail } from "@/global/data.ts"
import { appDetail } from "@/global/api.ts"
import { ref } from "vue"

import { Duration } from "moment-timezone"

const props = defineProps<{
  appId: number
  duration: Duration
}>()
const detail = ref<FileDetail | null>(null)
appDetail(props.appId).then((res) => (detail.value = res))
</script>

<template>
  <el-card style="height: 100%">
    <img :src="detail?.icon" alt="" />
    <h4>
      {{ detail?.name }}
    </h4>
    <p>{{ props.duration.humanize() }}</p>
  </el-card>
</template>

<style scoped></style>
