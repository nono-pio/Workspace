import { Component } from "@angular/core";

@Component({
  selector: "panel-file",
  templateUrl: "./panel-file.component.html",
  styleUrls: ["./panel-file.component.scss"],
})
export class PanelFileComponent {
  text = "Some text\nMulti line";
}
