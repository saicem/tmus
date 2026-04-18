<template>
  <div class="category-apps">
    <el-tabs v-model="categoryStore.activeTab" style="height: 100%;" @update:model-value="handleTabChange">
      <el-tab-pane :label="i18n.categoryPage.uncategorizedApplications" name="uncategorized">
        <div class="search-box">
          <el-input v-model="searchKeyword" :placeholder="i18n.categoryPage.searchPlaceholder" clearable
            @input="handleSearch">
            <template #prefix>
              <span class="search-icon">🔍</span>
            </template>
          </el-input>
        </div>
        <el-table :data="uncategorizedApps" style="height: 100%;width: 100%">
          <el-table-column prop="name" :label="i18n.categoryPage.applicationName" show-overflow-tooltip />
          <el-table-column prop="path" :label="i18n.categoryPage.path" show-overflow-tooltip />
          <el-table-column :label="i18n.categoryPage.operation" width="180">
            <template #default="{ row }">
              <el-select :placeholder="i18n.categoryPage.selectCategory" clearable
                @change="(val: string) => { if (val) { handleAssignCategory(row.id, val); } }">
                <el-option v-for="option in categoryOptions" :key="option.value" :label="option.label"
                  :value="option.value" />
              </el-select>
            </template>
          </el-table-column>
        </el-table>
      </el-tab-pane>
      <el-tab-pane v-if="categoryStore.selectedCategory" :label="categoryStore.selectedCategory.name"
        name="categorized">
        <el-table :data="categoryApps" style="height: 100%;width: 100%">
          <el-table-column prop="name" :label="i18n.categoryPage.applicationName" show-overflow-tooltip />
          <el-table-column prop="path" :label="i18n.categoryPage.path" show-overflow-tooltip />
          <el-table-column :label="i18n.categoryPage.operation" width="150">
            <template #default="{ row }">
              <el-select v-model-value="categoryStore.selectedCategory?.id"
                :placeholder="i18n.categoryPage.selectCategory" clearable style="width: 120px" @change="(val: string | null) => {
                  if (val) {
                    handleAssignCategory(row.id, val);
                  } else {
                    handleRemoveAppFromCategory(row.id);
                  }
                }">
                <el-option v-for="option in categoryOptions" :key="option.value" :label="option.label"
                  :value="option.value" />
              </el-select>
            </template>
          </el-table-column>
        </el-table>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue"
import { i18n } from "@/script/i18n.ts"
import { categoryStore } from "@/script/state.ts"
import type { FileDetail } from "@/script/models.ts"

defineProps<{
  uncategorizedApps: FileDetail[]
  categoryApps: FileDetail[]
  categoryOptions: Array<{ label: string, value: string }>
}>()

const emit = defineEmits<{
  (e: 'assignCategory', appId: number, categoryId: string): void
  (e: 'removeAppFromCategory', appId: number): void
  (e: 'search', keyword: string): void
}>()

const searchKeyword = ref("")

const handleTabChange = (value: string | number) => {
  categoryStore.activeTab = value as "uncategorized" | "categorized"
}

const handleAssignCategory = (appId: number, categoryId: string) => {
  emit('assignCategory', appId, categoryId)
}

const handleRemoveAppFromCategory = (appId: number) => {
  emit('removeAppFromCategory', appId)
}

const handleSearch = () => {
  emit('search', searchKeyword.value)
}
</script>

<style scoped>
.category-apps {
  flex: 1;
  border: 1px solid #e4e7ed;
  border-radius: 4px;
  padding: 15px;
  min-width: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.search-box {
  margin-bottom: 15px;
  flex-shrink: 0;
}

.search-icon {
  font-size: 14px;
  line-height: 1;
}

:deep(.el-tabs__content) {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

:deep(.el-tab-pane) {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.el-table {
  flex-grow: 1;
  min-width: 0;
  width: 100% !important;
  height: 100%;
}

.el-table__body-wrapper {
  min-width: 0;
  overflow-y: auto;
}

.el-table__header-wrapper {
  min-width: 0;
}

.el-table__footer-wrapper {
  min-width: 0;
}
</style>