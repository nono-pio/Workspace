import { Component, OnInit } from "@angular/core";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/api/notification";
import { FsService } from "./core/services/fs.service";
import { RibonItem } from "./shared/components/ribon/ribon.component";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.scss"],
})
export class AppComponent implements OnInit {
  theme: "light" | "dark" = "dark";
  ribonLeftTop: RibonItem[] = [
    new RibonItem("assets\\angular.svg", "angular", () => {}),
    new RibonItem("assets\\angular.svg", "angular", () => {}),
    new RibonItem("assets\\angular.svg", "angular", () => {}),
  ];
  ribonLeftBottom: RibonItem[] = [
    new RibonItem("assets\\tauri.svg", "tauri", () => {}),
  ];

  ribonRightTop: RibonItem[] = [
    new RibonItem("assets\\angular.svg", "angular", () => {}),
    new RibonItem("assets\\angular.svg", "angular", () => {}),
    new RibonItem("assets\\angular.svg", "angular", () => {}),
  ];
  ribonRightBottom: RibonItem[] = [];

  constructor(private fs: FsService) {}

  async ngOnInit() {
    let project = await this.fs.getDiskEntry(
      "C:\\Users\\nolan\\OneDrive\\Bureau\\Project\\logisim"
    );
    console.log(project);

    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === "granted";
    }
    if (permissionGranted) {
      sendNotification("Tauri is awesome!");
      sendNotification({ title: "TAURI", body: "Tauri is awesome!" });
    }
  }
}
