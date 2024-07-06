<script setup lang="ts">
const props = defineProps<{
  icon: string
  value: string
  unit: string
  illustration: string
}>()
</script>

<template>
  <el-card class="root">
    <div style="display: inline-block">
      <img :src="icon" alt="icon" />
    </div>
    <div style="align-items: start; display: inline-block">
      <el-statistic
        :title="props.illustration"
        :value="props.value + props.unit"
      />
    </div>
  </el-card>
</template>

<style scoped>
span {
  user-select: none;
}

.root {
  flex-direction: row;
  justify-content: start;
  padding: 16px 24px;
  width: 100%;
  gap: 16px;
}
</style>
