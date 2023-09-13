import { Injectable } from "@angular/core";
import { registerAll } from "@tauri-apps/api/globalShortcut";

@Injectable({
  providedIn: "root",
})
export class KeyboardShortcutsService {
  keyboardShortcuts: KeyboardShortcut[] = [];

  addShortcut(shortcut: KeyboardShortcut) {
    this.keyboardShortcuts.push(shortcut);
    console.log("Keys : " + shortcut.getKeysString());
    registerAll(shortcut.getKeysString(), shortcut.onTrigger);
  }
}

export class KeyboardShortcut {
  constructor(
    public name: string,
    public keys: Key[],
    public description: string,
    public onTrigger: () => void
  ) {}

  getKeysString(): string[] {
    return this.keys.map((key) => key.toString());
  }
}

export class Key {
  constructor(public key: string, public modifiers?: KeyModifiers[]) {}

  toString() {
    if (this.modifiers) {
      return this.modifiers.join("+") + "+" + this.key;
    } else {
      return this.key;
    }
  }
}

export enum KeyModifiers {
  CommandOrControl = "CommandOrControl",
  Shift = "Shift",
  Alt = "Alt",
}
