<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from "vue";
import { useRoute } from "vue-router";
import "element-plus/dist/index.css";
import TitleBar from "./components/TitleBar.vue";
import {invoke} from "@tauri-apps/api/core";
import LoginBar from "./components/LoginBar.vue";
import MainPage from "./pages/MainPage.vue";

const activeMenu = ref("home");
const route = useRoute();

const routeMenuMap: Record<string, string> = {
  "/home": "home",
  "/settings": "settings",
};

watch(
  () => route.path,
  (path) => {
    activeMenu.value = routeMenuMap[path] ?? "home";
  },
  { immediate: true },
);


const handleContextMenu = async (event: MouseEvent) => {
  event.preventDefault();
  // await invoke("prevent_context_menu");
  await invoke("show_contextmenu_command", { x: event.clientX, y: event.clientY });
};

onMounted(() => {
  document.addEventListener("contextmenu", handleContextMenu);
});

onUnmounted(() => {
  document.removeEventListener("contextmenu", handleContextMenu);
  stopAsideResize?.();
});

</script>

<template>
  <main class="page">
    <TitleBar />
    <LoginBar />
    <MainPage />
  </main>
</template>

<style scoped>
.page {
  height: 100vh;
  padding: 0;
  background: #ffffff;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.layout {
  flex: 1;
  overflow: hidden;
}

.sidebar-left {
  position: relative;
  flex-shrink: 0;
  border-right: 1px solid #dcdfe6;
  background: #ffffff;
  transition: width 0.2s ease;
  overflow: hidden;   /* 移除滚动条 */
}

.sidebar-rigth {
  position: relative;
  flex-shrink: 0;
  border-left: 1px solid #dcdfe6;
  background: #ffffff;
  transition: width 0.2s ease;
  overflow: hidden;   /* 移除滚动条 */
}

.sidebar-left.is-resizing,
.sidebar-rigth.is-resizing {
  transition: none;
}

.aside-resize-handle {
  position: absolute;
  top: 0;
  bottom: 0;
  width: 6px;
  cursor: col-resize;
  z-index: 1;
}

.aside-resize-handle-left {
  left: -3px;
}

.aside-resize-handle-right {
  right: -3px;
}

.sidebar-toggle {
  display: flex;
  justify-content: flex-end;
  padding: 8px 8px 4px;
}

.sidebar-toggle-btn {
  width: 32px;
  height: 32px;
  padding: 0;
}

.menu {
  border-right: none;
  --el-menu-bg-color: #ffffff;
  --el-menu-text-color: #2c3e50;
  --el-menu-active-color: #1a6fc4;
  --el-menu-hover-bg-color: #f5f7fa;
  --el-menu-active-bg-color: #ecf5ff;
}

.content {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 24px;
  overflow: hidden;
}

.card {
  width: 100%;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.batch-window-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}
</style>

<style>
html,
body,
#app {
  height: 100%;
  margin: 0;
  overflow: hidden;
  background: #ffffff;
}

.layout > .el-aside {
  background: #ffffff !important;
}

.layout > .el-aside .el-menu-item {
  color: #2c3e50;
  font-weight: 500;
}

.layout > .el-aside .el-menu-item .el-icon {
  color: #4a5568;
}

.layout > .el-aside .el-menu-item.is-active {
  color: #1a6fc4 !important;
  font-weight: 600;
}

.layout > .el-aside .el-menu-item.is-active .el-icon {
  color: #1a6fc4 !important;
}
</style>