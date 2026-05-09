<script lang="ts" setup>
import * as icons from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";
import { open, message } from "@tauri-apps/plugin-dialog";
import { join } from "@tauri-apps/api/path";
import { openPath } from "@tauri-apps/plugin-opener";

type FileInfoRow = {
    fileName: string;
    filePath: string;
    fileType: string;
    fileSize: string;
    fileCreateTime: string;
    fileModifyTime: string;
};

const fileList = ref<FileInfoRow[]>([]);


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
    const parsed = JSON.parse(file_info) as {
        file_name?: string;
        file_path?: string;
        file_type?: string;
        file_size?: string;
        file_create_time?: string;
        file_modify_time?: string;
    };
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
    const parsed = JSON.parse(file_list) as {
        file_name?: string;
        file_path?: string;
        file_type?: string;
        file_size?: string;
        file_create_time?: string;
        file_modify_time?: string;
    }[];
    fileList.value = parsed.map((file: any) => ({
        fileName: file.file_name ?? "",
        filePath: file.file_path ?? "",
        fileType: file.file_type ?? "",
        fileSize: file.file_size ?? "",
        fileCreateTime: file.file_create_time ?? "",
        fileModifyTime: file.file_modify_time ?? "",
    }));
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
        <el-row>
            <el-tooltip content="打开文件" placement="bottom">
                <el-button type="primary" size="small" :icon="icons.DocumentAdd" @click="handleOpenFile"/>
            </el-tooltip>
            <el-tooltip content="打开文件夹" placement="bottom">
                <el-button type="success" size="small" :icon="icons.FolderOpened" @click="handleOpenFolder"/>
            </el-tooltip>
        </el-row>
        <el-divider/>

        <el-table :data = "fileList" style="width: 100%" border empty-text="暂无文件数据" :header-cell-style="{ textAlign: 'center' }">
            <el-table-column type="index" label="序号" width="80" align="center" :resizable="false"/>
            <el-table-column prop="fileName" label="文件名" show-overflow-tooltip :resizable="false" />
            <el-table-column prop="fileType" label="文件类型" width="100" align="center" :resizable="false" />
            <el-table-column prop="fileSize" label="文件大小" width="100" align="center" :resizable="false"/>
            <el-table-column prop="fileCreateTime" align="center" label="创建时间" width="200" :resizable="false"/>
            <el-table-column prop="fileModifyTime" align="center" label="修改时间" width="200" :resizable="false"/>
            <el-table-column label="操作" width="150" :resizable="false" fixed="right" align="center">
                <template #default="scoped">
                    <el-button type="primary" text size="small" :icon="icons.View" @click="handlePreview(scoped.row)">
                        预览
                    </el-button>
                </template>
            </el-table-column>
        </el-table>

    </div>
</template>

<style scoped>
.common-layout {
    height: 100%;
    display: flex;
    flex-direction: column;
}
</style>