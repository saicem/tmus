<script lang="ts" setup>
import { ref, onMounted } from "vue"
import {
  getAllCategories,
  getBaseTime,
} from "@/script/cmd.ts"
import {
  CategorySimple,
} from "@/script/models.ts"
import AppDurationStat from "@/components/statistic/AppDurationStat.vue"
import AppDaysStat from "@/components/statistic/AppDaysStat.vue"
import CategoryDurationStat from "@/components/statistic/CategoryDurationStat.vue"
import CategoryDaysStat from "@/components/statistic/CategoryDaysStat.vue"
import CategoryRhythmStat from "@/components/statistic/CategoryRhythmStat.vue"
import StatisticBasicSelector from "@/components/statistic/StatisticBasicSelector.vue"
import { StatisticType } from "@/script/state"







const categories = ref<CategorySimple[]>([])
const baseTime = ref<number>(0)

const statisticType = ref<StatisticType>("AppDuration")


const loadCategories = async () => {
  try {
    const result = await getAllCategories()
    categories.value = result
  } catch (error) {
    console.error("Failed to load categories:", error)
  }
}

const loadBaseTime = async () => {
  try {
    baseTime.value = await getBaseTime()
  } catch (error) {
    console.error("Failed to load base time:", error)
  }
}

onMounted(async () => {
  await loadBaseTime()
  await loadCategories()
})
</script>

<template>
  <content-view-scrollbar>
    <div class="statistic-container">
      <StatisticBasicSelector v-model:statistic-type="statisticType" />

      <div class="results-section">
        <AppDurationStat v-if="statisticType === 'AppDuration'" />
        <AppDaysStat v-else-if="statisticType === 'AppDays'" />
        <CategoryDurationStat v-else-if="statisticType === 'CategoryDuration'" />
        <CategoryDaysStat v-else-if="statisticType === 'CategoryDays'" />
        <CategoryRhythmStat v-else-if="statisticType === 'CategoryRhythm'" />
      </div>
    </div>
  </content-view-scrollbar>
</template>

<style scoped>
.statistic-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 16px;
  max-width: 1200px;
  margin: 0 auto;
}

.results-section {
  width: 100%;
}
</style>
