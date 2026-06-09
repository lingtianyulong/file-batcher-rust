<script setup lang="ts">
  import { ref } from "vue";
  import TitleBar from "../components/TitleBar.vue";
  import { FolderOpened } from "@element-plus/icons-vue";
  import { Search, Refresh } from "@element-plus/icons-vue";
  import { open, message } from "@tauri-apps/plugin-dialog";

  const props = defineProps<{
    modelValue: boolean;
  }>();

  const emit = defineEmits<{
    (e: "update:modelValue", value: boolean): void;
  }>();

  const searchFormData = ref({
    searchPath: "",
    fileType: "",
  });

  const fileTypeOptions = ref([
    { label: "所有文件(*.*)", value: "all" },
    { label: "文本文件(*.txt)", value: "txt" },
    { label: "图片文件(*.jpg, *.png, *.gif, *.bmp, *.ico, *.webp)", value: "image" },
    { label: "音频文件(*.mp3, *.wav, *.ogg, *.flac, *.aac, *.m4a)", value: "audio" },
    { label: "视频文件(*.mp4, *.avi, *.mov, *.wmv, *.flv, *.mkv)", value: "video" },
    { label: "压缩文件(*.zip, *.rar, *.7z, *.tar, *.gz, *.bz2)", value: "zip" },
  ]);

  const handleOpenFolder = async () => {
    try {
      const selectedPath = await open({
        directory: true,
        multiple: false,
      });
      if (!selectedPath || Array.isArray(selectedPath)) {
        await message("已取消选择文件夹。", { title: "提示", kind: "info" });
        return;
      }
      searchFormData.value.searchPath = selectedPath;
    } catch (error) {
      console.error("open folder failed", error);
    }
  };

</script>

<template>
  <div class="search-config-page">
    <div class="title-bar">
      <TitleBar title="文件搜索" :showMinimize="true" :showMaximize="true" />
    </div>
    <div class="main-content">
      <el-container>
        <el-aside class="left-sider-border">
          <el-card shadow="never" style="width: 100%">
            <template #header>
              <div style="font-size: 20px; font-weight: bold">包含文件属性</div>
            </template>
            <div style="padding: 10px; margin-left: 20px">
              <el-checkbox-group class="checkbox-grid">
                <el-checkbox label="子文件夹" />
                <el-checkbox label="只读文件" />
                <el-checkbox label="系统文件" />
                <el-checkbox label="隐藏文件" />
              </el-checkbox-group>
            </div>
          </el-card>
          <el-card shadow="never" style="width: 100%; margin-top: 10px">
            <template #header>
              <div style="font-size: 20px; font-weight: bold">过滤条件</div>
            </template>
            <div style="padding: 10px; border: 1px solid #dcdfe6">
              <div style="font-size: 14px; font-weight: bold; margin-left: 10px">
                文件名
              </div>
              <div style="
                  margin-left: 10px;
                  display: flex;
                  align-items: center;
                  margin-top: 10px;
                  gap: 10px;
                ">
                <el-select style="width: 180px" placeholder="请选择">
                  <el-option label="包含" value="contains" />
                  <el-option label="不包含" value="notContains" />
                  <el-option label="等于" value="equals" />
                  <el-option label="不等于" value="notEquals" />
                  <el-option label="正则表达式" value="regex" />
                </el-select>
                <el-checkbox label="忽略大小写" style="margin-left: 10px" />
              </div>
              <div style="margin-left: 10px; margin-top: 10px">
                <el-input type="text" style="width: 350px" placeholder="请输入正则表达式或文件名" clearable />
              </div>
            </div>
            <div style="margin-top: 0px; border: 1px solid #dcdfe6; padding: 10px">
              <div style="font-size: 14px; font-weight: bold; margin-left: 10px">
                文件日期
              </div>
              <div style="margin-left: 10px; margin-top: 20px; margin-bottom: 10px">
                <el-date-picker type="daterange" range-separator="至" start-placeholder="开始日期" end-placeholder="结束日期"
                  value-format="YYYY-MM-DD" clearable single-panel />
              </div>
            </div>
            <div style="margin-top: 0px; border: 1px solid #dcdfe6; padding: 10px">
              <div style="font-size: 14px; font-weight: bold; margin-left: 10px">
                文件尺寸
              </div>
              <div style="margin-left: 10px; margin-top: 20px; margin-bottom: 10px" display: flex; align-items: center;
                gap: 10px;>
                <el-select style="width: 180px" placeholder="请选择">
                  <el-option label="大于" value="greaterThan" />
                  <el-option label="小于" value="lessThan" />
                  <el-option label="等于" value="equals" />
                  <el-option label="不等于" value="notEquals" />
                </el-select>
                <el-label style="margin-left: 25px">单位</el-label>
                <el-select style="width: 100px; margin-left: 10px" placeholder="单位">
                  <el-option label="KB" value="greaterThan" />
                  <el-option label="MB" value="lessThan" />
                  <el-option label="GB" value="equals" />
                </el-select>
              </div>
              <div style="margin-left: 10px; margin-top: 10px">
                <el-input-number :min="0" :step="1" :max="1000" controls-position="right" style="width: 180px" />
              </div>
            </div>
            <div style="margin-top: 0px; border: 1px solid #dcdfe6; padding: 10px">
              <div style="font-size: 14px; font-weight: bold; margin-left: 10px">
                结果限制
              </div>
              <div style="margin-left: 10px; margin-top: 20px; margin-bottom: 10px" display: flex; align-items: center;
                gap: 10px;>
                <el-input-number :min="0" :step="1" :max="1000" controls-position="right" style="width: 180px" />
              </div>
            </div>
          </el-card>
        </el-aside>
        <el-main class="main-content-container">
          <el-card shadow="never" :style="{
            height: '150px',
            display: 'flex',
            flexDirection: 'column',
          }">
            <el-form :model="searchFormData" style="width: 100%; height: 100%; padding: 20px">
              <el-row gutter="20">
                <el-col :span="24">
                  <el-form-item label="搜索路径">
                    <el-input v-model="searchFormData.searchPath" placeholder="请输入搜索路径" :style="{ width: '100%' }">
                      <template #append>
                        <el-button type="default" size="default" style="font-size: 16px; background-color: transparent"
                          :icon="FolderOpened" @click="handleOpenFolder" />
                      </template>
                    </el-input>
                  </el-form-item>
                </el-col>
              </el-row>
              <el-row gutter="20">
                <el-col :span="9">
                  <el-form-item label="文件名" style="margin-left: 15px">
                    <el-input placeholder="请输入文件名" :style="{ width: '100%' }" />
                  </el-form-item>
                </el-col>
                <el-col :span="9">
                  <el-form-item label="文件类型">
                    <el-tooltip :content="fileTypeOptions.find(option => option.value === searchFormData.fileType)?.label" placement="bottom" effect="light"
                      popper-class="search-tooltip">
                      <el-select v-model="searchFormData.fileType" placeholder="请选择文件类型" :style="{ width: '100%' }"
                        collapse-tags="true" collapse-tags-tooltip="true" tag-tooltip="true">
                        <el-option v-for="option in fileTypeOptions" :key="option.value" :label="option.label"
                          :value="option.value" />
                      </el-select>
                    </el-tooltip>

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
          <el-card shadow="never" class="result-card">
            <el-table class="result-table" border height="100%" :header-cell-style="{ textAlign: 'center' }">
              <el-table-column prop="fileName" label="名称" />
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
