<script lang="ts" setup>
import { ref, watch, onMounted, computed } from "vue"
import { ElMessage } from "element-plus"
import { getCategoryUsageDays, getAllCategories } from "@/script/cmd.ts"
import { CategoryDayCountRequest, CategorySimple } from "@/script/models.ts"
import { getTimeRangeByDays } from "@/script/time-util"
import ProgressChart from "@/components/chart/ProgressChart.vue"
import { statisticStore } from "@/script/state"
import { i18n } from "@/script/i18n.ts"

interface RawData {
  category: CategorySimple
  value: number
  proportion: number
}

type DisplayStyle = "Progress" | "Pie" | "Bar"

const displayStyles = [
  { label: i18n.value.statisticPage.displayStyle.progress, value: "Progress" },
  { label: i18n.value.statisticPage.displayStyle.pie, value: "Pie" },
  { label: i18n.value.statisticPage.displayStyle.bar, value: "Bar" },
]

const tooltip = computed(() => ({
  title: "name",
  items: [{
    name: i18n.value.statisticPage.types.categoryDays,
    field: 'value',
    valueFormatter: (value: number) => value + "天"
  }]
}))

const displayStyle = ref<DisplayStyle>("Progress")
const categoryIds = ref<number[]>([])
const data = ref<RawData[]>([])
const loadingData = ref<boolean>(false)
const categories = ref<CategorySimple[]>([])
const loadingCategories = ref<boolean>(false)
const timeRange = ref<[Date, Date]>(getTimeRangeByDays(1))

const formatValue = (value: number) => {
  return value.toFixed(0) + "天"
}

const loadCategories = async () => {
  loadingCategories.value = true
  try {
    categories.value = await getAllCategories()
  } catch (error) {
    console.error("Failed to load categories:", error)
    ElMessage.error("Failed to load categories")
  } finally {
    loadingCategories.value = false
  }
}

const loadData = async () => {
  if (categoryIds.value.length === 0) {
    data.value = []
    return
  }

  loadingData.value = true
  try {
    const response = await getCategoryUsageDays({
      startTime: timeRange.value[0].getTime(),
      endTime: timeRange.value[1].getTime(),
      categoryIds: [...categoryIds.value],
    })

    const totalDays = response.detail.reduce((sum: number, item: any) => sum + item.value, 0)
    data.value = response.detail.map((item: any) => ({
      category: item.category,
      value: item.value,
      proportion: item.value / totalDays
    }))
  } catch (error) {
    console.error("Failed to load category days data:", error)
    ElMessage.error("Failed to load category days data")
  } finally {
    loadingData.value = false
  }
}

watch(() => statisticStore.params, (newValue) => {
  if (newValue != undefined && newValue.type === "CategoryDayCountRequest") {
    const params = newValue as CategoryDayCountRequest
    categoryIds.value = params.categoryIds
    timeRange.value = [new Date(params.startTime), new Date(params.endTime)]
  }
})

watch([timeRange, categoryIds], () => {
  loadData()
})

onMounted(async () => {
  await loadCategories()
  await loadData()
})
</script>

<template>
  <div class="category-days-stat">
    <div class="config-section">
      <div class="config-item">
        <label class="config-label">{{ i18n.statisticPage.label.timeRange }}</label>
        <date-time-picker v-model="timeRange" type="daterange" />
      </div>

      <div class="config-item">
        <label class="config-label">{{ i18n.statisticPage.categories.title }}</label>
        <el-select v-model="categoryIds" :placeholder="i18n.statisticPage.categories.placeholder"
          class="category-select" multiple collapse-tags :loading="loadingCategories">
          <el-option v-for="category in categories" :key="category.id" :label="category.name" :value="category.id" />
        </el-select>
      </div>

      <div class="config-item">
        <label class="config-label">{{ i18n.statisticPage.label.displayStyle }}</label>
        <el-radio-group v-model="displayStyle">
          <el-radio-button v-for="style in displayStyles" :value="style.value" :label="style.label" />
        </el-radio-group>
      </div>
    </div>

    <div class="results-section">
      <el-skeleton :loading="loadingData" animated>
        <template #template>
          <div v-for="i in 5" :key="i" class="skeleton-item">
            <el-skeleton-item variant="h3" style="width: 50%" />
            <el-skeleton-item variant="text" style="width: 80%" />
            <el-skeleton-item variant="text" style="width: 60%" />
          </div>
        </template>

        <template #default>
          <div v-if="data.length === 0" class="empty-state">
            <el-empty description="No data available" />
          </div>

          <div v-else-if="displayStyle === 'Progress'">
            <ProgressChart :data="data.map(item => ({
              id: item.category.id,
              label: item.category.name,
              value: formatValue(item.value),
              percentage: Math.round(item.proportion * 1000) / 10
            }))" />
          </div>

          <div v-else-if="displayStyle === 'Pie'">
            <pie-chart :data="data.map(item => ({
              name: item.category.name,
              value: item.value,
              percentage: Math.round(item.proportion * 1000) / 10
            }))" :tooltip="tooltip" />
          </div>

          <div v-else-if="displayStyle === 'Bar'">
            <BarChart :data="data.map(item => ({
              name: item.category.name,
              value: item.value,
            }))" :yAxisLabelFormat="formatValue" :tooltip="tooltip" />
          </div>
        </template>
      </el-skeleton>
    </div>
  </div>
</template>

<style scoped>
.category-days-stat {
  width: 100%;
}

.config-section {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  padding: 16px;
  background: var(--el-fill-color-light);
  border-radius: 8px;
  margin-bottom: 16px;
}

.config-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
  flex: 1 1 200px;
}

.config-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--el-text-color-regular);
}

.category-select,
.time-range-picker {
  width: 100%;
}

.results-section {
  min-height: 400px;
  padding: 16px;
  background: var(--el-fill-color-light);
  border-radius: 8px;
}

.empty-state {
  min-height: 300px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.category-card {
  width: 300px;
  transition: all 0.3s ease;
}

.category-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.card-header h4 {
  margin: 0;
  font-size: 16px;
  font-weight: 500;
}

.days {
  font-size: 14px;
  color: var(--el-text-color-secondary);
}

.card-body {
  position: relative;
}

.percentage {
  position: absolute;
  right: 0;
  top: 0;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

/* 骨架屏样式 */
.skeleton-item {
  margin-bottom: 24px;
}
</style>
