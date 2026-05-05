<script setup lang="ts">
import { computed, ref } from "vue";
import { CloseBold, Lock, User } from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps<{
  modelValue: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
  (e: "login-success"): void;
}>();

const loginForm = ref({
  username: "",
  password: "",
});
const loginError = ref("");

const visible = computed({
  get: () => props.modelValue,
  set: (value: boolean) => emit("update:modelValue", value),
});

function closeDialog() {
  visible.value = false;
}

function handleLogin() {
  loginError.value = "";
  if (!loginForm.value.username || !loginForm.value.password) {
    loginError.value = "请输入账号和密码";
    return;
  }
  invoke("login_command", { username: loginForm.value.username, password: loginForm.value.password })
  .then((result) => {
    console.log("login success", result);
    emit("login-success");
    closeDialog();
  })
  .catch((error) => {
    console.error("login failed, the reason is {}", error);
    loginError.value = error;
  });
}
</script>

<template>
  <el-dialog v-model="visible" width="420px" :show-close="false" align-center class="login-dialog">
    <el-button class="login-close-btn" :icon="CloseBold" @click="closeDialog" />
    <div class="login-panel">
      <div class="login-avatar">Q</div>
      <div class="login-title">账号登录</div>
      <div class="login-subtitle">欢迎使用 FileBatcher</div>

      <el-form label-position="top" class="login-form">
        <el-form-item>
          <el-input v-model="loginForm.username" placeholder="QQ号 / 邮箱 / 手机号" clearable size="large">
            <template #prefix>
              <el-icon><User /></el-icon>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item>
          <el-input
            v-model="loginForm.password"
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
      </el-form>
      <div v-if="loginError" class="login-error-tip">{{ loginError }}</div>

      <el-button class="login-submit" type="primary" @click="handleLogin">登录</el-button>
      <div class="login-links">
        <el-button text>注册账号</el-button>
        <span class="login-link-divider">|</span>
        <el-button text>忘记密码</el-button>
      </div>


    </div>
  </el-dialog>
</template>

<style scoped>
.login-panel {
  position: relative;
  padding: 6px 18px 10px;
}

.login-close-btn {
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

:deep(.login-close-btn:hover) {
  background-color: #e81123;
  color: #fff;
}

.login-avatar {
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

.login-title {
  text-align: center;
  font-size: 20px;
  font-weight: 600;
  color: #1f2d3d;
}

.login-subtitle {
  margin-top: 4px;
  margin-bottom: 16px;
  text-align: center;
  font-size: 13px;
  color: #7c8a9b;
}

.login-form {
  margin-top: 2px;
}

.login-links {
  margin: 4px 0 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #7c8a9b;
}

.login-link-divider {
  margin: 0 4px;
  color: #c0c4cc;
}

.login-submit {
  width: 100%;
  height: 42px;
  font-size: 15px;
  border-radius: 8px;
}

.login-error-tip {
  margin: -4px 0 10px;
  min-height: 20px;
  color: #f56c6c;
  font-size: 13px;
  line-height: 20px;
}

:deep(.login-dialog .el-dialog) {
  border-radius: 14px;
  overflow: hidden;
}

:deep(.login-dialog .el-dialog__body) {
  padding: 14px 24px 18px;
}

:deep(.login-form .el-input__wrapper) {
  border-radius: 8px;
}

:deep(.login-links .el-button.is-text) {
  color: #4d8dff;
}
</style>
