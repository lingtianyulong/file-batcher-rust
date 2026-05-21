<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

async function handleBack() {
  try {
    await invoke("close_search_config_window_command");
  } catch (error) {
    console.error("关闭搜索配置窗口失败:", error);
  }
}
</script>

<template>
  <div class="search-config-page">
     <el-button @click="handleBack">返回</el-button>

  </div>
  <!-- <el-dialog
    v-model="visible"
    align-center
    class="search-config-dialog"
    title="搜索配置"
    show-close
    draggable
  > -->
    <!-- <div class="title_bar">
      <div class="title_bar_title">搜索配置</div>
      <el-button :icon="CloseBold" @click="closeDialog" />
    </div> -->
  <!-- </el-dialog> -->
  <!-- <div class="search-page">
    <h1>搜索配置</h1>
  </div> -->
</template>

<style scoped>
  .search-config-page {
    flex-direction: column;
    width: 100%;
    height: 100%;
  }

  .title_bar {
    width: 100%;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid #dcdfe6;
  }
</style>
