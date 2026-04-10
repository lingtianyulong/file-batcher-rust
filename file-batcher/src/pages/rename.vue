<script setup lang="ts">
import { ref } from "vue";
import { ElMessage } from "element-plus";
import * as icons from "@element-plus/icons-vue";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

type FileInfoRow = {
  fileName: string;
  filePath: string;
  fileType: string;
  fileSize: string;
  fileCreateTime: string;
  fileModifyTime: string;
};

// const prefix = ref("");
// const startNumber = ref(1);
// const paddingLength = ref(3);
// const previewCount = ref(5);
const fileInfo = ref<FileInfoRow[]>([]);

// const previewNames = computed(() => {
//   const total = Math.max(1, previewCount.value);
//   const start = Math.max(0, startNumber.value);
//   const padding = Math.max(1, paddingLength.value);
//   const safePrefix = prefix.value.trim();

//   return Array.from({ length: total }, (_, index) => {
//     const current = String(start + index).padStart(padding, "0");
//     return `${safePrefix}${current}`;
//   });
// });

// function handleApplyRename() {
//   ElMessage.info("这是重命名页面占位逻辑，后续可接入 Tauri 实际重命名命令。");
// }

async function handleOpenFile() {
  // 打开文件选择对话框并获取文件名
  const file_path = await open({
    multiple: false,
    directory: false,
  });

  if (!file_path || Array.isArray(file_path)) {
    ElMessage.info("未选择单个文件.");
    return;
  }

  const file_info = await invoke<string>("get_file_info", { filePath: file_path });
  const parsed = JSON.parse(file_info) as {
    file_name?: string;
    file_path?: string;
    file_type?: string;
    file_size?: string;
    file_create_time?: string;
    file_modify_time?: string;
  };
  fileInfo.value = [
    {
      fileName: parsed.file_name ?? "",
      filePath: parsed.file_path ?? "",
      fileType: parsed.file_type ?? "",
      fileSize: parsed.file_size ?? "",
      fileCreateTime: parsed.file_create_time ?? "",
      fileModifyTime: parsed.file_modify_time ?? "",
    },
  ];
}

async function handleOpenFolder() {
  const selectedPath = await open({
    directory: true,
    multiple: false,
  });

  if (!selectedPath) {
    ElMessage.info("已取消选择文件夹。");
    return;
  }

  ElMessage.success(`已选择文件夹: ${selectedPath}`);
}
</script>

<template>
  <el-card class="rename-card" shadow="never">
    <template #header>
      <div class="rename-header">
        <span>文件重命名</span>
      </div>
    </template>
    <el-form>
      <el-button type="primary" :icon="icons.DocumentAdd" @click="handleOpenFile">打开文件</el-button>
      <el-button type="default" :icon="icons.FolderOpened" @click="handleOpenFolder">打开文件夹</el-button>

    </el-form>
    <el-divider/>
    <el-table :data="fileInfo" style="width: 100%" border :header-cell-style="{ textAlign: 'center' }">
      <el-table-column type="index" label="序号" width="80" align="center"/>
      <el-table-column prop="fileName" label="文件名" align="center" show-overflow-tooltip/>
      <el-table-column prop="filePath" label="文件路径" show-overflow-tooltip />
      <el-table-column prop="fileType" label="文件类型" align="center" width="100"/>
      <el-table-column prop="fileSize" label="文件大小" align="center" width="100"/>
      <el-table-column prop="fileCreateTime" label="创建时间" align="center" show-overflow-tooltip/>
      <el-table-column prop="fileModifyTime" label="修改时间" align="center" show-overflow-tooltip/>
      <el-table-column label="操作" align="center" width="200px" fixed="right">
        <template #default>
          <el-button link type="primary" :icon="icons.Edit">重命名</el-button>
          <el-button link type="danger" :icon="icons.Delete">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
  </el-card>
</template>

<style scoped>
.rename-card {
  width: 100%;
  height: 100%;
}

.rename-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
</style>
