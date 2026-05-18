<script setup lang="ts">
import { Folder, Document } from '@element-plus/icons-vue'
import { Monitor, HardDrive } from '@lucide/vue'
import type { Component } from 'vue'
import { onMounted, ref, watch } from 'vue'
// import { basename } from '@tauri-apps/api/path'
import { useFolderStore } from '../stores/file-store'
import { invoke } from '@tauri-apps/api/core'

const folderStore = useFolderStore()

type RawDiskInfo = {
  diskName?: string
  fileSystem?: string
  mountPoint?: string
}

type TreeNode = {
  id: string
  label: string
  type: string
  path?: string
  children: TreeNode[]
}

const iconMap: Record<string, Component> = {
  'monitor': Monitor,
  'disk': HardDrive,
  'folder': Folder,
  'file': Document
}

function getNodeIcon(type: string) {
  return iconMap[type] ?? Document
}


/** 根节点固定 id，子节点用目录 path 作为 id 便于去重 */
const treeData = ref<TreeNode[]>([
  {
    id: '__root__',
    label: '我的电脑',
    type: 'monitor',
    children: []
  }
])

// async function addFolderNodeIfNeeded(dirPath: string) {
//   if (!dirPath) return
//   const root = treeData.value[0].children[0]
//   if (!root.children) {
//     root.children = []
//   }
//   const exists = root.children.some((n) => n.path === dirPath || n.id === dirPath)
//   if (exists) return
//   const label = await basename(dirPath)
//   root.children.push({
//     id: dirPath,
//     label,
//     type: 'folder',
//     path: dirPath,
//     children: []
//   })
// }

async function addDriveLetterNode(disk: RawDiskInfo) {
  if (!disk) {
    return
  }
  const root = treeData.value[0]
  if (!root.children) {
    root.children = []
  }
  const exists = root.children.some((n) => n.label === disk.mountPoint)
  if (exists) {
    return
  }
  root.children.push({
    id: disk.mountPoint ?? "",
    label: disk.mountPoint ?? "",
    type: 'disk',
    children: []
  })
}

onMounted(async () => {
  try {
    const disk_list = await invoke<string>("get_disk_list_command")
    console.log("disk_list", disk_list)
    const parsed = JSON.parse(disk_list) as {
      disk_name?: string
      file_system?: string
      mount_point?: string
    }[]

    console.log("parsed disk_list", parsed)
    const sortedDiskList = [...parsed].sort((a, b) => {
      const leftDriveLetter = a.mount_point?.slice(0, 2).toUpperCase() ?? ""
      const rightDriveLetter = b.mount_point?.slice(0, 2).toUpperCase() ?? ""
      return leftDriveLetter.localeCompare(rightDriveLetter)
    })
    sortedDiskList.forEach(async (disk) => {
      if (disk) {
        let driveLetter = disk.mount_point?.slice(0, 2) ?? ""
        if (driveLetter) {
          let info: RawDiskInfo = {
            mountPoint: driveLetter ?? "",
            fileSystem: disk.file_system ?? "",
            diskName: disk.disk_name ?? "",
          }
         await addDriveLetterNode(info)
        }
      }
    })
  } catch (error) {
    console.error("get disk info command failed", error)
  }
})

watch(
  () => folderStore.currentPath,
  (p) => {
    if (p) {
      // void addFolderNodeIfNeeded(p)
    }
  },
  { immediate: true }
)

</script>

<template>
  <div class="folder-tree-container">
    <el-tree :data="treeData" node-key="id"> 
      <template #default="{ node, data }">
        <div class="tree-node">
          <el-icon>
            <component :is="getNodeIcon(data.type)" />
          </el-icon> 
          <span>{{ node.label }}</span>
        </div>
      </template>
    </el-tree>
  </div>
  <el-card class="folder-tree-card" shadow="never">
    <span>磁盘信息</span>
  </el-card>
</template>

<style scoped>
.tree-node {
  display: flex;
  align-items: center;
  gap: 6px;
}

.folder-tree-container {
  height: 80%;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
  flex: 1;
}

.folder-tree-card {
  height: 20%;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
  flex: 1;
}


</style>