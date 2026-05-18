<script lang="ts" setup>
import FolderTree from "./FolderTree.vue";
import { useFolderStore } from '../stores/file-store';
import { onMounted, watch } from 'vue';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import * as icons from "@element-plus/icons-vue";
// import Row from "primevue/row";

const folderStore = useFolderStore();

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

    const file_list = await invoke<string>("get_file_list_command", { filePath: filePath });
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

onMounted(() => {
    if (folderStore.currentPath) {
        console.log("onMounted in main page", folderStore.currentPath);
        loadFileList(folderStore.currentPath);
    } else {
        // console.log("没有选择文件夹");
    }
    // console.log(folderStore.currentPath);
});

watch(
    () => {
        console.log("watch in main page", folderStore.currentPath);
        return folderStore.currentPath;
    },
    (newPath) => {
        // console.log("watch in main page", newPath);
        if (newPath) {
            console.log("watch new path in main page", newPath);
            loadFileList(newPath);
        } else {
            // console.log("没有选择文件夹");
        }
    }
)

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
                >
                    <el-table-column type="selection" width="48" align="center" />
                    <el-table-column prop="fileName" label="文件名" header-align="center" show-overflow-tooltip min-width="140" />
                    <el-table-column prop="fileType" label="文件类型" align="center" header-align="center" width="100" />
                    <el-table-column prop="fileSize" label="文件大小" align="center" header-align="center" width="100" />
                    <el-table-column prop="fileCreateTime" label="创建时间" align="center" header-align="center" min-width="160" />
                    <el-table-column prop="fileModifyTime" label="修改时间" align="center" header-align="center" min-width="160" />
                    <el-table-column label="操作" align="center" header-align="center" width="100" fixed="right">
                        <template #default>
                            <el-button type="primary" text size="small" style="font-size: 15px" :icon="icons.View">
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
    overflow: hidden;   /* 移除滚动条 */
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