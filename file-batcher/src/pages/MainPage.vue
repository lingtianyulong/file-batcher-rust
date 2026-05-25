<script lang="ts" setup>
  import FolderTree from "./FolderTree.vue";
  import { useFolderStore } from "../stores/file-store";
  import { onMounted, onUnmounted, watch } from "vue";
  import { ref } from "vue";
  import { invoke } from "@tauri-apps/api/core";
  import * as icons from "@element-plus/icons-vue";
  import { message } from "@tauri-apps/plugin-dialog";
  import { join } from "@tauri-apps/api/path";
  import { openPath, revealItemInDir } from "@tauri-apps/plugin-opener";
  import { listen } from "@tauri-apps/api/event";

  const folderStore = useFolderStore();
  const loading = ref(false);
  const currentSelectedRow = ref<FileInfoRow | null>(null);

  type FileInfoRow = {
    fileName: string;
    filePath: string;
    fileType: string;
    fileSize: string;
    fileCreateTime: string;
    fileModifyTime: string;
  };

  type RawFileInfo = {
    file_name?: string;
    file_path?: string;
    file_type?: string;
    file_size?: string;
    file_create_time?: string;
    file_modify_time?: string;
  };

  const fileList = ref<FileInfoRow[]>([]);
  const selectedRows = ref<FileInfoRow[]>([]);
  let unlistenMenuEvent: (() => void) | undefined;

  /** 后端 file_path 为目录路径，同目录下多文件相同，不能单独作为 row-key */
  function getRowKey(row: FileInfoRow) {
    return `${row.filePath}\0${row.fileName}`;
  }

  function handleSelectionChange(rows: FileInfoRow[]) {
    selectedRows.value = rows;
  }

  async function loadFileList(filePath: string) {
    if (!filePath) {
      console.log("loadFileList, filePath is empty");
      fileList.value = [];
      selectedRows.value = [];
      return;
    }

    if (filePath.length === 0) {
      console.log("loadFileList, filePath is empty");
      fileList.value = [];
      selectedRows.value = [];
      return;
    }

    const file_list = await invoke<string>("get_file_list_command", {
      filePath: filePath,
    });
    const parsed = JSON.parse(file_list) as RawFileInfo[];
    fileList.value = parsed.map((file) => ({
      fileName: file.file_name ?? "",
      filePath: file.file_path ?? "",
      fileType: file.file_type ?? "",
      fileSize: file.file_size ?? "",
      fileCreateTime: file.file_create_time ?? "",
      fileModifyTime: file.file_modify_time ?? "",
    }));
    selectedRows.value = [];
  }

  function formatDisplayTime(value: string) {
    return value.replace(" ", "\u00A0\u00A0");
  }

  async function handlePreview(row: FileInfoRow) {
    if (!row.fileName || !row.filePath) {
      await message("文件路径不完整，无法预览", {
        title: "警告",
        kind: "warning",
      });
      return;
    }
    try {
      const fullPath = await join(row.filePath, row.fileName);
      await openPath(fullPath);
    } catch (error) {
      await message(`预览失败：${String(error)}`, {
        title: "错误",
        kind: "error",
      });
    }
  }

  onMounted(async () => {
    if (folderStore.currentPath) {
      console.log("onMounted in main page", folderStore.currentPath);
      loading.value = true;
      loadFileList(folderStore.currentPath).finally(() => {
        loading.value = false;
      });
    }

    unlistenMenuEvent = await listen("menu_event", async (event) => {
      console.log("menu event in main page", event);
      const command = event.payload as string;
      switch (command) {
        case "open":
          console.log("open command in main page", command);
          if (currentSelectedRow.value) {
            const filePath = await join(
              currentSelectedRow.value.filePath,
              currentSelectedRow.value.fileName,
            );
            console.log("open command in main page filePath", filePath);
            await openPath(filePath);
          }
          break;
        case "open_folder":
          console.log(
            "open_folder command in main page",
            currentSelectedRow.value,
          );
          if (currentSelectedRow.value) {
            const dirPath = await join(
              currentSelectedRow.value.filePath,
              currentSelectedRow.value.fileName,
            );
            console.log("open_folder command in main page dirPath", dirPath);
            await revealItemInDir(dirPath);
          }
          break;
      }
    });
  });

  onUnmounted(() => {
    unlistenMenuEvent?.();
    unlistenMenuEvent = undefined;
  });

  watch(
    () => folderStore.currentPath,
    (newPath) => {
      if (newPath) {
        console.log("watch new path in main page", newPath);
        loading.value = true;
        loadFileList(newPath).finally(() => {
          loading.value = false;
        });
      }
    },
  );

  //   const unlisten = await listen("menu_event", (event) => {
  //     listen("menu_event", (event) => {
  //       console.log("menu event in main page", event);
  //     });
  //   });

  function handleRowContextmenu(
    row: FileInfoRow,
    column: any,
    event: PointerEvent,
  ) {
    event.preventDefault();
    console.log("handleCellContextmenu", row, column, event);
    currentSelectedRow.value = row;
    // 暂时使用系统菜单实现, 后续再使用 Floating UI 实现
    invoke("show_contextmenu_command", {
      x: event.clientX,
      y: event.clientY,
    }).finally(() => {
      currentSelectedRow.value = null;
    });
  }
</script>

<template>
  <el-container class="main-page-root">
    <el-aside class="left-siderbar siderbar">
      <FolderTree />
    </el-aside>
    <el-container class="main-page-right">
      <el-main class="main-page-main">
        <div class="file-list-container">
          <el-table
            class="file-list-table"
            :data="fileList"
            border
            :row-key="getRowKey"
            empty-text="暂无文件数据"
            height="100%"
            style="width: 100%; min-width: 50rem"
            @selection-change="handleSelectionChange"
            :header-cell-style="{ textAlign: 'center' }"
            v-loading="loading"
            element-loading-text="加载中..."
            @row-contextmenu="handleRowContextmenu"
          >
            <el-table-column type="selection" width="48" align="center" />
            <el-table-column
              prop="fileName"
              label="文件名"
              show-overflow-tooltip
              min-width="140"
            />
            <el-table-column
              prop="fileType"
              label="文件类型"
              align="center"
              width="100"
            />
            <el-table-column
              prop="fileSize"
              label="文件大小"
              align="center"
              width="100"
            />
            <el-table-column
              prop="fileCreateTime"
              label="创建时间"
              align="center"
              min-width="150"
              :width="200"
            >
              <template #default="scoped">
                {{ formatDisplayTime(scoped.row.fileCreateTime) }}
              </template>
            </el-table-column>
            <el-table-column
              prop="fileModifyTime"
              label="修改时间"
              align="center"
              min-width="150"
              :width="200"
            >
              <template #default="scoped">
                {{ formatDisplayTime(scoped.row.fileModifyTime) }}
              </template>
            </el-table-column>
            <el-table-column
              label="操作"
              align="center"
              width="100"
              fixed="right"
            >
              <template #default="scoped">
                <el-button
                  type="primary"
                  text
                  size="small"
                  style="font-size: 15px"
                  :icon="icons.View"
                  @click="handlePreview(scoped.row)"
                >
                  预览
                </el-button>
              </template>
            </el-table-column>
          </el-table>
        </div>
      </el-main>
      <el-footer class="footer">Footer</el-footer>
    </el-container>
  </el-container>
</template>

<!-- style scoped 只在当前组件中生效 -->
<style scoped>
  .main-page-root {
    flex: 1;
    min-height: 0;
    width: 100%;
    overflow: hidden;
  }

  .main-page-right {
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }

  .main-page-main {
    flex: 1;
    min-height: 0;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .left-siderbar {
    border-right: 1px solid #dcdfe6;
  }

  .footer {
    border-top: 1px solid #dcdfe6;
  }

  .siderbar {
    background: #ffffff;
    transition: width 0.2s ease;
    overflow: hidden; /* 移除滚动条 */
  }

  .file-list-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
    overflow: hidden;
  }

  .file-list-table {
    flex: 1 1 0;
    min-height: 0;
    overflow: hidden;
  }
</style>
