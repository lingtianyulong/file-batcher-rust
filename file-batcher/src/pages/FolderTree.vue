<template>
  <el-tree
    :data="treeData"
    node-key="id"
    default-expand-all
  >
    <template #default="{ node, data }">
      <div class="tree-node">
        <el-icon v-if="data.type === 'monitor'">
          <Monitor />
        </el-icon>

        <el-icon v-else-if="data.type === 'folder'">
          <Folder />
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
import { Monitor } from '@lucide/vue'
import { ref, watch } from 'vue'
import { basename } from '@tauri-apps/api/path'
import { useFolderStore } from '../stores/file-store'

const folderStore = useFolderStore()

/** 根节点固定 id，子节点用目录 path 作为 id 便于去重 */
const treeData = ref([
  {
    id: '__root__',
    label: '我的电脑',
    type: 'monitor',
    children: []
  }
])

async function addFolderNodeIfNeeded(dirPath) {
  if (!dirPath) return
  const root = treeData.value[0]
  if (!root.children) {
    root.children = []
  }
  const exists = root.children.some((n) => n.path === dirPath || n.id === dirPath)
  if (exists) return
  const label = await basename(dirPath)
  root.children.push({
    id: dirPath,
    label,
    type: 'folder',
    path: dirPath,
    children: []
  })
}

watch(
  () => folderStore.currentPath,
  (p) => {
    if (p) {
      void addFolderNodeIfNeeded(p)
    }
  },
  { immediate: true }
)

</script>

<style scoped>
.tree-node {
  display: flex;
  align-items: center;
  gap: 6px;
}
</style>