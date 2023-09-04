import { Component } from "@angular/core";

@Component({
  selector: "window-panel",
  templateUrl: "./panel.component.html",
  styleUrls: ["./panel.component.scss"],
})
export class PanelComponent {
  name: string = "Test";
  content: string = "Hello World!";
}
