<template>
  <div class="category-tree">
    <h3>{{ i18n.categoryPage.categoryTree }}</h3>
    <el-scrollbar>
      <el-tree :data="categories" node-key="id" :expand-on-click-node="false" default-expand-all
        @node-click="handleNodeClick">
        <template #default="{ data }">
          <div class="tree-node">
            <span v-if="editingCategoryId !== data.id" class="node-label">
              {{ data.name }}
              <span class="app-count">({{ data.appIds?.length || 0 }})</span>
            </span>
            <el-input v-else v-model="editingCategoryName" size="small" style="width: 150px"
              @keyup.enter="saveCategory" @click.stop />
            <div class="node-actions">
              <span v-if="editingCategoryId !== data.id">
                <el-button size="small" text type="primary" @click.stop="openAddDialog(data)">
                  {{ i18n.categoryPage.addChild }}
                </el-button>
                <el-button size="small" text type="primary" @click.stop="startEditCategory(data)">
                  {{ i18n.categoryPage.edit }}
                </el-button>
                <el-button size="small" text type="danger" @click.stop="handleDeleteCategory(data.id)">
                  {{ i18n.categoryPage.delete }}
                </el-button>
              </span>
              <span v-else>
                <el-button size="small" text type="primary" @click.stop="saveCategory">
                  {{ i18n.categoryPage.save }}
                </el-button>
                <el-button size="small" text @click.stop="cancelEdit">
                  {{ i18n.categoryPage.cancelEdit }}
                </el-button>
              </span>
            </div>
          </div>
        </template>
      </el-tree>
    </el-scrollbar>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue"
import { ElMessage, ElMessageBox } from "element-plus"
import { i18n } from "@/script/i18n.ts"
import type { Category as CategoryType } from "@/script/models.ts"

defineProps<{
  categories: CategoryType[]
}>()

const emit = defineEmits<{
  (e: 'nodeClick', data: CategoryType): void
  (e: 'addCategory', parent: CategoryType | null): void
  (e: 'updateCategory', id: string, name: string): void
  (e: 'deleteCategory', id: string): void
}>()

const editingCategoryId = ref<string | null>(null)
const editingCategoryName = ref("")

const handleNodeClick = (data: CategoryType) => {
  emit('nodeClick', data)
}

const openAddDialog = (parent: CategoryType | null = null) => {
  emit('addCategory', parent)
}

const startEditCategory = (category: CategoryType) => {
  editingCategoryId.value = category.id
  editingCategoryName.value = category.name
}

const saveCategory = () => {
  if (!editingCategoryId.value || !editingCategoryName.value.trim()) {
    ElMessage.warning(i18n.value.categoryPage.categoryNameCannotBeEmpty)
    return
  }
  emit('updateCategory', editingCategoryId.value, editingCategoryName.value.trim())
  editingCategoryId.value = null
  editingCategoryName.value = ""
}

const cancelEdit = () => {
  editingCategoryId.value = null
  editingCategoryName.value = ""
}

const handleDeleteCategory = async (categoryId: string) => {
  try {
    await ElMessageBox.confirm(i18n.value.categoryPage.confirmDelete, i18n.value.categoryPage.confirm, {
      confirmButtonText: i18n.value.categoryPage.confirm,
      cancelButtonText: i18n.value.categoryPage.cancel,
      type: "warning",
    })
    emit('deleteCategory', categoryId)
  } catch (error) {
    if (error !== "cancel") {
      ElMessage.error(i18n.value.categoryPage.failedToDeleteCategory)
      console.error(error)
    }
  }
}
</script>

<style scoped>
.category-tree {
  flex: 0 0 400px;
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  padding: 15px;
  min-width: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.category-tree h3 {
  margin-top: 0;
  margin-bottom: 15px;
  font-size: 16px;
  font-weight: 600;
  flex-shrink: 0;
}

:deep(.el-tree) {
  flex: 1;
  overflow: hidden;
}

:deep(.el-scrollbar) {
  flex: 1;
}

.tree-node {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 5px 0;
}

.node-label {
  font-weight: 500;
}

.app-count {
  color: #909399;
  font-size: 12px;
  margin-left: 5px;
}

.node-actions {
  display: flex;
  gap: 5px;
}
</style>