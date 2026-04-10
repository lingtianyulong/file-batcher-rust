<script setup lang="ts">
import { ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import "element-plus/dist/index.css";
import RenamePage from "./pages/rename.vue";

const activeMenu = ref("rename");
const router = useRouter();
const route = useRoute();

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
</script>

<template>
  <main class="page">
    <el-container class="layout" >
      <el-aside :width="'64px'" class="sidebar">
        <el-menu
          :default-active="activeMenu"
          :collapse="false"
          class="menu"
          @select="handleMenuSelect"
        >
          <el-menu-item index="rename">批量重命名</el-menu-item>
          <el-menu-item index="settings">设置</el-menu-item>
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
  </main>
</template>

<style scoped>
.page {
  height: 100vh;
  padding: 0;
  background: #f5f7fa;
  overflow: hidden;
}

.layout {
  height: 100%;
  overflow: hidden;
}

.sidebar {
  border-right: 1px solid #e4e7ed;
  background: #fff;
  transition: width 0.2s ease;
}

.sidebar-toggle {
  display: flex;
  justify-content: flex-end;
  padding: 8px 10px 4px;
}

.menu {
  border-right: none;
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
</style>

<style>
html,
body,
#app {
  height: 100%;
  margin: 0;
  overflow: hidden;
}
</style>