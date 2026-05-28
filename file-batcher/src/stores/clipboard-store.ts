import { defineStore } from 'pinia'

export type ClipboardMode = 'copy' | 'cut'

export const useClipboardStore = defineStore('clipboard', {
    state: () => ({
        files: [] as string[],
        mode: 'copy' as ClipboardMode
    }),
    actions: {
        cutFiles(files: string[]) {
            this.files = files
            this.mode = 'cut'
        },
        copyFiles(files: string[]) {
            this.files = files
            this.mode = 'copy'
        },
        clear() {
            this.files = []
        }

    }
})