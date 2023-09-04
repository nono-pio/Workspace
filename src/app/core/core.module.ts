import { NgModule } from "@angular/core";
import { CommonModule } from "@angular/common";
import { FsService } from "./services/fs.service";

@NgModule({
  providers: [FsService],
  imports: [CommonModule],
})
export class CoreModule {}
