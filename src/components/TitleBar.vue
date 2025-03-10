<script setup lang="ts">
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow"

const appWindow = getCurrentWebviewWindow()
</script>

<template>
  <div data-tauri-drag-region class="title-bar">
    <span></span>
    <div class="title-bar-button-box">
      <div class="title-bar-button" @click="appWindow.minimize">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="1em"
          height="1em"
          viewBox="0 0 24 24"
        >
          <path class="title-bar-icon" d="M20 14H4v-4h16" />
        </svg>
      </div>
      <div class="title-bar-button" @click="appWindow.toggleMaximize">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="1em"
          height="1em"
          viewBox="0 0 24 24"
        >
          <path class="title-bar-icon" d="M4 4h16v16H4zm2 4v10h12V8z" />
        </svg>
      </div>
      <div class="title-bar-button" @click="appWindow.close">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="1em"
          height="1em"
          viewBox="0 0 24 24"
        >
          <path
            class="title-bar-icon"
            d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12z"
          />
        </svg>
      </div>
    </div>
  </div>
</template>

<style scoped>
.title-bar-icon {
  fill: var(--font-color-bold);
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
