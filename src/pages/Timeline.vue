<script setup lang="ts">
import { computed, ref } from "vue"
import { durationByDayId } from "@/global/api.ts"
import TimelineItem from "@/components/TimelineItem.vue"
import moment, { Moment } from "moment-timezone"

const scrollDisable = computed(() => loading.value || noMore.value)
const noMore = ref(false)
const loading = ref(false)
const nextDate = ref<Moment>(moment())
const data = ref<{ date: Moment; data: Record<number, number> }[]>([])
const millisInDay = 1000 * 60 * 60 * 24

const load = async () => {
  console.log("load")
  loading.value = true
  const endDate = nextDate.value
  const startDate = endDate.clone().subtract(1, "week")
  const result = await durationByDayId(startDate, endDate)
  const ripeResult = Object.entries(result)
    .sort((a, b) => {
      return +b[0] - +a[0]
    })
    .map(([k, v]) => {
      return {
        date: moment(millisInDay * +k),
        data: v,
      }
    })
  data.value.push(...ripeResult)
  console.log("data", data)
  // TODO judge no more
  // if (newCursor === null) {
  //   console.log("no more data")
  //   noMore.value = true
  //   return
  // }
  nextDate.value = startDate.clone().subtract(1, "day")
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
    <div>
      <el-timeline-item
        v-for="({ date: date, data: data }, i) in data"
        :key="i"
        placement="top"
        :timestamp="date.format('YYYY-MM-DD')"
      >
        <div style="display: flex; flex-wrap: wrap; gap: 16px">
          <RouterLink
            v-for="[id, duration] in Object.entries(data)"
            :key="id"
            :to="'/detail/' + id"
          >
            <TimelineItem :app-id="+id" :duration="moment.duration(duration)" />
          </RouterLink>
        </div>
      </el-timeline-item>
    </div>
  </el-timeline>
  <div style="text-align: center">No more</div>
</template>

<style scoped></style>
