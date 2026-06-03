<script lang="ts" setup>
  import { ref } from "vue";
  import { invoke } from "@tauri-apps/api/core";
  import { listen, emit } from "@tauri-apps/api/event";
  import { onMounted, onUnmounted, watch } from "vue";
  import TitleBar from "../components/TitleBar.vue";
  import { basename, dirname, join, extname } from "@tauri-apps/api/path";
  import { reactive } from "vue";
  import { ElMessage } from "element-plus";

  const oldFilePath = ref<string>("");
  const oldFileName = ref<string>("");

  const formData = reactive({
    fileDir: "",
    oldFileName: "",
    newFileName: "",
    fileExtension: "",
  });

  let unlisten: (() => void) | undefined;

  onMounted(async () => {
    console.log("onMounted in rename page");

    unlisten = await listen("rename_old_file_path", (event) => {
      console.log("listen rename_old_file_path in rename page", event.payload);
      oldFilePath.value = (
        event.payload as { old_file_path: string }
      ).old_file_path;
    });
    formData.oldFileName = oldFilePath.value.split("/").pop() ?? "";
    const requestedOldFilePath = await invoke<string | null>(
      "request_rename_old_file_path_command",
    );
    if (requestedOldFilePath) {
      console.log(
        "requested old file path in rename page",
        requestedOldFilePath,
      );
      oldFilePath.value = requestedOldFilePath;
      formData.fileDir = await dirname(requestedOldFilePath);
      formData.oldFileName = await basename(requestedOldFilePath);
      formData.fileExtension = await extname(requestedOldFilePath);
    }
  });

  onUnmounted(() => {
    if (unlisten) {
      unlisten();
      unlisten = undefined;
    }
  });

  watch(
    () => {
      console.log("watch oldFilePath in rename page", oldFilePath.value);
      return oldFilePath.value;
    },
    async (newVal: string) => {
      console.log("watch oldFilePath new value", newVal);
      formData.oldFileName = await basename(newVal);
      formData.fileExtension = await extname(newVal);
      console.log("watch oldFileName", oldFileName.value);
    },
    {
      immediate: true,
    },
  );

  const handleRename = async () => {
    console.log("handleRename in rename page", formData);
    const oldfilePath = await join(formData.fileDir, formData.oldFileName);
    let newfilePath = await join(formData.fileDir, formData.newFileName);
    newfilePath = newfilePath + "." + formData.fileExtension;
    console.log("oldfilePath", oldfilePath);
    console.log("newfilePath", newfilePath);
    await invoke("rename_file_command", {
      source: oldfilePath,
      target: newfilePath,
    })
      .then(async () => {
        ElMessage.success("重命名成功");

        await emit("file_renamed", {
          file_dir: formData.fileDir,
          old_file_path: oldfilePath,
          new_file_path: newfilePath,
        });

        // await invoke("close_rename_window_command");
      })
      .catch((error) => {
        console.error("rename file failed:", error);
        ElMessage.error("重命名失败");
      })
      .finally(async () => {
        await invoke("close_rename_window_command");
      });
  };

  const handleCancel = async () => {
    console.log("handleCancel in rename page");
    await invoke("close_rename_window_command");
  };
</script>

<template>
  <div class="rename-page">
    <TitleBar title="文件重命名" :showMinimize="false" :showMaximize="false" />
  </div>
  <div>
    <div class="old-file-name">
      <el-form :model="formData" label-width="80px">
        <el-form-item label="旧文件名:">
          <el-text class="mx-1" type="info" truncate>{{
            formData.oldFileName
          }}</el-text>
        </el-form-item>
        <el-form-item label="新文件名:">
          <el-input
            v-model="formData.newFileName"
            placeholder="请输入新文件名"
            clearable
            style="max-width: 260px"
          />
        </el-form-item>
        <el-form-item label="文件格式:">
          <el-text class="mx-1" type="info" truncate>{{
            formData.fileExtension
          }}</el-text>
        </el-form-item>
      </el-form>
    </div>
    <div class="rename-footer">
      <el-button
        class="button-style"
        type="primary"
        @click="handleRename"
        :disabled="!formData.newFileName"
        >重命名</el-button
      >
      <el-button class="button-style" type="default" @click="handleCancel"
        >取消</el-button
      >
    </div>
  </div>
</template>

<style scoped>
  .rename-page {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 40px;
    overflow: hidden;
  }
  .old-file-name {
    display: flex;
    flex-direction: row;
    width: 100%;
    height: 100%;
    overflow: hidden;
    align-items: center;
    justify-content: start;
    padding: 0 16px;
  }

  .rename-footer {
    display: flex;
    flex-direction: row;
    width: 100%;
    height: 40px;
    overflow: hidden;
    align-items: center;
    justify-content: center;
    padding: 0 16px;
  }

  .button-style {
    width: 100px;
    height: 30px;
  }
</style>
