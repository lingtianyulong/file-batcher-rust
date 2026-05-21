<script setup lang="ts">
  import { computed } from "vue";
  import { CloseBold } from "@element-plus/icons-vue";

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

  function closeDialog() {
    visible.value = false;
  }
</script>

<template>
  <el-dialog
    v-model="visible"
    align-center
    class="search-config-dialog"
    title="搜索配置"
    show-close
  >
    <!-- <div class="title_bar">
      <div class="title_bar_title">搜索配置</div>
      <el-button :icon="CloseBold" @click="closeDialog" />
    </div> -->
  </el-dialog>
  <!-- <div class="search-page">
    <h1>搜索配置</h1>
  </div> -->
</template>

<style scoped>
  .search-config-dialog {
    flex-direction: column;
    width: 1000px;
    height: 800px;
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
