import { Component } from "@angular/core";
import { appWindow } from "@tauri-apps/api/window";
import {
  Key,
  KeyModifiers,
  KeyboardShortcut,
  KeyboardShortcutsService,
} from "src/app/core/services/keyboard-shortcuts.service";
import { RibonItem } from "./components/ribon/ribon.component";

@Component({
  selector: "window",
  templateUrl: "./window.component.html",
  styleUrls: ["./window.component.scss"],
})
export class WindowComponent {
  constructor(private keyboardShortcuts: KeyboardShortcutsService) {
    SHORTCUTS.forEach((shortcut) => keyboardShortcuts.addShortcut(shortcut));
  }

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

const SHORTCUTS: KeyboardShortcut[] = [
  new KeyboardShortcut(
    "Quit",
    [new Key("q", [KeyModifiers.CommandOrControl])],
    "quit the window",
    () => appWindow.close()
  ),
];
