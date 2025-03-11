<script setup lang="ts">
import { FileDetail } from "@/global/data.ts"
import { requestAppDetail } from "@/global/api.ts"
import { ref } from "vue"
import { showInFolder } from "@/global/cmd.ts"
import { i18n } from "@/global/i18n.ts"

const props = defineProps<{
  id: number
}>()
const detail = ref<FileDetail | null>(null)
requestAppDetail(props.id).then((res) => (detail.value = res))
</script>

<template>
  <el-card>
    <el-descriptions :title="detail?.name" border>
      <el-descriptions-item
        :rowspan="2"
        :label="i18n.detailPage.icon"
        align="center"
      >
        <el-image :src="detail?.icon" />
      </el-descriptions-item>
      <el-descriptions-item label="ID">
        {{ detail?.id }}
      </el-descriptions-item>
      <el-descriptions-item :label="i18n.detailPage.name">
        {{ detail?.name }}
      </el-descriptions-item>
      <el-descriptions-item :label="i18n.detailPage.exist">
        {{ detail?.exist }}
      </el-descriptions-item>
      <el-descriptions-item :label="i18n.detailPage.filePath">
        <el-link @click="showInFolder(detail?.path)">
          {{ detail?.path }}
        </el-link>
      </el-descriptions-item>
      <el-descriptions-item :label="i18n.detailPage.productName">
        {{ detail?.version?.productName }}
      </el-descriptions-item>
      <el-descriptions-item :label="i18n.detailPage.fileDescription">
        {{ detail?.version?.fileDescription }}
      </el-descriptions-item>
      <el-descriptions-item :label="i18n.detailPage.companyName">
        {{ detail?.version?.companyName }}
      </el-descriptions-item>
    </el-descriptions>
  </el-card>
</template>

<style scoped></style>
