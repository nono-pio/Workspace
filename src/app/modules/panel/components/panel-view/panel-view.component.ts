import { Component, Input, OnInit } from "@angular/core";
import { View } from "src/app/modules/view/interfaces/View";
import { Panel } from "../../interfaces/Panel";

@Component({
  selector: "panel-view",
  templateUrl: "./panel-view.component.html",
  styleUrls: ["./panel-view.component.scss"],
})
export class PanelViewComponent implements OnInit {
  @Input("panel")
  view!: PanelView;

  ngOnInit(): void {
    console.log(this.view);
  }
}

export class PanelView implements Panel {
  constructor(public view: View) {}
  getPanelComponent() {
    return PanelViewComponent;
  }
}
