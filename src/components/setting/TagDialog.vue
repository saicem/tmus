<script lang="ts" setup>
const model = defineModel<boolean>()
</script>

<template>
  <el-dialog v-model="model" title="应用标签"></el-dialog>
</template>

<style scoped></style>
