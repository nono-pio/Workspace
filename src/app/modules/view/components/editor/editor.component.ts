import { Component, Input, OnInit } from "@angular/core";

@Component({
  selector: "view-editor",
  templateUrl: "./editor.component.html",
  styleUrls: ["./editor.component.scss"],
})
export class EditorComponent implements OnInit {
  @Input()
  text: string = "";

  lines: string[] = [""];

  ngOnInit(): void {
    this.lines = this.text.split("\n");
  }

  change(e: KeyboardEvent) {
    console.log(e);
    this.ngOnInit();
  }
}
