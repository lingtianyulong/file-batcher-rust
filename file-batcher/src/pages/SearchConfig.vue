<script setup lang="ts">
import { ref } from "vue";
import TitleBar from "../components/TitleBar.vue";
import { FolderOpened } from "@element-plus/icons-vue";
import { Search, Refresh, } from "@element-plus/icons-vue";
// import { invoke } from "@tauri-apps/api/core";

const props = defineProps<{
  modelValue: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
}>();

const formData = ref({
  searchPath: "",
});

</script>

<template>
  <div class="search-config-page">
    <div class="title-bar">
      <TitleBar title="文件搜索" :showMinimize="true" :showMaximize="true" />
    </div>
    <div class="main-content">
      <el-container>
        <el-aside class="left-sider-border">
         <el-card shadow="hover" style="width: 100%">
          <template #header>
            <div> 包含文件属性 </div>
          </template>
          <div style="padding: 10px; margin-left: 20px;">
            <el-checkbox-group class="checkbox-grid">
              <el-checkbox label="子文件夹" />
              <el-checkbox label="只读文件" />
              <el-checkbox label="系统文件" />
              <el-checkbox label="隐藏文件" />
            </el-checkbox-group>
          </div>
         </el-card>
        </el-aside>
        <el-main class="main-content-container">
          <el-card shadow="hover" :style="{ height: '150px', display: 'flex', flexDirection: 'column' }">
            <el-form :model="formData" style="width: 100%; height: 100%; padding: 20px;">
              <el-row gutter="20">
                <el-col :span="24">
                  <el-form-item label="搜索路径">
                    <el-input placeholder="请输入搜索路径" :style="{ width: '100%' }" readonly>
                      <template #append>
                        <el-button type="default" size="default" style="font-size: 16px; background-color: transparent"
                          :icon="FolderOpened" />
                      </template>
                    </el-input>
                  </el-form-item>
                </el-col>
              </el-row>
              <el-row gutter="20">
                <el-col :span="9">
                  <el-form-item label="文件名" style="margin-left: 15px;">
                    <el-input placeholder="请输入文件名" :style="{ width: '100%' }" />
                  </el-form-item>
                </el-col>
                <el-col :span="9">
                  <el-form-item label="文件类型">
                    <el-select placeholder="请选择文件类型" :style="{ width: '100%' }">
                      <el-option label="所有文件" value="all" />
                      <el-option label="文本文件" value="txt" />
                      <el-option label="图片文件" value="image" />
                      <el-option label="音频文件" value="audio" />
                      <el-option label="视频文件" value="video" />
                      <el-option label="压缩文件" value="zip" />
                      <el-option label="其他文件" value="other" />
                    </el-select>
                  </el-form-item>
                </el-col>
                <el-col :span="6">
                  <el-button-group>
                    <el-button type="primary" :icon="Search"> 搜索 </el-button>
                    <el-button type="default" :icon="Refresh"> 重置 </el-button>
                  </el-button-group>
                </el-col>
              </el-row>
            </el-form>
          </el-card>
          <el-card shadow="hover" class="result-card">
            <el-table class="result-table" border height="100%">
              <el-table-column prop="fileName" label="文件名" />
              <el-table-column prop="filePath" label="文件路径" />
              <el-table-column prop="fileType" label="文件类型" />
              <el-table-column prop="fileSize" label="文件大小" />
              <el-table-column prop="fileCreateTime" label="创建时间" />
              <el-table-column prop="fileModifyTime" label="修改时间" />
            </el-table>
          </el-card>
        </el-main>
      </el-container>
    </div>
  </div>


</template>

<style scoped>
.search-config-page {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.title_bar {
  width: 100%;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid #dcdfe6;
}

.open-folder-button {
  font-size: 24px;
  width: 30px;
  height: 30px;
  border: none;
}

.main-content {
  flex: 1;
  min-height: 0;
  overflow: hidden;
  display: flex;
  width: 100%;
  height: 100%;
}

.main-content-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow: hidden;
}

.left-sider-border {
  border-right: 1px solid #dcdfe6;
  height: 100%;
  background-color: #ffffff;
  overflow: hidden;
  width: 25%;
  transition: width 0.2s ease;
  display: flex;
  flex-direction: column;
}

.result-card {
  margin-top: 10px;
  height: 100%;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
}

:deep(.el-card__body) {
  padding: 0;
  height: 100%;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
}

.result-table {
  flex: 1 1 0;
  min-height: 0;
  overflow: hidden;
}

.checkbox-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 10px;
}

.checkbox-grid :deep(.el-checkbox) {
  margin-right: 0;
}
</style>
