<script setup lang="ts">
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow"
import minimize from "@/assets/title-bar/minimize.svg"
import maximize from "@/assets/title-bar/maximize.svg"
import close from "@/assets/title-bar/close.svg"

const appWindow = getCurrentWebviewWindow()
</script>

<template>
  <div data-tauri-drag-region class="title-bar">
    <span></span>
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
}

.title-bar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 40px;
  height: 32px;
}

.title-bar-button:hover {
  background-color: var(--accent-color);
}
</style>
