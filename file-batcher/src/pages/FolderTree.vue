<script setup lang="ts">
import { Folder, Document } from '@element-plus/icons-vue'
import * as icons from '@lucide/vue'
import type { Component } from 'vue'
import { nextTick, onMounted, ref, useTemplateRef, watch } from 'vue'
import { useFolderStore } from '../stores/file-store'
import { invoke } from '@tauri-apps/api/core'
import { Icon } from '@iconify/vue'

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

type DirInfo = {
  name: string
  path: string
  isDir: boolean
  hasSubDir: boolean
  subDirs: DirInfo[]
  fileType?: string
}

type TreeNode = {
  id: string
  label: string
  type: string
  path?: string
  loaded?: boolean
  children?: TreeNode[]
}

const treeProps = {
  value: 'id',
  label: 'label',
  children: 'children',
}


const iconMap: Record<string, string> = {
  'zip': 'fluent:folder-zip-24-regular',
  'rar': 'fluent:folder-zip-24-regular',
  '7z': 'fluent:folder-zip-24-regular',

  'txt': 'fluent:document-bullet-list-24-regular',
  'pdf': 'vscode-icons:file-type-pdf2',
  'rs': 'vscode-icons:file-type-rust',
  'vue': 'vscode-icons:file-type-vue',

  'png': 'vscode-icons:file-type-image',
  'jpg': 'vscode-icons:file-type-image',
  'jpeg': 'vscode-icons:file-type-image',
  'gif': 'vscode-icons:file-type-image',
  'bmp': 'vscode-icons:file-type-image',

  'xml': 'vscode-icons:file-type-xml',

  'cs': 'vscode-icons:file-type-csharp',
  'cpp': 'vscode-icons:file-type-cpp',
  'c': 'vscode-icons:file-type-c',
}

function getFileIcon(type: string) {
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

/** el-tree-v2 会监听 defaultExpandedKeys，异步替换 children 后用它刷新可见节点 */
const expandedKeys = ref<string[]>(['__root__'])

function replaceTreeRoot(root: TreeNode) {
  treeData.value = [root]
}

function createLoadingChild(parentId: string): TreeNode {
  return {
    id: `${parentId}__loading__`,
    label: '加载中...',
    type: 'file'
  }
}

function ensureExpanded(id: string) {
  if (!expandedKeys.value.includes(id)) {
    expandedKeys.value = [...expandedKeys.value, id]
  }
}

function removeExpanded(id: string) {
  expandedKeys.value = expandedKeys.value.filter((key) => key !== id)
}

/** el-tree-v2 对深层 children 变更不敏感，需替换整条 data 才能刷新 */
function mergeChildrenIntoTree(
  nodes: TreeNode[],
  targetId: string,
  incoming: TreeNode[]
): TreeNode[] {
  return nodes.map((n) => {
    if (n.id === targetId) {
      const byId = new Map<string, TreeNode>()
      for (const c of incoming) {
        byId.set(c.id, c)
      }
      return { ...n, loaded: true, children: [...byId.values()] }
    }
    if (n.children?.length) {
      return { ...n, children: mergeChildrenIntoTree(n.children, targetId, incoming) }
    }
    return n
  })
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
        children: [createLoadingChild(mountPoint)]
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
async function handleNodeExpand(node: TreeNode) {
  if (node.type !== 'disk' && node.type !== 'folder') {
    return
  }
  ensureExpanded(node.id)
  if (node.loaded) {
    return
  }
  const dirPath = node.path ?? node.label
  try {
    const dir_info = await invoke<string>("get_dir_info_command", { dirPath })
    const parsed = JSON.parse(dir_info) as {
      name: string
      path: string
      isDir: boolean
      hasSubDir: boolean
      subDirs: DirInfo[]
    }
    const dirInfo: DirInfo = {
      name: parsed.name,
      path: parsed.path,
      isDir: parsed.isDir,
      hasSubDir: parsed.hasSubDir,
      subDirs: parsed.subDirs ?? []
    }
    const incoming: TreeNode[] = dirInfo.subDirs.map((sub) => ({
      id: sub.path,
      label: sub.name,
      type: sub.isDir ? 'folder' : (sub.fileType ?? 'file'),
      path: sub.path,
      children: sub.isDir ? [createLoadingChild(sub.path)] : undefined
    }))
    treeData.value = mergeChildrenIntoTree(treeData.value, node.id, incoming)
    ensureExpanded(node.id)
  } catch (e) {
    console.error("get_dir_info_command failed", e)
  }
}

/** 节点收起事件 */
function handleNodeCollapse(node: TreeNode) {
  removeExpanded(node.id)
}

const customColor = (precentage: number): string => {
  console.log("precentage", precentage)
  if (precentage < 80) {
    return '#409EFF' // 蓝色
  } else if (precentage < 95) {
    return '#F56C6C' // 红色
  } else {
    return '#8B0000' // 深红色
  }
}

</script>

<template>
  <div ref="treeContainerRef" class="folder-tree-container">
    <el-tree-v2
      :data="treeData"
      :props="treeProps"
      :default-expanded-keys="expandedKeys"
      :height="treeHeight"
      @node-click="handleNodeClick"
      @node-expand="handleNodeExpand"
      @node-collapse="handleNodeCollapse"
    >
      <template #default="{ node, data }">
        <div class="tree-node">
          <div v-if="data.type === 'monitor'">
            <el-icon>
              <Monitor />
            </el-icon>
          </div>
          <div v-else-if="data.type === 'disk'">
            <Icon icon="fluent:hard-drive-48-regular" />
          </div>
          <div v-else-if="data.type === 'folder'">
            <el-icon>
              <Folder />
            </el-icon>
          </div>
          <div v-else>
            <Icon :icon="getFileIcon(data.type)"/>
          </div>
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
          :color="customColor"
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