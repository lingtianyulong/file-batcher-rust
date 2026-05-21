<script setup lang="ts">

import { onMounted, onUnmounted } from "vue";
import "element-plus/dist/index.css";
import {invoke} from "@tauri-apps/api/core";

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
  <router-view />
</template>


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