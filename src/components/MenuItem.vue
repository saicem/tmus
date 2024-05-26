<script setup lang="ts">
import { tabStore } from "@/global/state.ts"

const props = defineProps<{
  tag: string
  text: string
}>()
</script>

<template>
  <div
    class="menu-item"
    :class="props.tag === tabStore.cur ? 'active' : 'inActive'"
    @click="tabStore.change(props.tag)"
  >
    <p>{{ props.text }}</p>
  </div>
</template>

<style scoped>
.active {
  background-color: var(--accent-color);
}

.menu-item:hover {
  box-shadow: 0 0 4px var(--accent-color);
  border: solid 1px var(--accent-color);
}

.menu-item {
  border: solid 1px transparent;
  border-radius: 8px;
  height: 40px;
  width: 40px;
  cursor: pointer;
  text-align: center;
  line-height: 40px;
}
</style>
