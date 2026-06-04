<script setup lang="ts">
  // import { computed } from "vue";
  import TitleBar from "../components/TitleBar.vue";
  import {FolderOpened} from "@element-plus/icons-vue";
  // import { invoke } from "@tauri-apps/api/core";

  const props = defineProps<{
    modelValue: boolean;
  }>();

  const emit = defineEmits<{
    (e: "update:modelValue", value: boolean): void;
  }>();

</script>

<template>
  <div class="search-config-page">
    <div class="title-bar">
      <TitleBar title="搜索条件" :showMinimize="true" :showMaximize="true" />
    </div>
    <div>
      <el-card shadow="hover">
        <el-form>
          <el-form-item label="搜索路径">
            <el-input placeholder="请输入搜索路径" :style="{ width: '95%' }" readonly>
              <template #append>
                <el-button type="default" size="default" style="font-size: 16px; background-color: transparent" :icon="FolderOpened"/>
              </template>
            </el-input>
          </el-form-item>
        </el-form>
      </el-card>
    </div>

  </div>
  <!-- <el-dialog
    v-model="visible"
    align-center
    class="search-config-dialog"
    title="搜索配置"
    show-close
    draggable
  > -->
  <!-- <div class="title_bar">
      <div class="title_bar_title">搜索配置</div>
      <el-button :icon="CloseBold" @click="closeDialog" />
    </div> -->
  <!-- </el-dialog> -->
  <!-- <div class="search-page">
    <h1>搜索配置</h1>
  </div> -->
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

</style>
