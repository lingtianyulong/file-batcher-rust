<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from "element-plus";
import "element-plus/dist/index.css";
import RenamePage from "./pages/rename.vue";

const greetMsg = ref("");
const name = ref("");
const loading = ref(false);
const isMenuCollapsed = ref(false);
const activeMenu = ref("rename");

async function greet() {
  if (!name.value.trim()) {
    ElMessage.warning("请输入姓名后再提交");
    return;
  }

  loading.value = true;
  try {
    greetMsg.value = await invoke("greet", { name: name.value });
    ElMessage.success("调用成功");
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <main class="page">
    <el-container class="layout">
      <el-aside :width="'64px'" class="sidebar">
        <el-menu
          v-model="activeMenu"
          :default-active="activeMenu"
          :collapse="false"
          class="menu"
        >
          <el-menu-item index="home">首页</el-menu-item>
          <el-menu-item index="rename">批量重命名</el-menu-item>
          <el-menu-item index="settings">设置</el-menu-item>
        </el-menu>
      </el-aside>

      <el-main class="content">
        <el-card class="card" shadow="hover">
          <template #header>
            <div class="card-header">
              <span>Tauri + Vue + Element</span>
              <el-tag type="success">Ready</el-tag>
            </div>
          </template>

          <el-form @submit.prevent="greet">
            <el-form-item label="姓名">
              <el-input v-model="name" placeholder="请输入姓名" clearable />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" :loading="loading" @click="greet">
                Greet
              </el-button>
            </el-form-item>
          </el-form>

          <el-alert
            v-if="greetMsg"
            :title="greetMsg"
            type="success"
            :closable="false"
            show-icon
          />
        </el-card>

        <RenamePage />
      </el-main>
    </el-container>
  </main>
</template>

<style scoped>
.page {
  min-height: 100vh;
  padding: 0;
  background: #f5f7fa;
}

.layout {
  min-height: 100vh;
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