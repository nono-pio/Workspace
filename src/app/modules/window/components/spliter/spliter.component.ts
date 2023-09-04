import { Component } from "@angular/core";

@Component({
  selector: "window-spliter",
  templateUrl: "./spliter.component.html",
  styleUrls: ["./spliter.component.scss"],
})
export class SpliterComponent {
  spliterChilds: Spliter[];

  constructor() {
    this.spliterChilds = [new Spliter(0.5, null), new Spliter(0.5, null)];
  }
}

type SpliterChild = Spliter | View;
class Spliter {
  constructor(public size: number, public spliterChild: SpliterChild) {}
}

type View = null;
