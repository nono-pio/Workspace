import { Injectable } from "@angular/core";
import { invoke } from "@tauri-apps/api";

@Injectable({
  providedIn: "root",
})
export class FsService {
  constructor() {}

  async getDiskEntry(path: string): Promise<DiskEntry> {
    const diskEntry = await invoke<Object>("get_disk_entry", {
      path: path,
    });

    console.log(diskEntry);

    return this.getDiskEntryFromObject(diskEntry);
  }

  getDiskEntryFromObject(diskEntry: any): DiskEntry {
    // folder or file
    if ("File" in diskEntry) {
      return this.getFileFromObject(diskEntry.File);
    } else if ("Folder" in diskEntry) {
      return this.getFolderFromObject(diskEntry.Folder);
    } else {
      throw new Error("Error");
    }
  }

  getFileFromObject(file: any): File {
    return new File(file.name, file.extension, file.data);
  }

  getFolderFromObject(folder: any): Folder {
    return new Folder(
      folder.name,
      folder.disk_entries.map((diskEntry: any) =>
        this.getDiskEntryFromObject(diskEntry)
      )
    );
  }
}

type DiskEntry = File | Folder;

class File {
  constructor(
    public name: string | null,
    public extension: string | null,
    public data: string | null
  ) {}
}

class Folder {
  constructor(public name: string | null, public diskEntries: DiskEntry[]) {}
}
