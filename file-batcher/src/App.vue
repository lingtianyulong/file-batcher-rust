<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from "element-plus";
import "element-plus/dist/index.css";

const greetMsg = ref("");
const name = ref("");
const loading = ref(false);

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
  </main>
</template>

<style scoped>
.page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
  background: #f5f7fa;
}

.card {
  width: 100%;
  max-width: 560px;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
</style>