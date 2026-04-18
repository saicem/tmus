<script setup lang="ts">
import { ref, onMounted, computed } from "vue"
import { ElMessage } from "element-plus"

import {
  getCategories,
  addCategory,
  updateCategory,
  deleteCategory,
  setAppCategory,
  removeAppFromCategory,
  getUncategorizedApps,
  getAllAppDetail,
} from "@/script/cmd.ts"
import { i18n } from "@/script/i18n.ts"
import type { Category as CategoryType, AddCategoryRequest, UpdateCategoryRequest, DeleteCategoryRequest, AssignCategoryRequest, RemoveAppCategoryRequest } from "@/script/models.ts"
import type { FileDetail } from "@/script/models.ts"
import { categoryStore } from "@/script/state"

const categories = ref<CategoryType>({ id: "root", name: "root", children: [], appIds: [] })
const apps = ref<FileDetail[]>([])
const uncategorizedApps = ref<FileDetail[]>([])
const addCategoryParent = ref<CategoryType | null>(null)

const loadCategories = async () => {
  try {
    const result = await getCategories()
    categories.value = result
  } catch (error) {
    ElMessage.error(i18n.value.categoryPage.failedToLoadCategories)
    console.error(error)
  }
}

const loadApps = async () => {
  try {
    apps.value = await getAllAppDetail()
  } catch (error) {
    ElMessage.error(i18n.value.categoryPage.failedToLoadApplications)
    console.error(error)
  }
}

const loadUncategorizedApps = async (keyword?: string) => {
  try {
    uncategorizedApps.value = await getUncategorizedApps(0, 100, keyword || undefined)
  } catch (error) {
    ElMessage.error(i18n.value.categoryPage.failedToLoadUncategorizedApplications)
    console.error(error)
  }
}

const openAddDialog = (parent: CategoryType | null = null) => {
  addCategoryParent.value = parent
  categoryStore.showAddDialog = true
}

const handleAddCategory = async (name: string, parentId: string) => {
  try {
    const request: AddCategoryRequest = {
      name,
      parentId,
    }
    await addCategory(request)
    ElMessage.success(i18n.value.categoryPage.categoryAddedSuccessfully)
    await loadCategories()
  } catch (error) {
    ElMessage.error(i18n.value.categoryPage.failedToAddCategory)
    console.error(error)
  }
}

const handleUpdateCategory = async (id: string, name: string) => {
  try {
    const request: UpdateCategoryRequest = {
      id,
      name,
    }
    await updateCategory(request)
    ElMessage.success(i18n.value.categoryPage.categoryUpdatedSuccessfully)
    await loadCategories()
  } catch (error) {
    ElMessage.error(i18n.value.categoryPage.failedToUpdateCategory)
    console.error(error)
  }
}

const handleDeleteCategory = async (categoryId: string) => {
  try {
    const request: DeleteCategoryRequest = { id: categoryId }
    await deleteCategory(request)
    ElMessage.success(i18n.value.categoryPage.categoryDeletedSuccessfully)
    if (categoryStore.selectedCategory?.id === categoryId) {
      categoryStore.selectedCategory = null
      categoryStore.activeTab = "uncategorized"
    }
    await loadCategories()
  } catch (error) {
    ElMessage.error(i18n.value.categoryPage.failedToDeleteCategory)
    console.error(error)
  }
}

const handleAssignCategory = async (appId: number, categoryId: string) => {
  try {
    const request: AssignCategoryRequest = {
      appId,
      categoryId,
    }
    await setAppCategory(request)
    ElMessage.success(i18n.value.categoryPage.applicationAssignedSuccessfully)
    await loadCategories()
    await loadApps()
    await loadUncategorizedApps()
  } catch (error) {
    ElMessage.error(i18n.value.categoryPage.failedToAssignApplication)
    console.error(error)
  }
}

const handleRemoveAppFromCategory = async (appId: number) => {
  try {
    const request: RemoveAppCategoryRequest = { appId }
    await removeAppFromCategory(request)
    ElMessage.success(i18n.value.categoryPage.applicationCategoryRemovedSuccessfully)
    await loadCategories()
    await loadUncategorizedApps()
  } catch (error) {
    ElMessage.error(i18n.value.categoryPage.failedToRemoveCategoryFromApplication)
    console.error(error)
  }
}

const handleNodeClick = (data: CategoryType) => {
  categoryStore.selectedCategory = data
  categoryStore.activeTab = "categorized"
}

const getCategoryApps = computed(() => {
  const category = categoryStore.selectedCategory
  if (!category) return []
  return category.appIds.map(appId => apps.value.find(app => app.id === appId)).filter(Boolean) as FileDetail[]
})

// Recursive function to build category options for select
const buildCategoryOptions = (categories: CategoryType[]): Array<{ label: string, value: string }> => {
  let options: Array<{ label: string, value: string }> = []
  for (const category of categories) {
    options.push({
      label: category.name,
      value: category.id
    })
    if (category.children.length > 0) {
      options = [...options, ...buildCategoryOptions(category.children)]
    }
  }
  return options
}

const categoryOptions = computed(() => {
  return buildCategoryOptions(categories.value.children)
})

onMounted(async () => {
  await loadCategories()
  await loadApps()
  await loadUncategorizedApps()
})
</script>

<template>
  <content-view class="category-page">
    <div class="category-header">
      <h2>{{ i18n.categoryPage.title }}</h2>
      <div class="add-category">
        <el-button type="primary" @click="openAddDialog(null)">{{ i18n.categoryPage.addRootCategory }}</el-button>
      </div>
    </div>

    <div class="category-content">
      <CategoryTree :categories="categories.children"
        @node-click="handleNodeClick" @add-category="openAddDialog" @update-category="handleUpdateCategory"
        @delete-category="handleDeleteCategory" />

      <AppList :uncategorized-apps="uncategorizedApps" :category-apps="getCategoryApps"
        :category-options="categoryOptions" @assign-category="handleAssignCategory"
        @remove-app-from-category="handleRemoveAppFromCategory" @search="loadUncategorizedApps" />
    </div>

    <AddCategoryDialog :parent-category="addCategoryParent"
      @add-category="handleAddCategory" />
  </content-view>
</template>

<style scoped>
.category-page {
  padding: 20px;
  height: 100vh;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
}

.category-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  flex-shrink: 0;
}

.category-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
}

.category-content {
  display: flex;
  gap: 20px;
  min-width: 0;
  flex: 1;
  overflow: hidden;
}

.add-category {
  display: flex;
  align-items: center;
}
</style>