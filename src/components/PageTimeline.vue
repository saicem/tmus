<script setup lang="ts">
import { queryRecords } from "@/global/command"
import { FocusRecord } from "@/global/data"
import moment from "moment"
import { ref } from "vue"

const records = ref<FocusRecord[]>([])

let end = moment()
let start = end.clone().subtract(1, "days")
queryRecords(start, end).then((res) => {
  console.log(res)
  records.value = res
})
</script>

<template>
  <a-timeline style="padding: 32px; width: 100%; height: 100%">
    <a-timeline-item v-for="record in records"
      >{{
        `${start.format()} ${record.duration.humanize()}`
      }}
      2015-09-01</a-timeline-item
    >
  </a-timeline>
</template>

<style scoped></style>
