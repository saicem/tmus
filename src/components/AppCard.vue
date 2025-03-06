<script setup lang="ts">
import { FileDetail } from "@/global/data.ts"
import { Duration } from "moment-timezone"
import { formatDuration } from "@/global/time-util.ts"

defineProps<{
  duration: Duration
  detail: FileDetail
}>()
</script>

<template>
  <el-card style="height: 100%">
    <img :src="detail?.icon" alt="" />
    <h4>
      {{ detail?.name }}
    </h4>
    <p>{{ formatDuration(duration) }}</p>
  </el-card>
</template>

<style scoped></style>
