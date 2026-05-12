<script lang="ts" setup>
import FileList from "./FileList.vue";
import FolderTree from "./FolderTree.vue";
import { useFolderStore } from '../stores/file-store';
import { onMounted, watch } from 'vue';

const folderStore = useFolderStore();

onMounted(() => {
    if (folderStore.currentPath) {
        console.log("onMounted in main page", folderStore.currentPath);
    } else {
        // console.log("没有选择文件夹");
    }
    // console.log(folderStore.currentPath);
});

watch(
    () => {
        console.log("watch in main page", folderStore.currentPath);
        return folderStore.currentPath;
    },
    (newPath) => {
        // console.log("watch in main page", newPath);
        if (newPath) {
            console.log("watch new path in main page", newPath);
        } else {
            // console.log("没有选择文件夹");
        }
    }
)

</script>

<template>
    <el-container>
        <el-aside class="left-siderbar siderbar">
            <FolderTree />
        </el-aside>
        <el-container>
          <el-main>
            <!-- <FileList /> -->
          </el-main>
          <el-footer class="footer">Footer</el-footer>
        </el-container>
    </el-container>
</template>

<!-- style scoped 只在当前组件中生效 -->
<style scoped>

.left-siderbar {
    border-right: 1px solid #dcdfe6;

}

.footer {
    border-top: 1px solid #dcdfe6;
}

.siderbar {
    background: #ffffff;
    transition: width 0.2s ease;
    overflow: hidden;   /* 移除滚动条 */
}


</style>