<script lang="ts" setup>
import { AppDurationAreaModel, FileDetail } from "@/script/models.ts"
import { onMounted, ref, watch } from "vue"
import { getAppDetail, getAppDurationArea, showInFolder } from "@/script/cmd.ts"
import { i18n } from "@/script/i18n.ts"
import { Chart } from "@antv/g2"
import moment from "moment-timezone"
import { timeZoneOffsetMillis } from "@/script/time-util.ts"
import { colorMode, config } from "@/script/state.ts"

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
    moment().valueOf(),
    timeZoneOffsetMillis()
  )
  renderAreaChart()
})

function formatDuration(ms: number) {
  if (ms < 1000) return `${ms}ms`
  const s = ms / 1000
  if (s < 60) return `${s.toFixed(1)}s`
  const m = s / 60
  if (m < 60) return `${m.toFixed(1)}m`
  const h = m / 60
  if (h < 24) return `${h.toFixed(1)}h`
  const d = h / 24
  return `${d.toFixed(1)}d`
}

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
    .theme({ type: colorMode.value })
    .area()
    .data(durationAreaData.value?.dateArea)
    .encode("x", "index")
    .encode("y", "value")
    .encode("shape", "hvh")
    .tooltip({ name: yName, channel: "y", valueFormatter: formatDuration })
    .axis("x", {
      title: xName,
      labelAutoHide: true,
      labelTransform: "rotate(0)",
    })
    .axis("y", { title: yName, labelFormatter: formatDuration })
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
    .theme({ type: colorMode.value })
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
  [() => config.value.theme, activeName, i18n],
  (_old, _new) => {
    renderAreaChart()
  },
  { deep: true }
)
</script>

<template>
  <div style="display: flex; flex-direction: column; row-gap: 16px">
    <el-card>
      <el-descriptions :title="detail?.name" border>
        <el-descriptions-item
          :label="i18n.detailPage.icon"
          :rowspan="2"
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
        <!--        TODO 通过别的方式表示 而非直接在表格中展示，比如在名称后边加个 感叹号 悬浮时标识文件不存在-->
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
</template>

<style scoped></style>
