<script setup lang="ts">
import { getAllAppDetail } from "@/script/cmd.ts"
import { onMounted, ref } from "vue"
import { FileDetail } from "@/script/data.ts"
import { config } from "@/script/state.ts"

const appList = ref<FileDetail[]>([])

onMounted(async () => {
  let result = await getAllAppDetail()
  if (config.value.filterUninstalledApp) {
    result = result.filter((app) => app.exist)
  }
  appList.value = result
})
</script>

<template>
  <div style="display: flex; flex-wrap: wrap; gap: 16px">
    <RouterLink v-for="app in appList" :key="app.id" :to="'/detail/' + app.id">
      <el-card>
        <div
          style="
            height: 40px;
            width: 230px;
            flex: auto;
            display: flex;
            align-items: center;
            gap: 8px;
          "
        >
          <div>
            <el-image
              style="width: 32px; height: 32px"
              :src="app?.icon ?? ''"
            />
          </div>
          <p
            style="
              overflow: hidden;
              white-space: nowrap;
              text-overflow: ellipsis;
              width: 100%;
              font-weight: bold;
              text-align: center;
            "
          >
            {{ app?.name }}
          </p>
        </div>
      </el-card>
    </RouterLink>
  </div>
</template>

<style scoped></style>
