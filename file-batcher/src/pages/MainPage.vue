<script lang="ts" setup>
import FileList from "./FileList.vue";
</script>

<template>
    <el-container>
        <el-aside class="left-siderbar siderbar">
            <div> left </div>
        </el-aside>
        <el-main>
            <FileList />
        </el-main>
        <el-aside class="right-siderbar siderbar">
            <div> right </div>
        </el-aside>
    </el-container>
</template>

<!-- style scoped 只在当前组件中生效 -->
<style scoped>

.left-siderbar {
    border-right: 1px solid #dcdfe6;

}

.right-siderbar {
    border-left: 1px solid #dcdfe6;
}

.siderbar {
    background: #ffffff;
    transition: width 0.2s ease;
    overflow: hidden;   /* 移除滚动条 */
}


</style>