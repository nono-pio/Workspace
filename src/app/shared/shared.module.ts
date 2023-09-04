import { CommonModule } from "@angular/common";
import { NgModule } from "@angular/core";
import { ClickableIconComponent } from "./components/clickable-icon/clickable-icon.component";

@NgModule({
  declarations: [ClickableIconComponent],
  imports: [CommonModule],
  exports: [ClickableIconComponent],
})
export class SharedModule {}
