import { Component, Input, OnInit } from "@angular/core";
import { DiskEntry, File, FsService } from "src/app/core/services/fs.service";

@Component({
  selector: "view-explorer",
  templateUrl: "./explorer-view.component.html",
  styleUrls: ["./explorer-view.component.scss"],
})
export class ExplorerViewComponent implements OnInit {
  @Input()
  source!: string;
  diskEntry: DiskEntry | undefined;

  constructor(private fs: FsService) {}

  async ngOnInit() {
    this.diskEntry = await this.fs.getDiskEntry(this.source);
    console.log(this.diskEntry);
  }

  isFile(diskEntry: DiskEntry) {
    return diskEntry instanceof File;
  }
}
