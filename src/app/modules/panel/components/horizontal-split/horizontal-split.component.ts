import { Component, Input } from "@angular/core";
import { Panel } from "../../interfaces/Panel";

@Component({
  selector: "panel-horizontale-split",
  templateUrl: "./horizontal-split.component.html",
  styleUrls: ["./horizontal-split.component.scss"],
})
export class HorizontalSplitComponent {
  @Input("panel")
  horizontalSplit!: HorizontalSplit;
}

class HorizontalSplit implements Panel {
  constructor(public childs: Panel[], public sizes: number[]) {}

  getPanelComponent() {
    return HorizontalSplitComponent;
  }
}
