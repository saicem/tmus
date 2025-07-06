<script setup lang="ts">
import { getAppRule, setAppRule } from "@/script/cmd.ts"
import { onMounted, ref } from "vue"
import { RuleConfig } from "@/script/data.ts"
import FileSelector from "@/components/common/FileSelector.vue"
import { i18n } from "@/script/i18n.ts"

const rule = ref<RuleConfig>({
  exclude: [],
  include: [],
  merge: [],
})

const modified = ref<boolean>(false)

onMounted(async () => {
  const result = await getAppRule()
  result.exclude.sort((a, b) => a.path.localeCompare(b.path))
  result.include.sort((a, b) => a.path.localeCompare(b.path))
  result.merge.sort((a, b) => a.path.localeCompare(b.path))
  rule.value = result
})

const model = defineModel<boolean>()
const tabModel = ref("exclude")
const addItem = () => {
  modified.value = true
  if (tabModel.value === "exclude") {
    rule.value?.exclude.push({ path: "" })
  } else if (tabModel.value === "include") {
    rule.value?.include.push({ path: "" })
  } else if (tabModel.value === "merge") {
    rule.value?.merge.push({ path: "", toPath: "" })
  }
}

const handleClose = (done: () => void) => {
  if (!modified.value) {
    done()
    return
  }
  ElMessageBox({
    message: i18n.value.ruleDialog.modifiedTip,
    showCancelButton: true,
    confirmButtonText: i18n.value.ruleDialog.ok,
    cancelButtonText: i18n.value.ruleDialog.cancel,
  }).then(() => {
    done()
  })
}
</script>

<template>
  <el-dialog
    :title="i18n.ruleDialog.excludeApp"
    v-model="model"
    :before-close="handleClose"
  >
    <el-tabs v-model="tabModel">
      <el-tab-pane :label="i18n.ruleDialog.excludeApp" name="exclude">
        <el-table :data="rule?.exclude" style="width: 100%" max-height="300">
          <el-table-column prop="path" :label="i18n.ruleDialog.path">
            <template #default="scope">
              <FileSelector
                name="file"
                v-model="scope.row.path"
                :change="() => (modified = true)"
              />
            </template>
          </el-table-column>
          <el-table-column
            fixed="right"
            :label="i18n.ruleDialog.operation"
            width="120"
          >
            <template #default="scope">
              <el-button
                link
                type="primary"
                size="small"
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
          <el-table-column prop="path" :label="i18n.ruleDialog.path">
            <template #default="scope">
              <FileSelector name="file" v-model="scope.row.path" />
            </template>
          </el-table-column>
          <el-table-column
            fixed="right"
            :label="i18n.ruleDialog.operation"
            width="120"
          >
            <template #default="scope">
              <el-button
                link
                type="primary"
                size="small"
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
          <el-table-column prop="path" :label="i18n.ruleDialog.path">
            <template #default="scope">
              <FileSelector name="file" v-model="scope.row.path" />
            </template>
          </el-table-column>
          <el-table-column prop="toPath" :label="i18n.ruleDialog.mergedPath">
            <template #default="scope">
              <FileSelector name="file" v-model="scope.row.toPath" />
            </template>
          </el-table-column>
          <el-table-column
            fixed="right"
            :label="i18n.ruleDialog.operation"
            width="120"
          >
            <template #default="scope">
              <el-button
                link
                type="primary"
                size="small"
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
        <el-button @click="model = false"
          >{{ i18n.ruleDialog.cancel }}
        </el-button>
        <el-button
          type="primary"
          @click="
            () => {
              setAppRule(rule)
              model = false
            }
          "
          >{{ i18n.ruleDialog.ok }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<style scoped></style>
