<script lang="ts" setup>
import { ref, watch, onMounted } from "vue"
import { ElMessage } from "element-plus"
import { i18n } from "@/script/i18n.ts"
import {
  getStatisticSchemeList,
  addStatisticScheme,
  deleteStatisticScheme,
} from "@/script/cmd.ts"
import { StatisticScheme } from "@/script/models.ts"
import { StatisticType } from "@/script/state.ts"

const statisticType = defineModel<StatisticType>("statisticType", {
  required: true,
})

const schemeList = ref<StatisticScheme>({ items: [] })
const selectedSchemeId = ref<number | undefined>(undefined)
const showSaveDialog = ref(false)
const schemeNameInput = ref("")

watch(selectedSchemeId, () => {
  if (selectedSchemeId.value === undefined) {
    return
  }
  const scheme = schemeList.value.items.find((item) => item.id === selectedSchemeId.value);
  if (scheme && scheme.detail) {
    if ('type' in scheme.detail) {
      statisticType.value = scheme.detail.type as StatisticType
    }
  }
})

const loadSchemeList = async () => {
  try {
    schemeList.value = await getStatisticSchemeList()
  } catch (error) {
    console.error("Failed to load scheme list:", error)
    ElMessage.error("Failed to load scheme list")
  }
}

const handleSaveScheme = async () => {
  if (!schemeNameInput.value.trim()) {
    ElMessage.warning("Please enter a scheme name")
    return
  }

  try {
    const request: any = {
      name: schemeNameInput.value.trim(),
    }
    await addStatisticScheme(request)
    await loadSchemeList()
    showSaveDialog.value = false
    schemeNameInput.value = ""
    ElMessage.success("Scheme saved successfully")
  } catch (error) {
    console.error("Failed to save scheme:", error)
    ElMessage.error("Failed to save scheme")
  }
}

const handleDeleteScheme = async (id: number) => {
  try {
    await deleteStatisticScheme(id)
    await loadSchemeList()
    if (selectedSchemeId.value === id) {
      selectedSchemeId.value = undefined
    }
    ElMessage.success("Scheme deleted successfully")
  } catch (error) {
    console.error("Failed to delete scheme:", error)
    ElMessage.error("Failed to delete scheme")
  }
}

onMounted(async () => {
  await loadSchemeList()
})
</script>

<template>
  <div class="section">
    <div class="group">
      <el-select v-model="selectedSchemeId" :placeholder="i18n.statisticPage.placeholder.selectPlan" clearable
        class="plan-select">
        <el-option v-for="scheme in schemeList.items" :key="scheme.id" :label="scheme.name" :value="scheme.id">
          <div class="plan-option">
            <span>{{ scheme.name }}</span>
            <el-button type="danger" link size="small" @click.stop="handleDeleteScheme(scheme.id)">
              Delete
            </el-button>
          </div>
        </el-option>
      </el-select>
      <el-button type="primary" @click="showSaveDialog = true">
        {{ i18n.statisticPage.plan.save }}
      </el-button>
    </div>

    <div class="group">
      <el-select v-model="statisticType" class="type-select">
        <el-option :label="i18n.statisticPage.types.appDuration" value="AppDuration" />
        <el-option :label="i18n.statisticPage.types.appDays" value="AppDays" />
        <el-option :label="i18n.statisticPage.types.categoryDuration" value="CategoryDuration" />
        <el-option :label="i18n.statisticPage.types.categoryDays" value="CategoryDays" />
        <el-option :label="i18n.statisticPage.types.categoryRhythm" value="CategoryRhythm" />
      </el-select>
    </div>

    <el-dialog v-model="showSaveDialog" :title="i18n.statisticPage.plan.title" width="400px">
      <el-input v-model="schemeNameInput" :placeholder="i18n.statisticPage.placeholder.planName" />
      <template #footer>
        <el-button @click="showSaveDialog = false">{{
          i18n.common.cancel
        }}</el-button>
        <el-button type="primary" @click="handleSaveScheme">{{
          i18n.statisticPage.plan.save
        }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.section {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  margin-bottom: 16px;
}

.group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.plan-option {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.plan-select {
  width: 150px;
  flex: 0 0 auto;
}

.type-select {
  width: 200px;
  flex: 0 0 auto;
}
</style>
