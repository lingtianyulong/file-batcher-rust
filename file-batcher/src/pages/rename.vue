<script lang="ts" setup>
  import { ref } from "vue";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMounted, onUnmounted, watch } from "vue";
  import TitleBar from "../components/TitleBar.vue";

  const oldFilePath = ref<string>("");
  let unlisten: (() => void) | undefined;

  onMounted(async () => {
    console.log("onMounted in rename page");

    unlisten = await listen("rename_old_file_path", (event) => {
      console.log("listen rename_old_file_path in rename page", event.payload);
      oldFilePath.value = (
        event.payload as { old_file_path: string }
      ).old_file_path;
    });

    const requestedOldFilePath = await invoke<string | null>(
      "request_rename_old_file_path_command",
    );
    if (requestedOldFilePath) {
      console.log(
        "requested old file path in rename page",
        requestedOldFilePath,
      );
      oldFilePath.value = requestedOldFilePath;
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
    (newVal: string) => {
      console.log("watch oldFilePath", newVal);
    },
    {
      immediate: true,
    },
  );
</script>

<template>
  <div class="rename-page">
    <TitleBar title="文件重命名" />
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
</style>
