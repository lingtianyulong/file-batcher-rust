<script setup lang="ts">
import { Folder, Document } from '@element-plus/icons-vue'
import { Monitor, HardDrive } from '@lucide/vue'
import type { Component } from 'vue'
import { nextTick, onMounted, ref, useTemplateRef, watch } from 'vue'
import { useFolderStore } from '../stores/file-store'
import { invoke } from '@tauri-apps/api/core'

const folderStore = useFolderStore()
const treeContainerRef = useTemplateRef<HTMLElement>('treeContainerRef')
const treeHeight = ref(200)
const diskInfo = ref<RawDiskInfo>({})

type RawDiskInfo = {
  diskName?: string
  fileSystem?: string
  mountPoint?: string
  totalSpace?: number
  usedSpace?: number
  freeSpace?: number
}

type TreeNode = {
  id: string
  label: string
  type: string
  path?: string
  children?: TreeNode[]
}

const treeProps = {
  value: 'id',
  label: 'label',
  children: 'children',
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

function hasDiskInfo(info: RawDiskInfo) {
  return Boolean(info.mountPoint ?? info.diskName ?? info.totalSpace)
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

/** el-tree-v2 仅在 data 引用变化时重建树，且子节点需父节点在 defaultExpandedKeys 中 */
const defaultExpandedKeys = ref<string[]>(['__root__'])

function replaceTreeRoot(root: TreeNode) {
  treeData.value = [root]
}

async function addDriveLetterNode(disk: RawDiskInfo) {
  if (!disk?.mountPoint) {
    return
  }
  const root = treeData.value[0]
  const children = root.children ?? []
  const mountPoint = disk.mountPoint
  if (children.some((n) => n.id === mountPoint)) {
    return
  }
  replaceTreeRoot({
    ...root,
    children: [
      ...children,
      {
        id: mountPoint,
        label: mountPoint,
        type: 'disk',
        children: []
      }
    ]
  })
}

onMounted(async () => {
  await nextTick()
  const container = treeContainerRef.value
  if (container && container.clientHeight > 0) {
    treeHeight.value = container.clientHeight
  }

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

    for (const disk of sortedDiskList) {
      const driveLetter = disk.mount_point?.slice(0, 2) ?? ""
      if (!driveLetter) continue
      const info: RawDiskInfo = {
        mountPoint: driveLetter,
        fileSystem: disk.file_system ?? "",
        diskName: disk.disk_name ?? "",
      }
      await addDriveLetterNode(info)
    }

    if (!hasDiskInfo(diskInfo.value)) {
      diskInfo.value = await getDiskInfo("C:")
    }

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

async function getDiskInfo(diskName: string): Promise<RawDiskInfo> {
  const disk_info = await invoke<string>("get_disk_info_command", { diskName })
  const parsed = JSON.parse(disk_info) as {
    disk_name?: string
    file_system?: string
    mount_point?: string
    total_space?: number
    used_space?: number
    free_space?: number
  }

  const info: RawDiskInfo = {
    diskName: parsed.disk_name ?? "",
    fileSystem: parsed.file_system ?? "",
    mountPoint: parsed.mount_point ?? "",
    totalSpace: parsed.total_space ?? 0,
    usedSpace: parsed.used_space ?? 0,
    freeSpace: parsed.free_space ?? 0
  }
  return info
}


/** 节点点击事件 */
async function handleNodeClick(node: TreeNode) {
  const info: RawDiskInfo = await getDiskInfo(node.label)
  diskInfo.value = info
}

/** 节点展开事件 */
function handleNodeExpand(node: TreeNode) {
  // console.log("handleNodeExpand", node)

}

</script>

<template>
  <div ref="treeContainerRef" class="folder-tree-container">
    <el-tree-v2
      :data="treeData"
      :props="treeProps"
      :default-expanded-keys="defaultExpandedKeys"
      :height="treeHeight"
      @node-click="handleNodeClick"
      @node-expand="handleNodeExpand"
    >
      <template #default="{ node, data }">
        <div class="tree-node">
          <el-icon>
            <component :is="getNodeIcon(data.type)" />
          </el-icon>
          <span>{{ node.label }}</span>
        </div>
      </template>
    </el-tree-v2>
  </div>
  <el-card class="folder-tree-card" shadow="never">
    <span style="font-size: 14px; font-weight: bold;">磁盘信息</span>
    <div class="disk-info">
      <div style="font-size: 13px;" v-if="diskInfo.diskName && diskInfo.mountPoint">{{ (diskInfo.diskName ?? "") + " (" + (diskInfo.mountPoint.slice(0, 2) ?? "") + ")" }}</div>
      <div v-if="diskInfo">
        <el-progress
          :percentage="diskInfo.totalSpace ? diskInfo.usedSpace ? diskInfo.usedSpace / diskInfo.totalSpace * 100 : 0 : 0"
          :show-text="false"
        />
      </div>
      <span style="font-size: 13px;">可用: {{ diskInfo.freeSpace ?? 0 }} GB, 总容量: {{ diskInfo.totalSpace ?? 0 }} GB</span>
    </div>
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

.folder-tree-card :deep(.el-card__body) {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.disk-info {
  display: flex;
  flex-direction: column;
  justify-content: center;
  min-height: 0;
  overflow: hidden;
  flex: 1;
  gap: 6px;
}

.disk-info-progress {
  display: flex;
  align-items: center;
  gap: 6px;
  flex: 1;
  overflow: hidden;
  flex-direction: row;
}

:deep(.el-progress-bar__inner) {
  border-radius: 2px;
}

:deep(.el-progress-bar__outer) {
  border-radius: 2px;
  height: 14px !important;
}


</style>