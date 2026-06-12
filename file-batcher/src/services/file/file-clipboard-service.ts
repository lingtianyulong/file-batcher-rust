import { invoke } from "@tauri-apps/api/core";
import { useClipboardStore } from "../../stores/clipboard-store";

export class FileClipboardService {
  static copy(files: string[]) {
    const store = useClipboardStore();
    store.copyFiles(files);
  }

  static cut(files: string[]) {
    const store = useClipboardStore();
    store.cutFiles(files);
  }

  static clear() {
    const store = useClipboardStore();
    store.clear();
  }

  static async paste(target: string) {
    const store = useClipboardStore();
    const files = store.files;
    const mode = store.mode;
    if (mode === "copy") {
      try {
        await invoke("paste_files_command", {
          sources: files,
          target,
          isCut: false,
        });
      } catch (error) {
        console.error("paste files failed", error);
        store.clear();
      }
    } else {
      try {
        await invoke("paste_files_command", {
          sources: files,
          target,
          isCut: true,
        });
      } catch (error) {
        console.error("paste files failed", error);
      } finally {
        store.clear();
      }
    }
  }
}
