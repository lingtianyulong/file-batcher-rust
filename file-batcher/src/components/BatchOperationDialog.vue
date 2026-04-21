<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { ElMessage, ElMessageBox } from "element-plus";
import * as icons from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api/core";

type FileInfoRow = {
  fileName: string;
  filePath: string;
  fileType: string;
  fileSize: string;
  fileCreateTime: string;
  fileModifyTime: string;
};

type RenameMode = "serial" | "prefix" | "suffix" | "replace";

const props = defineProps<{
  modelValue: boolean;
  fileList: FileInfoRow[];
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
  (e: "updated", files: FileInfoRow[]): void;
}>();

const activeTab = ref<"rename" | "convert">("rename");

// ============ 公共：可选择的文件 ============
const selectedRows = ref<FileInfoRow[]>([]);
const tableRef = ref<any>(null);

function handleSelectionChange(rows: FileInfoRow[]) {
  selectedRows.value = rows;
}

// 打开时默认全选
watch(
  () => props.modelValue,
  (visible) => {
    if (visible) {
      // 重置状态
      selectedRows.value = [];
      // 等待渲染后全选
      setTimeout(() => {
        if (tableRef.value && props.fileList.length > 0) {
          props.fileList.forEach((row) => tableRef.value.toggleRowSelection(row, true));
        }
      }, 0);
    }
  }
);

// ============ 批量重命名 ============
const renameMode = ref<RenameMode>("serial");

// 序号命名参数
const serialPrefix = ref("file_");
const serialSuffix = ref("");
const serialStart = ref(1);
const serialPadding = ref(3);

// 前缀 / 后缀
const addPrefix = ref("");
const addSuffix = ref("");

// 查找替换
const findText = ref("");
const replaceText = ref("");

// 拆分文件名为 base 和 ext
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

// 计算每一行的新文件名
const renamePreview = computed(() => {
  const rows = selectedRows.value;
  return rows.map((row, index) => {
    const { base, ext } = splitFileName(row.fileName);
    let newName = row.fileName;
    if (renameMode.value === "serial") {
      const num = padNumber(serialStart.value + index, Math.max(1, serialPadding.value));
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
const keepOriginalExt = ref(false); // 若原文件无扩展名是否仍追加

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
  successMessage: string
) {
  if (list.length === 0) {
    ElMessage.warning("请先选择要操作的文件");
    return;
  }

  // 过滤新旧相同
  const todo = list.filter((item) => item.oldName !== item.newName && item.newName.trim() !== "");
  if (todo.length === 0) {
    ElMessage.info("没有需要变更的文件");
    return;
  }

  // 检查新名称是否有重复
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
      { confirmButtonText: "确认", cancelButtonText: "取消", type: "warning" }
    );
  } catch {
    return;
  }

  running.value = true;
  let successCount = 0;
  const failed: { name: string; error: string }[] = [];
  const updated: FileInfoRow[] = [];

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
      item.row.fileCreateTime = parsed.file_create_time ?? item.row.fileCreateTime;
      item.row.fileModifyTime = parsed.file_modify_time ?? item.row.fileModifyTime;

      updated.push(item.row);
      successCount++;
    } catch (error) {
      failed.push({ name: item.oldName, error: String(error) });
    }
  }

  running.value = false;

  emit("updated", updated);

  if (failed.length === 0) {
    ElMessage.success(`${successMessage}：共成功 ${successCount} 个`);
    closeDialog();
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

function closeDialog() {
  emit("update:modelValue", false);
}
</script>

<template>
  <el-dialog
    :model-value="modelValue"
    title="批量操作"
    width="820px"
    :close-on-click-modal="false"
    @update:model-value="(v: boolean) => emit('update:modelValue', v)"
  >
    <el-tabs v-model="activeTab">
      <!-- 批量重命名 -->
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
              <el-input v-model="serialPrefix" placeholder="如：file_" clearable />
            </el-form-item>
            <el-form-item label="后缀">
              <el-input v-model="serialSuffix" placeholder="如：_v1（可空）" clearable />
            </el-form-item>
            <el-form-item label="起始序号">
              <el-input-number v-model="serialStart" :min="0" :step="1" />
              <span class="form-hint">位数</span>
              <el-input-number v-model="serialPadding" :min="1" :max="10" :step="1" />
            </el-form-item>
          </template>

          <template v-else-if="renameMode === 'prefix'">
            <el-form-item label="前缀文本">
              <el-input v-model="addPrefix" placeholder="在原文件名前添加的文本" clearable />
            </el-form-item>
          </template>

          <template v-else-if="renameMode === 'suffix'">
            <el-form-item label="后缀文本">
              <el-input v-model="addSuffix" placeholder="添加到扩展名前的文本" clearable />
            </el-form-item>
          </template>

          <template v-else-if="renameMode === 'replace'">
            <el-form-item label="查找">
              <el-input v-model="findText" placeholder="要被替换的文本" clearable />
            </el-form-item>
            <el-form-item label="替换为">
              <el-input v-model="replaceText" placeholder="替换成的文本（可空）" clearable />
            </el-form-item>
          </template>
        </el-form>
      </el-tab-pane>

      <!-- 格式转换 -->
      <el-tab-pane label="格式转换" name="convert">
        <el-form label-width="90px" class="batch-form">
          <el-form-item label="目标格式">
            <el-input v-model="targetExt" placeholder="如：txt、md、json（不含点）" clearable style="max-width: 260px" />
            <span class="form-hint">将文件扩展名替换为该格式</span>
          </el-form-item>
          <el-form-item label="无扩展名时">
            <el-switch v-model="keepOriginalExt" active-text="保持原文件名" inactive-text="追加扩展名" />
          </el-form-item>
        </el-form>
      </el-tab-pane>
    </el-tabs>

    <el-divider style="margin: 8px 0" />

    <div class="batch-section-title">
      <span>文件列表（已选 {{ selectedRows.length }} / {{ fileList.length }}）</span>
    </div>

    <el-table
      ref="tableRef"
      :data="fileList"
      border
      height="340"
      :header-cell-style="{ textAlign: 'center' }"
      @selection-change="handleSelectionChange"
    >
      <el-table-column type="selection" width="50" align="center" />
      <el-table-column type="index" label="序号" width="70" align="center" />
      <el-table-column prop="fileName" label="原文件名" align="center" show-overflow-tooltip />
      <el-table-column label="新文件名" align="center" show-overflow-tooltip>
        <template #default="{ row }">
          <template v-if="activeTab === 'rename'">
            <span class="preview-new">
              {{ renamePreview.find((p) => p.row === row)?.newName ?? row.fileName }}
            </span>
          </template>
          <template v-else>
            <span class="preview-new">
              {{ convertPreview.find((p) => p.row === row)?.newName ?? row.fileName }}
            </span>
          </template>
        </template>
      </el-table-column>
    </el-table>

    <template #footer>
      <el-button @click="closeDialog" :disabled="running">取消</el-button>
      <el-button
        v-if="activeTab === 'rename'"
        type="primary"
        :icon="icons.Edit"
        :loading="running"
        @click="handleRenameExecute"
      >执行重命名</el-button>
      <el-button
        v-else
        type="primary"
        :icon="icons.Refresh"
        :loading="running"
        @click="handleConvertExecute"
      >执行格式转换</el-button>
    </template>
  </el-dialog>
</template>

<style scoped>
.batch-form {
  margin-top: 8px;
}

.form-hint {
  color: #909399;
  font-size: 12px;
  margin: 0 8px;
}

.batch-section-title {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
  font-size: 13px;
  color: #606266;
}

.preview-new {
  color: #409eff;
  word-break: break-all;
}
</style>
