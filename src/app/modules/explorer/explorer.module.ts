import { CommonModule } from "@angular/common";
import { NgModule } from "@angular/core";
import { ExplorerViewComponent } from "./explorer-view/explorer-view.component";

@NgModule({
  declarations: [ExplorerViewComponent],
  imports: [CommonModule],
  exports: [ExplorerViewComponent],
})
export class ExplorerModule {}
