<script lang="ts" setup>
  import * as icons from "@element-plus/icons-vue";
  import { open, message } from "@tauri-apps/plugin-dialog";
  import { ref, onMounted, watch } from "vue";
  import { useFolderStore } from "../stores/file-store";
  import { invoke } from "@tauri-apps/api/core";

  const folderStore = useFolderStore();
  const folderPath = ref("");

  onMounted(() => {
    folderPath.value = folderStore.currentPath;
  });

  watch(
    () => folderStore.currentPath,
    (newPath) => {
      folderPath.value = newPath;
    },
  );

  async function handleOpenFolder() {
    try {
      const selectedPath = await open({
        directory: true,
        multiple: false,
      });

      if (!selectedPath || Array.isArray(selectedPath)) {
        await message("已取消选择文件夹。", { title: "提示", kind: "info" });
        return;
      }

      folderPath.value = selectedPath;
      folderStore.setPath(selectedPath);
    } catch (error) {
      await message(`打开搜索条件失败：${String(error)}`, {
        title: "错误",
        kind: "error",
      });
    }
  }

  async function handleOpenSearchConfig() {
    try {
      await invoke("open_search_config_window_command");
    } catch (error) {
      await message(`打开搜索配置失败：${String(error)}`, {
        title: "错误",
        kind: "error",
      });
    }
  }
</script>

<template>
  <div class="search-bar">
    <div class="open-folder">
      <el-lable class="lable">文件目录</el-lable>
      <el-input
        v-model="folderPath"
        type="text"
        :style="{ width: '65%' }"
        placeholder="加载文件目录"
        readonly
      />
      <el-button
        type="default"
        style="font-size: 16px; background-color: transparent"
        size="default"
        :icon="icons.FolderOpened"
        @click="handleOpenFolder"
      />
    </div>
    <div class="open-folder">
      <el-lable class="lable">搜索条件</el-lable>
      <el-input
        type="text"
        :style="{ width: '65%' }"
        placeholder="请输入搜索条件"
      />
      <el-button
        type="default"
        style="font-size: 16px; background-color: transparent"
        size="default"
        :icon="icons.Search"
        @click="handleOpenSearchConfig"
      />
      <el-tooltip
        content="设置搜索选项"
        placement="bottom"
        effect="light"
        popper-class="search-tooltip"
      >
        <el-button
          type="default"
          style="font-size: 16px; background-color: transparent"
          size="default"
          :icon="icons.Setting"
          @click="handleOpenSearchConfig"
        />
      </el-tooltip>
    </div>
  </div>
</template>

<style scoped>
  .lable {
    width: 65px;
    font-size: 15px;
    font-family: "Microsoft YaHei";
    margin-right: 10px;
  }

  :global(.search-tooltip.el-popper) {
    font-size: 13px;
    font-family: "Microsoft YaHei";
    background-color: #dcdfe6 !important;
    border-radius: 5px;
    padding: 5px;
  }

  :global(
    .search-tooltip.el-popper[data-popper-placement^="bottom"]
      .el-popper__arrow::before
  ) {
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
