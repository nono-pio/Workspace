import { Component } from "@angular/core";
import { RibonItem } from "./components/ribon/ribon.component";

@Component({
  selector: "window",
  templateUrl: "./window.component.html",
  styleUrls: ["./window.component.scss"],
})
export class WindowComponent {
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
}
