<script setup lang="ts">
import { computed, ref } from "vue";

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

const visible = computed({
  get: () => props.modelValue,
  set: (value: boolean) => emit("update:modelValue", value),
});

function closeDialog() {
  visible.value = false;
}

function handleLogin() {
  if (!loginForm.value.username || !loginForm.value.password) {
    return;
  }
  emit("login-success");
  closeDialog();
}
</script>

<template>
  <el-dialog v-model="visible" title="登录" width="420px">
    <el-form label-position="top">
      <el-form-item label="账号">
        <el-input v-model="loginForm.username" placeholder="请输入账号" />
      </el-form-item>
      <el-form-item label="密码">
        <el-input v-model="loginForm.password" type="password" show-password placeholder="请输入密码" />
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button @click="closeDialog">取消</el-button>
      <el-button type="primary" @click="handleLogin">登录</el-button>
    </template>
  </el-dialog>
</template>
