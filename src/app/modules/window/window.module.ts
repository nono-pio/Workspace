import { CommonModule } from "@angular/common";
import { NgModule } from "@angular/core";
import { SharedModule } from "src/app/shared/shared.module";
import { ExplorerModule } from "../explorer/explorer.module";
import { PanelModule } from "../panel/panel.module";
import { ViewModule } from "../view/view.module";
import { RibonComponent } from "./components/ribon/ribon.component";
import { SettingsComponent } from "./components/settings/settings.component";
import { SpliterComponent } from "./components/spliter/spliter.component";
import { StatusBarComponent } from "./components/status-bar/status-bar.component";
import { TitleBarComponent } from "./components/title-bar/title-bar.component";
import { WindowComponent } from "./window.component";

@NgModule({
  imports: [
    CommonModule,
    SharedModule,
    PanelModule,
    ViewModule,
    ExplorerModule,
  ],
  declarations: [
    WindowComponent,
    SpliterComponent,
    TitleBarComponent,
    RibonComponent,
    StatusBarComponent,
    SettingsComponent,
  ],
  exports: [WindowComponent, SettingsComponent],
})
export class WindowModule {}
