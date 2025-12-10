<script lang="ts" setup>
const props = defineProps<{
  icon: string
  content: string
  illustration: string
}>()
</script>

<template>
  <el-card style="width: 100%">
    <div class="card">
      <div style="flex: 1">
        <h3 style="margin: 0; font-size: 16px; font-weight: bold">
          {{ props.illustration }}
        </h3>
        <p style="margin: 0; color: #888; font-size: 14px">
          {{ props.content }}
        </p>
      </div>
      <img :src="icon" alt="icon" style="height: 48px; width: 48px" />
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
  padding: 8px;
}
</style>
