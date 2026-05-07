<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from "vue";
import { useRoute } from "vue-router";
import "element-plus/dist/index.css";
import TitleBar from "./components/TitleBar.vue";
import {invoke} from "@tauri-apps/api/core";
import LoginBar from "./components/LoginBar.vue";
import HomePage from "./pages/home.vue";

const activeMenu = ref("home");
const route = useRoute();

const routeMenuMap: Record<string, string> = {
  "/home": "home",
  "/settings": "settings",
};

const MIN_ASIDE_WIDTH = 120;
const MAX_ASIDE_WIDTH = 420;
const leftAsideWidth = ref(200);
const rightAsideWidth = ref(200);
const isAsideResizing = ref(false);
let stopAsideResize: (() => void) | undefined;
let resizeAnimationFrame = 0;

const clampAsideWidth = (width: number) => {
  return Math.min(MAX_ASIDE_WIDTH, Math.max(MIN_ASIDE_WIDTH, width));
};

const startAsideResize = (side: "left" | "right", event: MouseEvent) => {
  event.preventDefault();

  const startX = event.clientX;
  const startWidth = side === "left" ? leftAsideWidth.value : rightAsideWidth.value;
  const previousCursor = document.body.style.cursor;
  const previousUserSelect = document.body.style.userSelect;
  let latestWidth = startWidth;

  isAsideResizing.value = true;
  document.body.style.cursor = "col-resize";
  document.body.style.userSelect = "none";

  const applyLatestWidth = () => {
    const width = clampAsideWidth(latestWidth);

    if (side === "left") {
      leftAsideWidth.value = width;
    } else {
      rightAsideWidth.value = width;
    }
  };

  const handleMouseMove = (moveEvent: MouseEvent) => {
    const deltaX = moveEvent.clientX - startX;
    latestWidth = side === "left" ? startWidth + deltaX : startWidth - deltaX;

    if (resizeAnimationFrame) {
      return;
    }

    resizeAnimationFrame = window.requestAnimationFrame(() => {
      resizeAnimationFrame = 0;
      applyLatestWidth();
    });
  };

  const handleMouseUp = () => {
    document.removeEventListener("mousemove", handleMouseMove);
    document.removeEventListener("mouseup", handleMouseUp);
    if (resizeAnimationFrame) {
      window.cancelAnimationFrame(resizeAnimationFrame);
      resizeAnimationFrame = 0;
      applyLatestWidth();
    }
    isAsideResizing.value = false;
    document.body.style.cursor = previousCursor;
    document.body.style.userSelect = previousUserSelect;
    stopAsideResize = undefined;
  };

  stopAsideResize?.();
  stopAsideResize = handleMouseUp;
  document.addEventListener("mousemove", handleMouseMove);
  document.addEventListener("mouseup", handleMouseUp);
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
    <el-container>
      <el-aside class="sidebar-left" :class="{ 'is-resizing': isAsideResizing }" :width="`${leftAsideWidth}px`">
        <div class="sidebar-toggle">ASide Left</div>
        <div class="aside-resize-handle aside-resize-handle-right" @mousedown="startAsideResize('left', $event)" />
      </el-aside>
      <el-main class="content">
        <HomePage />
      </el-main>
      <el-aside class="sidebar-rigth" :class="{ 'is-resizing': isAsideResizing }" :width="`${rightAsideWidth}px`">
        <div class="aside-resize-handle aside-resize-handle-left" @mousedown="startAsideResize('right', $event)" />
        <div class="sidebar-toggle">ASide Right</div>
      </el-aside>
    </el-container>
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