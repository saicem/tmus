<script setup lang="ts">
const props = defineProps<{
  icon: string
  content: string
  illustration: string
}>()
</script>

<template>
  <el-card style="width: 100%">
    <div class="card">
      <img :src="icon" alt="icon" />
      <div style="flex: 1">
        <h3 style="margin: 0; font-size: 16px; font-weight: bold">
          {{ props.illustration }}
        </h3>
        <p style="margin: 0; color: #888; font-size: 14px">
          {{ props.content }}
        </p>
      </div>
    </div>
  </el-card>
</template>

<style scoped>
.card {
  display: flex;
  flex-direction: row;
  align-items: center;
  width: 100%;
  gap: 8px;
}
</style>
