import { Component } from "@angular/core";

@Component({
  selector: "window-status-bar",
  templateUrl: "./status-bar.component.html",
  styleUrls: ["./status-bar.component.scss"],
})
export class StatusBarComponent {
  status = "great";
}
