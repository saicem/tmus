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
    <el-descriptions :title="detail?.name" border>
      <el-descriptions-item :rowspan="2" label="Icon" align="center">
        <el-image :src="detail?.icon" />
      </el-descriptions-item>
      <el-descriptions-item label="Id">
        {{ detail?.id }}
      </el-descriptions-item>
      <el-descriptions-item label="Name">
        {{ detail?.name }}
      </el-descriptions-item>
      <el-descriptions-item label="Exist">
        {{ detail?.exist }}
      </el-descriptions-item>
      <el-descriptions-item label="Path">
        <el-link> {{ detail?.path }}</el-link>
      </el-descriptions-item>
      <el-descriptions-item label="ProductName">
        {{ detail?.version?.productName }}
      </el-descriptions-item>
      <el-descriptions-item label="FileDescription">
        {{ detail?.version?.fileDescription }}
      </el-descriptions-item>
      <el-descriptions-item label="CompanyName">
        {{ detail?.version?.companyName }}
      </el-descriptions-item>
    </el-descriptions>
  </el-card>
</template>

<style scoped></style>
