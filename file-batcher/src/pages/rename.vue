<script setup lang="ts">
import { computed, ref } from "vue";
import { ElMessage } from "element-plus";

const prefix = ref("");
const startNumber = ref(1);
const paddingLength = ref(3);
const previewCount = ref(5);

const previewNames = computed(() => {
  const total = Math.max(1, previewCount.value);
  const start = Math.max(0, startNumber.value);
  const padding = Math.max(1, paddingLength.value);
  const safePrefix = prefix.value.trim();

  return Array.from({ length: total }, (_, index) => {
    const current = String(start + index).padStart(padding, "0");
    return `${safePrefix}${current}`;
  });
});

function handleApplyRename() {
  ElMessage.info("这是重命名页面占位逻辑，后续可接入 Tauri 实际重命名命令。");
}
</script>

<template>
  <el-card class="rename-card" shadow="never">
    <template #header>
      <div class="rename-header">
        <span>批量重命名</span>
        <el-tag type="info">Preview</el-tag>
      </div>
    </template>

    <el-form label-width="96px">
      <el-form-item label="前缀">
        <el-input v-model="prefix" placeholder="例如：IMG_" clearable />
      </el-form-item>
      <el-form-item label="起始序号">
        <el-input-number v-model="startNumber" :min="0" />
      </el-form-item>
      <el-form-item label="序号位数">
        <el-input-number v-model="paddingLength" :min="1" :max="10" />
      </el-form-item>
      <el-form-item label="预览数量">
        <el-input-number v-model="previewCount" :min="1" :max="20" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="handleApplyRename">
          应用重命名
        </el-button>
      </el-form-item>
    </el-form>

    <el-divider>命名预览</el-divider>
    <el-space wrap>
      <el-tag
        v-for="(name, index) in previewNames"
        :key="`${name}-${index}`"
        type="success"
      >
        {{ name }}
      </el-tag>
    </el-space>
  </el-card>
</template>

<style scoped>
.rename-card {
  width: 100%;
}

.rename-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
</style>
