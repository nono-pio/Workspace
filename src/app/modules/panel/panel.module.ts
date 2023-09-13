import { CommonModule } from "@angular/common";
import { NgModule } from "@angular/core";
import { SharedModule } from "src/app/shared/shared.module";
import { ViewModule } from "../view/view.module";
import { HorizontalSplitComponent } from "./components/horizontal-split/horizontal-split.component";
import { PanelFileComponent } from "./components/panel-file/panel-file.component";
import { TablistComponent } from "./components/panel-file/tab-list/tab-list.component";
import { PanelViewComponent } from "./components/panel-view/panel-view.component";
import { VerticalSplitComponent } from "./components/vertical-split/vertical-split.component";

@NgModule({
  declarations: [
    PanelViewComponent,
    HorizontalSplitComponent,
    VerticalSplitComponent,
    PanelFileComponent,
    TablistComponent,
  ],
  imports: [CommonModule, SharedModule, ViewModule],
  exports: [
    PanelViewComponent,
    PanelFileComponent,
    HorizontalSplitComponent,
    VerticalSplitComponent,
  ],
})
export class PanelModule {}
