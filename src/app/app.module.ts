import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";

import { AppComponent } from "./app.component";
import { CoreModule } from "./core/core.module";
import { WindowModule } from "./modules/window/window.module";

@NgModule({
  declarations: [AppComponent],
  imports: [BrowserModule, WindowModule, CoreModule],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
