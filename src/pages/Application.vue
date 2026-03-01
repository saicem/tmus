<script lang="ts" setup>
import { getAllAppDetail } from "@/script/cmd.ts"
import { FileDetail } from "@/script/models.ts"
import { configStore } from "@/script/state.ts"
import ContentView from "@/components/global/ContentView.vue"
import { TableColumnCtx } from "element-plus"
import { router } from "@/script/route.ts"

const appList = ref<FileDetail[]>([])

onMounted(async () => {
  let result = await getAllAppDetail()
  if (configStore.filterUninstalledApp) {
    result = result.filter((app) => app.exist)
  }
  result.sort((a, b) => a.name.localeCompare(b.name))
  appList.value = result
})

const searchName = ref("")
const searchCompany = ref("")
const filterTableData = computed(() =>
  appList.value.filter(
    (data) =>
      (!searchName.value ||
        data.name.toLowerCase().includes(searchName.value.toLowerCase())) &&
      (!searchCompany.value ||
        data.version?.companyName
          ?.toLowerCase()
          .includes(searchCompany.value.toLowerCase()))
  )
)

function tableRowClassName({ row }: { row: FileDetail; rowIndex: number }) {
  return row.exist ? "" : "warning-row"
}

function rowDbClick(row: FileDetail, _column: TableColumnCtx, _event: Event) {
  router.push({ name: "detail", params: { id: row.id } })
}
</script>

<template>
  <content-view>
    <el-table
      :row-class-name="tableRowClassName"
      :data="filterTableData"
      @row-dblclick="rowDbClick"
      style="height: 100%; width: 100%"
    >
      <el-table-column>
        <template #header>
          <el-input
            v-model="searchName"
            size="small"
            placeholder="Type to search name"
          />
        </template>
        <template #default="scope">
          <div style="display: flex; align-items: center">
            <app-icon :icon="scope.row.icon" :size="16" />
            <p style="margin-left: 8px">{{ scope.row.name }}</p>
          </div>
        </template>
      </el-table-column>
      <el-table-column label="Company" prop="version.companyName">
        <template #header>
          <el-input
            v-model="searchCompany"
            size="small"
            placeholder="Type to search company"
          />
        </template>
      </el-table-column>
    </el-table>
  </content-view>
</template>

<style scoped></style>

<style>
.el-table .warning-row {
  --el-table-tr-bg-color: var(--el-color-warning-light-9);
}
</style>
