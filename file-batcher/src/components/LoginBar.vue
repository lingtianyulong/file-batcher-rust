<script setup lang="ts">
import { Setting, User, UserFilled } from "@element-plus/icons-vue";
import { ref } from "vue";
import LoginDialog from "./LoginDialog.vue";

const isLoggedIn = ref(false);
const loginDialogVisible = ref(false);

function openLoginDialog() {
  loginDialogVisible.value = true;
}

function handleLoginSuccess() {
  isLoggedIn.value = true;
}

function handleUserCommand(command: string) {
  if (command === "logout") {
    isLoggedIn.value = false;
  }
}

</script>

<template>
    <div class="login-style">
        <el-button type="default" style="border: none; font-size: 20px;"  circle :icon="Setting"></el-button>
        <el-button
          v-if="!isLoggedIn"
          type="default"
          style="border: none; font-size: 20px;"
          circle
          :icon="User"
          @click="openLoginDialog"
        />
        <el-dropdown v-else trigger="click" @command="handleUserCommand">
          <el-button type="default" style="border: none;" circle :icon="User" />
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="profile">
                <el-icon><UserFilled /></el-icon>
                个人中心
              </el-dropdown-item>
              <el-dropdown-item command="settings">
                <el-icon><Setting /></el-icon>
                账号设置
              </el-dropdown-item>
              <el-dropdown-item command="logout" divided>退出登录</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
    </div>
    <LoginDialog v-model="loginDialogVisible" @login-success="handleLoginSuccess" />
</template>

<style scoped>

.login-style {
  width: 100%;
  box-sizing: border-box;
  height: 40px;
  display: flex;
  justify-content: flex-end;
  align-items: center;
  background-color: #ffffff;
  border-bottom: 1px solid #e4e7ed;
  padding: 5px 5px 5px 20px;
}

</style>
