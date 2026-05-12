<template>
  <el-tree
    :data="treeData"
    node-key="id"
    default-expand-all
  >
    <template #default="{ node, data }">
      <div class="tree-node">
        <el-icon v-if="data.type === 'folder'">
            <Folder v-if="!node.expanded"/>
            <FolderOpened v-else/>
        </el-icon>

        <el-icon v-else>
            <Document />
        </el-icon>

        <span class="label">
          {{ node.label }}
        </span>
      </div>
    </template>
  </el-tree>
</template>

<script setup>
import { Folder, Document } from '@element-plus/icons-vue'

const treeData = [
  {
    id: 1,
    label: 'src',
    type: 'folder',
    children: [
      {
        id: 2,
        label: 'main.ts',
        type: 'file'
      }
    ]
  }
]
</script>

<style scoped>
.tree-node {
  display: flex;
  align-items: center;
  gap: 6px;
}
</style>