<script lang="ts" setup>
import { AppDurationAreaModel, FileDetail } from "@/script/models.ts"
import { getAppDetail, getAppDurationArea, showInFolder } from "@/script/cmd.ts"
import { i18n } from "@/script/i18n.ts"
import { Chart } from "@antv/g2"
import {
  formatDurationRough,
  timeZoneOffsetMillis,
} from "@/script/time-util.ts"
import { configStore, passiveStore } from "@/script/state.ts"
import { DeleteFilled } from "@element-plus/icons-vue"

const props = defineProps<{
  id: number
}>()
const detail = ref<FileDetail | null>(null)
const durationAreaData = ref<AppDurationAreaModel | null>(null)
const durationAreaChartContainer = ref<HTMLDivElement | null>(null)
const activeName = ref("durationDateArea")

onMounted(async () => {
  detail.value = await getAppDetail(props.id)
  durationAreaData.value = await getAppDurationArea(
    props.id,
    0,
    new Date().getTime(),
    timeZoneOffsetMillis()
  )
  renderAreaChart()
})

function renderAreaChart() {
  if (activeName.value == "durationDateArea") {
    renderDateAreaChart()
  } else {
    renderDayAreaChart()
  }
}

function renderDateAreaChart() {
  const xName = i18n.value.detailPage.durationAreaChart.dateChart.xName
  const yName = i18n.value.detailPage.durationAreaChart.dateChart.yName
  const chart = new Chart({
    container: durationAreaChartContainer!.value!,
    autoFit: true,
  })
  chart
    .theme({ type: passiveStore.theme })
    .area()
    .data(durationAreaData.value?.dateArea)
    .encode("x", "index")
    .encode("y", "value")
    .encode("shape", "hvh")
    .tooltip({ name: yName, channel: "y", valueFormatter: formatDurationRough })
    .axis("x", {
      title: xName,
      labelAutoHide: true,
      labelTransform: "rotate(0)",
    })
    .axis("y", { title: yName, labelFormatter: formatDurationRough })
  chart.render()
}

function renderDayAreaChart() {
  const xName = i18n.value.detailPage.durationAreaChart.dayChart.xName
  const yName = i18n.value.detailPage.durationAreaChart.dayChart.yName
  const chart = new Chart({
    container: durationAreaChartContainer!.value!,
    autoFit: true,
  })
  chart
    .theme({ type: passiveStore.theme })
    .area()
    .data(durationAreaData.value?.dayArea)
    .encode("x", "index")
    .encode("y", "value")
    .encode("shape", "hvh")
    .tooltip({ name: yName, channel: "y" })
    .axis("x", {
      title: xName,
      labelAutoHide: true,
      labelTransform: "rotate(0)",
    })
    .axis("y", { title: yName })
  chart.render()
}

watch(
  [() => configStore.theme, activeName, i18n],
  (_old, _new) => {
    renderAreaChart()
  },
  { deep: true }
)
</script>

<template>
  <content-view-scrollbar>
    <div style="display: flex; flex-direction: column; row-gap: 16px">
      <el-card>
        <el-descriptions direction="vertical">
          <template #title>
            <div style="display: flex; align-items: center">
              <app-icon :icon="detail?.icon" :size="32" />
              <h3 style="margin-left: 16px">
                {{ detail?.name }}
              </h3>
            </div>
          </template>
          <el-descriptions-item :label="i18n.detailPage.name">
            {{ detail?.name }}
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
          <el-descriptions-item :label="i18n.detailPage.filePath">
            <el-popover
              placement="top-start"
              :content="i18n.detailPage.fileHasBeenDeleted"
              :disabled="detail?.exist"
            >
              <template #reference>
                <div style="display: flex; align-items: center">
                  <el-icon
                    color="#f56c6c"
                    v-if="!detail?.exist"
                    style="margin-right: 8px"
                  >
                    <DeleteFilled />
                  </el-icon>
                  <el-link
                    @click="detail?.exist && showInFolder(detail?.path)"
                    :type="detail?.exist ? 'default' : 'danger'"
                  >
                    {{ detail?.path }}
                  </el-link>
                </div>
              </template>
            </el-popover>
          </el-descriptions-item>
        </el-descriptions>
      </el-card>
      <el-card>
        <el-tabs v-model="activeName">
          <el-tab-pane
            :label="i18n.detailPage.durationDateAreaTab"
            name="durationDateArea"
          />
          <el-tab-pane
            :label="i18n.detailPage.durationDayAreaTab"
            name="durationDayArea"
          />
        </el-tabs>
        <div ref="durationAreaChartContainer" />
      </el-card>
    </div>
  </content-view-scrollbar>
</template>

<style scoped></style>
