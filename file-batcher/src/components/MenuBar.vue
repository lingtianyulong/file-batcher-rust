<script setup lang="ts">
import { ref } from 'vue'

import MenuBar from 'primevue/menubar'

// const menu = ref()

const items = ref([
    {
        label: '文件',
        items: [
            {
                label: '打开',
                icon: 'pi pi-folder-open',
                command: () => {
                    console.log('打开')
                }
            },
            {
                label: '删除',
                icon: 'pi pi-trash',
                command: () => {
                    console.log('删除')
                }
            },
            {
                label: '退出',
                icon: 'pi pi-power-off',
                command: () => {
                    console.log('退出')
                }
            }
        ]
    },
    {
        label: '选项',
        items: [
            {
                label: '设置',
                icon: 'pi pi-cog',
                command: () => {
                    console.log('设置')
                }
            },
            {
                label: '主题',
                icon: 'pi pi-cog',
                command: () => {
                    console.log('主题')
                }
            }
        ]
    },
    {
        label: '工具',
        items: [
            {
                label: '批量处理',
                icon: 'pi pi-cog',
                command: () => {
                    console.log('批量处理')
                }
            },
            {
                label: '格式转换',
                icon: 'pi pi-cog',
                command: () => {
                    console.log('日志')
                }
            }
        ]
    },
    {
        label: '关于',
        command: () => {
            console.log('关于')
        }
    }
])


</script>

<template>
    <MenuBar :model="items" class="p-menubar menubar"/>
</template>

<style scoped>
.menubar {
    width: 100%;
    height: 40px;
    border-radius: 0;
    font-size: 14px;
    font-family: 'Microsoft YaHei';
    font-weight: bold;
}

</style>