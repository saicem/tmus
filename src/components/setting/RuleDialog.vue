<script lang="ts" setup>
import { getAppRule, setAppRule } from "@/script/cmd.ts"
import { RuleConfig } from "@/script/models.ts"
import { i18n } from "@/script/i18n.ts"

const rule = ref<RuleConfig>({
  exclude: [],
  include: [],
  merge: [],
})

const modified = ref<boolean>(false)
const model = defineModel<boolean>()
const tabModel = ref("exclude")

onMounted(async () => {
  const result = await getAppRule()
  result.exclude.sort((a, b) => a.path.localeCompare(b.path))
  result.include.sort((a, b) => a.path.localeCompare(b.path))
  result.merge.sort((a, b) => a.path.localeCompare(b.path))
  rule.value = result
})

function addItem() {
  modified.value = true
  if (tabModel.value === "exclude") {
    rule.value?.exclude.push({ path: "" })
  } else if (tabModel.value === "include") {
    rule.value?.include.push({ path: "" })
  } else if (tabModel.value === "merge") {
    rule.value?.merge.push({ path: "", toPath: "" })
  }
}

function handleClose(done: () => void) {
  if (!modified.value) {
    done()
    return
  }
  ElMessageBox({
    message: i18n.value.ruleDialog.modifiedTip,
    showCancelButton: true,
    confirmButtonText: i18n.value.common.ok,
    cancelButtonText: i18n.value.common.cancel,
  }).then(() => {
    done()
  })
}

function setRule() {
  setAppRule(rule.value)
  model.value = false
}
</script>

<template>
  <el-dialog
    v-model="model"
    :before-close="handleClose"
    :title="i18n.configPage.appRule"
  >
    <el-tabs v-model="tabModel">
      <el-tab-pane :label="i18n.ruleDialog.excludeApp" name="exclude">
        <el-table :data="rule?.exclude" max-height="300" style="width: 100%">
          <el-table-column :label="i18n.ruleDialog.path" prop="path">
            <template #default="scope">
              <file-selector
                v-model="scope.row.path"
                :change="() => (modified = true)"
                name="file"
              />
            </template>
          </el-table-column>
          <el-table-column
            :label="i18n.ruleDialog.operation"
            fixed="right"
            width="120"
          >
            <template #default="scope">
              <el-button
                link
                size="small"
                type="primary"
                @click.prevent="
                  () => {
                    rule?.exclude.splice(scope.$index, 1)
                    modified = true
                  }
                "
              >
                {{ i18n.ruleDialog.remove }}
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-tab-pane>

      <el-tab-pane :label="i18n.ruleDialog.includeApp" name="include">
        <el-table :data="rule?.include" style="width: 100%">
          <el-table-column :label="i18n.ruleDialog.path" prop="path">
            <template #default="scope">
              <FileSelector v-model="scope.row.path" name="file" />
            </template>
          </el-table-column>
          <el-table-column
            :label="i18n.ruleDialog.operation"
            fixed="right"
            width="120"
          >
            <template #default="scope">
              <el-button
                link
                size="small"
                type="primary"
                @click.prevent="
                  () => {
                    rule?.include.splice(scope.$index, 1)
                    modified = true
                  }
                "
              >
                {{ i18n.ruleDialog.remove }}
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-tab-pane>

      <el-tab-pane :label="i18n.ruleDialog.mergeApp" name="merge">
        <el-table :data="rule?.merge" style="width: 100%">
          <el-table-column :label="i18n.ruleDialog.path" prop="path">
            <template #default="scope">
              <FileSelector v-model="scope.row.path" name="file" />
            </template>
          </el-table-column>
          <el-table-column :label="i18n.ruleDialog.mergedPath" prop="toPath">
            <template #default="scope">
              <FileSelector v-model="scope.row.toPath" name="file" />
            </template>
          </el-table-column>
          <el-table-column
            :label="i18n.ruleDialog.operation"
            fixed="right"
            width="120"
          >
            <template #default="scope">
              <el-button
                link
                size="small"
                type="primary"
                @click.prevent="
                  () => {
                    rule?.merge.splice(scope.$index, 1)
                    modified = true
                  }
                "
              >
                {{ i18n.ruleDialog.remove }}
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-tab-pane>
    </el-tabs>
    <template #footer>
      <div>
        <el-button @click="addItem">{{ i18n.ruleDialog.add }}</el-button>
        <el-button @click="model = false">{{ i18n.common.cancel }}</el-button>
        <el-button type="primary" @click="setRule"
          >{{ i18n.common.ok }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<style scoped></style>
