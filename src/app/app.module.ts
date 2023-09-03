import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";

import { AppComponent } from "./app.component";
import { PanelComponent } from './shared/components/panel/panel.component';
import { SpliterComponent } from './shared/components/spliter/spliter.component';
import { TitleBarComponent } from './shared/components/title-bar/title-bar.component';
import { RibonComponent } from './shared/components/ribon/ribon.component';
import { ClickableIconComponent } from './shared/components/clickable-icon/clickable-icon.component';
import { StatusBarComponent } from './shared/components/status-bar/status-bar.component';

@NgModule({
  declarations: [AppComponent, PanelComponent, SpliterComponent, TitleBarComponent, RibonComponent, ClickableIconComponent, StatusBarComponent],
  imports: [BrowserModule],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
