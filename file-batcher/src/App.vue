<script setup lang="ts">

import { onMounted, onUnmounted } from "vue";
import "element-plus/dist/index.css";
import TitleBar from "./components/TitleBar.vue";
import {invoke} from "@tauri-apps/api/core";
import LoginBar from "./components/LoginBar.vue";
import MainPage from "./pages/MainPage.vue";

const handleContextMenu = async (event: MouseEvent) => {
  event.preventDefault();
  await invoke("show_contextmenu_command", { x: event.clientX, y: event.clientY });
};

onMounted(() => {
  document.addEventListener("contextmenu", handleContextMenu);
});

onUnmounted(() => {
  document.removeEventListener("contextmenu", handleContextMenu);
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