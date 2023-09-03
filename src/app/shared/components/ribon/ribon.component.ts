import { Component, Input } from "@angular/core";

@Component({
  selector: "app-ribon",
  templateUrl: "./ribon.component.html",
  styleUrls: ["./ribon.component.scss"],
})
export class RibonComponent {
  @Input()
  itemTop!: RibonItem[];
  @Input()
  itemBottom: RibonItem[] = [];
}

export class RibonItem {
  constructor(
    public iconPath: string,
    public label: string,
    public onClick: () => void
  ) {}
}
