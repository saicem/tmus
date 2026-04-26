<script lang="ts" setup>
import { ref, watch, onMounted, computed } from "vue"
import { ElMessage } from "element-plus"
import { getAppTotalDuration, getAllCategories } from "@/script/cmd.ts"
import { AppDurationRequest, CategorySimple, FileDetail } from "@/script/models.ts"
import { formatDurationRough, getTimeRangeByDays } from "@/script/time-util"
import ProgressChart from "@/components/chart/ProgressChart.vue"
import { statisticStore } from "@/script/state"
import { i18n } from "@/script/i18n.ts"

interface RawData {
  app: FileDetail
  duration: number
  proportion: number
}

type DisplayStyle = "Card" | "Progress" | "Pie"

const displayStyles = [
  { label: i18n.value.statisticPage.displayStyle.card, value: "Card" },
  { label: i18n.value.statisticPage.displayStyle.progress, value: "Progress" },
  { label: i18n.value.statisticPage.displayStyle.pie, value: "Pie" },
]

const displayStyle = ref<DisplayStyle>("Card")
const categoryId = ref<number | undefined>(undefined)
const data = ref<RawData[]>([])
const loadingData = ref<boolean>(false)
const categories = ref<CategorySimple[]>([])
const loadingCategories = ref<boolean>(false)
const timeRange = ref<[Date, Date]>(getTimeRangeByDays(1))

const pieChartTooltip = computed(() => ({
  title: "name",
  items: [{
    name: i18n.value.statisticPage.types.appDuration,
    field: 'value',
    valueFormatter: (value: number) => formatDurationRough(value)
  }]
}))

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
  loadingData.value = true
  try {
    const response = await getAppTotalDuration({
      startTime: timeRange.value[0].getTime(),
      endTime: timeRange.value[1].getTime(),
      categoryId: categoryId.value,
    })

    const totalDuration = response.detail.reduce((sum: number, item: any) => sum + item.value, 0)
    data.value = response.detail.map((item: any) => ({
      app: item.app,
      duration: item.value,
      proportion: item.value / totalDuration
    }))
  } catch (error) {
    console.error("Failed to load app duration data:", error)
    ElMessage.error("Failed to load app duration data")
  } finally {
    loadingData.value = false
  }
}

watch(() => statisticStore.params, (newValue) => {
  if (newValue != undefined && newValue.type === "AppDurationRequest") {
    const params = newValue as AppDurationRequest
    categoryId.value = params.categoryId
    timeRange.value = [new Date(params.startTime), new Date(params.endTime)]
  }
})

watch([timeRange, categoryId], () => {
  loadData()
})

onMounted(async () => {
  await loadCategories()
  await loadData()
})
</script>

<template>
  <div class="app-duration-stat">
    <div class="config-section">
      <div class="config-item">
        <label class="config-label">Time Range</label>
        <date-time-picker v-model="timeRange" />
      </div>

      <div class="config-item">
        <label class="config-label">Categories</label>
        <el-select v-model="categoryId" placeholder="Select category" class="category-select" clearable
          :loading="loadingCategories">
          <el-option v-for="category in categories" :key="category.id" :label="category.name" :value="category.id" />
        </el-select>
      </div>

      <div class="config-item">
        <label class="config-label">Display Style</label>
        <el-radio-group v-model="displayStyle">
          <el-radio-button v-for="style in displayStyles" :key="style.value" :label="style.label"
            :value="style.value" />
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

          <div v-else-if="displayStyle === 'Card'">
            <app-card-group :data="data.map(item => ({
              app: item.app,
              value: formatDurationRough(item.duration)
            }))" />
          </div>

          <div v-else-if="displayStyle === 'Progress'">
            <ProgressChart :data="data.map(item => ({
              id: item.app.id,
              label: item.app.name,
              value: formatDurationRough(item.duration),
              percentage: Math.round(item.proportion * 1000) / 10
            }))" />
          </div>

          <div v-else-if="displayStyle === 'Pie'">
            <pie-chart :data="data.map(item => ({
              name: item.app.name,
              value: item.duration,
              percentage: Math.round(item.proportion * 1000) / 10
            }))" :tooltip="pieChartTooltip" />
          </div>
        </template>
      </el-skeleton>
    </div>
  </div>
</template>

<style scoped>
.app-duration-stat {
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

.card-view {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
}

.app-card {
  transition: all 0.3s ease;
}

.app-card:hover {
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

.duration {
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

/* 饼图和柱状图样式 */
.chart-container {
  height: 400px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.pie-placeholder,
.bar-placeholder {
  width: 100%;
  max-width: 600px;
}

.pie-placeholder h3,
.bar-placeholder h3 {
  text-align: center;
  margin-bottom: 24px;
  color: var(--el-text-color-primary);
}

.pie-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid var(--el-border-color);
}

.bar-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 8px 0;
  border-bottom: 1px solid var(--el-border-color);
}

.bar-item .app-name {
  width: 150px;
  flex-shrink: 0;
}

.bar-container {
  flex: 1;
  height: 20px;
  background: var(--el-fill-color);
  border-radius: 10px;
  overflow: hidden;
}

.bar {
  height: 100%;
  background: var(--el-color-primary);
  border-radius: 10px;
  transition: width 0.3s ease;
}

.bar-item .duration {
  width: 100px;
  flex-shrink: 0;
  text-align: right;
}

/* 骨架屏样式 */
.skeleton-item {
  margin-bottom: 24px;
}
</style>
