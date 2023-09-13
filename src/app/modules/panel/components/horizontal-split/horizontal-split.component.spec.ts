import { ComponentFixture, TestBed } from "@angular/core/testing";

import { HorizontalSplitComponent } from "./horizontal-split.component";

describe("HorizontalSplitComponent", () => {
  let component: HorizontalSplitComponent;
  let fixture: ComponentFixture<HorizontalSplitComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [HorizontalSplitComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(HorizontalSplitComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it("should create", () => {
    expect(component).toBeTruthy();
  });
});
