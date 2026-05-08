<script lang="ts" setup>
import * as icons from "@element-plus/icons-vue";

function handleOpenFile() {
    console.log("handleOpenFile");
}

function handleOpenFolder() {
    console.log("handleOpenFolder");
}

function handleOpenBatchDialog() {
    console.log("handleOpenBatchDialog");
}

</script>

<template>
    <div class="common-layout">
        <el-row>
            <el-button type="primary" size="small" :icon="icons.DocumentAdd" @click="handleOpenFile"></el-button>
            <el-button type="default" size="small" :icon="icons.FolderOpened" @click="handleOpenFolder"></el-button>
        </el-row>

        <el-table style="width: 100%">
            <el-table-column type="index" label="序号" width="80" align="center"/>
            <el-table-column prop="fileName" label="文件名" />
            <el-table-column prop="filePath" label="文件路径" />
            <el-table-column prop="fileType" label="文件类型" />
            <el-table-column prop="fileSize" label="文件大小" />
            <el-table-column prop="fileCreateTime" label="创建时间" />
            <el-table-column prop="fileModifyTime" label="修改时间" />
        </el-table>
        <!-- <el-container>
            <el-header>

            </el-header>
            <el-main>

            </el-main>
            <el-footer>
                <div> footer </div>
            </el-footer>
        </el-container> -->
    </div>
</template>

<style scoped>
.common-layout {
    height: 100%;
    display: flex;
    flex-direction: column;
}
</style>