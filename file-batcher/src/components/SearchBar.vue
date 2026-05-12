<script lang="ts" setup>
import * as icons from "@element-plus/icons-vue";

</script>

<template>
    <div class="search-bar">
        <div class="open-folder">
            <el-lable class="lable">文件目录</el-lable>
            <el-input type="text" :style="{ width: '65%' }" placeholder="加载文件目录" disabled/>
            <el-button type="default" style="font-size: 16px; background-color: transparent;" size="default" :icon="icons.FolderOpened"/>
        </div>
        <div class="open-folder">
              <el-lable class="lable">文件目录</el-lable>
            <el-input type="text" :style="{ width: '65%' }" placeholder="加载文件目录" disabled/>
            <el-button type="default" style="font-size: 16px; background-color: transparent;" size="default" :icon="icons.Search"/>
            <el-tooltip content="设置搜索选项" placement="bottom" effect="light" popper-class="search-tooltip">
               <el-button type="default" style="font-size: 16px; background-color: transparent;" size="default" :icon="icons.Setting"/>
            </el-tooltip>
        </div>
    </div>
</template>

<style scoped>
.lable {
    width: 65px;
    font-size: 15px;
    font-family: 'Microsoft YaHei';
    margin-right: 10px;
}

:global(.search-tooltip.el-popper) {
    font-size: 13px;
    font-family: 'Microsoft YaHei';
    background-color: #dcdfe6 !important;
    border-radius: 5px;
    padding: 5px;
}

:global(.search-tooltip.el-popper[data-popper-placement^='bottom'] .el-popper__arrow::before) {
    background-color: #dcdfe6 !important;
    border-color: #dcdfe6 !important;
}

.open-folder {
    width: 50%;
    height: 30px;
    border: none;
    font-size: 14px;
    display: flex;
    align-items: center;
    justify-content: flex-start;
    flex-direction: row;
    gap: 5px;
    margin-left: 20px;
}

.open-folder :deep(.p-floatlabel) {
    width: 70%;
    flex: none;
}

.search-bar {
    width: 100%;
    height: 50px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid #dcdfe6;
    margin-top: 0px;
    background-color: #f5f7fa;
    flex-direction: row;
}

</style>