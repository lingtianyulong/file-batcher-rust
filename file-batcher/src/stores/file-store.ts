import { defineStore } from 'pinia'

export const useFolderStore = defineStore('folder', {
    state: () => ({
        currentPath: '' as string,
        fileList: [] as string[]
    }),

    actions: {
        setPath(path: string) {
            this.currentPath = path
        },

        setFiles(files: string[]) {
            this.fileList = files
        }
    }
})