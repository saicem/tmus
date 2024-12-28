<script setup lang="ts">
import { computed, ref } from "vue"
import { FocusData } from "@/global/data.ts"
import { readReverse } from "@/global/api.ts"
import TimelineItem from "@/components/TimelineItem.vue"

const records = ref<FocusData[]>([])
const cursor = ref<number | null>(null)
const scrollDisable = computed(() => loading.value || noMore.value)
const noMore = ref(false)
const loading = ref(false)

const load = async () => {
  loading.value = true
  console.log("cursor.value", cursor.value)
  const [newRecords, newCursor] = await readReverse(cursor.value, 30)
  if (newCursor === null) {
    console.log("no more data")
    noMore.value = true
    return
  }
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
      <RouterLink :to="'/detail/' + record.id">
        <TimelineItem :data="record" />
      </RouterLink>
    </el-timeline-item>
  </el-timeline>
  <div style="text-align: center">No more</div>
</template>

<style scoped></style>
