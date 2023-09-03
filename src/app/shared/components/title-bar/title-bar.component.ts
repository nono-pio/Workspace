import { Component } from "@angular/core";
import { appWindow } from "@tauri-apps/api/window";

@Component({
  selector: "app-title-bar",
  templateUrl: "./title-bar.component.html",
  styleUrls: ["./title-bar.component.scss"],
})
export class TitleBarComponent {
  buttonLeft: string[];
  textCenter = "Workspace";
  constructor() {
    this.buttonLeft = ["File", "Edit", "Selection", "View", "Go"];
  }

  quit() {
    console.log("quit");

    appWindow.close();
  }

  minimize() {
    console.log("min");
    appWindow.minimize();
  }

  async maximize() {
    console.log("max");
    if (await appWindow.isMaximized()) {
      appWindow.unmaximize();
    } else {
      appWindow.maximize();
    }
  }

  startDrag() {
    console.log("drag");
    appWindow.startDragging();
  }
}
