<script lang="ts" setup>
export interface ProgressChartData {
    id: string | number
    label: string
    value: string
    percentage: number
}

const { data, title } = defineProps<{
    data: ProgressChartData[]
    title?: string
}>()

const formatPercentage = (percentage: number) => {
    if (percentage >= 0.1) {
        return percentage.toFixed(1) + '%'
    }
    return "<0.1%"
}
</script>

<template>
    <div class="progress-chart">
        <div v-if="title" class="chart-title">{{ title }}</div>
        <div class="progress-list">
            <div v-for="item in data" :key="item.id" class="progress-item">
                <div class="progress-header">
                    <slot :name="`icon-${item.id}`" :item="item">
                    </slot>
                    <div class="progress-label">
                        <span class="item-label">{{ item.label }}</span>
                        <span class="value-text">({{ item.value }})</span>
                    </div>
                </div>
                <el-progress :percentage="item.percentage" :format="formatPercentage" :stroke-width="6" />
            </div>
        </div>
    </div>
</template>

<style scoped>
.progress-chart {
    width: 100%;
}

.chart-title {
    font-size: 18px;
    font-weight: 600;
    color: var(--el-text-color-primary);
    margin-bottom: 16px;
    text-align: center;
}

.progress-list {
    display: flex;
    flex-direction: column;
    gap: 16px;
}

.progress-item {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.progress-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 14px;
}

.progress-label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 500;
    color: var(--el-text-color-primary);
}

.item-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    flex-shrink: 0;
}

.item-icon-placeholder {
    width: 20px;
    height: 20px;
    flex-shrink: 0;
}

.icon-image {
    width: 100%;
    height: 100%;
    object-fit: contain;
}

.item-label {
    flex: 1;
}

.progress-value {
    display: flex;
    align-items: center;
    color: var(--el-text-color-secondary);
    width: 50px;
}

.value-text {
    font-weight: 500;
}

.percentage-text {
    font-weight: 500;
}

.value-separator {
    font-weight: normal;
}
</style>
