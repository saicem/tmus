<template>
  <el-dialog v-model="categoryStore.showAddDialog" :title="i18n.categoryPage.addCategory" width="400px">
    <el-form @submit.prevent="handleAddCategory">
      <el-form-item :label="i18n.categoryPage.categoryName">
        <el-input v-model="categoryName" :placeholder="i18n.categoryPage.enterCategoryName" />
      </el-form-item>
      <el-form-item v-if="parentCategory" :label="i18n.categoryPage.parentCategory">
        <span>{{ parentCategory?.name || 'Unknown' }}</span>
      </el-form-item>
      <el-form-item v-else :label="i18n.categoryPage.parentCategory">
        <span>{{ i18n.categoryPage.rootLevel }}</span>
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button @click="categoryStore.showAddDialog = false">{{ i18n.categoryPage.cancel }}</el-button>
      <el-button type="primary" @click="handleAddCategory">{{ i18n.categoryPage.confirm }}</el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref } from "vue"
import { ElMessage } from "element-plus"
import { i18n } from "@/script/i18n.ts"
import type { CategoryId, Category as CategoryType } from "@/script/models.ts"
import { categoryStore } from "@/script/state";

const props = defineProps<{
  parentCategory: CategoryType | null
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'addCategory', name: string, parentId: CategoryId): void
}>()

const categoryName = ref("")

const handleAddCategory = () => {
  if (!categoryName.value.trim()) {
    ElMessage.warning(i18n.value.categoryPage.categoryNameCannotBeEmpty)
    return
  }
  emit('addCategory', categoryName.value.trim(), props.parentCategory?.id || 0)
  categoryStore.showAddDialog = false
  categoryName.value = ""
}

</script>