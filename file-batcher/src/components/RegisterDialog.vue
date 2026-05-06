<script setup lang="ts">
import { computed, ref } from "vue";
import { CloseBold, Lock, User } from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from "element-plus";

const props = defineProps<{
  modelValue: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
}>();

const registerForm = ref({
  username: "",
  password: "",
  confirmPassword: "",
});
const registerError = ref("");

const visible = computed({
  get: () => props.modelValue,
  set: (value: boolean) => emit("update:modelValue", value),
});

function closeDialog() {
  visible.value = false;
}

function handleRegister() {
  registerError.value = "";
  if (!registerForm.value.username || !registerForm.value.password || !registerForm.value.confirmPassword) {
    registerError.value = "请完整填写注册信息";
    return;
  }
  if (registerForm.value.password !== registerForm.value.confirmPassword) {
    registerError.value = "两次输入的密码不一致";
    return;
  }
  invoke("register_command", { username: registerForm.value.username, password: registerForm.value.password })
  .then((result) => {
    console.log("register success", result);
    ElMessage.success("注册成功");
    closeDialog();
  })
  .catch((error) => {
    console.error("register failed, the reason is {}", error);
    ElMessage.error("注册失败，请检查账号和密码");
    registerError.value = error;
  });
  closeDialog();
}
</script>

<template>
  <el-dialog v-model="visible" width="420px" :show-close="false" align-center class="register-dialog">
    <el-button class="register-close-btn" :icon="CloseBold" @click="closeDialog" />
    <div class="register-panel">
      <div class="register-avatar">R</div>
      <div class="register-title">注册账号</div>
      <div class="register-subtitle">创建你的 FileBatcher 账号</div>

      <el-form label-position="top" class="register-form">
        <el-form-item>
          <el-input v-model="registerForm.username" placeholder="请输入账号" clearable size="large">
            <template #prefix>
              <el-icon><User /></el-icon>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item>
          <el-input
            v-model="registerForm.password"
            type="password"
            show-password
            placeholder="请输入密码"
            size="large"
          >
            <template #prefix>
              <el-icon><Lock /></el-icon>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item>
          <el-input
            v-model="registerForm.confirmPassword"
            type="password"
            show-password
            placeholder="请再次输入密码"
            size="large"
          >
            <template #prefix>
              <el-icon><Lock /></el-icon>
            </template>
          </el-input>
        </el-form-item>
      </el-form>

      <div v-if="registerError" class="register-error-tip">{{ registerError }}</div>

      <el-button class="register-submit" type="primary" @click="handleRegister">注册</el-button>
    </div>
  </el-dialog>
</template>

<style scoped>
.register-panel {
  position: relative;
  padding: 6px 18px 10px;
}

.register-close-btn {
  position: absolute;
  top: 0px;
  right: 0px;
  width: 28px;
  height: 28px;
  padding: 0;
  z-index: 2;
  transition: background-color 0.2s ease, color 0.2s ease;
  --el-button-text-color: #909399;
  --el-button-bg-color: transparent;
  --el-button-border-color: transparent;
  --el-button-hover-text-color: #ffffff;
  --el-button-hover-bg-color: #e81123;
  --el-button-hover-border-color: #e81123;
}

:deep(.register-close-btn:hover) {
  background-color: #e81123;
  color: #fff;
}

.register-avatar {
  width: 72px;
  height: 72px;
  margin: 2px auto 14px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32px;
  font-weight: 700;
  color: #fff;
  background: linear-gradient(145deg, #32a9ff, #1677ff);
  box-shadow: 0 6px 18px rgba(22, 119, 255, 0.28);
}

.register-title {
  text-align: center;
  font-size: 20px;
  font-weight: 600;
  color: #1f2d3d;
}

.register-subtitle {
  margin-top: 4px;
  margin-bottom: 16px;
  text-align: center;
  font-size: 13px;
  color: #7c8a9b;
}

.register-form {
  margin-top: 2px;
}

.register-submit {
  width: 100%;
  height: 42px;
  font-size: 15px;
  border-radius: 8px;
}

.register-error-tip {
  margin: -4px 0 10px;
  min-height: 20px;
  color: #f56c6c;
  font-size: 13px;
  line-height: 20px;
}

:deep(.register-dialog .el-dialog) {
  border-radius: 14px;
  overflow: hidden;
}

:deep(.register-dialog .el-dialog__body) {
  padding: 14px 24px 18px;
}

:deep(.register-form .el-input__wrapper) {
  border-radius: 8px;
}
</style>
