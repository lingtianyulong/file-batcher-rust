<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import appIcon from "../assets/icon.png";
import { getCurrentWindow } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();
const isMaximized = ref(false);

async function updateMaximized() {
  isMaximized.value = await appWindow.isMaximized();
}

function minimize() {
  void appWindow.minimize();
}

function toggleMaximize() {
  void appWindow.toggleMaximize();
}

function close() {
  void appWindow.close();
}

onMounted(async () => {
  await updateMaximized();
  const unlisten = await appWindow.onResized(() => {
    void updateMaximized();
  });
  onUnmounted(() => unlisten());
});
</script>

<template>
  <div class="titlebar" data-tauri-drag-region>
    <div class="titlebar-title" data-tauri-drag-region>
      <img :src="appIcon" class="titlebar-icon" alt="icon" data-tauri-drag-region />
      <span class="titlebar-app-name" data-tauri-drag-region>FileBatcher</span>
    </div>
    <div class="titlebar-controls">
      <button class="titlebar-btn btn-minimize" @click="minimize" title="最小化">
        <svg width="10" height="1" viewBox="0 0 10 1"><rect width="10" height="1" fill="currentColor"/></svg>
      </button>
      <button class="titlebar-btn btn-maximize" @click="toggleMaximize" :title="isMaximized ? '还原' : '最大化'">
        <svg v-if="!isMaximized" width="10" height="10" viewBox="0 0 10 10"><rect x="0.5" y="0.5" width="9" height="9" fill="none" stroke="currentColor"/></svg>
        <svg v-else width="10" height="10" viewBox="0 0 1024 1024" fill="currentColor"><path d="M959.72 0H294.216a63.96 63.96 0 0 0-63.96 63.96v127.92H64.28A63.96 63.96 0 0 0 0.32 255.84V959.4a63.96 63.96 0 0 0 63.96 63.96h703.56a63.96 63.96 0 0 0 63.96-63.96V792.465h127.92a63.96 63.96 0 0 0 63.96-63.96V63.96A63.96 63.96 0 0 0 959.72 0zM767.84 728.505V959.4H64.28V255.84h703.56z m189.322 0H831.8V255.84a63.96 63.96 0 0 0-63.96-63.96H294.216V63.96H959.72z"/></svg>
      </button>
      <button class="titlebar-btn btn-close" @click="close" title="关闭">
        <svg width="10" height="10" viewBox="0 0 10 10"><line x1="0" y1="0" x2="10" y2="10" stroke="currentColor" stroke-width="1.2"/><line x1="10" y1="0" x2="0" y2="10" stroke="currentColor" stroke-width="1.2"/></svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  --titlebar-bg: rgba(255, 255, 255, 0.5);
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 36px;
  padding: 0 0 0 16px;
  background: var(--titlebar-bg);
  border-bottom: 1px solid rgba(228, 231, 237, 0.6);
  user-select: none;
  flex-shrink: 0;
}

.titlebar-title {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 8px;
  overflow: hidden;
}

.titlebar-icon {
  width: 16px;
  height: 16px;
  object-fit: contain;
  flex-shrink: 0;
}

.titlebar-app-name {
  font-size: 13px;
  font-weight: 600;
  color: #303133;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.titlebar-controls {
  display: flex;
  align-items: center;
  height: 100%;
}

.titlebar-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 46px;
  height: 100%;
  border: none;
  background: transparent;
  color: #606266;
  cursor: pointer;
  transition: background 0.15s ease, color 0.15s ease;
  padding: 0;
  outline: none;
}

.titlebar-btn:hover {
  background: rgba(0, 0, 0, 0.08);
  color: #303133;
}

.btn-close:hover {
  background: #e81123;
  color: #fff;
}
</style>
