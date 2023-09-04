import { CommonModule } from "@angular/common";
import { NgModule } from "@angular/core";
import { SharedModule } from "src/app/shared/shared.module";
import { PanelComponent } from "./components/panel/panel.component";
import { RibonComponent } from "./components/ribon/ribon.component";
import { SpliterComponent } from "./components/spliter/spliter.component";
import { StatusBarComponent } from "./components/status-bar/status-bar.component";
import { TitleBarComponent } from "./components/title-bar/title-bar.component";
import { WindowComponent } from "./window.component";

@NgModule({
  imports: [CommonModule, SharedModule],
  declarations: [
    WindowComponent,
    PanelComponent,
    SpliterComponent,
    TitleBarComponent,
    RibonComponent,
    StatusBarComponent,
  ],
  exports: [WindowComponent],
})
export class WindowModule {}
