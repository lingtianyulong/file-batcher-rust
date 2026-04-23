<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { EditPen, Expand, Fold, HomeFilled, Setting } from "@element-plus/icons-vue";
import "element-plus/dist/index.css";
import RenamePage from "./pages/rename.vue";
import BatchPage from "./pages/batch.vue";
import TitleBar from "./components/TitleBar.vue";

const activeMenu = ref("rename");
const router = useRouter();
const route = useRoute();
const isCollapsed = ref(false);

const isBatchWindow = computed(() => route.path === "/batch");

const menuRouteMap: Record<string, string> = {
  rename: "/rename",
  settings: "/settings",
};

const routeMenuMap: Record<string, string> = {
  "/rename": "rename",
  "/settings": "settings",
};

watch(
  () => route.path,
  (path) => {
    activeMenu.value = routeMenuMap[path] ?? "rename";
  },
  { immediate: true },
);

function handleMenuSelect(index: string) {
  const targetPath = menuRouteMap[index] ?? "/rename";
  if (targetPath !== route.path) {
    void router.push(targetPath);
  }
}

function toggleSidebar() {
  isCollapsed.value = !isCollapsed.value;
}
</script>

<template>
  <main class="page">
    <TitleBar />
    <template v-if="isBatchWindow">
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
            <el-menu-item index="rename">
              <el-icon><HomeFilled /></el-icon>
              <span>首页</span>
            </el-menu-item>
            <el-menu-item index="settings">
              <el-icon><Setting /></el-icon>
              <span>设置</span>
            </el-menu-item>
          </el-menu>
        </el-aside>

        <el-main class="content">
          <RenamePage v-if="activeMenu === 'rename'" />
          <el-card v-if="activeMenu === 'settings'" class="card" shadow="hover">
            <template #header>
              <div class="card-header">
                <span>设置</span>
              </div>
            </template>
            <el-empty description="设置功能开发中" />
          </el-card>
        </el-main>
      </el-container>
    </template>
  </main>
</template>

<style scoped>
.page {
  height: 100vh;
  padding: 0;
  background: transparent;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.layout {
  flex: 1;
  overflow: hidden;
}

.sidebar {
  border-right: 1px solid rgba(180, 190, 210, 0.7);
  background: rgba(255, 255, 255, 0.62);
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
  --el-menu-bg-color: transparent;
  --el-menu-text-color: #2c3e50;
  --el-menu-active-color: #1a6fc4;
  --el-menu-hover-bg-color: rgba(0, 0, 0, 0.08);
  --el-menu-active-bg-color: rgba(26, 111, 196, 0.15);
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
  background: transparent;
}

.layout > .el-aside {
  background: transparent !important;
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