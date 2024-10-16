<script setup lang="ts">
import api, { FocusData } from "@/global/api"
import { computed, ref } from "vue"

const records = ref<FocusData[]>([])
const cursor = ref<number | null>(null)
const scrollDisable = computed(() => loading.value || noMore.value)
const noMore = ref(false)
const loading = ref(false)

const load = async () => {
  loading.value = true
  console.log("cursor.value", cursor.value)
  if (cursor.value === 0) {
    console.log("no more data")
    noMore.value = true
    return
  }
  const [newRecords, newCursor] = await api.readReverse(cursor.value, 30)
  console.log("api.readReverse", records, cursor)
  records.value = records.value.concat(newRecords)
  cursor.value = newCursor
  loading.value = false
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
      <RouterLink to="/detail">
        <el-card>
          <h4>
            {{ api.appDetail(record.id).name }}
          </h4>
          <p>{{ record.duration.toISOString() }}</p>
        </el-card>
      </RouterLink>
    </el-timeline-item>
  </el-timeline>
  <center>No more</center>
</template>

<style scoped></style>
