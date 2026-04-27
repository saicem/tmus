<script lang="ts" setup>
import { ref, computed, watch, onMounted } from "vue"
import { i18n } from "@/script/i18n.ts"
import { TimeSpan, RhythmGroup as RhythmGroupModel, CategorySimple } from "@/script/models.ts"
import { getCategoryUsageRhythm, getAllCategories } from "@/script/cmd.ts"
import RhythmPolarChart from "@/components/chart/RhythmPolarChart.vue"

type DisplayStyle = "polar" | "bar"

interface RhythmGroup {
  timeRange: [Date, Date]
  categoryId?: number
}

const displayStyle = ref<DisplayStyle>("polar")
const timeSpan = ref<TimeSpan>("day")
const granularity = ref<number>(60 * 60 * 1000)
const groups = ref<RhythmGroup[]>([])
const chartData = ref<number[][]>([])
const loadingData = ref<boolean>(false)
const categories = ref<CategorySimple[]>([])
const loadingCategories = ref<boolean>(false)

const displayStyles = [
  { label: i18n.value.statisticPage.displayStyle.polar, value: "polar" },
  { label: i18n.value.statisticPage.displayStyle.bar, value: "bar" },
]

const granularityOptions = computed(() => timeSpan.value === "day" ? [
  { label: i18n.value.statisticPage.granularity.minute5, value: 5 * 60 * 1000 },
  { label: i18n.value.statisticPage.granularity.minute15, value: 15 * 60 * 1000 },
  { label: i18n.value.statisticPage.granularity.minute30, value: 30 * 60 * 1000 },
  { label: i18n.value.statisticPage.granularity.hour1, value: 60 * 60 * 1000 },
  { label: i18n.value.statisticPage.granularity.hour2, value: 2 * 60 * 60 * 1000 },
  { label: i18n.value.statisticPage.granularity.hour3, value: 3 * 60 * 60 * 1000 },
  { label: i18n.value.statisticPage.granularity.hour4, value: 4 * 60 * 60 * 1000 },
  { label: i18n.value.statisticPage.granularity.hour6, value: 6 * 60 * 60 * 1000 },
] : [
  { label: i18n.value.statisticPage.granularity.hour1, value: 60 * 60 * 1000 },
  { label: i18n.value.statisticPage.granularity.hour2, value: 2 * 60 * 60 * 1000 },
  { label: i18n.value.statisticPage.granularity.hour3, value: 3 * 60 * 60 * 1000 },
  { label: i18n.value.statisticPage.granularity.hour4, value: 4 * 60 * 60 * 1000 },
  { label: i18n.value.statisticPage.granularity.hour6, value: 6 * 60 * 60 * 1000 },
  { label: i18n.value.statisticPage.granularity.day1, value: 24 * 60 * 60 * 1000 },
])

const loadCategories = async () => {
  loadingCategories.value = true
  try {
    categories.value = await getAllCategories()
  } catch (error) {
    console.error("Failed to load categories:", error)
  } finally {
    loadingCategories.value = false
  }
}

const loadData = async () => {
  if (groups.value.length === 0) {
    chartData.value = []
    return
  }

  loadingData.value = true
  try {
    const groupsRequest: RhythmGroupModel[] = groups.value.map(g => ({
      startTime: g.timeRange[0].getTime(),
      endTime: g.timeRange[1].getTime(),
      categoryId: g.categoryId,
    }))

    const response = await getCategoryUsageRhythm({
      groups: groupsRequest,
      span: timeSpan.value,
      granularity: granularity.value,
    })

    chartData.value = response.values
  } catch (error) {
    console.error("Failed to load rhythm data:", error)
    chartData.value = []
  } finally {
    loadingData.value = false
  }
}

const addRhythmGroup = () => {
  const end = new Date()
  end.setHours(24, 0, 0, 0)
  const start = new Date(end)
  start.setDate(start.getDate() - 7)

  groups.value.push({
    timeRange: [start, end],
    categoryId: undefined,
  })
}

const removeRhythmGroup = (index: number) => {
  groups.value.splice(index, 1)
}

watch([timeSpan], () => {
  if (timeSpan.value === "day") {
    granularity.value = 60 * 60 * 1000
  } else {
    granularity.value = 24 * 60 * 60 * 1000
  }
})

watch([timeSpan, granularity, groups], () => {
  loadData()
}, { deep: true })


onMounted(async () => {
  await loadCategories()
  addRhythmGroup()
})
</script>

<template>
  <div class="category-rhythm-stat">
    <div class="config-section">
      <div class="config-item">
        <label class="config-label">{{ i18n.statisticPage.label.timeSpan }}</label>
        <el-radio-group v-model="timeSpan">
          <el-radio-button value="day">{{ i18n.statisticPage.timeSpan.day }}</el-radio-button>
          <el-radio-button value="week">{{ i18n.statisticPage.timeSpan.week }}</el-radio-button>
        </el-radio-group>
      </div>

      <div class="config-item">
        <label class="config-label">{{ i18n.statisticPage.label.granularity }}</label>
        <el-select v-model="granularity" class="granularity-select">
          <el-option v-for="opt in granularityOptions" :key="opt.value" :label="opt.label" :value="opt.value" />
        </el-select>
      </div>

      <div class="config-item">
        <label class="config-label">{{ i18n.statisticPage.label.displayStyle }}</label>
        <el-radio-group v-model="displayStyle">
          <el-radio-button v-for="style in displayStyles" :key="style.value" :value="style.value"
            :label="style.label" />
        </el-radio-group>
      </div>
    </div>

    <div class="groups-section">
      <div class="groups-header">
        <label class="config-label">{{ i18n.statisticPage.label.group }}</label>
        <el-button type="primary" size="small" @click="addRhythmGroup">
          {{ i18n.statisticPage.group.add }}
        </el-button>
      </div>

      <div v-for="(group, index) in groups" :key="index" class="group-item">
        <div class="group-content">
          <div class="group-field">
            <label>{{ i18n.statisticPage.label.timeRange }}</label>
            <el-date-picker v-model="group.timeRange" type="daterange" :clearable="false" size="small" />
          </div>
          <div class="group-field">
            <label>{{ i18n.statisticPage.categories.title }}</label>
            <el-select v-model="group.categoryId" size="small" class="group-category-select"
              :loading="loadingCategories" clearable>
              <el-option v-for="cat in categories" :key="cat.id" :label="cat.name" :value="cat.id" />
            </el-select>
          </div>
          <el-button type="danger" link size="small" @click="removeRhythmGroup(index)">
            {{ i18n.categoryPage.delete }}
          </el-button>
        </div>
      </div>
    </div>

    <div class="results-section">
      <el-skeleton :loading="loadingData" animated>
        <template #template>
          <div class="skeleton-chart"></div>
        </template>
        <template #default>
          <div v-if="chartData.length === 0" class="empty-state">
            <el-empty description="No data" />
          </div>
          <div v-else-if="displayStyle === 'polar'">
            <RhythmPolarChart :type="timeSpan" :granularity="granularity" :data="chartData" />
          </div>
        </template>
      </el-skeleton>
    </div>
  </div>
</template>

<style scoped>
.category-rhythm-stat {
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
  min-width: 200px;
}

.config-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--el-text-color-regular);
}

.granularity-select {
  width: 100%;
}

.groups-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 16px;
  background: var(--el-fill-color-light);
  border-radius: 8px;
  margin-bottom: 16px;
}

.groups-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.group-item {
  margin-bottom: 8px;
  padding: 12px;
  background: var(--el-bg-color);
  border-radius: 6px;
  border: 1px solid var(--el-border-color-light);
}

.group-content {
  display: flex;
  gap: 12px;
  align-items: flex-end;
  flex-wrap: wrap;
}

.group-field {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
  min-width: 200px;
}

.group-field label {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.group-category-select {
  width: 100%;
}

.results-section {
  min-height: 400px;
  padding: 16px;
  background: var(--el-fill-color-light);
  border-radius: 8px;
}

.skeleton-chart {
  height: 400px;
  background: var(--el-fill-color-light);
  border-radius: 8px;
}

.empty-state {
  min-height: 300px;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>