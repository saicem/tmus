<script lang="ts" setup>
import { computed } from "vue"
import { AppDuration } from "@/script/models.ts"
import AppProgress from "@/components/statistic/AppProgress.vue"

const props = defineProps<{
  data: AppDuration[]
}>()

const maxDurationMillis = computed(() =>
  Math.max(...props.data.map((duration) => duration.duration.asMilliseconds()))
)
</script>

<template>
  <div style="display: flex; flex-direction: column; gap: 16px">
    <RouterLink
      v-for="{ app: app, duration: duration } in data"
      :key="app.id"
      :to="'/detail/' + app.id"
    >
      <AppProgress
        :app="app"
        :duration="duration"
        :percentage="
          Math.round((duration.asMilliseconds() / maxDurationMillis) * 100)
        "
      />
    </RouterLink>
  </div>
</template>

<style scoped></style>
