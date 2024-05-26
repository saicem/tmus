<script setup lang="ts">
import moment, { Duration } from "moment"
import { computed } from "vue"

const props = defineProps<{
  dayOfYear: number
  duration: Duration
}>()

const hours = computed(() => {
  return props.duration.hours()
})
const minutes = computed(() => {
  return props.duration.minutes()
})
</script>

<template>
  <a-tooltip
    :title="`${moment().dayOfYear(props.dayOfYear).format('yyyy-MM-DD')} ${
      hours > 0 ? `${hours}h ${minutes}min` : `${minutes}min`
    }`"
  >
    <div
      :data-tag="Math.min(4, Math.ceil(props.duration.asHours() / 4))"
      class="block-unit"
    ></div>
  </a-tooltip>
</template>

<style scoped>
.block-unit {
  height: 10px;
  width: 10px;
  border-radius: 2px;
}

.block-unit[data-tag="4"] {
  background: var(--accent-color);
}

.block-unit[data-tag="3"] {
  background: var(--accent-color-1);
}

.block-unit[data-tag="2"] {
  background: var(--accent-color-2);
}

.block-unit[data-tag="1"] {
  background: var(--accent-color-3);
}

.block-unit[data-tag="0"] {
  background: var(--accent-color-4);
}

.block-unit:hover {
  box-shadow: 0 0 5px rgb(57, 120, 255);
}
</style>
