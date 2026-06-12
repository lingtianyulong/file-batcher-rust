<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from "vue";
import { ElMessage, ElMessageBox } from "element-plus";
import * as icons from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api/core";
import { emitTo, listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

type FileInfoRow = {
  fileName: string;
  filePath: string;
  fileType: string;
  fileSize: string;
  fileCreateTime: string;
  fileModifyTime: string;
};

type RenameMode = "serial" | "prefix" | "suffix" | "replace";

const MAIN_WINDOW_LABEL = "main";
const EVENT_REQUEST_FILES = "batch:request-files";
const EVENT_FILES = "batch:files";
const EVENT_FILE_UPDATED = "batch:file-updated";
const EVENT_FINISHED = "batch:finished";

const fileList = ref<FileInfoRow[]>([]);
const activeTab = ref<"rename" | "convert">("rename");

const selectedRows = ref<FileInfoRow[]>([]);
const tableRef = ref<any>(null);

let unlistenFiles: UnlistenFn | null = null;

function handleSelectionChange(rows: FileInfoRow[]) {
  selectedRows.value = rows;
}

function selectAllRows() {
  if (tableRef.value && fileList.value.length > 0) {
    fileList.value.forEach((row) =>
      tableRef.value.toggleRowSelection(row, true),
    );
  }
}

onMounted(async () => {
  // 监听主窗口推送的文件列表
  unlistenFiles = await listen<FileInfoRow[]>(EVENT_FILES, (event) => {
    fileList.value = Array.isArray(event.payload) ? event.payload.slice() : [];
    // 等待渲染完成后默认全选
    setTimeout(() => selectAllRows(), 0);
  });

  // 通知主窗口：本窗口已就绪，请发送文件列表
  try {
    await emitTo(MAIN_WINDOW_LABEL, EVENT_REQUEST_FILES, null);
  } catch (e) {
    console.error("request files from main window failed:", e);
  }
});

onBeforeUnmount(() => {
  if (unlistenFiles) {
    unlistenFiles();
    unlistenFiles = null;
  }
});

// ============ 批量重命名 ============
const renameMode = ref<RenameMode>("serial");

const serialPrefix = ref("file_");
const serialSuffix = ref("");
const serialStart = ref(1);
const serialPadding = ref(3);

const addPrefix = ref("");
const addSuffix = ref("");

const findText = ref("");
const replaceText = ref("");

function splitFileName(name: string): { base: string; ext: string } {
  const idx = name.lastIndexOf(".");
  if (idx <= 0 || idx === name.length - 1) {
    return { base: name, ext: "" };
  }
  return { base: name.slice(0, idx), ext: name.slice(idx) };
}

function padNumber(n: number, width: number): string {
  const s = String(n);
  if (width <= s.length) return s;
  return "0".repeat(width - s.length) + s;
}

const renamePreview = computed(() => {
  const rows = selectedRows.value;
  return rows.map((row, index) => {
    const { base, ext } = splitFileName(row.fileName);
    let newName = row.fileName;
    if (renameMode.value === "serial") {
      const num = padNumber(
        serialStart.value + index,
        Math.max(1, serialPadding.value),
      );
      newName = `${serialPrefix.value}${num}${serialSuffix.value}${ext}`;
    } else if (renameMode.value === "prefix") {
      newName = `${addPrefix.value}${row.fileName}`;
    } else if (renameMode.value === "suffix") {
      newName = `${base}${addSuffix.value}${ext}`;
    } else if (renameMode.value === "replace") {
      if (findText.value.length === 0) {
        newName = row.fileName;
      } else {
        newName = row.fileName.split(findText.value).join(replaceText.value);
      }
    }
    return { row, oldName: row.fileName, newName };
  });
});

// ============ 格式转换 ============
const targetExt = ref("txt");
const keepOriginalExt = ref(false);

const convertPreview = computed(() => {
  const rows = selectedRows.value;
  const ext = targetExt.value.trim().replace(/^\.+/, "");
  return rows.map((row) => {
    const { base, ext: oldExt } = splitFileName(row.fileName);
    let newName = row.fileName;
    if (ext.length === 0) {
      newName = row.fileName;
    } else if (oldExt === "" && !keepOriginalExt.value) {
      newName = `${row.fileName}.${ext}`;
    } else {
      newName = `${base}.${ext}`;
    }
    return { row, oldName: row.fileName, newName };
  });
});

// ============ 执行批量操作 ============
const running = ref(false);

async function executeBatch(
  list: { row: FileInfoRow; oldName: string; newName: string }[],
  successMessage: string,
) {
  if (list.length === 0) {
    ElMessage.warning("请先选择要操作的文件");
    return;
  }

  const todo = list.filter(
    (item) => item.oldName !== item.newName && item.newName.trim() !== "",
  );
  if (todo.length === 0) {
    ElMessage.info("没有需要变更的文件");
    return;
  }

  const nameSet = new Set<string>();
  for (const item of todo) {
    const key = `${item.row.filePath}::${item.newName}`;
    if (nameSet.has(key)) {
      ElMessage.error(`存在重名目标文件：${item.newName}`);
      return;
    }
    nameSet.add(key);
  }

  try {
    await ElMessageBox.confirm(
      `将对 ${todo.length} 个文件执行操作，是否继续？`,
      "确认批量操作",
      {
        confirmButtonText: "确认",
        cancelButtonText: "取消",
        type: "warning",
      },
    );
  } catch {
    return;
  }

  running.value = true;
  let successCount = 0;
  const failed: { name: string; error: string }[] = [];

  for (const item of todo) {
    try {
      const renamedFileInfo = await invoke<string>("rename_file_command", {
        fileDir: item.row.filePath,
        oldFileName: item.oldName,
        newFileName: item.newName,
      });
      const parsed = JSON.parse(renamedFileInfo) as {
        file_name?: string;
        file_path?: string;
        file_type?: string;
        file_size?: string;
        file_create_time?: string;
        file_modify_time?: string;
      };

      item.row.fileName = parsed.file_name ?? item.newName;
      item.row.filePath = parsed.file_path ?? item.row.filePath;
      item.row.fileType = parsed.file_type ?? item.row.fileType;
      item.row.fileSize = parsed.file_size ?? item.row.fileSize;
      item.row.fileCreateTime =
        parsed.file_create_time ?? item.row.fileCreateTime;
      item.row.fileModifyTime =
        parsed.file_modify_time ?? item.row.fileModifyTime;

      // 通知主窗口更新对应行
      try {
        await emitTo(MAIN_WINDOW_LABEL, EVENT_FILE_UPDATED, {
          oldFilePath: item.row.filePath, // 更新后的路径（同一文件夹）
          oldFileName: item.oldName,
          row: { ...item.row },
        });
      } catch (e) {
        console.error("emit file-updated failed:", e);
      }

      successCount++;
    } catch (error) {
      failed.push({ name: item.oldName, error: String(error) });
    }
  }

  running.value = false;

  try {
    await emitTo(MAIN_WINDOW_LABEL, EVENT_FINISHED, {
      success: successCount,
      failed: failed.length,
    });
  } catch (e) {
    console.error("emit finished failed:", e);
  }

  if (failed.length === 0) {
    ElMessage.success(`${successMessage}：共成功 ${successCount} 个`);
    await closeWindow();
  } else {
    ElMessage.warning(`成功 ${successCount} 个，失败 ${failed.length} 个`);
    console.warn("批量操作失败列表：", failed);
  }
}

async function handleRenameExecute() {
  await executeBatch(renamePreview.value, "批量重命名完成");
}

async function handleConvertExecute() {
  if (targetExt.value.trim() === "") {
    ElMessage.warning("请输入目标文件格式");
    return;
  }
  await executeBatch(convertPreview.value, "格式转换完成");
}

async function closeWindow() {
  try {
    await getCurrentWindow().close();
  } catch (e) {
    console.error("close window failed:", e);
  }
}
</script>

<template>
  <div class="batch-page">
    <el-card class="batch-card" shadow="never">
      <template #header>
        <div class="batch-header">
          <span>批量操作</span>
          <span class="batch-header-hint"
            >已选 {{ selectedRows.length }} / {{ fileList.length }}</span
          >
        </div>
      </template>

      <el-tabs v-model="activeTab">
        <el-tab-pane label="批量重命名" name="rename">
          <el-form label-width="90px" class="batch-form">
            <el-form-item label="重命名模式">
              <el-radio-group v-model="renameMode">
                <el-radio value="serial">序号命名</el-radio>
                <el-radio value="prefix">添加前缀</el-radio>
                <el-radio value="suffix">添加后缀</el-radio>
                <el-radio value="replace">查找替换</el-radio>
              </el-radio-group>
            </el-form-item>

            <template v-if="renameMode === 'serial'">
              <el-form-item label="前缀">
                <el-input
                  v-model="serialPrefix"
                  placeholder="如：file_"
                  clearable />
              </el-form-item>
              <el-form-item label="后缀">
                <el-input
                  v-model="serialSuffix"
                  placeholder="如：_v1（可空）"
                  clearable />
              </el-form-item>
              <el-form-item label="起始序号">
                <el-input-number v-model="serialStart" :min="0" :step="1" />
                <span class="form-hint">位数</span>
                <el-input-number
                  v-model="serialPadding"
                  :min="1"
                  :max="10"
                  :step="1" />
              </el-form-item>
            </template>

            <template v-else-if="renameMode === 'prefix'">
              <el-form-item label="前缀文本">
                <el-input
                  v-model="addPrefix"
                  placeholder="在原文件名前添加的文本"
                  clearable />
              </el-form-item>
            </template>

            <template v-else-if="renameMode === 'suffix'">
              <el-form-item label="后缀文本">
                <el-input
                  v-model="addSuffix"
                  placeholder="添加到扩展名前的文本"
                  clearable />
              </el-form-item>
            </template>

            <template v-else-if="renameMode === 'replace'">
              <el-form-item label="查找">
                <el-input
                  v-model="findText"
                  placeholder="要被替换的文本"
                  clearable />
              </el-form-item>
              <el-form-item label="替换为">
                <el-input
                  v-model="replaceText"
                  placeholder="替换成的文本（可空）"
                  clearable />
              </el-form-item>
            </template>
          </el-form>
        </el-tab-pane>

        <el-tab-pane label="格式转换" name="convert">
          <el-form label-width="90px" class="batch-form">
            <el-form-item label="目标格式">
              <el-input
                v-model="targetExt"
                placeholder="如：txt、md、json（不含点）"
                clearable
                style="max-width: 260px" />
              <span class="form-hint">将文件扩展名替换为该格式</span>
            </el-form-item>
            <el-form-item label="无扩展名时">
              <el-switch
                v-model="keepOriginalExt"
                active-text="保持原文件名"
                inactive-text="追加扩展名" />
            </el-form-item>
          </el-form>
        </el-tab-pane>
      </el-tabs>

      <el-divider style="margin: 8px 0" />

      <el-table
        ref="tableRef"
        :data="fileList"
        border
        height="340"
        :header-cell-style="{ textAlign: 'center' }"
        @selection-change="handleSelectionChange">
        <el-table-column type="selection" width="50" align="center" />
        <el-table-column type="index" label="序号" width="70" align="center" />
        <el-table-column
          prop="fileName"
          label="原文件名"
          align="center"
          show-overflow-tooltip />
        <el-table-column label="新文件名" align="center" show-overflow-tooltip>
          <template #default="{ row }">
            <template v-if="activeTab === 'rename'">
              <span class="preview-new">
                {{
                  renamePreview.find((p) => p.row === row)?.newName ??
                  row.fileName
                }}
              </span>
            </template>
            <template v-else>
              <span class="preview-new">
                {{
                  convertPreview.find((p) => p.row === row)?.newName ??
                  row.fileName
                }}
              </span>
            </template>
          </template>
        </el-table-column>
      </el-table>

      <template #footer>
        <div class="batch-footer">
          <el-button @click="closeWindow" :disabled="running">取消</el-button>
          <el-button
            v-if="activeTab === 'rename'"
            type="primary"
            :icon="icons.Edit"
            :loading="running"
            @click="handleRenameExecute"
            >执行重命名</el-button
          >
          <el-button
            v-else
            type="primary"
            :icon="icons.Refresh"
            :loading="running"
            @click="handleConvertExecute"
            >执行格式转换</el-button
          >
        </div>
      </template>
    </el-card>
  </div>
</template>

<style scoped>
.batch-page {
  width: 100%;
  height: 100%;
  padding: 16px;
  box-sizing: border-box;
  overflow: hidden;
}

.batch-card {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.batch-card :deep(.el-card__body) {
  flex: 1;
  overflow: auto;
}

.batch-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-weight: 600;
}

.batch-header-hint {
  font-weight: 400;
  font-size: 12px;
  color: #909399;
}

.batch-form {
  margin-top: 8px;
}

.form-hint {
  color: #909399;
  font-size: 12px;
  margin: 0 8px;
}

.preview-new {
  color: #409eff;
  word-break: break-all;
}

.batch-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
</style>
