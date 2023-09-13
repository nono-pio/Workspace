import { CommonModule } from "@angular/common";
import { NgModule } from "@angular/core";
import { EditorComponent } from "./components/editor/editor.component";

@NgModule({
  declarations: [EditorComponent],
  imports: [CommonModule],
  exports: [EditorComponent],
})
export class ViewModule {}
