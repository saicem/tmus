<script setup lang="ts">
import api from "@/global/api"
import { FocusRecord } from "@/global/data"
import moment from "moment"
import { ref } from "vue"

const records = ref<FocusRecord[]>([])
const cursor = ref<number | null>(null)
const scrollDisable = ref(false)

const load = async () => {
  console.log("cursor.value", cursor.value)
  if (cursor.value === 0) {
    console.log("no more data")
    scrollDisable.value = true
    return
  }
  const res = await api.readReverse(cursor.value, 100)
  console.log("api.readReverse", res)
  records.value = records.value.concat(res[0])
  cursor.value = res[1]
}
</script>

<template>
  <el-timeline
    v-infinite-scroll="load"
    :infinite-scroll-disabled="scrollDisable"
    :infinite-scroll-delay="400"
    infinite-scroll-distance="1000"
  >
    <el-timeline-item
      v-for="(record, index) in records"
      :key="index"
      placement="top"
      :timestamp="record.start.format()"
    >
      <el-card>
        <h4>
          {{ api.appDetail(record.id).name }}
        </h4>
        <p>{{ record.duration.toISOString() }}</p>
      </el-card>
    </el-timeline-item>
  </el-timeline>
</template>

<style scoped></style>
