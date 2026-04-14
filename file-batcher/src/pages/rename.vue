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

const fileInfo = ref<FileInfoRow[]>([]);
const renameDialogVisible = ref(false);
const renameFileName = ref("");
const currentRenameRow = ref<FileInfoRow | null>(null);

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

  const file_info = await invoke<string>("get_file_info_command", { filePath: file_path });
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
  const file_list = await invoke<string>("get_file_list_command", { filePath: selectedPath });
  const parsed = JSON.parse(file_list) as {
    file_name?: string;
    file_path?: string;
    file_type?: string;
    file_size?: string;
    file_create_time?: string;
    file_modify_time?: string;
  }[];

  fileInfo.value = parsed.map((file: any) => ({
    fileName: file.file_name ?? "",
    filePath: file.file_path ?? "",
    fileType: file.file_type ?? "",
    fileSize: file.file_size ?? "",
    fileCreateTime: file.file_create_time ?? "",
    fileModifyTime: file.file_modify_time ?? "",
  }));

}

async function handleRename(row: FileInfoRow) {
  console.log("handleRename", row);
  currentRenameRow.value = row;
  renameFileName.value = row.fileName;
  renameDialogVisible.value = true;
}

async function handleRenameConfirm() {
  if (!currentRenameRow.value) {
    ElMessage.warning("未找到要重命名的文件");
    return;
  }

  const newName = renameFileName.value.trim();
  if (!newName) {
    ElMessage.info("请输入新的文件名");
    return;
  }

  if (newName === currentRenameRow.value.fileName) {
    ElMessage.info("新文件名不能与原文件名相同");
    return;
  }

  try {
    const renamedFileInfo = await invoke<string>("rename_file_command", {
      fileDir: currentRenameRow.value.filePath,
      oldFileName: currentRenameRow.value.fileName,
      newFileName: newName,
    });
    const parsed = JSON.parse(renamedFileInfo) as {
      file_name?: string;
      file_path?: string;
      file_type?: string;
      file_size?: string;
      file_create_time?: string;
      file_modify_time?: string;
    };

    currentRenameRow.value.fileName = parsed.file_name ?? "";
    currentRenameRow.value.filePath = parsed.file_path ?? "";
    currentRenameRow.value.fileType = parsed.file_type ?? "";
    currentRenameRow.value.fileSize = parsed.file_size ?? "";
    currentRenameRow.value.fileCreateTime = parsed.file_create_time ?? "";
    currentRenameRow.value.fileModifyTime = parsed.file_modify_time ?? "";

    renameDialogVisible.value = false;
    renameFileName.value = "";
    currentRenameRow.value = null;
    ElMessage.success("文件重命名成功");
  } catch (error) {
    ElMessage.error(String(error));
  }
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
        <template #default="scoped">
          <el-button link type="primary" :icon="icons.Edit" @click="handleRename(scoped.row)">重命名</el-button>
          <el-button link type="danger" :icon="icons.Delete">删除</el-button>
        </template>
      </el-table-column>
    </el-table>

    <el-dialog v-model="renameDialogVisible" title="重命名文件" width="480px">
      <el-form label-width="100px">
        <el-form-item label="新文件名">
          <el-input
            v-model="renameFileName"
            placeholder="请输入新的文件名"
            clearable
            @keyup.enter="handleRenameConfirm"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="renameDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleRenameConfirm">确认</el-button>
      </template>
    </el-dialog>
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
