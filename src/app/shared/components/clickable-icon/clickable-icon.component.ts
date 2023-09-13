import { Component, Input } from "@angular/core";

@Component({
  selector: "app-clickable-icon",
  templateUrl: "./clickable-icon.component.html",
  styleUrls: ["./clickable-icon.component.scss"],
})
export class ClickableIconComponent {
  @Input("src")
  iconPath!: string;
  @Input()
  click!: () => void;
  @Input()
  alt!: string;
}
