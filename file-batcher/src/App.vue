<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from "vue";
import { useRoute } from "vue-router";
import "element-plus/dist/index.css";
import TitleBar from "./components/TitleBar.vue";
import {invoke} from "@tauri-apps/api/core";
import LoginBar from "./components/LoginBar.vue";

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
});

</script>

<template>
  <main class="page">
    <TitleBar />
    <LoginBar />
  </main>
  <!-- <main class="page">
    <TitleBar />
    <div :style="{width: '100%', height: '20px', color: 'red', paddingRight: '20px',
          display: 'flex', justifyContent: 'flex-end', alignItems: 'center'}">
      aaa
    </div> -->
    <!-- <template v-if="isBatchWindow">
      <div class="batch-window-content">
        <BatchPage />
      </div>
    </template>
    <template v-else>
      <el-container class="layout">
        <el-aside :width="isCollapsed ? '64px' : '150px'" class="sidebar">
          <div class="sidebar-toggle">
            <el-button text class="sidebar-toggle-btn" @click="toggleSidebar">
              <el-icon>
                <Expand v-if="isCollapsed" />
                <Fold v-else />
              </el-icon>
            </el-button>
          </div>
          <el-menu
            :default-active="activeMenu"
            :collapse="isCollapsed"
            class="menu"
            @select="handleMenuSelect"
          >
            <el-menu-item index="home">
              <el-icon><HomeFilled /></el-icon>
              <template #title>首页</template>
            </el-menu-item>
            <el-menu-item index="settings">
              <el-icon><Setting /></el-icon>
              <template #title>设置</template>
            </el-menu-item>
          </el-menu>
        </el-aside>

        <el-main class="content">
          <HomePage v-show="activeMenu === 'home'" />
          <el-card v-show="activeMenu === 'settings'" class="card" shadow="hover">
            <template #header>
              <div class="card-header">
                <span>设置</span>
              </div>
            </template>
            <el-empty description="设置功能开发中" />
          </el-card>
        </el-main>
      </el-container>
    </template> -->
  <!-- </main> -->
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

.sidebar {
  border-right: 1px solid #dcdfe6;
  background: #ffffff;
  transition: width 0.2s ease;
  overflow: hidden;   /* 移除滚动条 */
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