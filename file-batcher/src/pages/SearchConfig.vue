<script setup lang="ts">
  import { computed } from "vue";
import TitleBar from "../components/TitleBar.vue";
// import { invoke } from "@tauri-apps/api/core";

  const props = defineProps<{
    modelValue: boolean;
  }>();

  const emit = defineEmits<{
    (e: "update:modelValue", value: boolean): void;
  }>();

  const visible = computed({
    get: () => props.modelValue,
    set: (value: boolean) => emit("update:modelValue", value),
  });
</script>

<template>
  <div class="search-config-page">
    <TitleBar title="搜索条件" />

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
</style>
