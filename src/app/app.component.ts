import { Component, Injector, Input, OnInit } from "@angular/core";
import { FsService } from "./core/services/fs.service";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.scss"],
})
export class AppComponent implements OnInit {
  theme: "light" | "dark" = "dark";

  constructor(private fs: FsService) {}

  async ngOnInit() {
    let project = await this.fs.getDiskEntry(
      "C:\\Users\\nolan\\OneDrive\\Bureau\\Project\\logisim"
    );
    console.log(project);
  }
}
