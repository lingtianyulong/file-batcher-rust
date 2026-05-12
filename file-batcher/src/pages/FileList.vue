<script lang="ts" setup>
import * as icons from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api/core";
import { computed, ref, watch } from "vue";
import { open, message } from "@tauri-apps/plugin-dialog";
import { join } from "@tauri-apps/api/path";
import { openPath } from "@tauri-apps/plugin-opener";
import zhCn from "element-plus/es/locale/lang/zh-cn";

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

const NO_EXTENSION_KEY = "__no_extension__";
const elementLocale = {
    ...zhCn,
    el: {
        ...zhCn.el,
        table: {
            ...zhCn.el.table,
            confirmFilter: "确认",
            resetFilter: "重置",
        },
    },
};

const fileList = ref<FileInfoRow[]>([]);
const selectedFileTypes = ref<string[]>([]);
const currentPage = ref(1);
const pageSize = ref(10);

const getFileTypeKey = (fileType: string) => {
    const normalizedFileType = fileType.trim().toLowerCase();
    return normalizedFileType || NO_EXTENSION_KEY;
};

const getFileTypeLabel = (fileTypeKey: string) => {
    return fileTypeKey === NO_EXTENSION_KEY ? "无扩展名" : fileTypeKey;
};

const fileTypeOptions = computed(() => {
    const fileTypes = new Set<string>();
    for (const file of fileList.value) {
        fileTypes.add(getFileTypeKey(file.fileType));
    }
    return Array.from(fileTypes)
        .sort((a, b) => getFileTypeLabel(a).localeCompare(getFileTypeLabel(b)))
        .map((fileType) => ({
            label: getFileTypeLabel(fileType),
            value: fileType,
        }));
});

const filteredFileList = computed(() => {
    if (selectedFileTypes.value.length === 0) {
        return fileList.value;
    }
    const selectedFileTypeSet = new Set(selectedFileTypes.value);
    return fileList.value.filter((file) => selectedFileTypeSet.has(getFileTypeKey(file.fileType)));
});

const paginatedFileList = computed(() => {
    const startIndex = (currentPage.value - 1) * pageSize.value;
    return filteredFileList.value.slice(startIndex, startIndex + pageSize.value);
});

const handlePageSizeChange = () => {
    currentPage.value = 1;
};

watch(selectedFileTypes, () => {
    currentPage.value = 1;
});

watch([() => filteredFileList.value.length, pageSize], () => {
    const maxPage = Math.max(1, Math.ceil(filteredFileList.value.length / pageSize.value));
    if (currentPage.value > maxPage) {
        currentPage.value = maxPage;
    }
});

async function handleOpenFile() {
    const file_path = await open({
        multiple: false,
        directory: false,
    });
    if (!file_path || Array.isArray(file_path)) {
        await message("未选择文件", {title: "提示", kind: "info"});
        return;
    }
    const file_info = await invoke<string>("get_file_info_command", { filePath: file_path });
    const parsed = JSON.parse(file_info) as RawFileInfo;
    fileList.value = [
        {
            fileName: parsed.file_name ?? "",
            filePath: parsed.file_path ?? "",
            fileType: parsed.file_type ?? "",
            fileSize: parsed.file_size ?? "",
            fileCreateTime: parsed.file_create_time ?? "",
            fileModifyTime: parsed.file_modify_time ?? "",
        },
    ];
    selectedFileTypes.value = [];
    currentPage.value = 1;
}

async function handleOpenFolder() {
    console.log("handleOpenFolder");
    const selectedPath = await open({
        directory: true,
        multiple: false,
    });

    if (!selectedPath || Array.isArray(selectedPath)) {
        await message("已取消选择文件夹。", {title: "提示", kind: "info"});
        return;
    }
    const file_list = await invoke<string>("get_file_list_command", { filePath: selectedPath });
    const parsed = JSON.parse(file_list) as RawFileInfo[];
    fileList.value = parsed.map((file) => ({
        fileName: file.file_name ?? "",
        filePath: file.file_path ?? "",
        fileType: file.file_type ?? "",
        fileSize: file.file_size ?? "",
        fileCreateTime: file.file_create_time ?? "",
        fileModifyTime: file.file_modify_time ?? "",
    }));
    selectedFileTypes.value = [];
    currentPage.value = 1;
}

async function handlePreview(row: FileInfoRow) {
    if (!row.fileName || !row.filePath) {
        await message("文件路径不完整，无法预览", {title: "警告", kind: "warning"});
        return;
    }
    try {
        const fullPath = await join(row.filePath, row.fileName);
        await openPath(fullPath);
    } catch (error) {
        await message(`预览失败：${String(error)}`, {title: "错误", kind: "error"});
    }
}
</script>

<template>
    <div class="common-layout">
        <el-row class="file-list-toolbar" align="middle">
            <el-tooltip content="打开文件" placement="bottom" effect="light">
                <el-button type="primary" style="font-size: 16px;" size="default" :icon="icons.DocumentAdd" @click="handleOpenFile"/>
            </el-tooltip>
            <el-tooltip content="打开文件夹" placement="bottom" effect="light">
                <el-button type="success" size="default" style="font-size: 16px;" :icon="icons.FolderOpened" @click="handleOpenFolder"/>
            </el-tooltip>
            <el-select
                v-model="selectedFileTypes"
                class="file-type-filter"
                multiple
                clearable
                collapse-tags
                collapse-tags-tooltip
                placeholder="按文件格式过滤"
                :disabled="fileTypeOptions.length === 0"
            >
                <el-option
                    v-for="item in fileTypeOptions"
                    :key="item.value"
                    :label="item.label"
                    :value="item.value"
                />
            </el-select>
        </el-row>
        <el-divider/>

        <el-config-provider :locale="elementLocale">
            <div class="file-list-content">
                <div class="file-list-table">
                    <el-table :data="paginatedFileList" height="100%" style="width: 100%" border empty-text="暂无文件数据" :header-cell-style="{ textAlign: 'center' }">
                        <el-table-column type="index" label="序号" width="80" align="center" :resizable="false"/>
                        <el-table-column prop="fileName" label="文件名" show-overflow-tooltip />
                        <el-table-column prop="fileType" label="文件类型" width="100" align="center" :resizable="false">
                            <template #default="scoped">
                                {{ getFileTypeLabel(getFileTypeKey(scoped.row.fileType)) }}
                            </template>
                        </el-table-column>
                        <el-table-column prop="fileSize" label="文件大小" width="100" align="center" :resizable="false"/>
                        <el-table-column prop="fileCreateTime" align="center" label="创建时间" width="200" :resizable="false" sortable/>
                        <el-table-column prop="fileModifyTime" align="center" label="修改时间" width="200" :resizable="false" sortable/>
                        <el-table-column label="操作" width="150" :resizable="false" fixed="right" align="center">
                            <template #default="scoped">
                                <el-button type="primary" text size="small" style="font-size: 15px;" font-size="16px" :icon="icons.View" @click="handlePreview(scoped.row)">
                                    预览
                                </el-button>
                            </template>
                        </el-table-column>
                    </el-table>
                </div>
                <el-divider/>
                <div class="file-list-pagination-bar">
                    <el-pagination
                        v-model:current-page="currentPage"
                        v-model:page-size="pageSize"
                        class="file-list-pagination"
                        :page-sizes="[10, 20, 50, 100]"
                        layout="total, sizes, prev, pager, next, jumper"
                        :total="filteredFileList.length"
                        @size-change="handlePageSizeChange"
                    />
                </div>
            </div>
        </el-config-provider>
    </div>
</template>

<style scoped>
.common-layout {
    height: 100%;
    display: flex;
    flex-direction: column;
    min-height: 0;
    overflow: hidden;
}

.file-list-toolbar {
    gap: 12px;
}

.file-type-filter {
    width: 240px;
}

.file-list-content {
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

.file-list-pagination-bar {
    display: flex;
    justify-content: flex-end;
    flex-shrink: 0;
}

.file-list-pagination {
    justify-content: flex-end;
    flex-shrink: 0;
}
</style>