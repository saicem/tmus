<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window"
import minimize from "@/assets/title-bar/minimize.svg"
import maximize from "@/assets/title-bar/maximize.svg"
import close from "@/assets/title-bar/close.svg"
import { useDark, useToggle } from "@vueuse/core"

const isDark = useDark({
  selector: "html",
  attribute: "class",
  valueDark: "dark",
  valueLight: "light",
})
const toggleDark = useToggle(isDark)
</script>

<template>
  <div data-tauri-drag-region class="title-bar">
    <nav class="nav">
      <RouterLink class="nav-item" to="/">Home</RouterLink>
      <RouterLink class="nav-item" to="timeline">Timeline</RouterLink>
    </nav>
    <div
        @click="
          () => {
            toggleDark()
          }
        "
      >
        toggle
      </div>
    <div class="title-bar-button-box">
      <div class="title-bar-button" @click="appWindow.minimize">
        <img :src="minimize" alt="minimize" />
      </div>
      <div class="title-bar-button" @click="appWindow.toggleMaximize">
        <img :src="maximize" alt="maximize" />
      </div>
      <div class="title-bar-button" @click="appWindow.close">
        <img :src="close" alt="close" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.nav {
  margin-left: 20px;
  justify-content: start;
}

.nav-item {
  margin: 10px;
}

.search-box {
  height: 20px;
  width: 200px;
  line-height: 20px;
  font-size: small;
  text-align: center;
  color: var(--font-color);
}

.title-bar {
  height: 32px;
  user-select: none;
  display: flex;
  justify-content: space-between;
  align-items: center;
  top: 0;
  left: 0;
  right: 0;
  z-index: 999;
  background-color: var(--base-color-2);
  position: fixed;
  border-bottom: 1px solid var(--base-color-3);
}

.title-bar-button-box {
  display: flex;
  flex-direction: row;
  gap: 2px;
  margin-right: 6px;
}

.title-bar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}

.title-bar-button:hover {
  background-color: var(--accent-color);
}
</style>
