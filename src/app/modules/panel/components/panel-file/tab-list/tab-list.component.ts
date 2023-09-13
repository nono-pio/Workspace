import { Component, Input } from "@angular/core";

@Component({
  selector: "tab-list",
  templateUrl: "./tab-list.component.html",
  styleUrls: ["./tab-list.component.scss"],
})
export class TablistComponent {
  @Input()
  files: string[] = ["test1", "test2"];

  closeFile(index: number) {
    return () => {
      this.files.splice(index, 1);
    };
  }
}
